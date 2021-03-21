import { AnalysisResult, StructDefinition } from "../analyze/analysis_types.ts";
import { writeHeader, getIsForImpl, writeType } from "../utils/generationUtils.ts";
import { Writer } from "./writer.ts";

export function generateSerialize(analysisResult: AnalysisResult): string {
    const writer = new Writer();

    writeHeader(writer);
    writeUseDeclarations();

    for (const struct of analysisResult.structs) {
        writer.newLine();
        writeSerializableStruct(struct);
        writer.newLine();
        writeFromImpl(struct);
    }

    return writer.toString();

    function writeUseDeclarations() {
        writer.writeLine("use std::marker::PhantomData;");
        writer.writeLine("use serde::Serialize;");
        writer.writeLine("use swc_common::{Span, Spanned};");
        writer.writeLine("use crate::generated::*;");
    }

    function writeSerializableStruct(struct: StructDefinition) {
        const implFields = struct.fields.filter(f => getIsForImpl(analysisResult, f.type));
        const structFields = struct.fields.filter(f => !getIsForImpl(analysisResult, f.type) && f.name !== "span");

        writer.writeLine("#[derive(Serialize)]");
        writer.writeLine(`#[serde(rename = "${struct.name}", rename_all = "camelCase", tag = "nodeKind")]`);
        writer.writeLine(`pub struct Serializable${struct.name}<'a> {`);
        writer.indent(() => {
            writer.writeLine(`span: Span,`);

            for (const field of implFields) {
                writer.write(`${field.name}: `);
                writeType(writer, analysisResult, field.type, false);
                writer.write(",").newLine();
            }

            for (const field of structFields) {
                writer.write(`${field.name}: `);
                writeType(writer, analysisResult, field.type, true);
                writer.write(",").newLine();
            }

            writer.newLine();
            writer.writeLine("#[doc(hidden)]");
            writer.writeLine("#[serde(skip)]");
            writer.writeLine(`_phantom: PhantomData<&'a ()>,`);
        });
        writer.writeLine("}");
    }

    function writeFromImpl(struct: StructDefinition) {
        const implFields = struct.fields.filter(f => getIsForImpl(analysisResult, f.type));
        const structFields = struct.fields.filter(f => !getIsForImpl(analysisResult, f.type) && f.name !== "span");

        writer.writeLine(`impl<'a> From<${struct.name}<'a>> for Serializable${struct.name}<'a> {`);
        writer.indent(() => {
            writer.writeLine(`fn from(orig: ${struct.name}<'a>) -> Self {`);
            writer.indent(() => {
                writer.writeLine("Self {");
                writer.indent(() => {
                    writer.writeLine(`span: orig.span(),`);

                    for (const field of implFields) {
                        writer.writeLine(`${field.name}: orig.${field.name}().clone(),`);
                    }

                    for (const field of structFields) {
                        writer.writeLine(`${field.name}: orig.${field.name},`);
                    }

                    writer.writeLine(`_phantom: PhantomData,`);
                });
                writer.writeLine("}");
            });
            writer.writeLine("}");
        });
        writer.writeLine("}");
    }
}

