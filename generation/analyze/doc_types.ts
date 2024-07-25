export interface Crate {
  root: string;
  index: { [id: string]: Item };
  paths: { [id: string]: ItemSummary };
}

export interface Item {
  crate_id: number;
  name: string;
  visibility: "default" | "public" | "crate";
  docs: string | null;
  attrs: string[];
  inner: {
    "struct"?: StructInner;
    "struct_field"?: TypeInner;
    enum?: EnumInner;
    "variant"?: EnumVariantInner;
    "type"?: TypeInner;
  };
}

export interface StructInner {
  generics: Generics;
  kind: {
    plain: {
      /** Identifiers. */
      fields: string[];
    };
  };
  /** Identifiers. */
  impls: string[];
}

export interface EnumInner {
  generics: Generics;
  /** Identifiers */
  variants: string[];
}

export type EnumVariantInner = PlainEnumVariantInner | TupleEnumVariantInner | StructEnumVariantInner;

export interface PlainEnumVariantInner {
  variant_kind: "plain";
}

export interface TupleEnumVariantInner {
  variant_kind: "tuple";
  variant_inner: TypeInner[];
}

export interface StructEnumVariantInner {
  variant_kind: "struct";
  variant_inner: string[];
}

export interface Generics {
  params: string[];
  where_predicates: string[];
}

export interface TypeInner {
  "resolved_path": ResolvedPathTypeInner;
  "primitive": string;
}

export interface ResolvedPathTypeInner {
  name: string;
  args: GenericArgs;
  id: string;
  param_names: GenericBound[];
}

export interface GenericArgs {
  angle_bracketed: {
    args: GenericArg[];
  };
}

export interface GenericArg {
  lifetime: string;
  type: TypeInner;
  const: unknown; // todo
}

export interface GenericBound {
  trait: TypeInner;
  modifier: "none" | "maybe" | "maybe_const";
  generic_params: GenericParamDef;
}

export interface GenericParamDef {
  name: string;
  kind: unknown; // todo
}

export interface ItemSummary {
  crate_id: number;
  /** Fully qualified path. */
  path: string[];
  kind: keyof Item["inner"];
}
