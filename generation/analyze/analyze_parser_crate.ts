import type { EnumDefinition, EnumVariantDefinition } from "./analysis_types.ts";
import type { Crate, EnumVariantInner, Item, TypeInner } from "./doc_types.ts";
import { getEnumVariants, getTypeDefinition, sortNamedDefinitions } from "./helpers.ts";

export function analyzeParserCrate() {
    const crate: Crate = JSON.parse(Deno.readTextFileSync("swc_ecma_parser.json"));
    const tokenEnums = Array.from(getEnums());

    sortNamedDefinitions(tokenEnums);

    return {
        tokenEnums,
    };

    function* getEnums() {
        const allowedEnums = new Set(["Token", "BinOpToken", "Word", "Keyword"]);
        const enums = Object.keys(crate.index).map(key => crate.index[key])
            .filter(item => item.kind === "enum");
        for (const enumDec of enums) {
            if (enumDec.visibility !== "public") {
                continue;
            }
            if (!allowedEnums.has(enumDec.name)) {
                continue;
            }

            yield analyzeEnum(enumDec);
        }
    }

    function analyzeEnum(item: Item): EnumDefinition {
        return {
            name: item.name,
            docs: item.docs,
            variants: Array.from(getVariants()),
        };

        function* getVariants(): Iterable<EnumVariantDefinition> {
            for (const variantItem of getEnumVariants(crate, item)) {
                const inner = variantItem.inner as EnumVariantInner;
                switch (inner.variant_kind) {
                    case "tuple":
                        yield {
                            kind: "Tuple",
                            name: variantItem.name,
                            docs: variantItem.docs,
                            tupleArgs: inner.variant_inner!.map(t => getTypeDefinition(crate, t)),
                        };
                        break;
                    case "struct":
                        yield {
                            kind: "Struct",
                            name: variantItem.name,
                            docs: variantItem.docs,
                            fields: inner.variant_inner.map(id => {
                                const item = crate.index[id];
                                return {
                                    docs: item.docs,
                                    name: item.name,
                                    type: getTypeDefinition(crate, item.inner as TypeInner),
                                };
                            }),
                        };
                        break;
                    case "plain":
                        yield {
                            kind: "Plain",
                            name: variantItem.name,
                            docs: variantItem.docs,
                        };
                        break;
                }
            }
        }
    }
}
