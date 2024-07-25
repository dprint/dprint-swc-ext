import { AnalysisResult, PrimitiveTypeDefinition, TypeDefinition, TypeReferenceDefinition } from "../analyze/analysis_types.ts";
import { Writer } from "../deps.ts";

export function writeHeader(writer: Writer) {
  writer.writeLine("// This code is code generated.");
  writer.writeLine("// Run `./scripts/generate.sh` from the root directory to regenerate it.");
}

export function getIsForImpl(analysisResult: AnalysisResult, type: TypeDefinition): boolean {
  if (type.kind === "Primitive") {
    return true;
  }
  if (type.name === "Option" || type.name === "Vec") {
    return getIsForImpl(analysisResult, type.genericArgs[0]);
  }
  if (type.path[0] === "swc_ecma_ast") {
    return analysisResult.plainEnums.some(s => s.name === type.path[1]);
  }
  return true;
}

export function writeType(writer: Writer, analysisResult: AnalysisResult, type: TypeDefinition, writeStructReference: boolean): void {
  switch (type.kind) {
    case "Primitive":
      writePrimitive(type);
      break;
    case "Reference":
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
    const path = type.path.join("::").replace(/^swc_ecma_ast::/, "").replace("swc_common::syntax_pos::hygiene::SyntaxContext", "swc_common::SyntaxContext");
    if (
      type.path[0] === "swc_ecma_ast"
      && (analysisResult.astEnums.some(e => e.name === type.name) || analysisResult.astStructs.some(s => s.name === type.name))
    ) {
      if (type.genericArgs.length > 0) {
        throw new Error("Unhandled.");
      }
      if (analysisResult.astStructs.some(s => s.name === type.name) && writeStructReference) {
        writer.write("&'a ");
      }
      writer.write(path);
      writer.write("<'a>");
    } else if (type.genericArgs.length > 0) {
      if (path === "Vec") {
        writer.write("&'a [");
      } else {
        writer.write(path);
        writer.write("<");
      }
      writer.write(type.genericArgs.map(type => writeType(writer, analysisResult, type, writeStructReference)).join(", "));
      if (path === "Vec") {
        writer.write("]");
      } else {
        writer.write(">");
      }
    } else {
      writer.write(path);
    }
  }
}

export function getIsReferenceType(analysisResult: AnalysisResult, type: TypeDefinition): boolean {
  if (type.kind === "Primitive") {
    return false;
  }
  if (type.name === "Option") {
    return getIsReferenceType(analysisResult, type.genericArgs[0]);
  }

  if (type.name === "SyntaxContext") {
    return false;
  }

  const isSwcPlainEnumType = type != null && type.kind === "Reference" && analysisResult.plainEnums.some(e => e.name === type.name);
  if (isSwcPlainEnumType) {
    return false;
  }
  return true;
}

export function isSwcNodeEnumType(analysisResult: AnalysisResult, type: TypeDefinition | undefined): boolean {
  return type != null && type.kind === "Reference" && analysisResult.astEnums.some(e => e.name === type.name);
}

export function isSwcAstType(analysisResult: AnalysisResult, type: TypeDefinition | undefined): boolean {
  return getAstStructForType(analysisResult, type) != null;
}

export function getAstStructForType(analysisResult: AnalysisResult, type: TypeDefinition | undefined) {
  if (type == null || type.kind !== "Reference") {
    return undefined;
  }
  return analysisResult.astStructs.find(s => s.name === type.name);
}

export function isVecType(type: TypeDefinition | undefined): boolean {
  return type != null && type.kind === "Reference" && type.name === "Vec";
}

export function isOptionType(type: TypeDefinition | undefined): boolean {
  return type != null && type.kind === "Reference" && type.name === "Option";
}

export function isResultType(type: TypeDefinition | undefined): boolean {
  return type != null && type.kind === "Reference" && type.name === "Result";
}
