#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const { parseEnums, parseStructs, parseFunctions, isHandle } = require('./parse_vulkan_h');
const { blockToString, toSnakeCase, toPascalCase, getRawTypeName, getWrappedTypeName } = require('./utils');
const { HandleList } = require('./handles');
const { generateVkStructDefinition } = require('./structs');
const { generateVkEnumDefinition } = require('./enums');

const ROOT_DIR_PATH         = path.join(__dirname, '..');
const OUTPUT_DIR_NAME       = 'vk';
const OUTPUT_DIR_PATH       = path.join(ROOT_DIR_PATH, 'src', OUTPUT_DIR_NAME);
const STATIC_FILES_DIR_PATH = path.join(__dirname, 'static');

const GENERATED_HEADER = '// Generated by `scripts/generate_vk.js`\n\n';
const COPIED_HEADER = '// Copied from `scripts/static/`\n\n'

main();

function main() {
    const vkTypes = [
        ...generateEnums()
    ];

    writeVkTypes(vkTypes);
    copyStaticFiles();
    writeModFile(OUTPUT_DIR_PATH);
}

function copyStaticFiles() {
    const fileNames = fs.readdirSync(STATIC_FILES_DIR_PATH);

    fileNames.forEach(fileName => {
        const fileContent = fs.readFileSync(path.join(STATIC_FILES_DIR_PATH, fileName), 'utf8');
        const targetFilePath = path.join(OUTPUT_DIR_PATH, fileName);
        
        if (!fileContent.startsWith('// no-copy')) {
            fs.writeFileSync(targetFilePath, COPIED_HEADER + fileContent, 'utf8');
        }
    });
}

function writeVkTypes(types) {
    for (let type of types) {
        const { name, extension, definition } = type;

        const dirPath = extension ? path.join(OUTPUT_DIR_PATH, extension) : OUTPUT_DIR_PATH;
        const fileName = toSnakeCase(name) + '.rs';
        const filePath = path.join(dirPath, fileName);
        const fileContent = GENERATED_HEADER + definition.map(blockToString).join('\n\n');

        mkdir(dirPath);
        // console.log(fileContent);
        fs.writeFileSync(filePath, fileContent, 'utf8');
    }
}

function mkdir(dirPath) {
    if (!fs.existsSync(dirPath)) {
        fs.mkdirSync(dirPath);
    }
}

function writeModFile(dirPath) {
    const filePath = path.join(dirPath, 'mod.rs');
    const files = [];
    const directories = [];

    fs.readdirSync(dirPath).forEach(name => {
        if (name !== 'mod.rs') {
            const stats = fs.statSync(path.join(dirPath, name));

            if (stats.isFile()) {
                files.push(name);
            } else if (stats.isDirectory()) {
                directories.push(name);
            }
        }
    });

    const moduleNames = files.map(name => name.replace('.rs', ''));
    const content = [
        directories.map(name => `pub mod ${name};`),
        moduleNames.map(name => `mod ${name};`),
        moduleNames.map(name => `pub use self::${name}::*;`),
    ].map(list => list.join('\n')).filter(x => x).join('\n\n');

    fs.writeFileSync(filePath, GENERATED_HEADER + content);

    directories.forEach(dirName => writeModFile(path.join(dirPath, dirName)));
}

function generateVkTypes(parseFunction, generateFunction) {
    const cTypes = parseFunction();

    return cTypes.map(cDef => {
        const rustDefinition = generateFunction(cDef);

        return {
            name: cDef.name,
            extension: cDef.extension,
            definition: rustDefinition
        };
    });
}

function generateStructs() {
    return generateVkTypes(parseStructs, generateVkStructDefinition);
}

function generateEnums() {
    return generateVkTypes(parseEnums, generateVkEnumDefinition);
}

function generateHandles() {
    const cFunctions = parseFunctions();
    const handles = new HandleList();

    const destroyFunctions = cFunctions.filter(func => func.name.includes('Destroy'));
    const otherFunctions = cFunctions.filter(func => !func.name.includes('Destroy'));

    destroyFunctions.forEach(func => {
        const handleName = func.args.beforeLast().typeName;
        const hasParent = func.args.length > 2;
        const parent = hasParent ? func.args[0].typeName : 'VkInstance';

        handles.get(parent).addHandleToDestroy(handleName, func);

        if (hasParent) {
            handles.get(handleName).setParent(parent);
        }
    });

    otherFunctions.forEach(func => {
        const firstArg = func.args[0];
        const isFirstArgHandle = !firstArg.isPointer && isHandle(firstArg.typeName);
        const handleName = isFirstArgHandle ? firstArg.typeName : 'VkInstance';

        handles.get(handleName).addMethod(func);
    });

    console.log(handles.get('VkInstance').toString());

    return handles;
}