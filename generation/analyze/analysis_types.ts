export interface AnalysisResult {
    structs: StructDefinition[];
    enums: EnumDefinition[];
}

export interface StructDefinition {
    name: string;
    docs: string;
    fields: StructFieldDefinition[];
    parents: StructDefinition[];
}

export interface EnumDefinition {
    name: string;
    docs: string;
    /** If it only contains "plain" variants, meaning no tuple or struct variants. */
    isPlain: boolean;
    variants: EnumVariantDefinition[];
}

export interface EnumVariantDefinition {
    name: string;
    docs: string;
    tuple_args: TypeDefinition[] | undefined;
}

export interface StructFieldDefinition {
    name: string;
    docs: string;
    type: TypeDefinition;
}

export type TypeDefinition = PrimitiveTypeDefinition | TypeReferenceDefinition;

export interface PrimitiveTypeDefinition {
    kind: "primitive";
    text: string;
}

export interface TypeReferenceDefinition {
    kind: "reference";
    name: string;
    path: string[];
    generic_args: TypeDefinition[];
}
