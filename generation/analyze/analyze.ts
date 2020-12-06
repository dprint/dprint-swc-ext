import type { AnalysisResult, EnumDefinition, EnumVariantDefinition, StructDefinition, StructFieldDefinition, TypeDefinition } from "./analysis_types.ts";
import type { Crate, EnumInner, EnumVariantInner, Item, ItemSummary, ResolvedPathTypeInner, StructInner, TypeInner } from "./doc_types.ts";

export function analyze(): AnalysisResult {
    const file: Crate = JSON.parse(Deno.readTextFileSync("swc_ecma_ast.json"));
    const structs = Array.from(getStructs());
    const enums = Array.from(getEnums());

    fillStructParents({ structs, enums });

    return {
        structs,
        enums,
    };

    function* getStructs() {
        const structs = Object.keys(file.index).map(key => file.index[key])
            .filter(item => item.kind === "struct");
        for (const struct of structs) {
            if (struct.visibility !== "public") {
                continue;
            }
            if (struct.name === "Script") {
                continue;
            }

            // console.log(struct);
            yield analyzeStruct(struct);
            // console.log(JSON.stringify(analyzeStruct(struct), null, 2));
        }
    }

    function analyzeStruct(item: Item): StructDefinition {
        const inner = item.inner as StructInner;

        return {
            name: item.name,
            docs: item.docs,
            fields: Array.from(getFields()),
            parents: [],
        };

        function* getFields(): Iterable<StructFieldDefinition> {
            for (const fieldId of inner.fields) {
                const item = file.index[fieldId];
                if (item.visibility !== "public") {
                    continue;
                }
                if (item.name === "span") {
                    continue;
                }
                yield {
                    name: item.name,
                    docs: item.docs,
                    type: getTypeDefinition(item.inner as TypeInner),
                };
            }
        }
    }

    function* getEnums() {
        const enums = Object.keys(file.index).map(key => file.index[key])
            .filter(item => item.kind === "enum");
        for (const enumDec of enums) {
            if (enumDec.visibility !== "public") {
                continue;
            }
            if (enumDec.name === "Program" || enumDec.name === "TsSignatureDecl") {
                continue;
            }

            // console.log(enumDec);
            yield analyzeEnum(enumDec);
            // console.log(JSON.stringify(analyzeStruct(struct), null, 2));
        }
    }

    function analyzeEnum(item: Item): EnumDefinition {
        const inner = item.inner as EnumInner;
        const variants = Array.from(getVariants());
        return {
            name: item.name,
            docs: item.docs,
            isPlain: variants.every(v => v.tuple_args == null),
            variants,
        };

        function* getVariants(): Iterable<EnumVariantDefinition> {
            for (const variantId of inner.variants) {
                const item = file.index[variantId];
                yield {
                    name: item.name,
                    docs: item.docs,
                    tuple_args: getTupleArgs(item),
                };
            }

            function getTupleArgs(item: Item) {
                const inner = item.inner as EnumVariantInner;
                switch (inner.variant_kind) {
                    case "plain":
                        return undefined;
                    case "tuple":
                        return inner.variant_inner!.map(inner => getTypeDefinition(inner));
                }
            }
        }
    }

    function getTypeDefinition(type: TypeInner): TypeDefinition {
        switch (type.kind) {
            case "resolved_path":
                const itemSummary = file.paths[type.inner.id];
                return {
                    kind: "reference",
                    name: type.inner.name,
                    path: getPath(itemSummary),
                    generic_args: getGenericArgs(type),
                };
            case "primitive":
                return {
                    kind: "primitive",
                    text: type.inner,
                };
            default:
                throw new Error("Unknown type: " + JSON.stringify(type));
        }

        function getGenericArgs(type: ResolvedPathTypeInner) {
            return type.inner.args.angle_bracketed.args.map(a => getTypeDefinition(a.type));
        }

        function getPath(itemSummary: ItemSummary) {
            // simplify and resolve some of the paths
            if (equalsPath("swc_common", "syntax_pos", "Span")) {
                return ["swc_common", "Span"];
            }
            if (equalsPath("alloc", "boxed", "Box")) {
                return ["Box"];
            }
            if (equalsPath("alloc", "vec", "Vec")) {
                return ["Vec"];
            }
            if (equalsPath("core", "option", "Option")) {
                return ["Option"];
            }
            if (equalsPath("num_bigint", "bigint", "BigInt")) {
                return ["num_bigint", "BigInt"];
            }
            return itemSummary.path;

            function equalsPath(...args: string[]) {
                const path = itemSummary.path;
                if (args.length !== path.length) {
                    return false;
                }
                for (let i = 0; i < args.length; i++) {
                    if (args[i] !== path[i]) {
                        return false;
                    }
                }
                return true;
            }
        }
    }
}

export function getNamesUsedInBox(analysisResult: AnalysisResult) {
    const names = new Set<string>();
    for (const type of getTypes()) {
        for (const name of analyzeType(type)) {
            names.add(name);
        }
    }
    return Array.from(names).sort(); // determinism in output

    function* getTypes() {
        for (const struct of analysisResult.structs) {
            for (const field of struct.fields) {
                yield field.type;
            }
        }
        for (const enumDef of analysisResult.enums) {
            for (const variant of enumDef.variants) {
                if (variant.tuple_args != null) {
                    yield* variant.tuple_args;
                }
            }
        }
    }

    function analyzeType(type: TypeDefinition): string[] {
        if (type.kind === "primitive") {
            return [];
        }

        if (type.name === "Option" || type.name === "Vec") {
            return analyzeType(type.generic_args[0]);
        } else if (type.name === "Box") {
            const genericType = type.generic_args[0];
            const types = analyzeType(genericType);
            if (genericType.kind !== "primitive") {
                types.push(genericType.name);
            }
            return types;
        } else {
            return [];
        }
    }
}

function fillStructParents({ structs, enums }: AnalysisResult) {
    structs.forEach(analyzeStruct);

    function analyzeStruct(struct: StructDefinition) {
        struct.fields.forEach(f => analyzeType(f.type));

        function analyzeType(type: TypeDefinition) {
            if (type.kind === "primitive") {
                return;
            }

            if (type.name === "Option" || type.name === "Vec" || type.name === "Box") {
                analyzeType(type.generic_args[0]);
            } else {
                const childStruct = structs.find(s => s.name === type.name);
                if (childStruct != null) {
                    if (!childStruct.parents.includes(struct)) {
                        childStruct.parents.push(struct);
                    }
                } else {
                    const childEnum = enums.find(s => s.name === type.name);
                    if (childEnum != null) {
                        for (const variant of childEnum.variants) {
                            if (variant.tuple_args != null) {
                                for (const typeDef of variant.tuple_args) {
                                    analyzeType(typeDef);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
