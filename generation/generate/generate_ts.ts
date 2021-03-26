import { AnalysisResult, StructDefinition, TypeDefinition } from "../analyze/analysis_types.ts";
import { createWriter } from "../utils/createWriter.ts";
import { writeHeader } from "../utils/generationUtils.ts";

export function generateTypeScriptTypes(analysisResult: AnalysisResult): string {
    const writer = createWriter();
    writeHeader(writer);

    writer.writeLine(`import { BigIntValue, JsWord, Node, Span } from "./types.ts";`);
    writer.blankLine();

    for (const struct of analysisResult.structs) {
        writer.blankLine();
        writeJsDocs(struct.docs);
        writer.write(`export class ${struct.name} extends Node`).block(() => {
            writer.writeLine(`kind!: "${struct.name}";`);
            if (struct.parents.length > 0) {
                writer.write("parent!: ");
                for (const [i, parent] of struct.parents.entries()) {
                    if (i > 0) {
                        writer.newLine();
                        writer.indent().write("| ");
                    }
                    writer.write(parent.name);
                }
                writer.write(";").newLine();
            }
            for (const field of struct.fields) {
                writeJsDocs(field.docs);
                writer.write(field.name).write("!: ");
                writeType(field.type);
                writer.write(";").newLine();
            }
        });
    }

    for (const enumDef of analysisResult.enums) {
        writer.blankLine();
        writeJsDocs(enumDef.docs);
        if (enumDef.isPlain) {
            writer.write(`export enum ${enumDef.name}`).block(() => {
                for (const variant of enumDef.variants) {
                    writeJsDocs(variant.docs);
                    writer.writeLine(`${variant.name},`);
                    if (variant.tuple_args != null && variant.tuple_args.length > 0) {
                        throw new Error("Unexpected scenario where plain enum had variant with tuple args.");
                    }
                }
            });
        } else {
            writer.write(`export type ${enumDef.name} =`);
            for (const variant of enumDef.variants) {
                if (variant.tuple_args?.length !== 1) {
                    throw new Error("Expected 1 tuple arg.");
                }
                const firstTupleArg = variant.tuple_args[0];
                if (firstTupleArg.kind !== "reference") {
                    throw new Error("Expected reference type.");
                }
                writer.newLine().indent().write(`| `);
                writer.write(firstTupleArg.name);
            }
            writer.write(";");
        }
    }

    return writer.toString();

    function writeType(type: TypeDefinition) {
        switch (type.kind) {
            case "primitive":
                if (type.text === "bool") {
                    writer.write("boolean");
                } else if (type.text === "f64") {
                    writer.write("number");
                } else {
                    writer.write(type.text);
                }
                break;
            case "reference":
                if (type.name === "Option") {
                    if (type.generic_args.length !== 1) {
                        throw new Error("Expected 1 type argument.");
                    }
                    writeType(type.generic_args[0]);
                    writer.write(" | undefined");
                } else if (type.name === "Vec") {
                    if (type.generic_args.length !== 1) {
                        throw new Error("Expected 1 type argument.");
                    }
                    writer.write("Array<");
                    writeType(type.generic_args[0]);
                    writer.write(">");
                } else {
                    writer.write(type.name);
                    if (type.generic_args.length > 0) {
                        writer.write("<");
                        for (const [i, arg] of type.generic_args.entries()) {
                            writer.conditionalWrite(i > 0, ", ");
                            writeType(arg);
                        }
                        writer.write(">");
                    }
                    break;
                }
        }
    }

    function writeJsDocs(docs: string | undefined) {
        if (docs == null) {
            return;
        }
        writer.writeLine("/**");
        for (const line of docs.split(/\r?\n/)) {
            writer.writeLine(` * ${line}`);
        }
        writer.writeLine(" */");
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
            writer.write(`if (typeof obj === "object" && typeof obj.kind === "string")`).block(() => {
                writer.writeLine("visitNode(obj, node);");
            });
        });
    }).blankLine();

    writer.write("function getNodeClass(node: any)").block(() => {
        writer.write("switch (node.kind)").block(() => {
            for (const struct of analysisResult.structs) {
                writer.writeLine(`case "${struct.name}":`).indent(() => {
                    writer.writeLine(`return types.${struct.name};`);
                });
            }
            writer.writeLine("default:").indent(() => {
                writer.writeLine(`throw new Error("Unknown node kind: " + node.kind);`);
            });
        });
    });

    return writer.toString();
}
