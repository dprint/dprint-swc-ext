export interface Crate {
  root: string;
  index: { [id: string]: Item };
  paths: { [id: string]: ItemSummary };
}

export interface Item {
  crate_id: number;
  name: string;
  visibility: "default" | "public" | "crate";
  docs: string | undefined;
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

export interface EnumVariantInner {
  "kind": {
    "tuple": string[];
    "struct": {
      fields: string[];
    };
  } | "plain";
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
  path: string;
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
