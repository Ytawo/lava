const {
    toSnakeCase,
    toPascalCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    getStaticVkValueName,
    getFullWrappedType,
    getFullRawType,
    blockToString,
    isCount,
    areCountAndArray,
    isPlural,
    cToRustVarName,
    argToString,
    getFieldsInformation
} = require('./utils');

function generateVkStructDefinition(cDef) {
    const def = {
        rawTypeName: getRawVkTypeName(cDef.name),
        wrappedTypeName: getWrappedVkTypeName(cDef.name),
        fields: getFieldsInformation(cDef.fields)
    };


    if (isSelfDescribingStructureType(def.fields[0], 0)) {
        def.fields[0].toRaw = () => `vk_to_raw_value(&VkStructureType::${def.wrappedTypeName.substring(2)})`;
        def.fields[0].wrappedType = null;
    }

    def.generics = { types: '', specs: '', static: '', created: '' };

    return [
        genUses(def),
        genRawStructDeclaration(def),
        getWrappedStructDeclaration(def),
        genImplVkRawType(def),
        genImplVkWrappedType(def),
        genImplVkDefault(def)
    ];
}

function genUses(def) {
    const uses = new Set([
        'std::os::raw::c_char',
        'std::string::String',
        'std::vec::Vec',
        'std::ops::Deref',
        'std::ptr',
        'utils::vk_convert::*',
        'utils::vk_null::*',
        'utils::vk_ptr::*',
        'utils::vk_type::*'
    ]);

    for (let field of def.fields) {
        const typeName = field.wrappedTypeName;

        if (typeName.startsWith('Vk')) {
            let use = `vk::`;
            if (field.extension) { use += `${field.extension}::`; }
            use += toSnakeCase(typeName);
            use += `::*`;

            uses.add(use);
        }
    }

    return Array.from(uses).map(str => `use ${str};`);
}

function startsWith(str, prefix) {
    return str && str.startsWith(prefix);
}

function replaceGenericTypes(fields) {
    const startCode = 'A'.charCodeAt(0);
    const letters = [];
    const specs = [];
    const staticTypes = [];
    const createdTypeNames = [];
    
    for (let field of fields) {
        if (startsWith(field.wrappedType, 'T : ')) {
            const letter = String.fromCharCode(startCode + specs.length);
            const typeSpec = field.wrappedType.replace('T : ', `${letter} : `);
            const refTypeName = field.wrappedType.match(/T : Deref<Target=(.*)>/)[1];
            let targetTypeName = refTypeName;
            let isVec = false;

            if (targetTypeName.startsWith(']')) {
                targetTypeName = targetTypeName.substring(1, targetTypeName.length - 1);
                isVec = true;
            }

            let typeName = targetTypeName === 'str' ? 'String' : targetTypeName;

            if (isVec) {
                typeName = `Vec<${typeName}>`;
            }

            field.wrappedType = letter;
            letters.push(letter);
            specs.push(typeSpec);
            staticTypes.push(`&'static ${refTypeName}`)
            createdTypeNames.push(typeName);
        }
    }

    if (!letters.length) {
        return { types: '', specs: '', static: '', created: '' };
    }

    return {
        types: `<${letters.join(', ')}>`,
        specs: `\n    where\n${specs.map(spec => `        ${spec},\n`).join('')}`,
        static: `<${staticTypes.join(', ')}>`,
        created: `<${createdTypeNames.join(', ')}>`
    };
}

function genRawStructDeclaration(cDef) {
    return [
        `pub struct ${cDef.rawTypeName}`,
            cDef.fields.map(field => `${field.varName}: ${field.rawType},`)
    ];
}

function getWrappedStructDeclaration(def) {
    const fields = getWrappedFields(def);
    
    return [
        `pub struct ${def.wrappedTypeName}${def.generics.types}${def.generics.specs}`,
            fields.map(field => `pub ${field.varName}: ${field.wrappedType},`)
    ];
}

function isSelfDescribingStructureType(field, index) {
    return field.wrappedTypeName === 'VkStructureType' && index === 0;
}

function getWrappedFields(def) {
    return def.fields.filter((field, index) => field.wrappedType && !isSelfDescribingStructureType(field, index));
}

function canBeCreatedByUser(def) {
    def.fields.every(field => !field.wrappedType || field.toWrapped);
}

function genImplVkRawType(def) {
    const canBeConverted = canBeCreatedByUser(def);

    if (canBeConverted) {
        const wrappedFields = getWrappedFields(def);

        return [
            `impl VkRawType<${def.wrappedTypeName}${def.generics.created}> for ${def.rawTypeName}`, [
                `fn vk_to_wrapped(src: &${def.rawTypeName}) -> ${def.wrappedTypeName}${def.generics.created}`, [
                    def.wrappedTypeName,
                    wrappedFields.map((field, index) => `${field.varName}: ${genConvertStatement('toWrapped', 'src', wrappedFields, index)},`)
                ],
            ]
        ];
    }
}

function genImplVkWrappedType(def) {
    const canBeConverted = def.fields.every(field => field.toRaw);

    if (canBeConverted) {
        const rawFields = def.fields;

        return [
            `impl${def.generics.types} VkWrappedType<${def.rawTypeName}> for ${def.wrappedTypeName}${def.generics.types}${def.generics.specs}`, [
                `fn vk_to_raw(src: &${def.wrappedTypeName}${def.generics.types}, dst: &mut ${def.rawTypeName})`,
                rawFields.map((field, index) => `dst.${field.varName} = ${genConvertStatement('toRaw', 'src', rawFields, index)};`)
            ]
        ];
    }
}

function genImplVkDefault(def) {
    if (canBeCreatedByUser(def)) {
        const staticValueName = getStaticVkValueName(def.wrappedTypeName);
        const wrappedFields = getWrappedFields(def);

        return [
            `pub static ${staticValueName} : ${def.wrappedTypeName} = ${def.wrappedTypeName}`, [
                wrappedFields.map(field => `${field.varName}: ${field.defaultValue},`)
            ],
            `;\nimpl VkDefault for ${def.wrappedTypeName}`, [
                `fn vk_default() -> ${def.wrappedTypeName}`, [
                    staticValueName
                ]
            ]
        ];
    }
}

function getVarName(prefix, field) {
    return field && `${prefix}.${field.varName}`;
}

function genConvertStatement(key, prefix, fields, index) {
    const field = fields[index];
    const convertFunction = field[key];
    const varName = getVarName(prefix, field);
    const prevVarName = getVarName(prefix, fields[index - 1]);
    const nextVarName = getVarName(prefix, fields[index + 1]);

    return convertFunction(varName, prevVarName, nextVarName);
}

module.exports = {
    generateVkStructDefinition
};