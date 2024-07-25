import { NamedDefinition, TypeDefinition } from "./analysis_types.ts";
import { Crate, Item, ItemSummary, ResolvedPathTypeInner, TypeInner } from "./doc_types.ts";

export function* getEnumVariants(crate: Crate, item: Item) {
  const inner = item.inner.enum!;

  for (const variantId of inner.variants) {
    yield (crate.index[variantId]) as Item;
  }
}

export function getTypeDefinition(crate: Crate, type: TypeInner): TypeDefinition {
  if (type.resolved_path != null) {
    const inner = type.resolved_path;
    const itemSummary = crate.paths[inner.id];
    if (itemSummary != null) {
      const path = getPath(itemSummary);

      // we don't care about Boxed types because the result will use an arena
      if (path[0] === "Box") {
        return getTypeDefinition(crate, inner.args.angle_bracketed.args[0].type);
      }

      return {
        kind: "Reference",
        name: inner.name,
        path,
        genericArgs: getGenericArgs(inner),
      };
    }
    const item = crate.index[inner.id];
    if (item == null) {
      console.error(type);
      throw new Error(`Did not find path or item for ${inner.id}`);
    }
    return {
      kind: "Reference",
      name: item.name,
      path: [item.name],
      genericArgs: getGenericArgs(inner),
    };
  } else if (type.primitive != null) {
    return {
      kind: "Primitive",
      text: type.primitive,
    };
  } else {
    throw new Error("Unknown type: " + JSON.stringify(type));
  }

  function getGenericArgs(type: ResolvedPathTypeInner) {
    return type.args.angle_bracketed.args.map(a => getTypeDefinition(crate, a.type));
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

export function sortNamedDefinitions(items: NamedDefinition[]) {
  items.sort((a, b) => a.name < b.name ? -1 : a.name > b.name ? 1 : 0);
}
