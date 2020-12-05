import {
    AnalysisResult,
    EnumDefinition,
    PrimitiveTypeDefinition,
    StructDefinition,
    TypeDefinition,
    TypeReferenceDefinition,
} from "../analyze/analysis_types.ts";
import { getNamesUsedInBox } from "../analyze/analyze.ts";
import { nameToSnakeCase } from "../utils/stringUtils.ts";
import { Writer } from "./writer.ts";

export function generate(analysisResult: AnalysisResult) {
    const writer = new Writer();

    writeHeader();
    writeUseDeclarations();
    writePublicFunction();
    writeTraits();
    writeNode();

    for (const enumDef of analysisResult.enums.filter(e => !e.isPlain)) {
        writer.newLine();
        writeEnum(enumDef);
    }

    for (const struct of analysisResult.structs) {
        writer.newLine();
        handleStruct(struct);
    }

    return writer.toString();

    function writeHeader() {
        writer.write("// This code is code generated.").newLine();
        writer.write("// Run `deno run -A generation/main.ts` from the root directory to regenerate it.").newLine();
    }

    function writeUseDeclarations() {
        writer.write("use std::mem::{self, MaybeUninit};").newLine();
        writer.write("use swc_common::{Span, Spanned};").newLine();
        writer.write("use swc_ecma_ast::{");
        writer.write(analysisResult.enums.filter(e => e.isPlain).map(e => e.name).join(", "));
        writer.write("};");
        writer.newLine().newLine();
    }

    function writePublicFunction() {
        writer.write("pub fn get_reference_view(module: &swc_ecma_ast::Module) -> Module {").newLine();
        writer.indent(() => {
            writer.write("let module = unsafe { mem::transmute::<&swc_ecma_ast::Module, &'static swc_ecma_ast::Module>(&module) };").newLine();
            writer.write(`${getViewForFunctionName("Module")}(module)`).newLine();
        }).write("}").newLine().newLine();
    }

    function writeTraits() {
        writer.write("pub trait NodeTrait {").newLine();
        writer.indent(() => {
            writer.write("fn parent(&self) -> Option<&Node>;").newLine();
            writer.write("fn children(&self) -> Vec<Node>;").newLine();
        }).write("}").newLine().newLine();

        for (const name of getNamesUsedInBox(analysisResult)) {
            if (analysisResult.structs.some(s => s.name === name) || analysisResult.enums.some(s => s.name === name)) {
                writeTrait(`From<& Box<${name}>>`, "Node", () => {
                    writer.write(`fn from(boxed_node: &Box<${name}>) -> Node {`).newLine();
                    writer.indent(() => {
                        writer.write("(&**boxed_node).into()");
                    }).write("}").newLine();
                });
                writer.newLine();
            }
        }
    }

    function writeNode() {
        writer.write("#[derive(Clone)]").newLine();
        writer.write("pub enum Node {").newLine();
        writer.indent(() => {
            for (const struct of analysisResult.structs) {
                writer.write(`${struct.name}(&'static ${struct.name}),`).newLine();
            }
        });
        writer.write("}").newLine().newLine();

        writeTrait("Spanned", "Node", () => {
            implementTraitMethod("span", "Span");
        });

        writeTrait("NodeTrait", "Node", () => {
            implementTraitMethod("parent", "Option<&Node>");
            writer.newLine();
            implementTraitMethod("children", "Vec<Node>");
        });

        function implementTraitMethod(methodName: string, returnType: string) {
            writer.write(`fn ${methodName}(&self) -> ${returnType} {`).newLine();
            writer.indent(() => {
                writer.write("match self {").newLine();
                writer.indent(() => {
                    for (const struct of analysisResult.structs) {
                        const fullName = `Node::${struct.name}`;
                        writer.write(`${fullName}(node) => node.${methodName}(),`);
                        writer.newLine();
                    }
                }).write("}").newLine();
            }).write("}").newLine();
        }
    }

    function writeEnum(enumDef: EnumDefinition) {
        writeEnum();
        writer.newLine();
        writeEnumFunctions();

        function writeEnum() {
            writeDocs(enumDef.docs);
            writer.write(`pub enum ${enumDef.name} {`);
            writer.indent(() => {
                for (const variant of enumDef.variants) {
                    writer.newLine();
                    writeDocs(variant.docs);
                    writer.write(`${variant.name}`);
                    if (variant.tuple_args != null) {
                        writer.write("(");
                        for (const [i, arg] of variant.tuple_args.entries()) {
                            if (i > 0) {
                                writer.write(", ");
                            }
                            writeType(arg);
                        }
                        writer.write(")");
                    }
                    writer.write(",");
                }
            }).newLine();
            writer.write("}").newLine().newLine();

            writeTrait("Spanned", enumDef.name, () => {
                implementTraitMethod("span", "Span");
            });
            writer.newLine();
            writeTrait("NodeTrait", enumDef.name, () => {
                implementTraitMethod("parent", "Option<&Node>");
                writer.newLine();
                implementTraitMethod("children", "Vec<Node>");
            });

            writeTrait(`From<&${enumDef.name}>`, "Node", () => {
                writer.write(`fn from(node: &${enumDef.name}) -> Node {`).newLine();
                writer.indent(() => {
                    writer.write("match node {").newLine();
                    writer.indent(() => {
                        for (const variant of enumDef.variants) {
                            const fullName = `${enumDef.name}::${variant.name}`;
                            writer.write(`${fullName}(node) => node.into(),`);
                            writer.newLine();
                        }
                    }).write("}").newLine();
                }).write("}").newLine();
            });

            function implementTraitMethod(methodName: string, returnType: string) {
                writer.write(`fn ${methodName}(&self) -> ${returnType} {`).newLine();
                writer.indent(() => {
                    writer.write("match self {").newLine();
                    writer.indent(() => {
                        for (const variant of enumDef.variants) {
                            const fullName = `${enumDef.name}::${variant.name}`;
                            writer.write(`${fullName}(node) => node.${methodName}(),`);
                            writer.newLine();
                        }
                    }).write("}").newLine();
                }).write("}").newLine();
            }
        }

        function writeEnumFunctions() {
            writer.write(`fn ${getViewForFunctionName(enumDef.name)}(ref_node: &'static swc_ecma_ast::${enumDef.name}) -> ${enumDef.name} {`)
                .newLine();
            writer.indent(() => {
                // writer.write(`println!("Entered ${enumDef.name}");`).newLine();
                writer.write("match ref_node {").newLine();
                writer.indent(() => {
                    for (const variant of enumDef.variants) {
                        const fullName = `${enumDef.name}::${variant.name}`;
                        writer.write(`swc_ecma_ast::${fullName}(value) => ${fullName}(`);
                        if (variant.tuple_args?.length !== 1) {
                            throw new Error("Unhandled scenario where the variant's tuple args were not equal to 1.");
                        }
                        writeGetViewTypeExpression(variant.tuple_args[0]);
                        writer.write("),");
                        writer.newLine();
                    }
                }).write("}").newLine();
            }).write("}").newLine().newLine();

            writer.write(`fn ${getSetParentForFunctionName(enumDef.name)}(node: &mut ${enumDef.name}, parent: Node) {`).newLine();
            writer.indent(() => {
                writer.write("match node {");
                writer.indent(() => {
                    for (const variant of enumDef.variants) {
                        const fullName = `${enumDef.name}::${variant.name}`;
                        writer.newLine();
                        writer.write(`${fullName}(node) => {`).newLine();
                        writer.indent(() => {
                            if (variant.tuple_args?.length !== 1) {
                                throw new Error("Unhandled scenario where the variant's tuple args were not equal to 1.");
                            }
                            writeSetParentStatement(variant.tuple_args[0], "node", false);
                        });
                        writer.write("},");
                    }
                }).newLine().write("}").newLine();
            }).write("}").newLine();
        }
    }

    function handleStruct(struct: StructDefinition) {
        const implFields = struct.fields.filter(f => getIsForImpl(f.type));
        const structFields = struct.fields.filter(f => !getIsForImpl(f.type) && f.name !== "span");

        writeStruct();
        writer.newLine();
        writeStructFunction();

        function writeStruct() {
            writeDocs(struct.docs);
            writer.write(`pub struct ${struct.name} {`).newLine();
            writer.indent(() => {
                if (struct.name !== "Module") {
                    writer.write(`pub parent: Node,`).newLine();
                }
                writer.write(`pub inner: &'static swc_ecma_ast::${struct.name},`).newLine();

                for (const field of structFields) {
                    writeDocs(field.docs);
                    writer.write(`pub ${field.name}: `);
                    writeType(field.type);
                    writer.write(",").newLine();
                }
            });
            writer.write("}").newLine();

            if (implFields.length > 0) {
                writer.newLine();
                writer.write(`impl ${struct.name} {`);
                writer.indent(() => {
                    for (const field of implFields) {
                        const isReferenceType = getIsReferenceType(field.type);
                        writer.newLine();

                        writeDocs(field.docs);
                        writer.write(`pub fn ${field.name}(&self) -> `);
                        if (isReferenceType) {
                            writer.write("&");
                        }
                        writeType(field.type);
                        writer.write(` {`).newLine();
                        writer.indent(() => {
                            if (isReferenceType) {
                                writer.write("&");
                            }
                            writer.write(`self.inner.${field.name}`).newLine();
                        }).write("}").newLine();
                    }
                }).write("}").newLine();
            }

            writer.newLine();
            writeTrait("Spanned", struct.name, () => {
                writer.write("fn span(&self) -> Span {").newLine();
                writer.indent(() => {
                    writer.write("self.inner.span()").newLine();
                }).write("}").newLine();
            });

            writer.newLine();
            writeTrait(`From<&${struct.name}>`, "Node", () => {
                writer.write(`fn from(node: &${struct.name}) -> Node {`).newLine();
                writer.indent(() => {
                    writer.write(`let static_ref = unsafe { mem::transmute::<&${struct.name}, &'static ${struct.name}>(&node) };`)
                        .newLine();
                    writer.write(`Node::${struct.name}(static_ref)`).newLine();
                }).write("}").newLine();
            });

            writer.newLine();
            writeTrait("NodeTrait", struct.name, () => {
                writer.write("fn parent(&self) -> Option<&Node> {").newLine();
                writer.indent(() => {
                    if (struct.name === "Module") {
                        writer.write("None");
                    } else {
                        writer.write("Some(&self.parent)");
                    }
                    writer.newLine();
                }).write("}").newLine();
                writer.newLine();

                writer.write("fn children(&self) -> Vec<Node> {").newLine();
                writer.indent(() => {
                    if (structFields.length === 0) {
                        writer.write("Vec::with_capacity(0)");
                    } else {
                        writer.write(`let mut children = Vec::with_capacity(${getStructChildrenCapacityExpr()});`).newLine();
                        for (const field of structFields) {
                            writeAppendChild(field.type, `self.${field.name}`);
                        }
                        writer.write("children");
                    }
                    writer.newLine();
                }).write("}").newLine();
            });

            function writeAppendChild(type: TypeDefinition, name: string) {
                if (type.kind === "primitive") {
                    throw new Error("Should not have analyzed a primitive type here.");
                }
                if (type.name === "Option") {
                    writer.write(`if let Some(child) = &${name} {`).newLine();
                    writer.indent(() => {
                        writeAppendChild(type.generic_args[0], "child");
                    }).write("}").newLine();
                } else if (type.name === "Vec") {
                    writer.write(`for child in ${name}.iter() {`).newLine();
                    writer.indent(() => {
                        writeAppendChild(type.generic_args[0], "child");
                    }).write("}").newLine();
                    return `${name}.len()`;
                } else {
                    writer.write(`children.push(`);
                    if (name.includes(".")) {
                        writer.write(`(&${name})`);
                    } else {
                        writer.write(name);
                    }
                    writer.write(`.into());`).newLine();
                }
            }

            function getStructChildrenCapacityExpr() {
                return generateExpr(analyze());

                function analyze() {
                    let plainCount = 0;
                    const exprs: string[] = [];
                    for (const field of structFields) {
                        const expr = getTypeCapacityExpr(field.type, `self.${field.name}`);
                        if (expr === "1") {
                            plainCount++;
                        } else {
                            exprs.push(expr);
                        }
                    }
                    return { plainCount, exprs };
                }

                function getTypeCapacityExpr(type: TypeDefinition, name: string): string {
                    if (type.kind === "primitive") {
                        throw new Error("Should not have analyzed a primitive type here.");
                    }

                    if (type.name === "Option") {
                        return `match &${name} { Some(_value) => ${getTypeCapacityExpr(type.generic_args[0], "_value")}, None => 0, }`;
                    } else if (type.name === "Vec") {
                        return `${name}.len()`;
                    } else if (type.name === "Box") {
                        return getTypeCapacityExpr(type.generic_args[0], name);
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

        function writeStructFunction() {
            writer.write(`fn ${getViewForFunctionName(struct.name)}(ref_node: &'static swc_ecma_ast::${struct.name}`);
            writer.write(`) -> ${struct.name} {`).newLine();
            writer.indent(() => {
                // writer.write(`println!("Entered ${struct.name}");`).newLine();
                for (const field of structFields) {
                    writer.write(`let value = &ref_node.${field.name};`).newLine();
                    writer.write(`let field_${field.name} = `);
                    writeGetViewTypeExpression(field.type);
                    writer.write(";").newLine();
                }

                writer.write("let ");
                if (structFields.length > 0) {
                    writer.write("mut ");
                }
                writer.write(`node = ${struct.name} {`).newLine();
                writer.indent(() => {
                    writer.write("inner: ref_node,").newLine();
                    if (struct.name !== "Module") {
                        writer.write("parent: unsafe { MaybeUninit::uninit().assume_init() },").newLine();
                    }
                    for (const field of structFields) {
                        writer.write(`${field.name}: field_${field.name},`).newLine();
                    }
                }).write("};").newLine();
                if (structFields.length > 0) {
                    writer.write(`let child_parent_ref = unsafe { mem::transmute::<&${struct.name}, &'static ${struct.name}>(&node) };`)
                        .newLine();
                    writer.write(`let parent = Node::${struct.name}(child_parent_ref);`).newLine();

                    for (const [i, field] of structFields.entries()) {
                        const shouldClone = i < structFields.length - 1;
                        writeSetParentStatement(field.type, `node.${field.name}`, shouldClone);
                    }
                }

                // writer.write(`println!("Exited ${struct.name}");`).newLine();
                writer.write("node").newLine();
            }).write("}").newLine();
        }
    }

    function writeTrait(traitName: string, name: string, body: () => void) {
        writer.write(`impl ${traitName} for ${name} {`).newLine();
        writer.indent(() => {
            body();
        }).write("}").newLine();
    }

    function writeGetViewTypeExpression(type: TypeDefinition) {
        if (type.kind === "primitive") {
            throw new Error("Primitive types not handled here.");
        }

        if (type.name === "Option") {
            writer.write(`match value {`).newLine();
            writer.indent(() => {
                writer.write("Some(value) => Some(");
                writeGetViewTypeExpression(type.generic_args[0]);
                writer.write("),").newLine();
                writer.write("None => None,").newLine();
            }).write("}");
        } else if (type.name === "Vec") {
            writer.write("value.iter().map(|value| ");
            writeGetViewTypeExpression(type.generic_args[0]);
            writer.write(").collect()");
        } else if (type.name === "Box") {
            writer.write("Box::new(");
            writeGetViewTypeExpression(type.generic_args[0]);
            writer.write(")");
        } else {
            writer.write(`${getViewForFunctionName(type.name)}(value)`);
        }
    }

    function writeSetParentStatement(type: TypeDefinition, name: string, shouldClone: boolean) {
        if (type.kind === "primitive") {
            throw new Error("Primitive types not handled here.");
        }

        if (type.name === "Option") {
            writer.write(`if let Some(node) = ${name}.as_mut() {`).newLine();
            writer.indent(() => {
                writeSetParentStatement(type.generic_args[0], "node", shouldClone);
            }).write("}").newLine();
        } else if (type.name === "Vec") {
            writer.write(`for node in ${name}.iter_mut() {`).newLine();
            writer.indent(() => {
                writeSetParentStatement(type.generic_args[0], "node", true);
            });
            writer.write("}").newLine();
        } else if (type.name === "Box") {
            writeSetParentStatement(type.generic_args[0], name, shouldClone);
        } else if (type.path[0] === "swc_ecma_ast" && analysisResult.enums.some(e => !e.isPlain && e.name === type.path[1])) {
            writer.write(`${getSetParentForFunctionName(type.name)}(`);
            if (name.includes(".")) {
                writer.write("&mut ");
            }
            writer.write(`${name}, parent`);
            if (shouldClone) {
                writer.write(".clone()");
            }
            writer.write(");").newLine();
        } else {
            writer.write(`${name}.parent = parent`);
            if (shouldClone) {
                writer.write(".clone()");
            }
            writer.write(";").newLine();
        }
    }

    function writeType(type: TypeDefinition) {
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
            writer.write(path);
            if (type.generic_args.length > 0) {
                writer.write("<");
                writer.write(type.generic_args.map(writeType).join(", "));
                writer.write(">");
            }
        }
    }

    function getIsForImpl(type: TypeDefinition): boolean {
        if (type.kind === "primitive") {
            return true;
        }
        if (type.name === "Option" || type.name === "Box" || type.name === "Vec") {
            return getIsForImpl(type.generic_args[0]);
        }
        if (type.path[0] === "swc_ecma_ast") {
            return analysisResult.enums.some(s => s.isPlain && s.name === type.path[1]);
        }
        return true;
    }

    function getIsReferenceType(type: TypeDefinition): boolean {
        if (type.kind === "primitive") {
            return false;
        }
        if (type.name === "Option") {
            return getIsReferenceType(type.generic_args[0]);
        }
        if (type.path[0] === "swc_ecma_ast") {
            return analysisResult.enums.some(s => s.isPlain && s.name === type.path[1]);
        }
        return true;
    }

    function writeDocs(docs: string) {
        if (docs.length === 0) {
            return;
        }
        const lines = docs.split(/\r?\n/);
        for (const line of lines) {
            writer.write(`/// ${line}`).newLine();
        }
    }

    function getViewForFunctionName(name: string) {
        return `get_view_for_${nameToSnakeCase(name)}`;
    }

    function getSetParentForFunctionName(name: string) {
        return `set_parent_for_${nameToSnakeCase(name)}`;
    }
}
