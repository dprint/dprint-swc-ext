import { AnalysisResult, AstEnumDefinition, AstStructDefinition, EnumDefinition, TypeDefinition } from "../analyze/analysis_types.ts";
import { createWriter } from "../utils/create_writer.ts";
import { nameToSnakeCase, snakeToCamelCase } from "../utils/string_utils.ts";
import { isOptionType, isSwcAstType, isSwcNodeEnumType, isVecType, writeHeader } from "./helpers.ts";

export function generateSerialize(analysisResult: AnalysisResult): string {
    const writer = createWriter();

    writeHeader(writer);
    writeUseDeclarations();

    writer.blankLine();
    writer.write("pub struct FileSerializer<'a, TWrite: Write, TJsonFormatter: JsonFormatter>").block(() => {
        writer.writeLine("w: &'a mut TWrite,");
        writer.writeLine("f: &'a mut TJsonFormatter,");
        writer.writeLine("multi_byte_chars: Vec<MultiByteChar>,");
    });

    writer.blankLine();
    writer.write("impl<'a, TWrite: Write, TJsonFormatter: JsonFormatter> FileSerializer<'a, TWrite, TJsonFormatter>").block(() => {
        writer.write("pub fn new(w: &'a mut TWrite, f: &'a mut TJsonFormatter, file_text: &str) -> Self").block(() => {
            writer.write("FileSerializer").block(() => {
                writer.writeLine("w,");
                writer.writeLine("f,");
                writer.writeLine("multi_byte_chars: get_multi_byte_chars(file_text),");
            });
        });

        for (const [i, struct] of analysisResult.astStructs.entries()) {
            writer.blankLine();
            writeAstStructMethod(struct, i);
        }

        for (const enumDef of analysisResult.astEnums) {
            writer.blankLine();
            writeAstEnumMethod(enumDef);
        }

        writer.blankLine();
        writeTokenSerialization();
        writer.blankLine();
        writeCommentSerialization();
        writer.blankLine();
        writeSpanSerialization();
    });

    writer.newLineIfLastNot();

    return writer.toString();

    function writeUseDeclarations() {
        writer.writeLine("use std::io::{Error, Write};");
        writer.writeLine("use serde_json::ser::{Formatter as JsonFormatter, to_string as to_json_string};");
        writer.writeLine("use swc_common::{Span, Spanned, comments::{Comment, CommentKind, SingleThreadedCommentsMapInner}};");
        writer.writeLine("use swc_ecmascript::parser::token::{BinOpToken, Keyword, Token, TokenAndSpan, Word};");
        writer.writeLine("use swc_ecmascript::ast::*;");
        writer.writeLine("use super::*;");
    }

    function writeAstStructMethod(struct: AstStructDefinition, index: number) {
        writer.write(
            `pub fn serialize_${nameToSnakeCase(struct.name)}(`
                + `&mut self, node: &${struct.name}) -> Result<(), Error>`,
        ).block(() => {
            writeObject(() => {
                writeObjectStrKey("kind", true);
                writeObjectValue(() => writer.writeLine(`self.f.write_u32(self.w, ${index})?;`));
                writer.writeLine("self.serialize_span_props(&node.span(), false)?;");

                for (const field of struct.fields.filter(f => f.name !== "span")) {
                    writeObjectStrKey(snakeToCamelCase(field.name), false);
                    writeObjectValue(() => writeTypeValueSerialization(`&node.${field.innerName}`, field.type));
                }
            });
            writer.writeLine("Ok(())");
        });
    }

    function writeAstEnumMethod(enumDef: AstEnumDefinition) {
        writer.write(
            `pub fn serialize_${nameToSnakeCase(enumDef.name)}(`
                + `&mut self, node: &${enumDef.name}) -> Result<(), Error>`,
        ).block(() => {
            writer.write("match node").block(() => {
                for (const variant of enumDef.variants) {
                    writer.writeLine(`${enumDef.name}::${variant.name}(node) => self.serialize_${nameToSnakeCase(variant.tupleArg.name)}(node)?,`);
                }
            });
            writer.writeLine("Ok(())");
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
            writer.write(`fn serialize_${nameToSnakeCase(enumName)}(&mut self, value: &${enumName}) -> Result<(), Error>`)
                .block(() => {
                    writeEnumSerialization("value", tokenEnum);
                    writer.writeLine("Ok(())");
                });
        }
    }

    function writeSerializeTokenAndSpansFunction() {
        writer.write(`pub fn serialize_token_and_spans(&mut self, tokens: &Vec<TokenAndSpan>) -> Result<(), Error>`).block(
            () => {
                writeArray(() => {
                    writer.write("for (i, token_and_span) in tokens.iter().enumerate()").block(() => {
                        writeArrayValue(() => {
                            writer.writeLine("self.serialize_token_and_span(&token_and_span)?;");
                        }, `i == 0`);
                    });
                });
                writer.writeLine("Ok(())");
            },
        );
    }

    function writeSerializeTokenAndSpanFunction() {
        writer.write(`pub fn serialize_token_and_span(&mut self, token_and_span: &TokenAndSpan) -> Result<(), Error>`)
            .block(() => {
                writeObject(() => {
                    // span
                    writer.writeLine(`self.serialize_span_props(&token_and_span.span, true)?;`);
                    // had_line_break
                    writeObjectStrKey("hadLineBreak", false);
                    writeObjectValue(() => writer.writeLine(`self.f.write_bool(self.w, token_and_span.had_line_break)?;`));
                    // token
                    writeObjectStrKey("token", false);
                    writeObjectValue(() => writer.writeLine(`self.serialize_token(&token_and_span.token)?;`));
                });
                writer.writeLine("Ok(())");
            });
    }

    function writeEnumSerialization(valueName: string, tokenEnum: EnumDefinition) {
        writer.write(`match ${valueName}`).block(() => {
            for (const [i, variant] of tokenEnum.variants.entries()) {
                switch (variant.kind) {
                    case "Plain":
                        writer.writeLine(`${tokenEnum.name}::${variant.name} => self.f.write_u32(self.w, ${i})?,`);
                        break;
                    case "Tuple":
                        writer.write(`${tokenEnum.name}::${variant.name}(`);
                        if (tokenEnum.name === "Token" && variant.name === "Error") {
                            writer.write("_) => ");
                            writer.write(`panic!("Serializing an AST containing an Error is not currently supported."),`);
                            break;
                        }
                        for (const [i, _] of variant.tupleArgs.entries()) {
                            writer.conditionalWrite(i > 0, ", ");
                            writer.write(`item${i}`);
                        }
                        writer.write(`) =>`).block(() => {
                            writeObject(() => {
                                // kind method
                                writeObjectStrKey("kind", true);
                                writeObjectValue(() => {
                                    writer.writeLine(`self.f.write_u32(self.w, ${i})?;`);
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
                                    writer.writeLine(`self.f.write_u32(self.w, ${i})?;`);
                                });
                                for (const field of variant.fields) {
                                    writeObjectStrKey(snakeToCamelCase(field.name), false);
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
            writer.writeLine(`self.serialize_${nameToSnakeCase(type.name)}(${name})?;`);
        } else if (type.kind === "Reference" && isSwcAstType(analysisResult, type)) {
            writer.writeLine(`self.serialize_${nameToSnakeCase(type.name)}(${name})?;`);
        } else if (type.kind === "Reference" && isSwcNodeEnumType(analysisResult, type)) {
            writer.writeLine(`self.serialize_${nameToSnakeCase(type.name)}(${name})?;`);
        } else if (type.kind === "Reference" && isOptionType(type)) {
            writer.write(`match ${name} `).inlineBlock(() => {
                writer.write(`Some(value) => `).block(() => {
                    writeTypeValueSerialization("value", type.genericArgs[0]);
                });
                writer.writeLine(`None => self.f.write_null(self.w)?,`);
            });
        } else if (type.kind === "Reference" && isVecType(type)) {
            writeArray(() => {
                writer.write(`for (i, item) in ${name.replace(/^&/, "")}.iter().enumerate() `).inlineBlock(() => {
                    writeArrayValue(() => {
                        writeTypeValueSerialization("item", type.genericArgs[0]);
                    }, "i == 0");
                });
            });
        } else if (type.kind === "Reference" && type.name === "Span") {
            writer.writeLine(`self.serialize_span(${name})?;`);
        } else {
            writer.writeLine(`write!(self.w, "{}", to_json_string(${name})?)?;`);
        }
    }

    function writeCommentSerialization() {
        writer.blankLine();
        writeCommentsSerializationFunction();
        writer.blankLine();
        writeCommentVecSerializationFunction();
        writer.blankLine();
        writeCommentSerializationFunction();
    }

    function writeCommentsSerializationFunction() {
        writer.write(
            `pub fn serialize_comments(&mut self, `
                + `leading: &SingleThreadedCommentsMapInner, trailing: &SingleThreadedCommentsMapInner`
                + `) -> Result<(), Error>`,
        ).block(() => {
            writeObject(() => {
                // leading
                writeObjectStrKey("leading", true);
                writeObjectValue(() => {
                    writeHashMapSerialization("leading");
                });

                // trailing
                writeObjectStrKey("trailing", false);
                writeObjectValue(() => {
                    writeHashMapSerialization("trailing");
                });
            });
            writer.writeLine("Ok(())");
        });

        function writeHashMapSerialization(name: string) {
            writeObject(() => {
                writer.write(`for (i, (key, value)) in ${name}.iter().enumerate()`).block(() => {
                    writeObjectKey(() => {
                        writeString(() => {
                            writer.writeLine(`self.f.write_string_fragment(self.w, &key.0.to_string())?;`);
                        });
                    }, "i == 0");
                    writeObjectValue(() => {
                        writer.writeLine("self.serialize_comment_vec(value)?;");
                    });
                });
            });
        }
    }

    function writeCommentVecSerializationFunction() {
        writer.write(`pub fn serialize_comment_vec(&mut self, comments: &Vec<Comment>) -> Result<(), Error>`).block(() => {
            writeArray(() => {
                writer.write(`for (i, comment) in comments.iter().enumerate()`).block(() => {
                    writeArrayValue(() => {
                        writer.writeLine("self.serialize_comment(comment)?;");
                    }, "i == 0");
                });
            });
            writer.writeLine("Ok(())");
        });
    }

    function writeCommentSerializationFunction() {
        // not worth downloading swc_common to analyze this, so hardcode it
        writer.write(`pub fn serialize_comment(&mut self, comment: &Comment) -> Result<(), Error>`).block(() => {
            writeObject(() => {
                // span
                writer.writeLine("self.serialize_span_props(&comment.span, true)?;");
                // text
                writeObjectStrKey("text", false);
                writeObjectValue(() =>
                    writeString(() => {
                        writer.writeLine("self.f.write_string_fragment(self.w, &comment.text)?;");
                    })
                );
                // kind
                writeObjectStrKey("kind", false);
                writeObjectValue(() => {
                    writer.write("self.f.write_u32(self.w, ");
                    writer.write("match comment.kind ").inlineBlock(() => {
                        writer.writeLine("CommentKind::Line => 0,");
                        writer.writeLine("CommentKind::Block => 1,");
                    }).write(")?;").newLine();
                });
            });
            writer.writeLine("Ok(())");
        });
    }

    function writeSpanSerialization() {
        writer.write(`fn serialize_span(&mut self, span: &Span) -> Result<(), Error>`).block(() => {
            writeObject(() => {
                writer.writeLine("self.serialize_span_props(span, true)?;");
            });
            writer.writeLine("Ok(())");
        });

        writer.blankLine();
        writer.write(`fn serialize_span_props(&mut self, span: &Span, is_first_prop: bool) -> Result<(), Error>`).block(() => {
            writeObjectStrKey("start", "is_first_prop");
            writeObjectValue(() => {
                writer.writeLine("self.f.write_u32(self.w, byte_pos_to_char_pos(&self.multi_byte_chars, span.lo()))?;");
            });
            writeObjectStrKey("end", false);
            writeObjectValue(() => {
                // todo: performance improvement here where it figures out lo and hi char position at the same time
                writer.writeLine("self.f.write_u32(self.w, byte_pos_to_char_pos(&self.multi_byte_chars, span.hi()))?;");
            });
            writer.writeLine("Ok(())");
        });
    }

    function writeArray(inner: () => void) {
        writer.writeLine("self.f.begin_array(self.w)?;");
        inner();
        writer.writeLine("self.f.end_array(self.w)?;");
    }

    function writeArrayValue(inner: () => void, isFirst: boolean | string) {
        writer.writeLine(`self.f.begin_array_value(self.w, ${isFirst})?;`);
        inner();
        writer.writeLine("self.f.end_array_value(self.w)?;");
    }

    function writeObject(inner: () => void) {
        writer.writeLine("self.f.begin_object(self.w)?;");
        inner();
        writer.writeLine("self.f.end_object(self.w)?;");
    }

    function writeObjectStrKey(name: string, isFirst: boolean | string) {
        writeObjectKey(() => {
            writeString(() => {
                writer.writeLine(`self.f.write_string_fragment(self.w, "${name}")?;`);
            });
        }, isFirst);
    }

    function writeString(inner: () => void) {
        writer.writeLine("self.f.begin_string(self.w)?;");
        inner();
        writer.writeLine("self.f.end_string(self.w)?;");
    }

    function writeObjectKey(inner: () => void, isFirst: boolean | string) {
        writer.writeLine(`self.f.begin_object_key(self.w, ${isFirst})?;`);
        inner();
        writer.writeLine("self.f.end_object_key(self.w)?;");
    }

    function writeObjectValue(inner: () => void) {
        writer.writeLine("self.f.begin_object_value(self.w)?;");
        inner();
        writer.writeLine("self.f.end_object_value(self.w)?;");
    }
}
