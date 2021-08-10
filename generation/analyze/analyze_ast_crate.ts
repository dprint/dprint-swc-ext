import type {
  AstEnumDefinition,
  AstEnumVariantDefinition,
  AstStructDefinition,
  AstStructFieldDefinition,
  PlainEnumDefinition,
  PlainEnumVariantDefinition,
  TypeDefinition,
} from "./analysis_types.ts";
import type { Crate, EnumInner, EnumVariantInner, Item, StructInner, TupleEnumVariantInner, TypeInner } from "./doc_types.ts";
import { getEnumVariants, getTypeDefinition, sortNamedDefinitions } from "./helpers.ts";

export function analyzeAstCrate() {
  const crate: Crate = JSON.parse(Deno.readTextFileSync("swc_ecma_ast.json"));
  const astStructs = Array.from(getAstStructs());
  const { astEnums, plainEnums } = getEnums();

  sortNamedDefinitions(astStructs);
  sortNamedDefinitions(astEnums);
  sortNamedDefinitions(plainEnums);

  fillStructParents(astStructs, astEnums);

  return {
    astStructs,
    astEnums,
    plainEnums,
  };

  function* getAstStructs() {
    const structs = Object.keys(crate.index).map(key => crate.index[key])
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

  function analyzeStruct(item: Item): AstStructDefinition {
    const inner = item.inner as StructInner;

    return {
      name: item.name,
      docs: item.docs,
      fields: Array.from(getFields()),
      parents: [],
    };

    function* getFields(): Iterable<AstStructFieldDefinition> {
      for (const fieldId of inner.fields) {
        const item = crate.index[fieldId];
        if (item.visibility !== "public") {
          continue;
        }
        if (item.name === "span") {
          continue;
        }
        yield {
          name: getNewFieldName(item.name),
          innerName: item.name,
          docs: item.docs,
          type: getTypeDefinition(crate, item.inner as TypeInner),
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

  function getEnums() {
    const astEnums: AstEnumDefinition[] = [];
    const plainEnums: PlainEnumDefinition[] = [];

    const enums = Object.keys(crate.index).map(key => crate.index[key])
      .filter(item => item.kind === "enum");
    for (const enumDec of enums) {
      if (enumDec.visibility !== "public") {
        continue;
      }
      if (enumDec.name === "Program" || enumDec.name === "TsSignatureDecl" || enumDec.name === "TargetEnv") {
        continue;
      }

      if (isAstEnum(enumDec)) {
        astEnums.push(analyzeAstEnum(enumDec));
      } else {
        plainEnums.push(analyzePlainEnum(enumDec));
      }

      // console.log(enumDec);
      // console.log(JSON.stringify(analyzeStruct(struct), null, 2));
    }

    return { astEnums, plainEnums };
  }

  function isAstEnum(item: Item) {
    const inner = item.inner as EnumInner;
    const firstVariant = inner.variants[0];
    if (firstVariant == null) {
      return false;
    }
    const variantInner = crate.index[firstVariant].inner as EnumVariantInner;
    return variantInner.variant_kind === "tuple";
  }

  function analyzeAstEnum(item: Item): AstEnumDefinition {
    return {
      name: item.name,
      docs: item.docs,
      variants: Array.from(getVariants()),
    };

    function* getVariants(): Iterable<AstEnumVariantDefinition> {
      for (const variantItem of getEnumVariants(crate, item)) {
        yield {
          name: variantItem.name,
          docs: variantItem.docs,
          tupleArg: getTupleArg(variantItem),
        };
      }

      function getTupleArg(variantItem: Item) {
        const inner = variantItem.inner as TupleEnumVariantInner;
        if (inner.variant_kind !== "tuple") {
          throw new Error("Unexpected scenario where the enum inner was not a tuple.");
        }
        if (inner.variant_inner?.length !== 1) {
          throw new Error("Unhandled scenario where the tuple did not have one variant.");
        }

        const definition = getTypeDefinition(crate, inner.variant_inner[0]);
        if (definition.kind !== "Reference") {
          throw new Error("Expected a reference type.");
        }
        return definition;
      }
    }
  }

  function analyzePlainEnum(item: Item): PlainEnumDefinition {
    return {
      name: item.name,
      docs: item.docs,
      variants: Array.from(getVariants()),
    };

    function* getVariants(): Iterable<PlainEnumVariantDefinition> {
      for (const variantItem of getEnumVariants(crate, item)) {
        yield {
          kind: "Plain",
          name: variantItem.name,
          docs: variantItem.docs,
        };
      }
    }
  }
}

function fillStructParents(astStructs: AstStructDefinition[], astEnums: AstEnumDefinition[]) {
  astStructs.forEach(analyzeStruct);

  function analyzeStruct(struct: AstStructDefinition) {
    struct.fields.forEach(f => analyzeType(f.type));

    function analyzeType(type: TypeDefinition) {
      if (type.kind === "Primitive") {
        return;
      }

      if (type.name === "Option" || type.name === "Vec" || type.name === "Box") {
        analyzeType(type.genericArgs[0]);
      } else {
        const childStruct = astStructs.find(s => s.name === type.name);
        if (childStruct != null) {
          if (!childStruct.parents.includes(struct)) {
            childStruct.parents.push(struct);
          }
        } else {
          const childEnum = astEnums.find(s => s.name === type.name);
          if (childEnum != null) {
            for (const variant of childEnum.variants) {
              if (variant.tupleArg != null) {
                analyzeType(variant.tupleArg);
              }
            }
          }
        }
      }
    }
  }
}
