import type {
    AnalysisResult,
    EnumDefinition,
    EnumVariantDefinition,
    NamedDefinition,
    StructDefinition,
    StructFieldDefinition,
    TypeDefinition,
} from "./analysis_types.ts";
import type { Crate, EnumInner, EnumVariantInner, Item, ItemSummary, ResolvedPathTypeInner, StructInner, TypeInner } from "./doc_types.ts";

export function analyze(): AnalysisResult {
    const file: Crate = JSON.parse(Deno.readTextFileSync("swc_ecma_ast.json"));
    const structs = Array.from(getStructs());
    const enums = Array.from(getEnums());
    const compareFn = (a: NamedDefinition, b: NamedDefinition) => a.name < b.name ? -1 : a.name > b.name ? 1 : 0;

    structs.sort(compareFn);
    enums.sort(compareFn);

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
                    name: getNewFieldName(item.name),
                    inner_name: item.name,
                    docs: item.docs,
                    type: getTypeDefinition(item.inner as TypeInner),
                };
            }
        }

        function getNewFieldName(fieldName: string) {
            if (fieldName === "kind") {
                switch (item.name) {
                    case "VarDecl":
                        return "decl_kind";
                    case "ClassMethod":
                    case "PrivateMethod":
                        return "method_kind";
                    case "TsKeywordType":
                        return "keyword_kind";
                    case "Str":
                        return "str_kind";
                    default:
                        throw new Error(`Unhandled custom name for ${item.name}.`);
                }
            } else {
                return fieldName;
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
            isPlain: variants.every(v => v.tuple_arg == null),
            variants,
        };

        function* getVariants(): Iterable<EnumVariantDefinition> {
            for (const variantId of inner.variants) {
                const item = file.index[variantId];
                yield {
                    name: item.name,
                    docs: item.docs,
                    tuple_arg: getTupleArg(item),
                };
            }

            function getTupleArg(item: Item) {
                const inner = item.inner as EnumVariantInner;
                switch (inner.variant_kind) {
                    case "plain":
                        return undefined;
                    case "tuple":
                        if (inner.variant_inner!.length !== 1) {
                            throw new Error("Unhandled scenario where the tuple did not have one variant.");
                        }
                        return getTypeDefinition(inner.variant_inner![0]);
                }
            }
        }
    }

    function getTypeDefinition(type: TypeInner): TypeDefinition {
        switch (type.kind) {
            case "resolved_path":
                const itemSummary = file.paths[type.inner.id];
                const path = getPath(itemSummary);

                // we don't care about Boxed types because the result will use an arena
                if (path[0] === "Box") {
                    return getTypeDefinition(type.inner.args.angle_bracketed.args[0].type);
                }

                return {
                    kind: "reference",
                    name: type.inner.name,
                    path,
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
                            if (variant.tuple_arg != null) {
                                analyzeType(variant.tuple_arg);
                            }
                        }
                    }
                }
            }
        }
    }
}
