export interface AnalysisResult {
    structs: StructDefinition[];
    enums: EnumDefinition[];
}

export interface NamedDefinition {
    name: string;
}

export interface StructDefinition extends NamedDefinition {
    docs: string | undefined;
    fields: StructFieldDefinition[];
    parents: StructDefinition[];
}

export interface EnumDefinition extends NamedDefinition {
    docs: string | undefined;
    /** If it only contains "plain" variants, meaning no tuple or struct variants. */
    isPlain: boolean;
    variants: EnumVariantDefinition[];
}

export interface EnumVariantDefinition extends NamedDefinition {
    docs: string | undefined;
    tuple_arg: TypeDefinition | undefined;
}

export interface StructFieldDefinition extends NamedDefinition {
    inner_name: string;
    docs: string;
    type: TypeDefinition;
}

export type TypeDefinition = PrimitiveTypeDefinition | TypeReferenceDefinition;

export interface PrimitiveTypeDefinition {
    kind: "primitive";
    text: string;
}

export interface TypeReferenceDefinition extends NamedDefinition {
    kind: "reference";
    path: string[];
    generic_args: TypeDefinition[];
}
