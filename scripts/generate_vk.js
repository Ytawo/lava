#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const { getAllEnums, getAllBitFlags, getAllStructs, getAllHandles, getAllFunctions, isHandle } = require('./vulkan_src');
const { blockToString, toSnakeCase, toPascalCase, getRawTypeName, getWrappedTypeName } = require('./utils');
const { HandleList } = require('./handles');
const { generateVkStructDefinition } = require('./structs');
const { generateVkEnumDefinition } = require('./enums');
const { generateVkBitFlagsDefinition } = require('./bit_flags');
const { generateVkHandleDefinition } = require('./handles');
const { generateFunctionTableDefinition } = require('./function_table');
const { generateRootTypeDefinition } = require('./root_type');

const ROOT_DIR_PATH         = path.join(__dirname, '..');
const OUTPUT_DIR_PATH       = path.join(ROOT_DIR_PATH, 'src', 'vk');
const STATIC_FILES_DIR_PATH = path.join(__dirname, 'static');

const GENERATED_HEADER = '// Generated by `scripts/generate_vk.js`\n\n';
const COPIED_HEADER = '// Copied from `scripts/static/`\n\n';

const STATIC_VK_FUNCTIONS = [
    'vkEnumerateInstanceVersion',
    'vkEnumerateInstanceExtensionProperties',
    'vkEnumerateInstanceLayerProperties',
    'vkCreateInstance'
];

main();

function main() {
    const vkTypes = [
        generateRootType(),
        generateFunctionTable(),
        // ...generateEnums(),
        // ...generateBitFlags(),
        ...generateStructs(),
        ...generateHandles()
    ];

    writeVkTypes(vkTypes);
    copyStaticFiles(STATIC_FILES_DIR_PATH, OUTPUT_DIR_PATH);
    writeModFile(OUTPUT_DIR_PATH);
}

function copyStaticFiles(srcDirPath, dstDirPath) {
    const fileNames = fs.readdirSync(srcDirPath);

    mkdir(dstDirPath);

    fileNames.forEach(fileName => {
        const sourcePath = path.join(srcDirPath, fileName);
        const targetPath = path.join(dstDirPath, fileName);
        const stats = fs.statSync(sourcePath);

        if (stats.isFile()) {
            const fileContent = fs.readFileSync(sourcePath, 'utf8');

            if (!fileContent.startsWith('// no-copy')) {
                fs.writeFileSync(targetPath, COPIED_HEADER + fileContent, 'utf8');
            }
        } else if (stats.isDirectory()) {
            copyStaticFiles(sourcePath, targetPath);
        }
    
    });
}

function writeVkTypes(types) {
    for (let type of types) {
        const { name, extension, definition } = type;

        const dirPath = extension ? path.join(OUTPUT_DIR_PATH, extension) : OUTPUT_DIR_PATH;
        const fileName = toSnakeCase(name) + '.rs';
        const filePath = path.join(dirPath, fileName);
        const fileContent = GENERATED_HEADER + definition.filter(x => x).map(blockToString).join('\n\n');

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
        moduleNames.map(name => `pub mod ${name};`),
        moduleNames.map(name => `pub use self::${name}::*;`),
    ].map(list => list.join('\n')).filter(x => x).join('\n\n');

    fs.writeFileSync(filePath, GENERATED_HEADER + content);

    directories.forEach(dirName => writeModFile(path.join(dirPath, dirName)));
}

function generateVkTypes(cTypes, generateFunction) {
    return cTypes.map(cDef => {
        const rustDefinition = generateFunction(cDef);

        return {
            name: cDef.name,
            extension: cDef.extension,
            definition: rustDefinition
        };
    });
}

function generateRootType() {
    return {
        name: 'Vk',
        extension: '',
        definition: generateRootTypeDefinition(getAllFunctions().filter(func => STATIC_VK_FUNCTIONS.includes(func.name)))
    };
}

function generateFunctionTable() {
    return {
        name: 'VkInstanceFunctionTable',
        extension: '',
        definition: generateFunctionTableDefinition(getAllFunctions().filter(func => !STATIC_VK_FUNCTIONS.includes(func.name)))
    };
}

function generateEnums() {
    return generateVkTypes(getAllEnums(), generateVkEnumDefinition);
}

function generateBitFlags() {
    return generateVkTypes(getAllBitFlags(), generateVkBitFlagsDefinition);
}

function generateStructs() {
    const structs = getAllStructs()
        .filter(struct => struct.fields.every(field => !field.fullType.includes('PFN')));

    return generateVkTypes(structs, generateVkStructDefinition);
}

function generateHandles() {
    const handles = getAllHandles();
    const functions = getAllFunctions().filter(func => !STATIC_VK_FUNCTIONS.includes(func.name));

    for (let handle of handles) {
        handle.functions = [];
    }

    const destroyFunctions = functions.filter(func => func.name.includes('Destroy'));

    for (let destroyFunction of destroyFunctions) {
        const parentArg = destroyFunction.args.first();
        const destroyedArg = destroyFunction.args.beforeLast();

        if (parentArg !== destroyedArg) {
            const parent = { name: parentArg.typeName, extension: parentArg.extension };

            handles.find(handle => handle.name === destroyedArg.typeName).parent = parent;
        }
    }

    const vkInstance = handles.find(h => h.name === 'VkInstance');

    for (let func of functions) {
        const firstArgType = func.args[0].typeName;
        const secondArgType = func.args[1] && func.args[1].typeName;

        let handle = handles.find(handle => handle.parent && firstArgType === handle.parent.name && secondArgType === handle.name);

        if (!handle) {
            handle = handles.find(handle => firstArgType === handle.name);
        }

        if (!handle) {
            handle = vkInstance;
        }

        handle.functions.push(func);
    }

    return generateVkTypes(handles, generateVkHandleDefinition);
}

function _generateHandles() {
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