import type { EnumDefinition, EnumVariantDefinition } from "./analysis_types.ts";
import type { Crate, Item, TypeInner } from "./doc_types.ts";
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
      .filter(item => item.inner.enum != null);
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
        const inner = variantItem.inner.variant!;
        if (inner.kind == null) {
          console.log("Variant", variantItem);
          throw new Error("No kind for variant: " + variantItem.name);
        } else if (inner.kind === "plain") {
          yield {
            kind: "Plain",
            name: variantItem.name,
            docs: variantItem.docs,
          };
        } else if (inner.kind.tuple != null) {
          yield {
            kind: "Tuple",
            name: variantItem.name,
            docs: variantItem.docs,
            tupleArgs: inner.kind.tuple.map(id => {
              const item = crate.index[id];
              return getTypeDefinition(crate, item.inner.struct_field ?? item.inner as TypeInner);
            }),
          };
        } else if (inner.kind.struct != null) {
          yield {
            kind: "Struct",
            name: variantItem.name,
            docs: variantItem.docs,
            fields: inner.kind.struct.fields.map(id => {
              const item = crate.index[id];
              return {
                docs: item.docs,
                name: item.name,
                type: getTypeDefinition(crate, item.inner.struct_field ?? item.inner as TypeInner),
              };
            }),
          };
        } else {
          console.log(variantItem);
          console.log(inner);
          throw new Error("Unknown kind: " + (inner as any).variant_kind);
        }
      }
    }
  }
}
