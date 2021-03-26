import { AnalysisResult, StructDefinition } from "../analyze/analysis_types.ts";
import { createWriter } from "../utils/createWriter.ts";
import { getIsForImpl, writeHeader, writeType } from "../utils/generationUtils.ts";

export function generateSerialize(analysisResult: AnalysisResult): string {
    const writer = createWriter();

    writeHeader(writer);
    writeUseDeclarations();

    for (const struct of analysisResult.structs) {
        writer.blankLine();
        writeSerializableStruct(struct);
        writer.blankLine();
        writeFromImpl(struct);
    }

    writer.newLineIfLastNot();

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
        writer.writeLine(`#[serde(rename = "${struct.name}", rename_all = "camelCase", tag = "kind")]`);
        writer.write(`pub struct Serializable${struct.name}<'a>`).block(() => {
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
    }

    function writeFromImpl(struct: StructDefinition) {
        const implFields = struct.fields.filter(f => getIsForImpl(analysisResult, f.type));
        const structFields = struct.fields.filter(f => !getIsForImpl(analysisResult, f.type) && f.name !== "span");

        writer.write(`impl<'a> From<${struct.name}<'a>> for Serializable${struct.name}<'a>`).block(() => {
            writer.write(`fn from(orig: ${struct.name}<'a>) -> Self`).block(() => {
                writer.write("Self").block(() => {
                    writer.writeLine(`span: orig.span(),`);

                    for (const field of implFields) {
                        writer.writeLine(`${field.name}: orig.${field.name}().clone(),`);
                    }

                    for (const field of structFields) {
                        writer.writeLine(`${field.name}: orig.${field.name},`);
                    }

                    writer.writeLine(`_phantom: PhantomData,`);
                });
            });
        });
    }
}
