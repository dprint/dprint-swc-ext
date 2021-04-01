import { AnalysisResult, AstStructDefinition, DocableDefinition, EnumDefinition, PlainEnumDefinition, TypeDefinition } from "../analyze/analysis_types.ts";
import { createWriter } from "../utils/create_writer.ts";
import { getIsForImpl, isOptionType, isVecType, writeHeader } from "./helpers.ts";

export function generateTypeScriptTypes(analysisResult: AnalysisResult): string {
    const writer = createWriter();
    writeHeader(writer);

    writer.writeLine(`import { BigIntValue, BaseNode, Span } from "./types.ts";`);

    for (const enumDef of analysisResult.astEnums) {
        writer.blankLine();
        writeJsDocs(enumDef);
        writer.write(`export type ${enumDef.name} =`);
        for (const variant of enumDef.variants) {
            const tupleArg = variant.tupleArg;
            if (tupleArg.kind !== "Reference") {
                throw new Error("Expected reference type.");
            }
            writer.newLine().indent().write(`| `);
            writer.write(tupleArg.name);
        }
        writer.write(";");
    }

    for (const enumDef of analysisResult.plainEnums) {
        writer.blankLine();
        writePlainEnum(enumDef);
    }

    for (const enumDef of analysisResult.tokenEnums) {
        writer.blankLine();
        const isPlain = enumDef.variants.every(v => v.kind === "Plain");
        if (isPlain) {
            writePlainEnum(enumDef);
        } else {
            writeTokenEnum(enumDef);
        }
    }

    // NodeKind enum
    writer.blankLine();
    writer.write(`export enum NodeKind`).block(() => {
        for (const struct of analysisResult.astStructs) {
            writer.writeLine(`${struct.name},`);
        }
    });

    // Node type
    writer.blankLine();
    writer.write(`export type Node =`);
    for (const struct of analysisResult.astStructs) {
        writer.newLine();
        writer.write(`| ${struct.name}`);
    }
    writer.write(";").newLine();

    for (const struct of analysisResult.astStructs) {
        writer.blankLine();
        writeAstStruct(struct);
    }

    writer.newLine();

    return writer.toString();

    function writeAstStruct(struct: AstStructDefinition) {
        const structFields = struct.fields.filter(f => !getIsForImpl(analysisResult, f.type) && f.name !== "span");
        writeJsDocs(struct);
        writer.write(`export class ${struct.name} extends BaseNode`).block(() => {
            writer.writeLine(`kind!: NodeKind.${struct.name};`);
            if (struct.parents.length > 0) {
                writer.write("parent!: ");
                // settled on this number arbitrarily... it's too noisy to write them all
                if (struct.parents.length <= 4) {
                    for (const [i, parent] of struct.parents.entries()) {
                        if (i > 0) {
                            writer.newLine();
                            writer.indent().write("| ");
                        }
                        writer.write(parent.name);
                    }
                } else {
                    writer.write("Node");
                }
                writer.write(";").newLine();
            }
            for (const field of struct.fields) {
                writeJsDocs(field);
                writer.write(field.name).write("!: ");
                writeType(field.type);
                writer.write(";").newLine();
            }

            writer.blankLine();
            writeGetChildrenMethod();
        });

        function writeGetChildrenMethod() {
            writer.write("getChildren(): Node[]").block(() => {
                if (structFields.length === 0) {
                    writer.writeLine("return new Array(0);");
                } else {
                    writer.writeLine(`const children: Node[] = new Array(${getStructChildrenCapacityExpr()});`);
                    writer.writeLine(`let i = 0;`);
                    for (const field of structFields) {
                        writeAppendChild(field.type, `this.${field.name}`);
                    }
                    writer.writeLine("return children;");
                }
            });

            function writeAppendChild(type: TypeDefinition, name: string) {
                if (type.kind === "Primitive") {
                    throw new Error("Should not have analyzed a primitive type here.");
                }
                if (isOptionType(type)) {
                    writer.write(`if (${name} != null)`).block(() => {
                        writeAppendChild(type.genericArgs[0], name);
                    });
                } else if (isVecType(type)) {
                    writer.write(`for (const child of ${name})`).block(() => {
                        writeAppendChild(type.genericArgs[0], "child");
                    });
                } else {
                    writer.write(`children[i++] = `);
                    writer.write(name);
                    writer.write(";").newLine();
                }
            }

            function getStructChildrenCapacityExpr() {
                return generateExpr(analyze());

                function analyze() {
                    let plainCount = 0;
                    const exprs: string[] = [];
                    for (const field of structFields) {
                        const expr = getTypeCapacityExpr(field.type, `this.${field.name}`);
                        if (expr === "1") {
                            plainCount++;
                        } else {
                            exprs.push(expr);
                        }
                    }
                    return { plainCount, exprs };
                }

                function getTypeCapacityExpr(type: TypeDefinition, name: string): string {
                    if (type.kind === "Primitive") {
                        throw new Error("Should not have analyzed a primitive type here.");
                    }

                    if (type.name === "Option") {
                        return `(${name} == null ? 0 : ${getTypeCapacityExpr(type.genericArgs[0], name)})`;
                    } else if (type.name === "Vec") {
                        return `${name}.length`;
                    } else {
                        return "1";
                    }
                }

                function generateExpr({ plainCount, exprs }: { plainCount: number; exprs: string[] }) {
                    let finalExpr = "";
                    if (plainCount > 0) {
                        finalExpr = plainCount.toString();
                    }
                    for (const expr of exprs) {
                        if (finalExpr.length > 0) {
                            finalExpr += " + ";
                        }
                        finalExpr += expr;
                    }
                    return finalExpr;
                }
            }
        }
    }

    function writePlainEnum(enumDef: EnumDefinition | PlainEnumDefinition) {
        writeJsDocs(enumDef);
        writer.write(`export enum ${enumDef.name}`).block(() => {
            for (const variant of enumDef.variants) {
                writeJsDocs(variant);
                writer.writeLine(`${variant.name},`);
            }
        });
    }

    function writeTokenEnum(enumDef: EnumDefinition) {
        writeJsDocs(enumDef);
        writer.write(`export type ${enumDef.name} =`);
        for (const variant of enumDef.variants) {
            writer.newLine().indent().write("| ");
            if (variant.kind === "Plain") {
                writer.write(`${enumDef.name}Kind.${variant.name}`);
            } else {
                writer.write(`${enumDef.name}${variant.name}`);
            }
        }
        writer.write(";");

        writer.blankLine();
        writer.write(`export enum ${enumDef.name}Kind`).block(() => {
            for (const variant of enumDef.variants) {
                if (variant.kind === "Plain") {
                    writeJsDocs(variant);
                }
                writer.writeLine(`${variant.name},`);
            }
        });

        for (const variant of enumDef.variants.filter(v => v.kind !== "Plain")) {
            writer.blankLine();
            writeJsDocs(variant);
            writer.write(`export interface ${enumDef.name}${variant.name}`).block(() => {
                writer.writeLine(`kind: ${enumDef.name}Kind.${variant.name};`);
                switch (variant.kind) {
                    case "Struct":
                        for (const field of variant.fields) {
                            writer.write(`${field.name}: `);
                            writeType(field.type);
                            writer.write(";").newLine();
                        }
                        break;
                    case "Tuple":
                        writer.write(`inner: `);
                        if (variant.tupleArgs.length == 1) {
                            writeType(variant.tupleArgs[0]);
                            writer.write(";").newLine();
                        } else {
                            writer.write(`[`);
                            for (const [i, tupleArg] of variant.tupleArgs.entries()) {
                                writer.conditionalWrite(i > 0, ", ");
                                writeType(tupleArg);
                            }
                            writer.write(`];`).newLine();
                        }
                        break;
                }
            });
        }
    }

    function writeType(type: TypeDefinition, isInArray = false) {
        switch (type.kind) {
            case "Primitive":
                if (type.text === "bool") {
                    writer.write("boolean");
                } else if (type.text === "f64") {
                    writer.write("number");
                } else {
                    writer.write(type.text);
                }
                break;
            case "Reference":
                if (isOptionType(type)) {
                    if (type.genericArgs.length !== 1) {
                        throw new Error("Expected 1 type argument.");
                    }
                    writeType(type.genericArgs[0]);
                    if (isInArray) {
                        // arrays are serialized with empty values being `null` instead of `undefined`
                        writer.write(" | null");
                    } else {
                        writer.write(" | undefined");
                    }
                } else if (isVecType(type)) {
                    if (type.genericArgs.length !== 1) {
                        throw new Error("Expected 1 type argument.");
                    }
                    writer.write("Array<");
                    writeType(type.genericArgs[0], true);
                    writer.write(">");
                } else if (type.name === "AssignOpToken") {
                    writer.write("AssignOp");
                } else if (type.name === "JsWord") {
                    writer.write("string");
                } else {
                    writer.write(type.name);
                    if (type.genericArgs.length > 0) {
                        writer.write("<");
                        for (const [i, arg] of type.genericArgs.entries()) {
                            writer.conditionalWrite(i > 0, ", ");
                            writeType(arg);
                        }
                        writer.write(">");
                    }
                    break;
                }
        }
    }

    function writeJsDocs(node: DocableDefinition) {
        if (node.docs == null) {
            return;
        }
        const lines = node.docs.split(/\r?\n/);
        if (lines.length === 1 && lines[0].trim().length > 0) {
            writer.writeLine(`/** ${lines[0].trim()} */`);
        } else if (lines.length > 1) {
            writer.writeLine("/**");
            for (const line of lines) {
                writer.writeLine(` * ${line.trim()}`);
            }
            writer.writeLine(" */");
        }
    }
}

export function generateTypeScriptSetup(analysisResult: AnalysisResult): string {
    const writer = createWriter();

    writeHeader(writer);

    writer.writeLine(`import * as types from "./types.generated.ts";`).blankLine();

    writer.write("export function setupModule(module: any): types.Module").block(() => {
        writer.writeLine(`visitNode(module, undefined);`);
        writer.writeLine(`return module as types.Module;`);
    }).blankLine();

    writer.write(`function visitNode(node: any, parent: any)`).block(() => {
        writer.writeLine("Object.setPrototypeOf(node, getNodeClass(node).prototype);");
        writer.writeLine("node.parent = parent;");
        writer.blankLine();
        writer.write("for (const key of Object.keys(node))").block(() => {
            writer.writeLine("const obj = node[key];");
            writer.write(`if (obj != null && typeof obj === "object" && typeof obj.kind === "string")`).block(() => {
                writer.writeLine("visitNode(obj, node);");
            });
        });
    }).blankLine();

    writer.write("function getNodeClass(node: any)").block(() => {
        writer.write("switch (node.kind)").block(() => {
            for (const struct of analysisResult.astStructs) {
                writer.writeLine(`case types.NodeKind.${struct.name}:`).indent(() => {
                    writer.writeLine(`return types.${struct.name};`);
                });
            }
            writer.writeLine("default:").indent(() => {
                writer.writeLine(`throw new Error("Unknown node kind: " + node.kind);`);
            });
        });
    }).newLine();

    return writer.toString();
}
