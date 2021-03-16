import {
    AnalysisResult,
    EnumDefinition,
    EnumVariantDefinition,
    StructDefinition,
    TypeDefinition,
} from "../analyze/analysis_types.ts";
import { nameToSnakeCase } from "../utils/stringUtils.ts";
import { 
    writeHeader,
    getIsForImpl,
    writeType,
    getIsReferenceType,
} from "../utils/generationUtils.ts";
import { Writer } from "./writer.ts";

export function generate(analysisResult: AnalysisResult): string {
    const writer = new Writer();

    writeHeader(writer);
    writeUseDeclarations();
    writeBumpAllocator();
    writePublicFunctions();
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

    function writeUseDeclarations() {
        writer.writeLine("use std::mem::{self, MaybeUninit};");
        writer.writeLine("use bumpalo::Bump;");
        writer.writeLine("use swc_common::{Span, Spanned};");
        writer.write("pub use swc_ecmascript::ast::{self as swc_ast, ");
        writer.write(analysisResult.enums.filter(e => e.isPlain).map(e => e.name).join(", "));
        writer.write("};").newLine();
        writer.writeLine("use crate::comments::*;");
        writer.writeLine("use crate::tokens::*;");
        writer.writeLine("use crate::types::*;");
        writer.newLine();
        writer.writeLine(`#[cfg(feature = "serialize")]`);
        writer.writeLine("use serde::Serialize;");
        writer.newLine();
    }

    function writeBumpAllocator() {
        // todo: probably do something else... how would this work across many files on different threads?
        writer.writeLine("thread_local! {");
        writer.indent(() => {
            writer.writeLine("static LOCAL_BUMP_ALLOCATOR: std::cell::RefCell<Bump> = std::cell::RefCell::new(Bump::new());");
        }).write("}").newLine().newLine();
    }

    function writePublicFunctions() {
        writer.writeLine("pub fn with_ast_view<'a, T>(info: ProgramInfo, with_view: impl FnOnce(Program<'a>) -> T) -> T {");
        writer.indent(() => {
            writer.writeLine("match info.program {");
            writer.indent(() => {
                writer.writeLine("swc_ast::Program::Module(module) => {");
                writer.indent(() => {
                    writer.writeLine("with_ast_view_for_module(ModuleInfo {");
                    writer.indent(() => {
                        writer.writeLine("module,");
                        writer.writeLine("source_file: info.source_file,");
                        writer.writeLine("tokens: info.tokens,");
                        writer.writeLine("comments: info.comments,");
                    }).write("}, |module| with_view(Program::Module(module)))").newLine();
                });
                writer.writeLine("},");
                writer.writeLine("swc_ast::Program::Script(script) => {");
                writer.indent(() => {
                    writer.writeLine("with_ast_view_for_script(ScriptInfo {");
                    writer.indent(() => {
                        writer.writeLine("script,");
                        writer.writeLine("source_file: info.source_file,");
                        writer.writeLine("tokens: info.tokens,");
                        writer.writeLine("comments: info.comments,");
                    }).write("}, |script| with_view(Program::Script(script)))").newLine();
                });
                writer.writeLine("},");
            });
            writer.writeLine("}");
        }).write("}").newLine().newLine();

        writer.writeLine("pub fn with_ast_view_for_module<'a, T>(info: ModuleInfo, with_view: impl FnOnce(&'a Module<'a>) -> T) -> T {");
        writer.indent(() => {
            writer.writeLine("LOCAL_BUMP_ALLOCATOR.with(|bump_cell| {");
            writer.indent(() => {
                writer.writeLine("let mut bump_borrow = bump_cell.borrow_mut();");
                // hack to avoid yet another lifetime
                writer.writeLine("let bump_ref = unsafe { mem::transmute::<&Bump, &'a Bump>(&bump_borrow) };");
                writer.writeLine("let info_ref = unsafe { mem::transmute::<&ModuleInfo, &'a ModuleInfo<'a>>(&info) };");
                writer.writeLine(`let ast_view = ${getViewForFunctionName("Module")}(info_ref, bump_ref);`);
                writer.writeLine(`let result = with_view(ast_view);`);
                writer.writeLine("bump_borrow.reset();");
                writer.writeLine("result");
            }).write("})").newLine();
        }).write("}").newLine().newLine();

        writer.writeLine("pub fn with_ast_view_for_script<'a, T>(info: ScriptInfo, with_view: impl FnOnce(&'a Script<'a>) -> T) -> T {");
        writer.indent(() => {
            writer.writeLine("LOCAL_BUMP_ALLOCATOR.with(|bump_cell| {");
            writer.indent(() => {
                writer.writeLine("let mut bump_borrow = bump_cell.borrow_mut();");
                // hack to avoid yet another lifetime
                writer.writeLine("let bump_ref = unsafe { mem::transmute::<&Bump, &'a Bump>(&bump_borrow) };");
                writer.writeLine("let info_ref = unsafe { mem::transmute::<&ScriptInfo, &'a ScriptInfo<'a>>(&info) };");
                writer.writeLine(`let ast_view = ${getViewForFunctionName("Script")}(info_ref, bump_ref);`);
                writer.writeLine(`let result = with_view(ast_view);`);
                writer.writeLine("bump_borrow.reset();");
                writer.writeLine("result");
            }).write("})").newLine();
        }).write("}").newLine().newLine();
    }

    function writeNode() {
        writer.writeLine("#[derive(Clone, Copy)]");
        writer.writeLine("pub enum Node<'a> {");
        writer.indent(() => {
            for (const struct of analysisResult.structs) {
                writer.writeLine(`${struct.name}(&'a ${struct.name}<'a>),`);
            }
        });
        writer.writeLine("}").newLine();

        writer.writeLine("impl<'a> Node<'a> {");
        writer.indent(() => {
            writer.writeLine("pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {");
            writer.indent(() => {
                writer.writeLine("T::to(self)");
            }).write("}").newLine().newLine();

            writer.writeLine("pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {");
            writer.indent(() => {
                writer.writeLine("if let Some(result) = T::to(self) {");
                writer.indent(() => {
                    writer.writeLine("result");
                }).write("} else {").newLine();
                writer.indent(() => {
                    writer.writeLine(`panic!("Tried to cast node of type {} to {}.", self.kind(), T::kind())`);
                });
                writer.write("}").newLine();
            }).write("}").newLine().newLine();

            writer.writeLine("pub fn is<T: CastableNode<'a>>(&self) -> bool {");
            writer.indent(() => {
                writer.writeLine("self.kind() == T::kind()");
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
            writer.newLine();

            implementTraitMethod("kind", "NodeKind", false, (fullName, struct) => {
                writer.write(`${fullName}(_) => NodeKind::${struct.name}`);
            });
        });
        writer.newLine();

        writer.writeLine("#[derive(Clone, PartialEq, Debug, Copy)]");
        writer.writeLine("pub enum NodeKind {");
        writer.indent(() => {
            for (const struct of analysisResult.structs) {
                writer.writeLine(`${struct.name},`);
            }
        });
        writer.writeLine("}").newLine();

        writer.writeLine("impl std::fmt::Display for NodeKind {");
        writer.indent(() => {
            writer.writeLine("fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {");
            writer.indent(() => {
                writer.writeLine(`write!(f, "{}", match self {`);
                writer.indent(() => {
                    for (const struct of analysisResult.structs) {
                        writer.writeLine(`NodeKind::${struct.name} => "${struct.name}",`);
                    }
                }).write("})").newLine();
            }).write("}").newLine();
        });
        writer.writeLine("}").newLine();

        function implementTraitMethod(
            methodName: string,
            returnType: string,
            hasLifetime = false,
            customMatchWrite?: (fullName: string, struct: StructDefinition) => void,
        ) {
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
                        if (customMatchWrite != null) {
                            customMatchWrite(fullName, struct);
                        } else {
                            writer.write(`${fullName}(node) => node.${methodName}()`);
                        }
                        writer.write(",");
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
            writer.writeLine("#[derive(Copy, Clone)]");
            writer.writeLine(`#[cfg_attr(feature = "serialize", derive(Serialize))]`);
            writer.writeLine(`#[cfg_attr(feature = "serialize", serde(untagged))]`);
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
                            writeType(writer, analysisResult, arg, true);
                        }
                        writer.write(")");
                    }
                    writer.write(",");
                }
            }).newLine();
            writer.write("}").newLine().newLine();

            writer.writeLine(`impl<'a> ${enumDef.name}<'a> {`).indent(() => {
                writer.writeLine("pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {");
                writer.indent(() => {
                    writer.writeLine("T::to(&self.into())");
                }).write("}").newLine().newLine();

                writer.writeLine("pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {");
                writer.indent(() => {
                    writer.writeLine(`let node: Node<'a> = self.into();`);
                    writer.writeLine("if let Some(result) = T::to(&node) {");
                    writer.indent(() => {
                        writer.writeLine("result");
                    }).write("} else {").newLine();
                    writer.indent(() => {
                        writer.writeLine(`panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())`);
                    });
                    writer.write("}").newLine();
                }).write("}").newLine().newLine();

                writer.writeLine("pub fn is<T: CastableNode<'a>>(&self) -> bool {");
                writer.indent(() => {
                    writer.writeLine("self.kind() == T::kind()");
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
                writer.newLine();
                implementTraitMethod("kind", "NodeKind", false, (fullName, variant) => {
                    if (variant.tuple_args?.length === 1 && isSwcStructType(variant.tuple_args[0])) {
                        const variantType = variant.tuple_args[0];
                        if (variantType.kind !== "reference") {
                            throw new Error("Unhandled.");
                        }
                        writer.write(`${fullName}(_) => NodeKind::${variantType.name}`);
                    } else {
                        writer.write(`${fullName}(node) => node.kind()`);
                    }
                });
            });
            writer.newLine();

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
            writer.newLine();

            writeTrait(`From<${enumDef.name}<'a>>`, "Node<'a>", () => {
                writer.writeLine(`fn from(node: ${enumDef.name}<'a>) -> Node<'a> {`);
                writer.indent(() => {
                    writer.writeLine("match node {");
                    writer.indent(() => {
                        for (const variant of enumDef.variants) {
                            const fullName = `${enumDef.name}::${variant.name}`;
                            writer.write(`${fullName}(node) => node.into(),`);
                            writer.newLine();
                        }
                    }).write("}").newLine();
                }).write("}").newLine();
            });

            function implementTraitMethod(
                methodName: string,
                returnType: string,
                hasSelfLifetime = false,
                customMatchWrite?: (fullName: string, variant: EnumVariantDefinition) => void,
            ) {
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
                            if (customMatchWrite != null) {
                                customMatchWrite(fullName, variant);
                            } else {
                                writer.write(`${fullName}(node) => node.${methodName}()`);
                            }
                            writer.write(",");
                            writer.newLine();
                        }
                    }).write("}").newLine();
                }).write("}").newLine();
            }
        }

        function writeEnumFunctions() {
            writer.write(
                `fn ${
                    getViewForFunctionName(enumDef.name)
                }<'a>(inner: &'a swc_ast::${enumDef.name}, parent: Node<'a>, bump: &'a Bump) -> ${enumDef.name}<'a> {`,
            )
                .newLine();
            writer.indent(() => {
                // writer.writeLine(`println!("Entered ${enumDef.name}");`);
                writer.writeLine("match inner {");
                writer.indent(() => {
                    for (const variant of enumDef.variants) {
                        const fullName = `${enumDef.name}::${variant.name}`;
                        writer.write(`swc_ast::${fullName}(value) => ${fullName}(`);
                        if (variant.tuple_args?.length !== 1) {
                            throw new Error("Unhandled scenario where the variant's tuple args were not equal to 1.");
                        }
                        writeGetViewTypeExpression(variant.tuple_args[0], false, "value");
                        writer.write("),");
                        writer.newLine();
                    }
                }).write("}").newLine();
            }).write("}").newLine();
        }
    }

    function handleStruct(struct: StructDefinition) {
        const implFields = struct.fields.filter(f => getIsForImpl(analysisResult, f.type));
        const structFields = struct.fields.filter(f => !getIsForImpl(analysisResult, f.type) && f.name !== "span");

        writeStruct();
        writer.newLine();
        writeStructFunction();

        function writeStruct() {
            writeDocs(struct.docs);
            writer.writeLine("#[derive(Clone)]");
            writer.writeLine(`#[cfg_attr(feature = "serialize", derive(Serialize))]`);
            writer.writeLine(`#[cfg_attr(feature = "serialize", serde(into = "crate::generated_serialize::Serializable${struct.name}"))]`);
            writer.writeLine(`pub struct ${struct.name}<'a> {`);
            writer.indent(() => {
                if (struct.parents.length > 0) {
                    if (struct.parents.length === 1) {
                        writer.writeLine(`pub parent: &'a ${struct.parents[0].name}<'a>,`);
                    } else {
                        writer.writeLine(`pub parent: Node<'a>,`);
                    }
                }
                if (struct.name === "Module" || struct.name === "Script") {
                    writer.writeLine("pub source_file: Option<&'a swc_common::SourceFile>,");
                    writer.writeLine("pub tokens: Option<&'a TokenContainer<'a>>,");
                    writer.writeLine("pub comments: Option<&'a CommentContainer<'a>>,");
                }
                writer.writeLine(`pub inner: &'a swc_ast::${struct.name},`);

                for (const field of structFields) {
                    writeDocs(field.docs);
                    writer.write(`pub ${field.name}: `);
                    writeType(writer, analysisResult, field.type, true);
                    writer.write(",").newLine();
                }
            });
            writer.write("}").newLine();

            if (implFields.length > 0) {
                writer.newLine();
                writer.write(`impl<'a> ${struct.name}<'a> {`);
                writer.indent(() => {
                    for (const field of implFields) {
                        const isReferenceType = getIsReferenceType(analysisResult, field.type);
                        writer.newLine();

                        writeDocs(field.docs);
                        writer.write(`pub fn ${field.name}(&self) -> `);
                        if (isReferenceType) {
                            writer.write("&");
                        }
                        writeType(writer, analysisResult, field.type, false);
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
                            writeAppendChild(field.type, `self.${field.name}`, false, false);
                        }
                        writer.write("children");
                    }
                    writer.newLine();
                }).write("}").newLine();
                writer.newLine();

                writer.writeLine("fn into_node(&self) -> Node<'a> {");
                writer.indent(() => {
                    writer.writeLine("self.into()");
                }).write("}").newLine();
                writer.newLine();

                writer.writeLine("fn kind(&self) -> NodeKind {");
                writer.indent(() => {
                    writer.writeLine(`NodeKind::${struct.name}`);
                }).write("}").newLine();
            });

            writer.newLine();
            writeTrait("CastableNode<'a>", `${struct.name}<'a>`, () => {
                writer.writeLine("fn to(node: &Node<'a>) -> Option<&'a Self> {");
                writer.indent(() => {
                    writer.writeLine(`if let Node::${struct.name}(node) = node {`);
                    writer.indent(() => {
                        writer.writeLine("Some(node)");
                    }).write("} else {").newLine();
                    writer.indent(() => {
                        writer.writeLine("None");
                    }).write("}").newLine();
                }).write("}").newLine();

                writer.writeLine("fn kind() -> NodeKind {");
                writer.indent(() => {
                    writer.writeLine(`NodeKind::${struct.name}`);
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
                writer.write(`source_file_info: &'a ModuleInfo<'a>`);
            } else if (struct.name === "Script") {
                writer.write(`source_file_info: &'a ScriptInfo<'a>`);
            } else {
                writer.write(`inner: &'a swc_ast::${struct.name}, `);
                writer.write(`parent: Node<'a>`);
            }
            writer.write(", bump: &'a Bump");
            writer.write(`) -> &'a ${struct.name}<'a> {`).newLine();
            writer.indent(() => {
                if (struct.name === "Module") {
                    writer.writeLine("let inner = source_file_info.module;");
                } else if (struct.name === "Script") {
                    writer.writeLine("let inner = source_file_info.script;");
                }
                if (struct.name === "Module" || struct.name === "Script") {
                    writer.writeLine("let tokens = source_file_info.tokens.map(|t| &*bump.alloc(TokenContainer::new(t)));");
                    writer.writeLine(
                        `let comments = source_file_info.comments.map(|c| &*bump.alloc(CommentContainer::new(`,
                    );
                    writer.indent(() => {
                        writer.writeLine("c,");
                        writer.writeLine(`tokens.expect("Tokens must be provided when using comments."),`);
                        writer.writeLine(`source_file_info.source_file.expect("Source file must be provided when using comments"),`);
                    });
                    writer.writeLine(")));");
                }
                // writer.writeLine(`println!("Entered ${struct.name}");`);

                writer.write(`let node = bump.alloc(${struct.name} {`).newLine();
                writer.indent(() => {
                    writer.write("inner,").newLine();
                    if (struct.parents.length > 0) {
                        if (struct.parents.length === 1) {
                            writer.write(`parent: parent.expect::<${struct.parents[0].name}>()`);
                        } else {
                            writer.write("parent");
                        }
                        writer.write(",").newLine();
                    }
                    if (struct.name === "Module" || struct.name === "Script") {
                        writer.write("source_file: source_file_info.source_file,").newLine();
                        writer.write("tokens,").newLine();
                        writer.write("comments,").newLine();
                    }
                    for (const field of structFields) {
                        if (isVecType(field.type)) {
                            writer.writeLine(`${field.name}: Vec::with_capacity(inner.${field.name}.len()),`);
                        } else if (isOptionType(field.type)) {
                            writer.writeLine(`${field.name}: None,`);
                        } else {
                            writer.writeLine(`${field.name}: unsafe { MaybeUninit::uninit().assume_init() },`);
                        }
                    }
                }).write("});").newLine();
                if (structFields.length > 0) {
                    // hack to get it to avoid the borrow checker (because `.into()` will do a transmute)
                    writer.writeLine(`let parent: Node<'a> = (&*node).into();`);
                    // writer.writeLine(`let parent = Node::${struct.name}(node);`);

                    for (const [i, field] of structFields.entries()) {
                        const shouldCloneParent = i < structFields.length - 1;
                        if (isVecType(field.type)) {
                            writer.write(`node.${field.name}.extend(`);
                        } else {
                            writer.write(`node.${field.name} = `);
                        }
                        writeGetViewTypeExpression(field.type, shouldCloneParent, `&inner.${field.name}`);
                        if (isVecType(field.type)) {
                            writer.write(")");
                        }
                        writer.write(";").newLine();
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

    function writeGetViewTypeExpression(type: TypeDefinition, shouldCloneParent: boolean, name: string) {
        if (type.kind === "primitive") {
            throw new Error("Primitive types not handled here.");
        }

        if (type.name === "Option") {
            writer.writeLine(`match ${name} {`);
            writer.indent(() => {
                writer.write("Some(value) => Some(");
                writeGetViewTypeExpression(type.generic_args[0], shouldCloneParent, "value");
                if (isVecType(type.generic_args[0])) {
                    writer.write(".collect()");
                }
                writer.write("),").newLine();
                writer.writeLine("None => None,");
            }).write("}");
        } else if (type.name === "Vec") {
            writer.write(`${name.replace(/^&/, "")}.iter().map(|value| `);
            writeGetViewTypeExpression(type.generic_args[0], true, "value");
            writer.write(")");
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
            writer.write(`${getViewForFunctionName(type.name)}(${name}, parent`);
            if (shouldCloneParent) {
                writer.write(".clone()");
            }
            writer.write(`, bump)`);
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

    function isOptionType(type: TypeDefinition | undefined): boolean {
        return type != null && type.kind === "reference" && type.name === "Option";
    }

    function writeDocs(docs: string | undefined) {
        if (docs == null || docs.length === 0) {
            return;
        }
        const lines = docs.split(/\r?\n/);
        for (const line of lines) {
            writer.write("///");
            if (line.trim().length > 0) {
                writer.write(` ${line.trimRight()}`);
            }
            writer.newLine();
        }
    }

    function getViewForFunctionName(name: string) {
        return `get_view_for_${nameToSnakeCase(name)}`;
    }
}
