import { AnalysisResult, AstEnumDefinition, AstEnumVariantDefinition, AstStructDefinition, TypeDefinition } from "../analyze/analysis_types.ts";
import { createWriter } from "../utils/create_writer.ts";
import { nameToSnakeCase } from "../utils/string_utils.ts";
import {
  getAstStructForType,
  getIsForImpl,
  getIsReferenceType,
  isOptionType,
  isSwcAstType,
  isSwcNodeEnumType,
  isVecType,
  writeHeader,
  writeType,
} from "./helpers.ts";

export function generate(analysisResult: AnalysisResult): string {
  const writer = createWriter();

  writeHeader(writer);
  writeUseDeclarations();
  writeBumpAllocator();
  writePublicFunctions();
  writeNode();

  for (const enumDef of analysisResult.astEnums) {
    writer.blankLine();
    writeEnum(enumDef);
  }

  for (const struct of analysisResult.astStructs) {
    writer.blankLine();
    handleStruct(struct);
  }

  writer.newLineIfLastNot();

  return writer.toString();

  function writeUseDeclarations() {
    writer.writeLine("use std::cell::RefCell;");
    writer.writeLine("use std::mem;");
    writer.writeLine("use bumpalo::Bump;");
    writer.writeLine("use swc_common::{Span, Spanned};");
    writer.write("pub use swc_ecmascript::ast::{self as swc_ast, ");
    writer.write(analysisResult.plainEnums.map(e => e.name).join(", "));
    writer.write("};").newLine();
    writer.writeLine("use crate::comments::*;");
    writer.writeLine("use crate::tokens::*;");
    writer.writeLine("use crate::types::*;");
    writer.blankLine();
  }

  function writeBumpAllocator() {
    // todo: probably do something else... how would this work across many files on different threads?
    writer.write("thread_local!").block(() => {
      writer.writeLine("static LOCAL_BUMP_ALLOCATOR: RefCell<Bump> = RefCell::new(Bump::new());");
    }).blankLine();
  }

  function writePublicFunctions() {
    writer.write("pub fn with_ast_view<'a, T>(info: ProgramInfo, with_view: impl FnOnce(Program<'a>) -> T) -> T").block(() => {
      writer.write("match info.program").block(() => {
        writer.write("swc_ast::Program::Module(module) =>").block(() => {
          writer.write("with_ast_view_for_module(ModuleInfo ").inlineBlock(() => {
            writer.writeLine("module,");
            writer.writeLine("source_file: info.source_file,");
            writer.writeLine("tokens: info.tokens,");
            writer.writeLine("comments: info.comments,");
          }).write(", |module| with_view(Program::Module(module)))");
        });
        writer.write("swc_ast::Program::Script(script) =>").block(() => {
          writer.write("with_ast_view_for_script(ScriptInfo ").inlineBlock(() => {
            writer.writeLine("script,");
            writer.writeLine("source_file: info.source_file,");
            writer.writeLine("tokens: info.tokens,");
            writer.writeLine("comments: info.comments,");
          }).write(", |script| with_view(Program::Script(script)))");
        });
      });
    }).blankLine();

    writer.write("pub fn with_ast_view_for_module<'a, T>(info: ModuleInfo, with_view: impl FnOnce(&'a Module<'a>) -> T) -> T").block(() => {
      writer.write("LOCAL_BUMP_ALLOCATOR.with(|bump_cell| ").inlineBlock(() => {
        writer.writeLine("let mut bump_borrow = bump_cell.borrow_mut();");
        // hack to avoid yet another lifetime
        writer.writeLine("let bump_ref = unsafe { mem::transmute::<&Bump, &'a Bump>(&bump_borrow) };");
        writer.writeLine("let info_ref = unsafe { mem::transmute::<&ModuleInfo, &'a ModuleInfo<'a>>(&info) };");
        writer.writeLine(`let ast_view = ${getViewForFunctionName("Module")}(info_ref, bump_ref);`);
        writer.writeLine(`let result = with_view(ast_view);`);
        writer.writeLine("bump_borrow.reset();");
        writer.writeLine("result");
      }).write(")").newLine();
    }).blankLine();

    writer.write("pub fn with_ast_view_for_script<'a, T>(info: ScriptInfo, with_view: impl FnOnce(&'a Script<'a>) -> T) -> T").block(() => {
      writer.write("LOCAL_BUMP_ALLOCATOR.with(|bump_cell| ").inlineBlock(() => {
        writer.writeLine("let mut bump_borrow = bump_cell.borrow_mut();");
        // hack to avoid yet another lifetime
        writer.writeLine("let bump_ref = unsafe { mem::transmute::<&Bump, &'a Bump>(&bump_borrow) };");
        writer.writeLine("let info_ref = unsafe { mem::transmute::<&ScriptInfo, &'a ScriptInfo<'a>>(&info) };");
        writer.writeLine(`let ast_view = ${getViewForFunctionName("Script")}(info_ref, bump_ref);`);
        writer.writeLine(`let result = with_view(ast_view);`);
        writer.writeLine("bump_borrow.reset();");
        writer.writeLine("result");
      }).write(")").newLine();
    }).blankLine();
  }

  function writeNode() {
    writer.writeLine("#[derive(Clone, Copy)]");
    writer.write("pub enum Node<'a>").block(() => {
      for (const struct of analysisResult.astStructs) {
        writer.writeLine(`${struct.name}(&'a ${struct.name}<'a>),`);
      }
    }).blankLine();

    writer.write("impl<'a> Node<'a>").block(() => {
      writer.write("pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T>").block(() => {
        writer.writeLine("T::to(self)");
      }).blankLine();

      writer.write("pub fn expect<T: CastableNode<'a>>(&self) -> &'a T").block(() => {
        writer.write("if let Some(result) = T::to(self) ").inlineBlock(() => {
          writer.writeLine("result");
        }).write(" else ").inlineBlock(() => {
          writer.writeLine(`panic!("Tried to cast node of type {} to {}.", self.kind(), T::kind())`);
        });
      }).blankLine();

      writer.write("pub fn is<T: CastableNode<'a>>(&self) -> bool").block(() => {
        writer.writeLine("self.kind() == T::kind()");
      });
    }).blankLine();

    writeTrait("Spanned", "Node<'a>", () => {
      implementTraitMethod("span", "Span");
    });
    writer.blankLine();

    writeTrait("NodeTrait<'a>", "Node<'a>", () => {
      implementTraitMethod("parent", "Option<Node<'a>>", false, (fullName, struct) => {
        writer.write(`${fullName}(node) => NodeTrait::parent(*node)`);
      });
      writer.blankLine();
      implementTraitMethod("children", "Vec<Node<'a>>");
      writer.blankLine();
      implementTraitMethod("as_node", "Node<'a>");
      writer.blankLine();

      implementTraitMethod("kind", "NodeKind", false, (fullName, struct) => {
        writer.write(`${fullName}(_) => NodeKind::${struct.name}`);
      });
    });
    writer.blankLine();

    writer.writeLine("#[derive(Clone, PartialEq, Debug, Copy)]");
    writer.write("pub enum NodeKind").block(() => {
      for (const struct of analysisResult.astStructs) {
        writer.writeLine(`${struct.name},`);
      }
    }).blankLine();

    writer.write("impl std::fmt::Display for NodeKind").block(() => {
      writer.write("fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result").block(() => {
        writer.write(`write!(f, "{}", match self `).inlineBlock(() => {
          for (const struct of analysisResult.astStructs) {
            writer.writeLine(`NodeKind::${struct.name} => "${struct.name}",`);
          }
        }).write(")");
      });
    });

    function implementTraitMethod(
      methodName: string,
      returnType: string,
      hasLifetime = false,
      customMatchWrite?: (fullName: string, struct: AstStructDefinition) => void,
    ) {
      writer.write(`fn ${methodName}`);
      if (hasLifetime) {
        writer.write("<'a>");
      }
      writer.write(`(&`);
      if (hasLifetime) {
        writer.write("'a ");
      }
      writer.write(`self) -> ${returnType}`).block(() => {
        writer.write("match self").block(() => {
          for (const struct of analysisResult.astStructs) {
            const fullName = `Node::${struct.name}`;
            if (customMatchWrite != null) {
              customMatchWrite(fullName, struct);
            } else {
              writer.write(`${fullName}(node) => node.${methodName}()`);
            }
            writer.write(",");
            writer.newLine();
          }
        });
      });
    }
  }

  function writeEnum(enumDef: AstEnumDefinition) {
    writeEnum();
    writer.blankLine();
    writeEnumFunctions();

    function writeEnum() {
      writeDocs(enumDef.docs);
      writer.writeLine("#[derive(Copy, Clone)]");
      writer.write(`pub enum ${enumDef.name}<'a>`).block(() => {
        for (const variant of enumDef.variants) {
          writer.newLineIfLastNot();
          writeDocs(variant.docs);
          writer.write(`${variant.name}`);
          if (variant.tupleArg != null) {
            writer.write("(");
            writeType(writer, analysisResult, variant.tupleArg, true);
            writer.write(")");
          }
          writer.write(",");
        }
      }).blankLine();

      writer.write(`impl<'a> ${enumDef.name}<'a>`).block(() => {
        writer.write("pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T>").block(() => {
          writer.writeLine("T::to(&self.into())");
        }).blankLine();

        writer.write("pub fn expect<T: CastableNode<'a>>(&self) -> &'a T").block(() => {
          writer.writeLine(`let node: Node<'a> = self.into();`);
          writer.write("if let Some(result) = T::to(&node) ").inlineBlock(() => {
            writer.writeLine("result");
          }).write(" else ").inlineBlock(() => {
            writer.writeLine(`panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())`);
          });
        }).blankLine();

        writer.write("pub fn is<T: CastableNode<'a>>(&self) -> bool").block(() => {
          writer.writeLine("self.kind() == T::kind()");
        });

        // add a preferred non-nullable `parent()` method if all the variants have a parent
        if (enumDef.variants.every(v => getAstStructForType(analysisResult, v.tupleArg)?.parents.length ?? 0 > 0)) {
          const variantStructs = enumDef.variants.map(v => getAstStructForType(analysisResult, v.tupleArg)!);
          writer.write("pub fn parent(&self) -> ");
          if (variantStructs.every(s => s.parents.length === 1 && s.parents[0].name === variantStructs[0].parents[0].name)) {
            writer.write(`&'a ${variantStructs[0].parents[0].name}<'a>`).block(() => {
              writer.writeLine(`NodeTrait::parent(self).unwrap().expect::<${variantStructs[0].parents[0].name}>()`);
            });
          } else {
            writer.write("Node<'a>").block(() => {
              writer.writeLine("NodeTrait::parent(self).unwrap()");
            });
          }
        }
      }).blankLine();

      writeTrait("Spanned", `${enumDef.name}<'a>`, () => {
        implementTraitMethod("span", "Span");
      });
      writer.blankLine();

      writeTrait("NodeTrait<'a>", `${enumDef.name}<'a>`, () => {
        implementTraitMethod("parent", "Option<Node<'a>>", false, (fullName, variant) => {
          writer.write(`${fullName}(node) => NodeTrait::parent(`);
          if (variant.tupleArg != null && isSwcAstType(analysisResult, variant.tupleArg)) {
            writer.write("*");
          }
          writer.write(`node)`);
        });
        writer.blankLine();
        implementTraitMethod("children", "Vec<Node<'a>>", false);
        writer.blankLine();
        implementTraitMethod("as_node", "Node<'a>", false);
        writer.blankLine();
        implementTraitMethod("kind", "NodeKind", false, (fullName, variant) => {
          if (variant.tupleArg != null && isSwcAstType(analysisResult, variant.tupleArg)) {
            const variantType = variant.tupleArg;
            if (variantType.kind !== "Reference") {
              throw new Error("Unhandled.");
            }
            writer.write(`${fullName}(_) => NodeKind::${variantType.name}`);
          } else {
            writer.write(`${fullName}(node) => node.kind()`);
          }
        });
      });
      writer.blankLine();

      writeTrait(`From<&${enumDef.name}<'a>>`, "Node<'a>", () => {
        writer.write(`fn from(node: &${enumDef.name}<'a>) -> Node<'a>`).block(() => {
          writer.write("match node").block(() => {
            for (const variant of enumDef.variants) {
              const fullName = `${enumDef.name}::${variant.name}`;
              writer.write(`${fullName}(node) => `);
              if (isSwcAstType(analysisResult, variant.tupleArg)) {
                writer.write(`(*node).into(),`);
              } else {
                writer.write(`node.into(),`);
              }
              writer.newLine();
            }
          });
        });
      });
      writer.blankLine();

      writeTrait(`From<${enumDef.name}<'a>>`, "Node<'a>", () => {
        writer.write(`fn from(node: ${enumDef.name}<'a>) -> Node<'a>`).block(() => {
          writer.write("match node").block(() => {
            for (const variant of enumDef.variants) {
              const fullName = `${enumDef.name}::${variant.name}`;
              writer.write(`${fullName}(node) => node.into(),`);
              writer.newLine();
            }
          });
        });
      });

      function implementTraitMethod(
        methodName: string,
        returnType: string,
        hasSelfLifetime = false,
        customMatchWrite?: (fullName: string, variant: AstEnumVariantDefinition) => void,
      ) {
        writer.write(`fn ${methodName}`);
        if (hasSelfLifetime) {
          writer.write("<'a>");
        }
        writer.write(`(&`);
        if (hasSelfLifetime) {
          writer.write("'a ");
        }
        writer.write(`self) -> ${returnType}`).block(() => {
          writer.write("match self").block(() => {
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
          });
        });
      }
    }

    function writeEnumFunctions() {
      writer.write(
        `fn ${getViewForFunctionName(enumDef.name)}<'a>(inner: &'a swc_ast::${enumDef.name}, bump: &'a Bump) -> ${enumDef.name}<'a>`,
      ).block(() => {
        // writer.writeLine(`println!("Entered ${enumDef.name}");`);
        writer.write("match inner").block(() => {
          for (const variant of enumDef.variants) {
            const fullName = `${enumDef.name}::${variant.name}`;
            writer.write(`swc_ast::${fullName}(value) => ${fullName}(`);
            writeGetViewTypeExpression(variant.tupleArg!, "value");
            writer.write("),");
            writer.newLine();
          }
        });
      }).blankLine();

      writer.write(
        `fn ${getSetParentForFunctionName(enumDef.name)}<'a>(node: &${enumDef.name}<'a>, parent: Node<'a>)`,
      ).block(() => {
        // writer.writeLine(`println!("Entered ${enumDef.name}");`);
        writer.write("match node").block(() => {
          for (const variant of enumDef.variants) {
            const fullName = `${enumDef.name}::${variant.name}`;
            writer.write(`${fullName}(value) => `);
            writeSetParentExpression(variant.tupleArg!, "value");
            writer.write(",");
            writer.newLine();
          }
        });
      });
    }
  }

  function handleStruct(struct: AstStructDefinition) {
    const implFields = struct.fields.filter(f => getIsForImpl(analysisResult, f.type));
    const structFields = struct.fields.filter(f => !getIsForImpl(analysisResult, f.type) && f.name !== "span");

    writeStruct();
    writer.blankLine();
    writeStructFunctions();

    function writeStruct() {
      writeDocs(struct.docs);
      writer.writeLine("#[derive(Clone)]");
      writer.write(`pub struct ${struct.name}<'a>`).block(() => {
        if (struct.parents.length > 0) {
          if (struct.parents.length === 1) {
            writer.writeLine(`parent: Option<&'a ${struct.parents[0].name}<'a>>,`);
          } else {
            writer.writeLine(`parent: Option<Node<'a>>,`);
          }
        }
        if (struct.name === "Module" || struct.name === "Script") {
          writer.writeLine("pub source_file: Option<&'a dyn SourceFile>,");
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

      if (implFields.length > 0 || struct.parents.length > 0) {
        writer.blankLineIfLastNot();
        writer.write(`impl<'a> ${struct.name}<'a>`).block(() => {
          if (struct.parents.length > 0) {
            writer.write("pub fn parent(&self) -> ");
            if (struct.parents.length === 1) {
              writer.write(`&'a ${struct.parents[0].name}<'a>`);
            } else {
              writer.write(`Node<'a>`);
            }
            writer.block(() => {
              writer.writeLine(`self.parent.unwrap()`);
            });
          }

          for (const field of implFields) {
            const isReferenceType = getIsReferenceType(analysisResult, field.type);
            if (!writer.isAtStartOfFirstLineOfBlock()) {
              writer.blankLineIfLastNot();
            }

            writeDocs(field.docs);
            writer.write(`pub fn ${field.name}(&self) -> `);
            if (isReferenceType) {
              writer.write("&");
            }
            writeType(writer, analysisResult, field.type, false);
            writer.block(() => {
              if (isReferenceType) {
                writer.write("&");
              }
              writer.write(`self.inner.${field.innerName}`).newLine();
            });
          }
        });
      }

      writer.blankLineIfLastNot();
      writeTrait("Spanned", `${struct.name}<'a>`, () => {
        writer.write("fn span(&self) -> Span").block(() => {
          writer.writeLine("self.inner.span()");
        });
      });

      writer.blankLine();
      writeTrait(`From<&${struct.name}<'a>>`, "Node<'a>", () => {
        writer.write(`fn from(node: &${struct.name}<'a>) -> Node<'a>`).block(() => {
          // hack to not require people having to specify the lifetime twice
          writer.writeLine(`let node = unsafe { mem::transmute::<&${struct.name}<'a>, &'a ${struct.name}<'a>>(node) };`);
          writer.writeLine(`Node::${struct.name}(node)`);
        });
      });

      writer.blankLine();
      writeTrait("NodeTrait<'a>", `${struct.name}<'a>`, () => {
        writer.write("fn parent(&self) -> Option<Node<'a>>").block(() => {
          if (struct.parents.length === 0) {
            writer.write("None");
          } else {
            if (struct.parents.length === 1) {
              writer.write("Some(self.parent.unwrap().into())");
            } else {
              writer.write("Some(self.parent.unwrap().clone())");
            }
          }
          writer.newLine();
        });
        writer.blankLine();
        writeChildrenMethod();
        writer.blankLine();

        writer.write("fn as_node(&self) -> Node<'a>").block(() => {
          writer.writeLine("self.into()");
        });
        writer.blankLine();

        writer.write("fn kind(&self) -> NodeKind").block(() => {
          writer.writeLine(`NodeKind::${struct.name}`);
        });
      });

      writer.blankLine();
      writeTrait("CastableNode<'a>", `${struct.name}<'a>`, () => {
        writer.write("fn to(node: &Node<'a>) -> Option<&'a Self>").block(() => {
          writer.write(`if let Node::${struct.name}(node) = node `).inlineBlock(() => {
            writer.writeLine("Some(node)");
          }).write(" else ").inlineBlock(() => {
            writer.writeLine("None");
          });
        }).blankLine();

        writer.write("fn kind() -> NodeKind").block(() => {
          writer.writeLine(`NodeKind::${struct.name}`);
        });
      });

      function writeChildrenMethod() {
        writer.write("fn children(&self) -> Vec<Node<'a>>").block(() => {
          if (structFields.length === 0) {
            writer.write("Vec::with_capacity(0)");
          } else {
            writer.writeLine(`let mut children = Vec::with_capacity(${getStructChildrenCapacityExpr()});`);
            for (const field of structFields) {
              writeAppendChild(field.type, `self.${field.name}`, false, false);
            }
            writer.write("children");
          }
        });

        function writeAppendChild(type: TypeDefinition, name: string, inOption: boolean, inVec: boolean) {
          if (type.kind === "Primitive") {
            throw new Error("Should not have analyzed a primitive type here.");
          }
          if (isOptionType(type)) {
            writer.write(`if let Some(child) = ${name}`);
            if (isSwcNodeEnumType(analysisResult, type.genericArgs[0]) || isVecType(type.genericArgs[0])) {
              writer.write(".as_ref()");
            }
            writer.block(() => {
              writeAppendChild(type.genericArgs[0], "child", true, inVec);
            });
          } else if (isVecType(type)) {
            writer.write(`for child in ${name}.iter()`).block(() => {
              writeAppendChild(type.genericArgs[0], "child", false, true);
            });
          } else {
            writer.write(`children.push(`);
            if (inVec) {
              if (isSwcAstType(analysisResult, type)) {
                writer.write(`(*${name})`);
              } else {
                writer.write(name);
              }
            } else if (inOption) {
              writer.write(name);
            } else if (isSwcNodeEnumType(analysisResult, type)) {
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
            if (type.kind === "Primitive") {
              throw new Error("Should not have analyzed a primitive type here.");
            }

            if (type.name === "Option") {
              return `match &${name} { Some(_value) => ${getTypeCapacityExpr(type.genericArgs[0], "_value")}, None => 0, }`;
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
    }

    function writeStructFunctions() {
      writer.write(`fn ${getViewForFunctionName(struct.name)}<'a>(`);
      if (struct.name === "Module") {
        writer.write(`source_file_info: &'a ModuleInfo<'a>`);
      } else if (struct.name === "Script") {
        writer.write(`source_file_info: &'a ScriptInfo<'a>`);
      } else {
        writer.write(`inner: &'a swc_ast::${struct.name}`);
      }
      writer.write(", bump: &'a Bump");
      writer.write(`) -> &'a ${struct.name}<'a>`);
      writer.block(() => {
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
            writer.writeLine("c.leading,");
            writer.writeLine("c.trailing,");
            writer.writeLine(`tokens.expect("Tokens must be provided when using comments."),`);
            writer.writeLine(`source_file_info.source_file.expect("Source file must be provided when using comments"),`);
          });
          writer.writeLine(")));");
        }
        // writer.writeLine(`println!("Entered ${struct.name}");`);

        writer.write(`let node = bump.alloc(${struct.name} `).inlineBlock(() => {
          writer.write("inner,").newLine();
          if (struct.parents.length > 0) {
            writer.write("parent: None,").newLine();
          }
          if (struct.name === "Module" || struct.name === "Script") {
            writer.write("source_file: source_file_info.source_file,").newLine();
            writer.write("tokens,").newLine();
            writer.write("comments,").newLine();
          }
          for (const field of structFields) {
            writer.write(`${field.name}: `);
            writeGetViewTypeExpression(field.type, `&inner.${field.name}`);
            if (isVecType(field.type)) {
              writer.write(".collect()");
            }
            writer.write(`,`).newLine();
          }
        }).write(");").newLine();
        if (structFields.length > 0) {
          writer.writeLine(`let parent: Node<'a> = (&*node).into();`);

          for (const field of structFields) {
            if (isVecType(field.type)) {
              writeSetParentExpression(field.type, `node.${field.name}`);
            } else {
              writeSetParentExpression(field.type, `&node.${field.name}`);
              writer.write(";");
            }
            writer.newLine();
          }
        }

        // writer.writeLine(`println!("Exited ${struct.name}");`);
        writer.writeLine("node");
      });

      if (struct.parents.length > 0) {
        writer.blankLine();
        writer.write(`fn ${getSetParentForFunctionName(struct.name)}<'a>(`);
        writer.write(`node: &${struct.name}<'a>, `);
        writer.write(`parent: Node<'a>)`).block(() => {
          // For some reason having a `Cell<Option<T>>` field for the parent in the struct
          // was causing infering lifetimes to not work at all. The workaround here is to
          // get rid of the cell and just do here what's done in an UnsafeCell...
          // Seems to work, but not sure if it's ok to do...
          writer.write("unsafe").block(() => {
            writer.writeLine(`let node_ptr = node as *const ${struct.name}<'a> as *mut ${struct.name}<'a>;`);
            writer.write(`(*node_ptr).parent.replace(`);
            if (struct.parents.length === 1) {
              writer.write(`parent.expect::<${struct.parents[0].name}>()`);
            } else {
              writer.write("parent");
            }
            writer.write(");");
          });
        });
      }
    }
  }

  function writeTrait(traitName: string, name: string, body: () => void) {
    writer.write("impl");
    if (traitName.endsWith("<'a>") || name.endsWith("<'a>")) {
      writer.write("<'a>");
    }
    writer.write(` ${traitName} for ${name}`).block(() => {
      body();
    });
  }

  function writeGetViewTypeExpression(type: TypeDefinition, name: string) {
    if (type.kind === "Primitive") {
      throw new Error("Primitive types not handled here.");
    }

    if (type.name === "Option") {
      writer.write(`match ${name} `).inlineBlock(() => {
        writer.write("Some(value) => Some(");
        writeGetViewTypeExpression(type.genericArgs[0], "value");
        if (isVecType(type.genericArgs[0])) {
          writer.write(".collect()");
        }
        writer.write("),").newLine();
        writer.writeLine("None => None,");
      });
    } else if (type.name === "Vec") {
      writer.write(`${name.replace(/^&/, "")}.iter().map(|value| `);
      writeGetViewTypeExpression(type.genericArgs[0], "value");
      writer.write(")");
    } else {
      writer.write(`${getViewForFunctionName(type.name)}(${name}, bump)`);
    }
  }

  function writeSetParentExpression(type: TypeDefinition, name: string) {
    if (type.kind === "Primitive") {
      throw new Error("Primitive types not handled here.");
    }

    if (type.name === "Option") {
      writer.write(`if let Some(value) = ${name} `).inlineBlock(() => {
        writeSetParentExpression(type.genericArgs[0], "value");
      });
    } else if (type.name === "Vec") {
      writer.write(`for value in ${name.replace(/^&/, "")}.iter() `).inlineBlock(() => {
        writeSetParentExpression(type.genericArgs[0], "value");
      });
    } else {
      writer.write(`${getSetParentForFunctionName(type.name)}(${name}, parent)`);
    }
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

  function getSetParentForFunctionName(name: string) {
    return `set_parent_for_${nameToSnakeCase(name)}`;
  }
}
