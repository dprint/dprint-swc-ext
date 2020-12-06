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
        writer.writeLine("// This code is code generated.");
        writer.writeLine("// Run `deno run -A generation/main.ts` from the root directory to regenerate it.");
    }

    function writeUseDeclarations() {
        writer.writeLine("use std::mem::{self, MaybeUninit};");
        writer.writeLine("use swc_common::{Span, Spanned};");
        writer.write("use swc_ecma_ast::{");
        writer.write(analysisResult.enums.filter(e => e.isPlain).map(e => e.name).join(", "));
        writer.write("};").newLine();
        writer.writeLine("use crate::types::*;");
        writer.newLine();
    }

    function writePublicFunction() {
        writer.writeLine("pub fn with_ast_view<'a>(swc_module: swc_ecma_ast::Module, with_view: impl Fn(Module<'a>) -> Module<'a>) -> swc_ecma_ast::Module {");
        writer.indent(() => {
            writer.writeLine("let swc_module_ref = unsafe { mem::transmute::<&swc_ecma_ast::Module, &'a swc_ecma_ast::Module>(&swc_module) };");
            writer.writeLine(`let module = ${getViewForFunctionName("Module")}(swc_module_ref);`);
            writer.writeLine(`let _ = with_view(module);`);
            writer.writeLine(`swc_module`);
        }).write("}").newLine().newLine();
    }

    function writeTraits() {
        for (const name of getNamesUsedInBox(analysisResult)) {
            if (analysisResult.structs.some(s => s.name === name) || analysisResult.enums.some(s => s.name === name)) {
                writeTrait(`From<&Box<${name}<'a>>>`, "Node<'a>", () => {
                    writer.writeLine(`fn from(boxed_node: &Box<${name}<'a>>) -> Node<'a> {`);
                    writer.indent(() => {
                        writer.writeLine("(&**boxed_node).into()");
                    }).write("}").newLine();
                });
                writer.newLine();
            }
        }
    }

    function writeNode() {
        writer.writeLine("#[derive(Clone)]");
        writer.writeLine("pub enum Node<'a> {");
        writer.indent(() => {
            for (const struct of analysisResult.structs) {
                writer.writeLine(`${struct.name}(&'a ${struct.name}<'a>),`);
            }
        });
        writer.writeLine("}").newLine();

        writer.writeLine("impl<'a> Node<'a> {");
        writer.indent(() => {
            writer.writeLine("pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {");
            writer.indent(() => {
                writer.writeLine("T::try_cast(self)");
            }).write("}").newLine().newLine();

            writer.writeLine("pub fn to<T: CastableNode<'a>>(&self) -> &'a T {");
            writer.indent(() => {
                writer.writeLine(`T::try_cast(self).expect("Tried to cast node to incorrect type.")`);
            }).write("}").newLine();
        }).write("}").newLine().newLine();

        writeTrait("Spanned", "Node<'a>", () => {
            implementTraitMethod("span", "Span");
        });

        writeTrait("NodeTrait<'a>", "Node<'a>", () => {
            implementTraitMethod("parent", "Option<Node<'a>>");
            writer.newLine();
            implementTraitMethod("children", "Vec<Node<'a>>");
        });

        function implementTraitMethod(methodName: string, returnType: string) {
            writer.writeLine(`fn ${methodName}(&self) -> ${returnType} {`);
            writer.indent(() => {
                writer.writeLine("match self {");
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
            writer.write(`pub enum ${enumDef.name}<'a> {`);
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

            writer.writeLine(`impl<'a> ${enumDef.name}<'a> {`).indent(() => {
                writer.writeLine("pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {");
                writer.indent(() => {
                    writer.writeLine("T::try_cast(&self.into())");
                }).write("}").newLine().newLine();

                writer.writeLine("pub fn to<T: CastableNode<'a>>(&self) -> &'a T {");
                writer.indent(() => {
                    writer.writeLine(`T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")`);
                }).write("}").newLine();
            }).write("}").newLine().newLine();

            writeTrait("Spanned", `${enumDef.name}<'a>`, () => {
                implementTraitMethod("span", "Span");
            });
            writer.newLine();
            writeTrait("NodeTrait<'a>", `${enumDef.name}<'a>`, () => {
                implementTraitMethod("parent", "Option<Node<'a>>");
                writer.newLine();
                implementTraitMethod("children", "Vec<Node<'a>>");
            });

            writeTrait(`From<&${enumDef.name}<'a>>`, "Node<'a>", () => {
                writer.writeLine(`fn from(node: &${enumDef.name}<'a>) -> Node<'a> {`);
                writer.indent(() => {
                    writer.writeLine("match node {");
                    writer.indent(() => {
                        for (const variant of enumDef.variants) {
                            const fullName = `${enumDef.name}::${variant.name}`;
                            writer.writeLine(`${fullName}(node) => node.into(),`);
                        }
                    }).write("}").newLine();
                }).write("}").newLine();
            });

            function implementTraitMethod(methodName: string, returnType: string) {
                writer.writeLine(`fn ${methodName}(&self) -> ${returnType} {`);
                writer.indent(() => {
                    writer.writeLine("match self {");
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
            writer.write(`fn ${getViewForFunctionName(enumDef.name)}<'a>(ref_node: &'a swc_ecma_ast::${enumDef.name}) -> ${enumDef.name}<'a> {`)
                .newLine();
            writer.indent(() => {
                // writer.writeLine(`println!("Entered ${enumDef.name}");`);
                writer.writeLine("match ref_node {");
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

            writer.writeLine(`fn ${getSetParentForFunctionName(enumDef.name)}<'a>(node: &mut ${enumDef.name}<'a>, parent: Node<'a>) {`);
            writer.indent(() => {
                writer.write("match node {");
                writer.indent(() => {
                    for (const variant of enumDef.variants) {
                        const fullName = `${enumDef.name}::${variant.name}`;
                        writer.newLine();
                        writer.writeLine(`${fullName}(node) => {`);
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
            writer.writeLine(`pub struct ${struct.name}<'a> {`);
            writer.indent(() => {
                if (struct.parents.length > 0) {
                    if (struct.parents.length === 1) {
                        writer.writeLine(`pub parent: &'a ${struct.parents[0].name}<'a>,`);
                    } else {
                        writer.writeLine(`pub parent: Node<'a>,`);
                    }
                }
                writer.writeLine(`pub inner: &'a swc_ecma_ast::${struct.name},`);

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
                writer.write(`impl<'a> ${struct.name}<'a> {`);
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
            writeTrait("Spanned", `${struct.name}<'a>`, () => {
                writer.writeLine("fn span(&self) -> Span {");
                writer.indent(() => {
                    writer.writeLine("self.inner.span()");
                }).write("}").newLine();
            });

            writer.newLine();
            writeTrait(`From<&${struct.name}<'a>>`, "Node<'a>", () => {
                writer.writeLine(`fn from(node: &${struct.name}) -> Node<'a> {`);
                writer.indent(() => {
                    writer.writeLine(`let static_ref = unsafe { mem::transmute::<&${struct.name}, &'a ${struct.name}>(&node) };`);
                    writer.writeLine(`Node::${struct.name}(static_ref)`);
                }).write("}").newLine();
            });

            writer.newLine();
            writeTrait("NodeTrait<'a>", `${struct.name}<'a>`, () => {
                writer.writeLine("fn parent(&self) -> Option<Node<'a>> {");
                writer.indent(() => {
                    if (struct.parents.length === 0) {
                        writer.write("None");
                    } else {
                        if (struct.parents.length === 1) {
                            writer.write("Some(self.parent.into())");
                        } else {
                            writer.write("Some(self.parent.clone())");
                        }
                    }
                    writer.newLine();
                }).write("}").newLine();
                writer.newLine();

                writer.writeLine("fn children(&self) -> Vec<Node<'a>> {");
                writer.indent(() => {
                    if (structFields.length === 0) {
                        writer.write("Vec::with_capacity(0)");
                    } else {
                        writer.writeLine(`let mut children = Vec::with_capacity(${getStructChildrenCapacityExpr()});`);
                        for (const field of structFields) {
                            writeAppendChild(field.type, `self.${field.name}`);
                        }
                        writer.write("children");
                    }
                    writer.newLine();
                }).write("}").newLine();
                writer.newLine();
            });

            writer.newLine();
            writeTrait("CastableNode<'a>", `${struct.name}<'a>`, () => {
                writer.writeLine("fn try_cast(node: &Node<'a>) -> Option<&'a Self> {");
                writer.indent(() => {
                    writer.writeLine(`if let Node::${struct.name}(node) = node {`);
                    writer.indent(() => {
                        writer.writeLine("Some(node)");
                    }).write("} else {").newLine();
                    writer.indent(() => {
                        writer.writeLine("None");
                    }).write("}").newLine();
                }).write("}").newLine();
                writer.newLine();
            });

            function writeAppendChild(type: TypeDefinition, name: string) {
                if (type.kind === "primitive") {
                    throw new Error("Should not have analyzed a primitive type here.");
                }
                if (type.name === "Option") {
                    writer.writeLine(`if let Some(child) = &${name} {`);
                    writer.indent(() => {
                        writeAppendChild(type.generic_args[0], "child");
                    }).write("}").newLine();
                } else if (type.name === "Vec") {
                    writer.writeLine(`for child in ${name}.iter() {`);
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
            writer.write(`fn ${getViewForFunctionName(struct.name)}<'a>(ref_node: &'a swc_ecma_ast::${struct.name}`);
            writer.write(`) -> ${struct.name}<'a> {`).newLine();
            writer.indent(() => {
                // writer.writeLine(`println!("Entered ${struct.name}");`);
                for (const field of structFields) {
                    writer.writeLine(`let value = &ref_node.${field.name};`);
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
                    if (struct.parents.length > 0) {
                        writer.write("parent: unsafe { MaybeUninit::uninit().assume_init() },").newLine();
                    }
                    for (const field of structFields) {
                        writer.writeLine(`${field.name}: field_${field.name},`);
                    }
                }).write("};").newLine();
                if (structFields.length > 0) {
                    writer.writeLine(`let child_parent_ref = unsafe { mem::transmute::<&${struct.name}, &'a ${struct.name}>(&node) };`);
                    writer.writeLine(`let parent = Node::${struct.name}(child_parent_ref);`);

                    for (const [i, field] of structFields.entries()) {
                        const shouldClone = i < structFields.length - 1;
                        writeSetParentStatement(field.type, `node.${field.name}`, shouldClone);
                    }
                }

                // writer.writeLine(`println!("Exited ${struct.name}");`);
                writer.writeLine("node");
            }).write("}").newLine();
        }
    }

    function writeTrait(traitName: string, name: string, body: () => void) {
        writer.write("impl");
        if (traitName.endsWith("<'a>") || name.endsWith("<'a>")) {
            writer.write("<'a>");
        }
        writer.write(` ${traitName} for ${name} {`).newLine();
        writer.indent(() => {
            body();
        }).write("}").newLine();
    }

    function writeGetViewTypeExpression(type: TypeDefinition) {
        if (type.kind === "primitive") {
            throw new Error("Primitive types not handled here.");
        }

        if (type.name === "Option") {
            writer.writeLine(`match value {`);
            writer.indent(() => {
                writer.write("Some(value) => Some(");
                writeGetViewTypeExpression(type.generic_args[0]);
                writer.write("),").newLine();
                writer.writeLine("None => None,");
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
            writer.writeLine(`if let Some(node) = ${name}.as_mut() {`);
            writer.indent(() => {
                writeSetParentStatement(type.generic_args[0], "node", shouldClone);
            }).write("}").newLine();
        } else if (type.name === "Vec") {
            writer.writeLine(`for node in ${name}.iter_mut() {`);
            writer.indent(() => {
                writeSetParentStatement(type.generic_args[0], "node", true);
            });
            writer.writeLine("}");
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
            writer.write(`${name}.parent = `);
            const structOfType = analysisResult.structs.find(s => s.name === type.name);
            if (structOfType != null && structOfType.parents.length === 1) {
                writer.write(`parent.to::<${structOfType.parents[0].name}>()`);
            } else {
                writer.write(`parent`);
                if (shouldClone) {
                    writer.write(".clone()");
                }
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
            if (analysisResult.enums.some(e => !e.isPlain && e.name === type.name) || analysisResult.structs.some(s => s.name === type.name)) {
                if (type.generic_args.length > 0) {
                    throw new Error("Unhandled.");
                }
                writer.write("<'a>");
            } else if (type.generic_args.length > 0) {
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
            writer.writeLine(`/// ${line}`);
        }
    }

    function getViewForFunctionName(name: string) {
        return `get_view_for_${nameToSnakeCase(name)}`;
    }

    function getSetParentForFunctionName(name: string) {
        return `set_parent_for_${nameToSnakeCase(name)}`;
    }
}
