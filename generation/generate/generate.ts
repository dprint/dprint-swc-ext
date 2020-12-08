import {
    AnalysisResult,
    EnumDefinition,
    PrimitiveTypeDefinition,
    StructDefinition,
    TypeDefinition,
    TypeReferenceDefinition,
} from "../analyze/analysis_types.ts";
import { nameToSnakeCase } from "../utils/stringUtils.ts";
import { Writer } from "./writer.ts";

export function generate(analysisResult: AnalysisResult) {
    const writer = new Writer();

    writeHeader();
    writeUseDeclarations();
    writePublicFunction();
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
        writer.writeLine("use std::cell::UnsafeCell;");
        writer.writeLine("use std::mem;");
        writer.writeLine("use bumpalo::Bump;");
        writer.writeLine("use swc_common::{Span, Spanned};");
        writer.write("use swc_ecmascript::ast::{self as swc_ast, ");
        writer.write(analysisResult.enums.filter(e => e.isPlain).map(e => e.name).join(", "));
        writer.write("};").newLine();
        writer.writeLine("use crate::types::*;");
        writer.newLine();
    }

    function writePublicFunction() {
        writer.writeLine("pub fn with_ast_view<'a, T>(source_file_info: SourceFileInfo, with_view: impl FnOnce(&'a Module<'a>) -> T) -> T {");
        writer.indent(() => {
            writer.writeLine("let bump = Bump::new();");
            // hack to avoid yet another lifetime
            writer.writeLine("let bump_ref = unsafe { mem::transmute::<&Bump, &'a Bump>(&bump) };");
            writer.writeLine("let info_ref = unsafe { mem::transmute::<&SourceFileInfo, &'a SourceFileInfo<'a>>(&source_file_info) };");
            writer.writeLine(`let ast_view = ${getViewForFunctionName("Module")}(info_ref, bump_ref);`);
            writer.writeLine(`with_view(ast_view)`);
        }).write("}").newLine().newLine();
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
        writer.newLine();

        writeTrait("NodeTrait<'a>", "Node<'a>", () => {
            implementTraitMethod("parent", "Option<Node<'a>>");
            writer.newLine();
            implementTraitMethod("children", "Vec<Node<'a>>");
            writer.newLine();
            implementTraitMethod("into_node", "Node<'a>");
        });

        function implementTraitMethod(methodName: string, returnType: string, hasLifetime = false) {
            writer.write(`fn ${methodName}`);
            if (hasLifetime) {
                writer.write("<'a>");
            }
            writer.write(`(&`);
            if (hasLifetime) {
                writer.write("'a ");
            }
            writer.write(`self) -> ${returnType} {`).newLine();
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
                            writeType(arg, true);
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
                implementTraitMethod("parent", "Option<Node<'a>>", false);
                writer.newLine();
                implementTraitMethod("children", "Vec<Node<'a>>", false);
                writer.newLine();
                implementTraitMethod("into_node", "Node<'a>", false);
            });

            writeTrait(`From<&${enumDef.name}<'a>>`, "Node<'a>", () => {
                writer.writeLine(`fn from(node: &${enumDef.name}<'a>) -> Node<'a> {`);
                writer.indent(() => {
                    writer.writeLine("match node {");
                    writer.indent(() => {
                        for (const variant of enumDef.variants) {
                            const fullName = `${enumDef.name}::${variant.name}`;
                            writer.write(`${fullName}(node) => `);
                            if (isSwcStructType(variant.tuple_args?.[0])) {
                                writer.write(`(*node).into(),`);
                            } else {
                                writer.write(`node.into(),`);
                            }
                            writer.newLine();
                        }
                    }).write("}").newLine();
                }).write("}").newLine();
            });

            function implementTraitMethod(methodName: string, returnType: string, hasSelfLifetime = false) {
                writer.write(`fn ${methodName}`);
                if (hasSelfLifetime) {
                    writer.write("<'a>");
                }
                writer.write(`(&`);
                if (hasSelfLifetime) {
                    writer.write("'a ");
                }
                writer.write(`self) -> ${returnType} {`).newLine();
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
            writer.write(`fn ${getViewForFunctionName(enumDef.name)}<'a>(ref_node: &'a swc_ast::${enumDef.name}, bump: &'a Bump) -> ${enumDef.name}<'a> {`)
                .newLine();
            writer.indent(() => {
                // writer.writeLine(`println!("Entered ${enumDef.name}");`);
                writer.writeLine("match ref_node {");
                writer.indent(() => {
                    for (const variant of enumDef.variants) {
                        const fullName = `${enumDef.name}::${variant.name}`;
                        writer.write(`swc_ast::${fullName}(value) => ${fullName}(`);
                        if (variant.tuple_args?.length !== 1) {
                            throw new Error("Unhandled scenario where the variant's tuple args were not equal to 1.");
                        }
                        writeGetViewTypeExpression(variant.tuple_args[0]);
                        writer.write("),");
                        writer.newLine();
                    }
                }).write("}").newLine();
            }).write("}").newLine().newLine();

            writer.writeLine(`fn ${getSetParentForFunctionName(enumDef.name)}<'a>(node: &${enumDef.name}<'a>, parent: Node<'a>) {`);
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
                        writer.writeLine(`inner_parent: UnsafeCell<Option<&'a ${struct.parents[0].name}<'a>>>,`);
                    } else {
                        writer.writeLine(`inner_parent: UnsafeCell<Option<Node<'a>>>,`);
                    }
                }
                if (struct.name === "Module") {
                    writer.writeLine("pub text: Option<&'a str>,");
                    writer.writeLine("pub tokens: Option<&'a Vec<swc_ecmascript::parser::token::TokenAndSpan>>,");
                }
                writer.writeLine(`pub inner: &'a swc_ast::${struct.name},`);

                for (const field of structFields) {
                    writeDocs(field.docs);
                    writer.write(`pub ${field.name}: `);
                    writeType(field.type, true);
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
                        writeType(field.type, false);
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
                writer.writeLine(`fn from(node: &${struct.name}<'a>) -> Node<'a> {`);
                writer.indent(() => {
                    // hack to not require people having to specify the lifetime twice
                    writer.writeLine(`let node = unsafe { mem::transmute::<&${struct.name}<'a>, &'a ${struct.name}<'a>>(node) };`);
                    writer.writeLine(`Node::${struct.name}(node)`);
                }).write("}").newLine();
            });

            writer.newLine();
            writeTrait("NodeTrait<'a>", `&'a ${struct.name}<'a>`, () => {
                writer.writeLine("fn parent(&self) -> Option<Node<'a>> {");
                writer.indent(() => {
                    if (struct.parents.length === 0) {
                        writer.write("None");
                    } else {
                        if (struct.parents.length === 1) {
                            writer.write("Some(unsafe { (*(*self.inner_parent.get()).as_ref().unwrap()).into() })");
                        } else {
                            writer.write("Some(unsafe { (*self.inner_parent.get()).as_ref().unwrap().clone() })");
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
                            writeAppendChild(field.type, `self.${field.name}`, false, false);
                        }
                        writer.write("children");
                    }
                    writer.newLine();
                }).write("}").newLine();
                writer.newLine();

                writer.writeLine("fn into_node(&self) -> Node<'a> {");
                writer.indent(() => {
                    writer.writeLine("(*self).into()");
                }).write("}").newLine();
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
            });

            function writeAppendChild(type: TypeDefinition, name: string, inOption: boolean, inVec: boolean) {
                if (type.kind === "primitive") {
                    throw new Error("Should not have analyzed a primitive type here.");
                }
                if (type.name === "Option") {
                    writer.write(`if let Some(child) = ${name}`);
                    if (isSwcNodeEnumType(type.generic_args[0]) || isVecType(type.generic_args[0])) {
                        writer.write(".as_ref()");
                    }
                    writer.write(" {").newLine();
                    writer.indent(() => {
                        writeAppendChild(type.generic_args[0], "child", true, inVec);
                    }).write("}").newLine();
                } else if (type.name === "Vec") {
                    writer.writeLine(`for child in ${name}.iter() {`);
                    writer.indent(() => {
                        writeAppendChild(type.generic_args[0], "child", false, true);
                    }).write("}").newLine();
                    return `${name}.len()`;
                } else {
                    writer.write(`children.push(`);
                    if (inVec) {
                        if (isSwcStructType(type)) {
                            writer.write(`(*${name})`);
                        } else {
                            writer.write(name);
                        }
                    } else if (inOption) {
                        writer.write(name);
                    } else if (isSwcNodeEnumType(type)) {
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
            writer.write(`fn ${getViewForFunctionName(struct.name)}<'a>(`);
            if (struct.name === "Module") {
                writer.write(`source_file_info: &'a SourceFileInfo<'a>`);
            } else {
                writer.write(`ref_node: &'a swc_ast::${struct.name}`);
            }
            writer.write(", bump: &'a Bump");
            writer.write(`) -> &'a ${struct.name}<'a> {`).newLine();
            writer.indent(() => {
                if (struct.name === "Module") {
                    writer.writeLine("let ref_node = source_file_info.module;");
                }
                // writer.writeLine(`println!("Entered ${struct.name}");`);
                for (const field of structFields) {
                    writer.writeLine(`let value = &ref_node.${field.name};`);
                    writer.write(`let field_${field.name} = `);
                    writeGetViewTypeExpression(field.type);
                    writer.write(";").newLine();
                }

                writer.write(`let node = bump.alloc(${struct.name} {`).newLine();
                writer.indent(() => {
                    writer.write("inner: ref_node,").newLine();
                    if (struct.parents.length > 0) {
                        writer.write("inner_parent: UnsafeCell::new(None),").newLine();
                    }
                    if (struct.name === "Module") {
                        writer.write("text: source_file_info.file_text,").newLine();
                        writer.write("tokens: source_file_info.tokens,").newLine();
                    }
                    for (const field of structFields) {
                        writer.writeLine(`${field.name}: field_${field.name},`);
                    }
                }).write("});").newLine();
                if (structFields.length > 0) {
                    writer.writeLine(`let parent = Node::${struct.name}(node);`);

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
            /*writer.write("{").newLine();
            writer.indent(() => {
                writer.writeLine("let mut vec = BumpVec::with_capacity_in(value.len(), bump);");
                writer.writeLine("for value in value.iter() {");
                writer.indent(() => {
                    writer.write("vec.push(");
                    writeGetViewTypeExpression(type.generic_args[0]);
                    writer.write(")").newLine();
                }).write("}").newLine();
                writer.writeLine("vec");
            }).write("}");*/
        } else {
            writer.write(`${getViewForFunctionName(type.name)}(value, bump)`);
        }
    }

    function writeSetParentStatement(type: TypeDefinition, name: string, shouldClone: boolean) {
        if (type.kind === "primitive") {
            throw new Error("Primitive types not handled here.");
        }

        if (type.name === "Option") {
            writer.write(`if let Some(child) = ${name}`);
            if (isSwcNodeEnumType(type.generic_args[0]) || isVecType(type.generic_args[0])) {
                writer.write(".as_ref()");
            }
            writer.write(" {").newLine();
            writer.indent(() => {
                writeSetParentStatement(type.generic_args[0], "child", shouldClone);
            }).write("}").newLine();
        } else if (type.name === "Vec") {
            writer.writeLine(`for node in ${name}.iter() {`);
            writer.indent(() => {
                writeSetParentStatement(type.generic_args[0], "node", true);
            });
            writer.writeLine("}");
        } else if (isSwcNodeEnumType(type)) {
            writer.write(`${getSetParentForFunctionName(type.name)}(`);
            if (name.includes(".")) {
                writer.write("&");
            }
            writer.write(`${name}, parent`);
            if (shouldClone) {
                writer.write(".clone()");
            }
            writer.write(");").newLine();
        } else {
            writer.write("unsafe { ");
            writer.write(`*${name}.inner_parent.get() = Some(`);
            const structOfType = analysisResult.structs.find(s => s.name === type.name);
            if (structOfType != null && structOfType.parents.length === 1) {
                writer.write(`parent.to::<${structOfType.parents[0].name}>()`);
            } else {
                writer.write(`parent`);
                if (shouldClone) {
                    writer.write(".clone()");
                }
            }
            writer.write("); }").newLine();
        }
    }

    function writeType(type: TypeDefinition, writeStructReference: boolean) {
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
            if (analysisResult.enums.some(e => !e.isPlain && e.name === type.name) || analysisResult.structs.some(s => s.name === type.name)) {
                if (type.generic_args.length > 0) {
                    throw new Error("Unhandled.");
                }
                if (analysisResult.structs.some(s => s.name === type.name) && writeStructReference) {
                    writer.write("&'a ");
                }
                writer.write(path);
                writer.write("<'a>");
            } else if (type.generic_args.length > 0) {
                writer.write(path);
                writer.write("<");
                writer.write(type.generic_args.map(type => writeType(type, writeStructReference)).join(", "));
                writer.write(">");
            } else {
                writer.write(path);
            }
        }
    }

    function isSwcNodeEnumType(type: TypeDefinition | undefined): boolean {
        return type != null && type.kind === "reference" && analysisResult.enums.some(e => !e.isPlain && e.name === type.name);
    }

    function isSwcStructType(type: TypeDefinition | undefined): boolean {
        return type != null && type.kind === "reference" && analysisResult.structs.some(s => s.name === type.name);
    }

    function isVecType(type: TypeDefinition | undefined): boolean {
        return type != null && type.kind === "reference" && type.name === "Vec";
    }

    function getIsForImpl(type: TypeDefinition): boolean {
        if (type.kind === "primitive") {
            return true;
        }
        if (type.name === "Option" || type.name === "Vec") {
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
