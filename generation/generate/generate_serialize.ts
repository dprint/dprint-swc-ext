import { AnalysisResult, AstStructDefinition, EnumDefinition, TypeDefinition } from "../analyze/analysis_types.ts";
import { createWriter } from "../utils/create_writer.ts";
import { getIsForImpl, writeHeader, writeType } from "../utils/generation_utils.ts";
import { nameToSnakeCase, snakeCaseToCamel } from "../utils/string_utils.ts";

export function generateSerialize(analysisResult: AnalysisResult): string {
    const writer = createWriter();

    writeHeader(writer);
    writeUseDeclarations();

    for (const struct of analysisResult.astStructs) {
        writer.blankLine();
        writeSerializableStruct(struct);
        writer.blankLine();
        writeFromImpl(struct);
    }

    writer.blankLine();
    writeTokenSerialization();

    writer.newLineIfLastNot();

    return writer.toString();

    function writeUseDeclarations() {
        writer.writeLine("use std::marker::PhantomData;");
        writer.writeLine("use std::io::{Error, Write};");
        writer.writeLine("use serde::Serialize;");
        writer.writeLine("use serde_json::ser::{Formatter as JsonFormatter, to_string as to_json_string};");
        writer.writeLine("use swc_common::{Span, Spanned, comments::{Comment}};");
        writer.writeLine("use swc_ecmascript::parser::token::{BinOpToken, Keyword, Token, TokenAndSpan, Word};");
        writer.writeLine("use crate::generated::*;");
    }

    function writeSerializableStruct(struct: AstStructDefinition) {
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

    function writeFromImpl(struct: AstStructDefinition) {
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

    // todo: requires cleanup... the output of this is super verbose and hard to read

    function writeTokenSerialization() {
        writeSerializeTokenAndSpansFunction();
        writer.blankLine();
        writeSerializeTokenAndSpanFunction();
        writer.blankLine();
        writeEnumSerializationFunction("BinOpToken");
        writer.blankLine();
        writeEnumSerializationFunction("Keyword");
        writer.blankLine();
        writeEnumSerializationFunction("Token");
        writer.blankLine();
        writeEnumSerializationFunction("Word");

        function writeEnumSerializationFunction(enumName: string) {
            const tokenEnum = analysisResult.tokenEnums.find(e => e.name === enumName)!;
            writer.write(`fn serialize_${nameToSnakeCase(enumName)}(w: &mut impl Write, f: &mut impl JsonFormatter, value: &${enumName}) -> Result<(), Error>`)
                .block(() => {
                    writeEnumSerialization("value", tokenEnum);
                    writer.writeLine("Ok(())");
                });
        }
    }

    function writeSerializeTokenAndSpansFunction() {
        writer.write(`pub fn serialize_token_and_spans(w: &mut impl Write, f: &mut impl JsonFormatter, tokens: &Vec<TokenAndSpan>) -> Result<(), Error>`).block(
            () => {
                writeArray(() => {
                    writer.write("for (i, token_and_span) in tokens.iter().enumerate()").block(() => {
                        writeArrayValue(() => {
                            writer.writeLine("serialize_token_and_span(w, f, &token_and_span)?;");
                        }, `i == 0`);
                    });
                });
                writer.writeLine("Ok(())");
            },
        );
    }

    function writeSerializeTokenAndSpanFunction() {
        writer.write(`pub fn serialize_token_and_span(w: &mut impl Write, f: &mut impl JsonFormatter, token_and_span: &TokenAndSpan) -> Result<(), Error>`)
            .block(() => {
                writeObject(() => {
                    // span
                    writeObjectStrKey("span", true);
                    writeObjectValue(() => writer.writeLine(`write!(w, "{}", to_json_string(&token_and_span.span)?)?;`));
                    // had_line_break
                    writeObjectStrKey("hadLineBreak", false);
                    writeObjectValue(() => writer.writeLine(`f.write_bool(w, token_and_span.had_line_break)?;`));
                    // token
                    writeObjectStrKey("token", false);
                    writeObjectValue(() => writer.writeLine(`serialize_token(w, f, &token_and_span.token)?;`));
                });
                writer.writeLine("Ok(())");
            });
    }

    function writeEnumSerialization(valueName: string, tokenEnum: EnumDefinition) {
        writer.write(`match ${valueName}`).block(() => {
            for (const [i, variant] of tokenEnum.variants.entries()) {
                switch (variant.kind) {
                    case "Plain":
                        writer.writeLine(`${tokenEnum.name}::${variant.name} => f.write_u32(w, ${i})?,`);
                        break;
                    case "Tuple":
                        writer.write(`${tokenEnum.name}::${variant.name}(`);
                        for (const [i, _] of variant.tupleArgs.entries()) {
                            writer.conditionalWrite(i > 0, ", ");
                            writer.write(`item${i}`);
                        }
                        writer.write(`) =>`).block(() => {
                            writeObject(() => {
                                // kind method
                                writeObjectStrKey("kind", true);
                                writeObjectValue(() => {
                                    writer.writeLine(`f.write_u32(w, ${i})?;`);
                                });
                                if (variant.tupleArgs.length === 1) {
                                    writeObjectStrKey("inner", false);
                                    writeObjectValue(() => {
                                        writeTypeValueSerialization(`&item0`, variant.tupleArgs[0]);
                                    });
                                } else if (variant.tupleArgs.length > 1) {
                                    writeArray(() => {
                                        for (const [i, tupleArg] of variant.tupleArgs.entries()) {
                                            writeArrayValue(() => {
                                                writeTypeValueSerialization(`&item${i}`, tupleArg);
                                            }, i === 0);
                                        }
                                    });
                                }
                            });
                        });
                        break;
                    case "Struct":
                        writer.write(`${tokenEnum.name}::${variant.name} `).inlineBlock(() => {
                            for (const field of variant.fields) {
                                writer.writeLine(`${field.name},`);
                            }
                        });
                        writer.write(` =>`).block(() => {
                            writeObject(() => {
                                // kind method
                                writeObjectStrKey("kind", true);
                                writeObjectValue(() => {
                                    writer.writeLine(`f.write_u32(w, ${i})?;`);
                                });
                                for (const field of variant.fields) {
                                    writeObjectStrKey(snakeCaseToCamel(field.name), false);
                                    writeObjectValue(() => {
                                        writeTypeValueSerialization(`&${field.name}`, field.type);
                                    });
                                }
                            });
                        });
                        break;
                    default:
                        const _assertNever: never = variant;
                        throw new Error("Not handled situation.");
                }
            }
        });
    }

    function writeTypeValueSerialization(name: string, type: TypeDefinition) {
        const customNames = new Set(["BinOpToken", "Word", "Keyword", "Token"]);
        if (type.kind === "Reference" && customNames.has(type.name)) {
            writer.writeLine(`serialize_${nameToSnakeCase(type.name)}(w, f, ${name})?;`);
        } else if (type.kind === "Reference" && type.name === "Error") {
            writer.writeLine(`panic!("Serializing an AST containing an Error is not currently supported.");`);
        } else {
            writer.writeLine(`write!(w, "{}", to_json_string(${name})?)?;`);
        }
    }

    function writeArray(inner: () => void) {
        writer.writeLine("f.begin_array(w)?;");
        inner();
        writer.writeLine("f.end_array(w)?;");
    }

    function writeArrayValue(inner: () => void, isFirst: boolean | string) {
        writer.writeLine(`f.begin_array_value(w, ${isFirst})?;`);
        inner();
        writer.writeLine("f.end_array_value(w)?;");
    }

    function writeObject(inner: () => void) {
        writer.writeLine("f.begin_object(w)?;");
        inner();
        writer.writeLine("f.end_object(w)?;");
    }

    function writeObjectStrKey(name: string, isFirst: boolean) {
        writeObjectKey(() => {
            writer.writeLine("f.begin_string(w)?;");
            writer.writeLine(`f.write_string_fragment(w, "${name}")?;`);
            writer.writeLine("f.end_string(w)?;");
        }, isFirst);
    }

    function writeObjectKey(inner: () => void, isFirst: boolean) {
        writer.writeLine(`f.begin_object_key(w, ${isFirst})?;`);
        inner();
        writer.writeLine("f.end_object_key(w)?;");
    }

    function writeObjectValue(inner: () => void) {
        writer.writeLine("f.begin_object_value(w)?;");
        inner();
        writer.writeLine("f.end_object_value(w)?;");
    }
}
