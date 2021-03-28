import { AnalysisResult, AstEnumDefinition, AstStructDefinition, EnumDefinition, TypeDefinition } from "../analyze/analysis_types.ts";
import { createWriter } from "../utils/create_writer.ts";
import { nameToSnakeCase, snakeToCamelCase } from "../utils/string_utils.ts";
import { isOptionType, isSwcAstType, isSwcNodeEnumType, isVecType, writeHeader } from "./helpers.ts";

export function generateSerialize(analysisResult: AnalysisResult): string {
    const writer = createWriter();

    writeHeader(writer);
    writeUseDeclarations();

    for (const [i, struct] of analysisResult.astStructs.entries()) {
        writer.blankLine();
        writeAstStructFunction(struct, i);
    }

    for (const enumDef of analysisResult.astEnums) {
        writer.blankLine();
        writeAstEnumFunction(enumDef);
    }

    writer.blankLine();
    writeTokenSerialization();
    writer.blankLine();
    writeCommentSerialization();

    writer.newLineIfLastNot();

    return writer.toString();

    function writeUseDeclarations() {
        writer.writeLine("use std::io::{Error, Write};");
        writer.writeLine("use serde_json::ser::{Formatter as JsonFormatter, to_string as to_json_string};");
        writer.writeLine("use swc_common::{Spanned, comments::{Comment, CommentKind, SingleThreadedCommentsMapInner}};");
        writer.writeLine("use swc_ecmascript::parser::token::{BinOpToken, Keyword, Token, TokenAndSpan, Word};");
        writer.writeLine("use swc_ecmascript::ast::*;");
    }

    function writeAstStructFunction(struct: AstStructDefinition, index: number) {
        writer.write(
            `pub fn serialize_${nameToSnakeCase(struct.name)}(`
                + `w: &mut impl Write, f: &mut impl JsonFormatter, node: &${struct.name}) -> Result<(), Error>`,
        ).block(() => {
            writeObject(() => {
                writeObjectStrKey("kind", true);
                writeObjectValue(() => writer.writeLine(`f.write_u32(w, ${index})?;`));
                writeObjectStrKey("span", false);
                writeObjectValue(() => writer.writeLine(`write!(w, "{}", to_json_string(&node.span())?)?;`));

                for (const field of struct.fields.filter(f => f.name !== "span")) {
                    writeObjectStrKey(snakeToCamelCase(field.name), false);
                    writeObjectValue(() => writeTypeValueSerialization(`&node.${field.innerName}`, field.type));
                }
            });
            writer.writeLine("Ok(())");
        });
    }

    function writeAstEnumFunction(enumDef: AstEnumDefinition) {
        writer.write(
            `pub fn serialize_${nameToSnakeCase(enumDef.name)}(`
                + `w: &mut impl Write, f: &mut impl JsonFormatter, node: &${enumDef.name}) -> Result<(), Error>`,
        ).block(() => {
            writer.write("match node").block(() => {
                for (const variant of enumDef.variants) {
                    writer.writeLine(`${enumDef.name}::${variant.name}(node) => serialize_${nameToSnakeCase(variant.tupleArg.name)}(w, f, node)?,`);
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
            writer.writeLine(`serialize_${nameToSnakeCase(type.name)}(w, f, ${name})?;`);
        } else if (type.kind === "Reference" && isSwcAstType(analysisResult, type)) {
            writer.writeLine(`serialize_${nameToSnakeCase(type.name)}(w, f, ${name})?;`);
        } else if (type.kind === "Reference" && isSwcNodeEnumType(analysisResult, type)) {
            writer.writeLine(`serialize_${nameToSnakeCase(type.name)}(w, f, ${name})?;`);
        } else if (type.kind === "Reference" && isOptionType(type)) {
            writer.write(`match ${name} `).inlineBlock(() => {
                writer.write(`Some(value) => `).block(() => {
                    writeTypeValueSerialization("value", type.genericArgs[0]);
                });
                writer.writeLine(`None => f.write_null(w)?,`);
            });
        } else if (type.kind === "Reference" && isVecType(type)) {
            writeArray(() => {
                writer.write(`for (i, item) in ${name.replace(/^&/, "")}.iter().enumerate() `).inlineBlock(() => {
                    writeArrayValue(() => {
                        writeTypeValueSerialization("item", type.genericArgs[0]);
                    }, "i == 0");
                });
            });
        } else {
            writer.writeLine(`write!(w, "{}", to_json_string(${name})?)?;`);
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
            `pub fn serialize_comments(w: &mut impl Write, f: &mut impl JsonFormatter, `
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
                            writer.writeLine(`f.write_string_fragment(w, &key.0.to_string())?;`);
                        });
                    }, "i == 0");
                    writeObjectValue(() => {
                        writer.writeLine("serialize_comment_vec(w, f, value)?;");
                    });
                });
            });
        }
    }

    function writeCommentVecSerializationFunction() {
        writer.write(`pub fn serialize_comment_vec(w: &mut impl Write, f: &mut impl JsonFormatter, comments: &Vec<Comment>) -> Result<(), Error>`).block(() => {
            writeArray(() => {
                writer.write(`for (i, comment) in comments.iter().enumerate()`).block(() => {
                    writeArrayValue(() => {
                        writer.writeLine("serialize_comment(w, f, comment)?;");
                    }, "i == 0");
                });
            });
            writer.writeLine("Ok(())");
        });
    }

    function writeCommentSerializationFunction() {
        // not worth downloading swc_common to analyze this, so hardcode it
        writer.write(`pub fn serialize_comment(w: &mut impl Write, f: &mut impl JsonFormatter, comment: &Comment) -> Result<(), Error>`).block(() => {
            writeObject(() => {
                // span
                writeObjectStrKey("span", true);
                writeObjectValue(() => writer.writeLine(`write!(w, "{}", to_json_string(&comment.span)?)?;`));
                // text
                writeObjectStrKey("text", false);
                writeObjectValue(() =>
                    writeString(() => {
                        writer.writeLine("f.write_string_fragment(w, &comment.text)?;");
                    })
                );
                // kind
                writeObjectStrKey("kind", false);
                writeObjectValue(() => {
                    writer.write("f.write_u32(w, ");
                    writer.write("match comment.kind ").inlineBlock(() => {
                        writer.writeLine("CommentKind::Line => 0,");
                        writer.writeLine("CommentKind::Block => 1,");
                    }).write(")?;").newLine();
                });
            });
            writer.writeLine("Ok(())");
        });
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
            writeString(() => {
                writer.writeLine(`f.write_string_fragment(w, "${name}")?;`);
            });
        }, isFirst);
    }

    function writeString(inner: () => void) {
        writer.writeLine("f.begin_string(w)?;");
        inner();
        writer.writeLine("f.end_string(w)?;");
    }

    function writeObjectKey(inner: () => void, isFirst: boolean | string) {
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
