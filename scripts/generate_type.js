#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const ROOT = path.join(__dirname, '..');
const DST_DIR_NAME = 'vk';
const DST_DIR_PATH = path.join(ROOT, 'src', DST_DIR_NAME);
const VULKAN_SDK_PATH = process.env.VULKAN_SDK;
const VULKAN_H = fs.readFileSync(path.join(__dirname, `vulkan.h`), 'utf8');
const TYPES_TO_GENERATE = process.argv.slice(2);

const PRIMITIVE_TYPE = {
    uint32_t: 'u32',
    uint16_t: 'u16',
    uint8_t: 'u8',
    int32_t: 'i32',
    int16_t: 'i16',
    int8_t: 'i8',
    char: 'u8',
    float: 'f32',
    double: 'f64',
    size_t: 'usize',
    VkBool32: 'u32',
    VkDeviceSize: 'u64',
    VkSampleCountFlags: 'u32'
};

generateTypes(TYPES_TO_GENERATE);

function generateTypes(types) {
    types.forEach(generateType);
}

function generateType(name) {
    if (!generateStruct(name) && !generateEnum(name) && !generateBitFlags(name)) {
        throw new Error(`cannot find type ${name}`);
    }
}

function writeVkType(name, blocks) {
    const moduleName = cToRustVarName(name);
    const filePath = path.join(DST_DIR_PATH, `${moduleName}.rs`);
    const fileContent = blocks.join('\n\n');

    fs.writeFileSync(filePath, fileContent, 'utf8');
}

function getPrimitiveType(type) {
    return PRIMITIVE_TYPE[type];
}

function generateStruct(name) {
    const match = VULKAN_H.match(new RegExp(`typedef struct ${name} {\n([^}]+)\n}`, 'm'));

    if (!match) return false;

    const rawTypeName = toRawTypeName(name);
    const trueTypeName = toTrueTypeName(name);

    const usedTypes = new Set();
    const rawDefLines = [];
    const trueDefLines = [];
    const fromRawToTrueLines = [];
    const fromTrueToRawLines = [];

    usedTypes.add('std::convert::From');

    match[1].split('\n').forEach(line => {
        let [type, name] = line.split(' ').filter(x => x);
        let rustPrimitiveType = getPrimitiveType(type);
        let isPrimitiveType = !!rustPrimitiveType;
        let rawRustType = isPrimitiveType ? rustPrimitiveType : toRawTypeName(type);
        let trueRustType = isPrimitiveType ? rustPrimitiveType : toTrueTypeName(type);
        let isArray = name.includes('[');
        let isString = false;

        if (type === 'VkBool32') {
            trueRustType = 'bool';
        }

        name = name.substring(0, name.length - 1);

        if (isArray) {
            isString = type === 'char';

            const start = name.indexOf('[');
            const end = name.indexOf(']');
            const constantName = name.substring(start + 1, end);
            const constantValue = isNaN(+constantName) ? findConstant(constantName) : constantName;

            name = name.substring(0, start);
            rawRustType = `[${rawRustType}; ${constantValue}]`;

            if (isString) {
                trueRustType = 'String';
            } else {
                trueRustType = `[${trueRustType}; ${constantValue}]`;
            }
        }

        let rustName = cToRustVarName(name);

        rawDefLines.push(`${rustName}: ${rawRustType}`);
        trueDefLines.push(`pub ${rustName}: ${trueRustType}`);

        const sourceField = `value.${rustName}`;
        let rawToTrueFieldConversion;

        if (isString) {
            usedTypes.add('std::string::String');
            usedTypes.add('std::ffi::CStr');

            rawToTrueFieldConversion = `unsafe { String::from_utf8_unchecked((&value.device_name).to_vec().into_iter().filter(|x| *x != 0).collect()) }`
        } else if (rustPrimitiveType) {
            if (type === 'VkBool32') {
                rawToTrueFieldConversion = `${sourceField} != 0`
            } else {
                rawToTrueFieldConversion = sourceField;
            }
        } else {
            usedTypes.add(`${DST_DIR_NAME}::${cToRustVarName(trueRustType)}::*`);

            rawToTrueFieldConversion = `${trueRustType}::from(&${sourceField})`;
        }

        fromRawToTrueLines.push(`${rustName}: ${rawToTrueFieldConversion}`);
    });

    const useDelaractions = Array.from(usedTypes.values()).map(str => `use ${str};`).join('\n');

    const rawDefinition = [
        `#[repr(C)]`,
        `pub struct ${rawTypeName} {`,
        rawDefLines.map(line => `    ${line}`).join(',\n'),
        `}`
    ].join('\n');

    const trueDefinition = [
        `#[derive(Debug)]`,
        `pub struct ${trueTypeName} {`,
        trueDefLines.map(line => `    ${line}`).join(',\n'),
        `}`
    ].join('\n');

    const fromDefinition = [
        `impl<'a> From<&'a ${rawTypeName}> for ${trueTypeName} {`,
        `    fn from(value: &'a ${rawTypeName}) -> Self {`,
        `        ${trueTypeName} {`,
        fromRawToTrueLines.map(line => `            ${line}`).join(',\n'),
        `        }`,
        `    }`,
        `}`
    ].join('\n');

    writeVkType(name, [useDelaractions, rawDefinition, trueDefinition, fromDefinition]);

    return true;
}

function generateEnum(name) {
    const match = VULKAN_H.match(new RegExp(`typedef enum ${name}\\s+{\n([^}]+)\n}`, 'm'));

    if (!match) return false;

    const rawTypeName = toRawTypeName(name);
    const rawDefinition = `pub type ${rawTypeName} = i32;`;
    const trueTypeName = toTrueTypeName(name);
    let enumPrefix = name.replace(/[a-z]+/g, str => `${str.toUpperCase()}_`);

    if (name === 'VkResult') {
        enumPrefix = 'VK_';
    }

    const trueDefFields = [];
    const fromLines = [];
    const formatLines = [];

    match[1].split('\n').forEach(line => {
        const match = line.match(/^\s*([A-Z_]+)\s*=\s*(-?\d+),?$/);

        if (!match) return;

        const valueName = match[1];
        const valueInt = match[2];

        if (!valueName.startsWith(enumPrefix)) {
            throw new Error(`enum value ${valueName} does not start with prefix ${enumPrefix}`);
        }

        const rustValue = cToRustEnumValue(valueName.substring(enumPrefix.length));

        trueDefFields.push(`${rustValue} = ${valueInt}`);
        fromLines.push(`${valueInt} => ${trueTypeName}::${rustValue}`);
        formatLines.push(`${trueTypeName}::${rustValue} => write!(f, "${rustValue}")`);
    });

    const useDelaractions = [
        'std::convert::From',
    ].map(l => `use ${l};`).join('\n');

    const trueDefinition = [
        `#[repr(i32)]`,
        `#[derive(Debug, PartialEq, Copy, Clone)]`,
        `pub enum ${trueTypeName} {`,
        trueDefFields.map(l => `    ${l}`).join(',\n'),
        `}`
    ].join('\n');

    const fromRawToTrueDefinition = [
        `impl<'a> From<&'a i32> for ${trueTypeName} {`,
        `    fn from(value: &'a i32) -> Self {`,
        `        unsafe { *((value as *const i32) as *const ${trueTypeName}) }`,
        `    }`,
        `}`
    ].join('\n');

    const fromTrueToRawDefinition = [
        `impl<'a> From<&'a ${trueTypeName}> for i32 {`,
        `    fn from(value: &'a ${trueTypeName}) -> Self {`,
        `        *value as i32`,
        `    }`,
        `}`
    ].join('\n');

    writeVkType(name, [useDelaractions, rawDefinition, trueDefinition, fromRawToTrueDefinition, fromTrueToRawDefinition]);

    return true;
}

function generateBitFlags(name) {
    const bitsFlagTypeName = name.substring(0, name.length - 1) + 'Bits';
    const match = VULKAN_H.match(new RegExp(`typedef enum ${bitsFlagTypeName}\\s+{\n([^}]+)\n}`, 'm'));

    if (!match) return false;

    const rawTypeName = toRawTypeName(name);
    const trueTypeName = toTrueTypeName(name);

    const capitalized = capitalizeVarName(name);
    const prefix = capitalized.substring(0, capitalized.indexOf('FLAGS'));
    const suffix = '_BIT';

    const fields = match[1].split('\n').map(line => {
        const match = line.match(/^\s*([A-Z_]+)\s*=\s*(0x[\dA-F]{8}),?\s*$/);

        if (!match) {
            throw new Error(`for enum ${name}: unexpected field "${line}"`);
        }

        return {
            name: match[1],
            value: match[2]
        };
    }).filter(({value}) => value !== '0x7FFFFFFF');

    if (name === 'VkQueueFlags') {
        fields.push({ name: 'VK_QUEUE_PROTECTED_BIT', value: '0x00000010'});
    }

    fields.forEach(field => field.rustName = cToRustEnumValue(strip(field.name, prefix, suffix)));

    const useDelaractions = [
        'std::convert::From',
    ].map(l => `use ${l};`).join('\n');

    const rawDefinition = `pub type ${rawTypeName} = u32;`;

    const trueDefinition = [
        `pub struct ${trueTypeName} {`,
        fields.map(field => `    pub ${field.rustName}`).join(',\n'),
        `}`
    ].join('\n');

    const fromRawToTrueDefinition = [
        `impl<'a> From<&'a u32> for ${trueTypeName} {`,
        `    fn from(value: &'a u32) -> Self {`,
        `        ${trueTypeName} {`,
        fields.map(field => `            ${field.rustName}: (value | ${field.value}) > 0`).join(',\n'),
        `        }`,
        `    }`,
        `}`
    ].join('\n');

    writeVkType(name, [useDelaractions, rawDefinition, trueDefinition, fromRawToTrueDefinition]);

    return true;
}

function strip(str, prefix, suffix) {
    return str.substring(prefix.length, str.indexOf(suffix));
}

function findConstant(name) {
    const match = VULKAN_H.match(new RegExp(`#define\\s+${name}\\s+([0-9.]+)`));

    if (!match) {
        throw new Error(`cannot find constant ${name}`);
    }

    return match[1];
}

function cToRustVarName(name) {    
    return name
        .replace(/[A-Z]+/g, str => `_${str.toLowerCase()}`)
        .replace(/__/g, '_')
        .replace(/^_/, '')
        .replace(/(\d)_d$/, '_$1d');
}

function capitalizeVarName(name) {
    return name.replace(/[A-Z]/g, '_$&').toUpperCase().substring(1);
}

function cToRustEnumValue(name) {
    return `_${name}`.toLowerCase().replace(/_[a-z]/g, str => str.charAt(1).toUpperCase());
}

function toRawTypeName(name) {
    return `Raw${name}`;
}

function toTrueTypeName(name) {
    return name;
}