export interface AnalysisResult {
  astStructs: AstStructDefinition[];
  astEnums: AstEnumDefinition[];
  plainEnums: PlainEnumDefinition[];
  tokenEnums: EnumDefinition[];
}

export interface NamedDefinition {
  name: string;
}

export interface DocableDefinition {
  docs: string | undefined;
}

export interface AstStructDefinition extends NamedDefinition, DocableDefinition {
  fields: AstStructFieldDefinition[];
  parents: AstStructDefinition[];
}

export interface AstEnumDefinition extends NamedDefinition, DocableDefinition {
  variants: AstEnumVariantDefinition[];
}

export interface AstEnumVariantDefinition extends NamedDefinition, DocableDefinition {
  tupleArg: TypeReferenceDefinition;
}

export interface StructFieldDefinition extends NamedDefinition, DocableDefinition {
  type: TypeDefinition;
}

export interface AstStructFieldDefinition extends StructFieldDefinition {
  innerName: string;
}

/** Enum that is used like a value. */
export interface PlainEnumDefinition extends NamedDefinition, DocableDefinition {
  variants: PlainEnumVariantDefinition[];
}

export interface EnumDefinition extends NamedDefinition, DocableDefinition {
  variants: EnumVariantDefinition[];
}

export type EnumVariantDefinition = PlainEnumVariantDefinition | TupleEnumVariantDefinition | StructEnumVariantDefinition;

export interface PlainEnumVariantDefinition extends NamedDefinition, DocableDefinition {
  kind: "Plain";
}

export interface TupleEnumVariantDefinition extends NamedDefinition, DocableDefinition {
  kind: "Tuple";
  tupleArgs: TypeDefinition[];
}

export interface StructEnumVariantDefinition extends NamedDefinition, DocableDefinition {
  kind: "Struct";
  fields: StructFieldDefinition[];
}

export type TypeDefinition = PrimitiveTypeDefinition | TypeReferenceDefinition;

export interface PrimitiveTypeDefinition {
  kind: "Primitive";
  text: string;
}

export interface TypeReferenceDefinition extends NamedDefinition {
  kind: "Reference";
  path: string[];
  genericArgs: TypeDefinition[];
}
