#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const VULKAN_SDK_PATH = process.env.VULKAN_SDK || "C:\\VulkanSDK\\1.1.77.0";
const INCLUDE_DIR_NAME = process.platform === 'win32' ? 'Include' : 'include';
const VULKAN_H = fs.readFileSync(path.join(VULKAN_SDK_PATH, INCLUDE_DIR_NAME, `vulkan`, `vulkan_core.h`), 'utf8');

const SPECIAL_FUNCTIONS = [
    'VkResult glfwCreateWindowSurface(VkInstance instance, GLFWwindow* window, const VkAllocationCallbacks* allocator, VkSurfaceKHR* surface);',
    'VkResult vkCreateDebugReportCallbackEXT(VkInstance instance, const VkDebugReportCallbackCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugReportCallbackEXT* pCallback);',
    'void vkDestroyDebugReportCallbackEXT(VkInstance instance, VkDebugReportCallbackEXT callback, const VkAllocationCallbacks* pAllocator);'
].join('\n');

const EXTENSIONS = ['KHR', 'EXT', 'GOOGLE', 'NV', 'NVX', 'AMD'];

const ENUMS = parseEnums();
const BIT_FLAGS = parseBitFlags();
const STRUCTS = parseStructs();
const HANDLES = parseHandles();

const structs = getAllStructs().filter(struct => struct.fields.some(field => field.fullType.includes('char')));
console.log(structs.map(s => s.name));

function getByName(obj, typeName) {
    const { name, extension } = parseName(typeName);

    return obj[extension][name];
}

function getAll(obj) {
    return Object.values(obj).reduce((acc, subObj) => acc.concat(Object.values(subObj)), []);
}

function getAllEnums() { return getAll(ENUMS); }
function getAllBitFlags() { return getAll(BIT_FLAGS); }
function getAllStructs() { return getAll(STRUCTS); }
function getAllHandles() { return getAll(HANDLES); }

function getEnumByName(name) { return getByName(ENUMS, name); }
function getBitFlagsByName(name) { return getByName(BIT_FLAGS, name); }
function getStructByName(name) { return getByName(STRUCTS, name); }
function getHandleByName(name) { return getByName(HANDLES, name); }

function isHandle(name) { return !!getByName(HANDLES, name); }

function parseName(str) {
    let extension = '';

    for (let ext of EXTENSIONS) {
        if (str.endsWith(ext)) {
            extension = ext.toLowerCase();
        }
    }

    return {
        extension: extension,
        name: str.substring(0, str.length - extension.length)
    };
}

function listToObj(array) {
    const types = {};

    array.forEach(type => {
        const byExtension = types[type.extension] || (types[type.extension] = {});
        byExtension[type.name] = type;
    });

    return types;
}

function parseStructs() {
    const regexp = /typedef struct \w+ {\n([^}]+)\n}/gmi;
    const match = VULKAN_H.match(regexp);

    const structs = match.map(str => {
        const structName = str.split(' ', 3)[2];
        const structNameInfo = parseName(structName);
        const name = structNameInfo.name;
        const extension = structNameInfo.extension;
        const fieldsStr = str.substring(str.indexOf('{') + 2, str.indexOf('}') - 1);

        const fields = fieldsStr.split('\n').filter(x => x).map(line => {
            const match = line.match(/\s*([\w* ]+)\s+(\w+)(?:\[(\w+)\])?;\s*$/);

            if (!match) {
                throw new Error(`unexpected line for struct ${structName}: "${line}"`);
            }

            // const name = match[2].trim().replace('BitFlags', 'Flags');
            const name = match[2].trim();
            const fullType = match[1].trim();
            const fieldName = fullType.replace(/(?:const )?(\w+)\*?/, '$1');
            const fieldTypeNameInfo = parseName(fieldName);
            const typeName = fieldTypeNameInfo.name;
            const extension = fieldTypeNameInfo.extension;
            const isPointer = fullType.endsWith('*');
            const isConst = fullType.startsWith('const ');
            const arraySize = parseConstant(match[3]);

            return { name, extension, fullType, typeName, isPointer, isConst, arraySize };
        });

        return { name, extension, fields };
    });

    return listToObj(structs);
}

function parseEnums() {
    const regexp = /typedef enum \w+ {\n([^}]+)\n}/gmi;
    const match = VULKAN_H.match(regexp);

    const enums = match.map(str => {
        const structName = str.split(' ', 3)[2];
        const structNameInfo = parseName(structName);
        const name = structNameInfo.name;
        const extension = structNameInfo.extension;
        const fieldsStr = str.substring(str.indexOf('{') + 2, str.indexOf('}') - 1);

        if (name.endsWith('FlagBits')) {
            return null;
        }

        const fields = fieldsStr.split('\n').map(line => {
            const match = line.match(/^\s*([0-9A-Z_]+)\s*=\s*(-?\d+),?$/);

            if (!match) {
                return null;
            }

            return {
                name: match[1].trim(),
                value: match[2].trim()
            };
        }).filter(x => x);

        return { name, extension, fields };
    }).filter(x => x);

    return listToObj(enums);
}

function parseBitFlags() {
    const defined = {};
    const flagBitsRegexp = /typedef enum \w+FlagBits[A-Z]* {\n([^}]+)\n}/gmi;
    const match = VULKAN_H.match(flagBitsRegexp);

    match.forEach(str => {
        const name = str.split(' ', 3)[2];
        const fieldsStr = str.substring(str.indexOf('{') + 2, str.indexOf('}') - 1);

        const fields = fieldsStr.split('\n').map(line => {
            const match = line.match(/^\s*([0-9A-Z_]+)\s*=\s*(0x[\dA-F]{8})|([A-Z_]+)|(0),?\s*$/);

            if (!match) {
                throw new Error(`for enum ${name}: unexpected field "${line}"`);
            }

            return {
                name: match[1],
                value: match[2] || match[3] || match[4]
            };
        }).filter(({value}) => value !== '0x7FFFFFFF' && value.startsWith('0x'));

        defined[name] = fields;
    });

    const flagsRegexp = /typedef VkFlags \w+;/g
    const match2 = VULKAN_H.match(flagsRegexp);

    const bitFlags = match2.map(str => {
        const fullName = str.substring(str.lastIndexOf(' ') + 1, str.indexOf(';'));
        const flagBitsName = fullName.replace('Flags', 'FlagBits');
        const nameInfo = parseName(fullName);
        const fields = defined[flagBitsName] || [];

        return {
            name: nameInfo.name,
            extension: nameInfo.extension,
            fields: fields
        };
    });

    return listToObj(bitFlags);
}

function parseHandles() {
    const regexp = /(VK_DEFINE_HANDLE|VK_DEFINE_NON_DISPATCHABLE_HANDLE)\(\w+\)\n/gm;
    const match = VULKAN_H.match(regexp);

    const handles = match.map(line => {
        const handleName = line.substring(line.indexOf('(') + 1, line.indexOf(')'));
        const nameInfo = parseName(handleName);

        return {
            name: nameInfo.name,
            extension: nameInfo.extension
        };
    });

    return listToObj(handles);
}

function parseConstant(name) {
    if (!name) {
        return null;
    }

    if (!isNaN(+name)) {
        return name;
    }

    const match = VULKAN_H.match(new RegExp(`#define\\s+${name}\\s+([0-9.]+)`));

    if (!match) {
        throw new Error(`cannot find constant ${name}`);
    }

    return match[1];
}

function parseFunctions() {
    const regexp = /(?:VKAPI_ATTR\s+)?(VkResult|void)\s+(?:VKAPI_CALL\s+)?(\w+)\s*\(([^;]+)\)/gm;
    const match = VULKAN_H.match(regexp);

    const functions = match.map(str => {
        const words = str.split(/\W+/g);
        const type = words[1];
        const name = words[3];
        const args = str.substring(str.indexOf('(') + 1, str.indexOf(')')).split(',').map(x => x.trim()).map(argStr => {
            const spaceIndex = argStr.lastIndexOf(' ');
            const name = argStr.substring(spaceIndex + 1);
            const fullType = argStr.substring(0, spaceIndex).trim();
            const typeName = removeSuffix(fullType.replace(/(?:const )?(\w+)\*?/, '$1'));
            const isPointer = fullType.endsWith('*');
            const isConst = fullType.startsWith('const ');

            return { name, fullType, typeName, isPointer, isConst };
        });

        return { name, type, args };
    });

    return functions;
}

module.exports = {
    getAllEnums,
    getAllBitFlags,
    getAllStructs,
    getAllHandles,
    getEnumByName,
    getBitFlagsByName,
    getStructByName,
    getHandleByName,
    isHandle
}