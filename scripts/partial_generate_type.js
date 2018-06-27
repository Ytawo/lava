#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const {
    cToRustVarName,
    capitalizeVarName,
    cToRustEnumValue,
    toRawTypeName,
    toTrueTypeName
} = require('./utils');

const ARGV = process.argv.slice(2);

const {
    PRIMITIVE_TYPE,
    VULKAN_H,
    DST_DIR_PATH
} = require('./constants');

const FORCE = ARGV.includes('-f') || ARGV.includes('--force');

main(ARGV);

function main(argv) {
    argv.filter(name => !name.startsWith('-')).forEach(partialGenerateStruct);
}

function writeTemplate(name, blocks) {
    blocks.unshift('// Template generated by `scripts/generate_type.js`');

    const moduleName = cToRustVarName(name);
    const filePath = path.join(DST_DIR_PATH, `${moduleName}.rs`);
    const fileContent = blocks.map(b => Array.isArray(b) ? b.join('\n') : b).join('\n\n');

    if (!FORCE && fs.existsSync(filePath)) {
        throw new Error(`error: file "${filePath}" already exists (-f to ignore)`);
    }

    fs.writeFileSync(filePath, fileContent, 'utf8');
}

function partialGenerateStruct(name) {
    const match = VULKAN_H.match(new RegExp(`typedef struct ${name} {\n([^}]+)\n}`, 'm'));

    if (!match) {
        throw new Error(`cannot find struct ${name}`);
    }

    const rawTypeName = toRawTypeName(name);
    const trueTypeName = toTrueTypeName(name);

    const fields = match[1].split('\n').map(line => {
        const match = line.match(/\s*([\w* ]+)\s+(\w+);\s*$/);

        if (!match) {
            throw new Error(`unexpected line for struct ${name}: "${line}"`);
        }

        const rustName = cToRustVarName(match[2]);
        const rustType = match[1].trim()
            .replace(/const /g, '')
            .replace(/\w+/g, str => PRIMITIVE_TYPE[str] || `Raw${str}`)
            .replace(/(\w+)\*/g, (_, type) => `*${rustName === 'p_next' ? 'const' : 'mut'} ${type}`)

        const field = {
            type: rustType,
            name: rustName
        };

        return field;
    });

    const usages = [
        'std::default::Default',
        'std::convert::From',
        'std::ops::Drop',
        'std::ptr',
        'libc::*',
        'vk::*'
    ].map(str => `use ${str};`);

    const rawDefinition = [
        `#[repr(C)]`,
        `pub struct ${rawTypeName} {`,
        ...fields.map(({type, name}) => `    ${name}: ${type},`),
        `}`
    ];

    const trueDefinition = [
        `pub struct ${trueTypeName} {`,
        `}`
    ];

    const defaultImpl = [
        `impl Default for ${trueTypeName} {`,
        `    fn default() -> Self {`,
        `        ${trueTypeName} {`,
        `        }`,
        `    }`,
        `}`
    ];

    const rawFromTrueImpl = [
        `impl<'a> From<&'a ${trueTypeName}> for ${rawTypeName} {`,
        `    fn from(v: &'a ${trueTypeName}) -> Self {`,
        `        unsafe {`,
        `            ${rawTypeName} {`,
        ...fields.map(({type, name}) => `                ${name}: ${typeToDefaultValue(type, trueTypeName)},`),
        `            }`,
        `        }`,
        `    }`,
        `}`
    ];

    const dropImpl = [
        `impl Drop for ${rawTypeName} {`,
        `    fn drop(&mut self) {`,
        `        unsafe {`,
        ...fields.filter(({type}) => type.includes('*mut')).map(({name, type}) => `            ${typeToFreeFunction(type, name)}(self.${name});`),
        `        }`,
        `    }`,
        `}`
    ];

    writeTemplate(name, [usages, rawDefinition, trueDefinition, defaultImpl, rawFromTrueImpl, dropImpl]);
}

function typeToDefaultValue(type, structName) {
    if (type === 'VkStructureType') {
        return `VkStructureType::${structName.substr(2)}`;
    } else if (type.includes('*const')) {
        return 'ptr::null()';
    } else if (type.includes('*mut')) {
        return 'ptr::null_mut()';
    } else {
        return '0';
    }
}

function typeToFreeFunction(type, name) {
    if (type === '*mut *mut i8') {
        return 'free_c_string_array';
    } else if (type === '*mut i8') {
        return 'free_c_string';
    } else if (name.endsWith('s')) {
        return 'free_c_array';
    } else {
        return 'free_c_ptr';
    }
}