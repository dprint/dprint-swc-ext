import { TypeDefinition } from "./analysis_types.ts";
import { Crate, EnumInner, Item, ItemSummary, ResolvedPathTypeInner, TypeInner } from "./doc_types.ts";

export function* getEnumVariants(crate: Crate, item: Item) {
    const inner = item.inner as EnumInner;

    for (const variantId of inner.variants) {
        yield (crate.index[variantId]) as Item;
    }
}

export function getTypeDefinition(crate: Crate, type: TypeInner): TypeDefinition {
    switch (type.kind) {
        case "resolved_path":
            const itemSummary = crate.paths[type.inner.id];
            const path = getPath(itemSummary);

            // we don't care about Boxed types because the result will use an arena
            if (path[0] === "Box") {
                return getTypeDefinition(crate, type.inner.args.angle_bracketed.args[0].type);
            }

            return {
                kind: "Reference",
                name: type.inner.name,
                path,
                genericArgs: getGenericArgs(type),
            };
        case "primitive":
            return {
                kind: "Primitive",
                text: type.inner,
            };
        default:
            throw new Error("Unknown type: " + JSON.stringify(type));
    }

    function getGenericArgs(type: ResolvedPathTypeInner) {
        return type.inner.args.angle_bracketed.args.map(a => getTypeDefinition(crate, a.type));
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
