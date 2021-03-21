import { AnalysisResult, PrimitiveTypeDefinition, TypeDefinition, TypeReferenceDefinition } from "../analyze/analysis_types.ts";
import { Writer } from "../deps.ts";

export function writeHeader(writer: Writer) {
    writer.writeLine("// This code is code generated.");
    writer.writeLine("// Run `./scripts/generate.sh` from the root directory to regenerate it.");
}

export function getIsForImpl(analysisResult: AnalysisResult, type: TypeDefinition): boolean {
    if (type.kind === "primitive") {
        return true;
    }
    if (type.name === "Option" || type.name === "Vec") {
        return getIsForImpl(analysisResult, type.generic_args[0]);
    }
    if (type.path[0] === "swc_ecma_ast") {
        return analysisResult.enums.some(s => s.isPlain && s.name === type.path[1]);
    }
    return true;
}

export function writeType(writer: Writer, analysisResult: AnalysisResult, type: TypeDefinition, writeStructReference: boolean): void {
    switch (type.kind) {
        case "primitive":
            writePrimitive(type);
            break;
        case "reference":
            writeReference(type);
            break;
        default:
            const _assertNever: never = type;
            throw new Error("Not handled type.");
    }

    function writePrimitive(type: PrimitiveTypeDefinition) {
        writer.write(type.text);
    }

    function writeReference(type: TypeReferenceDefinition) {
        const path = type.path.join("::").replace(/^swc_ecma_ast::/, "");
        if (analysisResult.enums.some(e => !e.isPlain && e.name === type.name) || analysisResult.structs.some(s => s.name === type.name)) {
            if (type.generic_args.length > 0) {
                throw new Error("Unhandled.");
            }
            if (analysisResult.structs.some(s => s.name === type.name) && writeStructReference) {
                writer.write("&'a ");
            }
            writer.write(path);
            writer.write("<'a>");
        } else if (type.generic_args.length > 0) {
            writer.write(path);
            writer.write("<");
            writer.write(type.generic_args.map(type => writeType(writer, analysisResult, type, writeStructReference)).join(", "));
            writer.write(">");
        } else {
            writer.write(path);
        }
    }
}

export function getIsReferenceType(analysisResult: AnalysisResult, type: TypeDefinition): boolean {
    if (type.kind === "primitive") {
        return false;
    }
    if (type.name === "Option") {
        return getIsReferenceType(analysisResult, type.generic_args[0]);
    }

    const isSwcPlainEnumType = type != null && type.kind === "reference" && analysisResult.enums.some(e => e.isPlain && e.name === type.name);
    if (isSwcPlainEnumType) {
        return false;
    }
    return true;
}
