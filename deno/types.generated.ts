// This code is code generated.
// Run `./scripts/generate.sh` from the root directory to regenerate it.
import { BigIntValue, JsWord, Node, Span } from "./types.ts";

export type BlockStmtOrExpr =
  | BlockStmt
  | Expr;

export type ClassMember =
  | Constructor
  | ClassMethod
  | PrivateMethod
  | ClassProp
  | PrivateProp
  | TsIndexSignature
  | EmptyStmt;

export type Decl =
  | ClassDecl
  | FnDecl
  | VarDecl
  | TsInterfaceDecl
  | TsTypeAliasDecl
  | TsEnumDecl
  | TsModuleDecl;

export type DefaultDecl =
  | ClassExpr
  | FnExpr
  | TsInterfaceDecl;

export type ExportSpecifier =
  | ExportNamespaceSpecifier
  | ExportDefaultSpecifier
  | ExportNamedSpecifier;

export type Expr =
  | ThisExpr
  | ArrayLit
  | ObjectLit
  | FnExpr
  | UnaryExpr
  | UpdateExpr
  | BinExpr
  | AssignExpr
  | MemberExpr
  | CondExpr
  | CallExpr
  | NewExpr
  | SeqExpr
  | Ident
  | Lit
  | Tpl
  | TaggedTpl
  | ArrowExpr
  | ClassExpr
  | YieldExpr
  | MetaPropExpr
  | AwaitExpr
  | ParenExpr
  | JSXMemberExpr
  | JSXNamespacedName
  | JSXEmptyExpr
  | JSXElement
  | JSXFragment
  | TsTypeAssertion
  | TsConstAssertion
  | TsNonNullExpr
  | TsAsExpr
  | PrivateName
  | OptChainExpr
  | Invalid;

export type ExprOrSuper =
  | Super
  | Expr;

export type ImportSpecifier =
  | ImportNamedSpecifier
  | ImportDefaultSpecifier
  | ImportStarAsSpecifier;

export type JSXAttrName =
  | Ident
  | JSXNamespacedName;

export type JSXAttrOrSpread =
  | JSXAttr
  | SpreadElement;

export type JSXAttrValue =
  | Lit
  | JSXExprContainer
  | JSXElement
  | JSXFragment;

export type JSXElementChild =
  | JSXText
  | JSXExprContainer
  | JSXSpreadChild
  | JSXElement
  | JSXFragment;

export type JSXElementName =
  | Ident
  | JSXMemberExpr
  | JSXNamespacedName;

export type JSXExpr =
  | JSXEmptyExpr
  | Expr;

/**
 * Used for `obj` property of `JSXMemberExpr`.
 */
export type JSXObject =
  | JSXMemberExpr
  | Ident;

export type Lit =
  | Str
  | Bool
  | Null
  | Number
  | BigInt
  | Regex
  | JSXText;

export type ModuleDecl =
  | ImportDecl
  | ExportDecl
  | NamedExport
  | ExportDefaultDecl
  | ExportDefaultExpr
  | ExportAll
  | TsImportEqualsDecl
  | TsExportAssignment
  | TsNamespaceExportDecl;

export type ModuleItem =
  | ModuleDecl
  | Stmt;

export type ObjectPatProp =
  | KeyValuePatProp
  | AssignPatProp
  | RestPat;

export type ParamOrTsParamProp =
  | TsParamProp
  | Param;

export type Pat =
  | BindingIdent
  | ArrayPat
  | RestPat
  | ObjectPat
  | AssignPat
  | Invalid
  | Expr;

export type PatOrExpr =
  | Expr
  | Pat;

export type Prop =
  | Ident
  | KeyValueProp
  | AssignProp
  | GetterProp
  | SetterProp
  | MethodProp;

export type PropName =
  | Ident
  | Str
  | Number
  | ComputedPropName
  | BigInt;

export type PropOrSpread =
  | SpreadElement
  | Prop;

export type Stmt =
  | BlockStmt
  | EmptyStmt
  | DebuggerStmt
  | WithStmt
  | ReturnStmt
  | LabeledStmt
  | BreakStmt
  | ContinueStmt
  | IfStmt
  | SwitchStmt
  | ThrowStmt
  | TryStmt
  | WhileStmt
  | DoWhileStmt
  | ForStmt
  | ForInStmt
  | ForOfStmt
  | Decl
  | ExprStmt;

export type TsEntityName =
  | TsQualifiedName
  | Ident;

/**
 * 
 * - Invalid: [Ident] with empty symbol.
 */
export type TsEnumMemberId =
  | Ident
  | Str;

export type TsFnOrConstructorType =
  | TsFnType
  | TsConstructorType;

export type TsFnParam =
  | BindingIdent
  | ArrayPat
  | RestPat
  | ObjectPat;

export type TsLit =
  | Number
  | Str
  | Bool
  | BigInt
  | TsTplLitType;

export type TsModuleName =
  | Ident
  | Str;

export type TsModuleRef =
  | TsEntityName
  | TsExternalModuleRef;

/**
 * `namespace A.B { }` is a namespace named `A` with another TsNamespaceDecl as
 * its body.
 */
export type TsNamespaceBody =
  | TsModuleBlock
  | TsNamespaceDecl;

export type TsParamPropParam =
  | BindingIdent
  | AssignPat;

export type TsThisTypeOrIdent =
  | TsThisType
  | Ident;

export type TsType =
  | TsKeywordType
  | TsThisType
  | TsFnOrConstructorType
  | TsTypeRef
  | TsTypeQuery
  | TsTypeLit
  | TsArrayType
  | TsTupleType
  | TsOptionalType
  | TsRestType
  | TsUnionOrIntersectionType
  | TsConditionalType
  | TsInferType
  | TsParenthesizedType
  | TsTypeOperator
  | TsIndexedAccessType
  | TsMappedType
  | TsLitType
  | TsTypePredicate
  | TsImportType;

export type TsTypeElement =
  | TsCallSignatureDecl
  | TsConstructSignatureDecl
  | TsPropertySignature
  | TsMethodSignature
  | TsIndexSignature;

export type TsTypeQueryExpr =
  | TsEntityName
  | TsImportType;

export type TsUnionOrIntersectionType =
  | TsUnionType
  | TsIntersectionType;

export type VarDeclOrExpr =
  | VarDecl
  | Expr;

export type VarDeclOrPat =
  | VarDecl
  | Pat;

export enum Accessibility {
  Public,
  Protected,
  Private,
}

export enum AssignOp {
  /**
   * `=`
   */
  Assign,
  /**
   * `+=`
   */
  AddAssign,
  /**
   * `-=`
   */
  SubAssign,
  /**
   * `*=`
   */
  MulAssign,
  /**
   * `/=`
   */
  DivAssign,
  /**
   * `%=`
   */
  ModAssign,
  /**
   * `<<=`
   */
  LShiftAssign,
  /**
   * `>>=`
   */
  RShiftAssign,
  /**
   * `>>>=`
   */
  ZeroFillRShiftAssign,
  /**
   * `|=`
   */
  BitOrAssign,
  /**
   * `^=`
   */
  BitXorAssign,
  /**
   * `&=`
   */
  BitAndAssign,
  /**
   * `**=`
   */
  ExpAssign,
  /**
   * `&&=`
   */
  AndAssign,
  /**
   * `||=`
   */
  OrAssign,
  /**
   * `??=`
   */
  NullishAssign,
}

export enum BinaryOp {
  /**
   * `==`
   */
  EqEq,
  /**
   * `!=`
   */
  NotEq,
  /**
   * `===`
   */
  EqEqEq,
  /**
   * `!==`
   */
  NotEqEq,
  /**
   * `<`
   */
  Lt,
  /**
   * `<=`
   */
  LtEq,
  /**
   * `>`
   */
  Gt,
  /**
   * `>=`
   */
  GtEq,
  /**
   * `<<`
   */
  LShift,
  /**
   * `>>`
   */
  RShift,
  /**
   * `>>>`
   */
  ZeroFillRShift,
  /**
   * `+`
   */
  Add,
  /**
   * `-`
   */
  Sub,
  /**
   * `*`
   */
  Mul,
  /**
   * `/`
   */
  Div,
  /**
   * `%`
   */
  Mod,
  /**
   * `|`
   */
  BitOr,
  /**
   * `^`
   */
  BitXor,
  /**
   * `&`
   */
  BitAnd,
  /**
   * `||`
   */
  LogicalOr,
  /**
   * `&&`
   */
  LogicalAnd,
  /**
   * `in`
   */
  In,
  /**
   * `instanceof`
   */
  InstanceOf,
  /**
   * `**`
   */
  Exp,
  /**
   * `??`
   */
  NullishCoalescing,
}

export enum EsVersion {
  Es3,
  Es5,
  Es2015,
  Es2016,
  Es2017,
  Es2018,
  Es2019,
  Es2020,
}

export enum MethodKind {
  Method,
  Getter,
  Setter,
}

/**
 * THis enum determines how string literal should be printed.
 */
export enum StrKind {
  /**
   * Span of string points to original source code, and codegen should use
   * it.
   * **Note**: Giving wrong value to this field will result in invalid
   * codegen.
   */
  Normal,
  /**
   * If the span of string does not point a string literal, mainly because
   * this string is synthesized, this variant should be used.
   */
  Synthesized,
}

export enum TruePlusMinus {
  True,
  Plus,
  Minus,
}

export enum TsKeywordTypeKind {
  TsAnyKeyword,
  TsUnknownKeyword,
  TsNumberKeyword,
  TsObjectKeyword,
  TsBooleanKeyword,
  TsBigIntKeyword,
  TsStringKeyword,
  TsSymbolKeyword,
  TsVoidKeyword,
  TsUndefinedKeyword,
  TsNullKeyword,
  TsNeverKeyword,
  TsIntrinsicKeyword,
}

export enum TsTypeOperatorOp {
  /**
   * `keyof`
   */
  KeyOf,
  /**
   * `unique`
   */
  Unique,
  /**
   * `readonly`
   */
  ReadOnly,
}

export enum UnaryOp {
  /**
   * `-`
   */
  Minus,
  /**
   * `+`
   */
  Plus,
  /**
   * `!`
   */
  Bang,
  /**
   * `~`
   */
  Tilde,
  /**
   * `typeof`
   */
  TypeOf,
  /**
   * `void`
   */
  Void,
  /**
   * `delete`
   */
  Delete,
}

export enum UpdateOp {
  /**
   * `++`
   */
  PlusPlus,
  /**
   * `--`
   */
  MinusMinus,
}

export enum VarDeclKind {
  /**
   * `var`
   */
  Var,
  /**
   * `let`
   */
  Let,
  /**
   * `const`
   */
  Const,
}

export enum BinOpToken {
  /**
   * `==`
   */
  EqEq,
  /**
   * `!=`
   */
  NotEq,
  /**
   * `===`
   */
  EqEqEq,
  /**
   * `!==`
   */
  NotEqEq,
  /**
   * `<`
   */
  Lt,
  /**
   * `<=`
   */
  LtEq,
  /**
   * `>`
   */
  Gt,
  /**
   * `>=`
   */
  GtEq,
  /**
   * `<<`
   */
  LShift,
  /**
   * `>>`
   */
  RShift,
  /**
   * `>>>`
   */
  ZeroFillRShift,
  /**
   * `+`
   */
  Add,
  /**
   * `-`
   */
  Sub,
  /**
   * `*`
   */
  Mul,
  /**
   * `/`
   */
  Div,
  /**
   * `%`
   */
  Mod,
  /**
   * `|`
   */
  BitOr,
  /**
   * `^`
   */
  BitXor,
  /**
   * `&`
   */
  BitAnd,
  /**
   * `**`
   */
  Exp,
  /**
   * `||`
   */
  LogicalOr,
  /**
   * `&&`
   */
  LogicalAnd,
  /**
   * `??`
   */
  NullishCoalescing,
}

/**
 * Array literal.
 */
export class ArrayLit extends Node {
  kind!: "ArrayLit";
  parent!: Node;
  elems!: Array<ExprOrSpread | undefined>;
}

export class ArrayPat extends Node {
  kind!: "ArrayPat";
  parent!: Node;
  elems!: Array<Pat | undefined>;
  /**
   * Only in an ambient context
   */
  optional!: boolean;
  type_ann!: TsTypeAnn | undefined;
}

export class ArrowExpr extends Node {
  kind!: "ArrowExpr";
  parent!: Node;
  params!: Array<Pat>;
  body!: BlockStmtOrExpr;
  is_async!: boolean;
  is_generator!: boolean;
  type_params!: TsTypeParamDecl | undefined;
  return_type!: TsTypeAnn | undefined;
}

export class AssignExpr extends Node {
  kind!: "AssignExpr";
  parent!: Node;
  op!: AssignOp;
  left!: PatOrExpr;
  right!: Expr;
}

export class AssignPat extends Node {
  kind!: "AssignPat";
  parent!: Node;
  left!: Pat;
  right!: Expr;
  type_ann!: TsTypeAnn | undefined;
}

/**
 * `{key}` or `{key = value}`
 */
export class AssignPatProp extends Node {
  kind!: "AssignPatProp";
  parent!: ObjectPat;
  key!: Ident;
  value!: Expr | undefined;
}

export class AssignProp extends Node {
  kind!: "AssignProp";
  parent!: ObjectLit;
  key!: Ident;
  value!: Expr;
}

export class AwaitExpr extends Node {
  kind!: "AwaitExpr";
  parent!: Node;
  arg!: Expr;
}

export class BigInt extends Node {
  kind!: "BigInt";
  parent!: Node;
  value!: BigIntValue;
}

export class BinExpr extends Node {
  kind!: "BinExpr";
  parent!: Node;
  op!: BinaryOp;
  left!: Expr;
  right!: Expr;
}

/**
 * Identifer used as a pattern.
 */
export class BindingIdent extends Node {
  kind!: "BindingIdent";
  parent!: Node;
  id!: Ident;
  type_ann!: TsTypeAnn | undefined;
}

/**
 * Use when only block statements are allowed.
 */
export class BlockStmt extends Node {
  kind!: "BlockStmt";
  parent!: Node;
  stmts!: Array<Stmt>;
}

export class Bool extends Node {
  kind!: "Bool";
  parent!: Node;
  value!: boolean;
}

export class BreakStmt extends Node {
  kind!: "BreakStmt";
  parent!: Node;
  label!: Ident | undefined;
}

export class CallExpr extends Node {
  kind!: "CallExpr";
  parent!: Node;
  callee!: ExprOrSuper;
  args!: Array<ExprOrSpread>;
  type_args!: TsTypeParamInstantiation | undefined;
}

export class CatchClause extends Node {
  kind!: "CatchClause";
  parent!: TryStmt;
  /**
   * es2019
   * 
   * The param is null if the catch binding is omitted. E.g., try { foo() }
   * catch { bar() }
   */
  param!: Pat | undefined;
  body!: BlockStmt;
}

export class Class extends Node {
  kind!: "Class";
  parent!: ClassDecl
    | ClassExpr;
  decorators!: Array<Decorator>;
  body!: Array<ClassMember>;
  super_class!: Expr | undefined;
  is_abstract!: boolean;
  type_params!: TsTypeParamDecl | undefined;
  super_type_params!: TsTypeParamInstantiation | undefined;
  /**
   * Typescript extension.
   */
  implements!: Array<TsExprWithTypeArgs>;
}

export class ClassDecl extends Node {
  kind!: "ClassDecl";
  parent!: Node;
  ident!: Ident;
  declare!: boolean;
  class!: Class;
}

/**
 * Class expression.
 */
export class ClassExpr extends Node {
  kind!: "ClassExpr";
  parent!: Node;
  ident!: Ident | undefined;
  class!: Class;
}

export class ClassMethod extends Node {
  kind!: "ClassMethod";
  parent!: Class;
  key!: PropName;
  function!: Function;
  method_kind!: MethodKind;
  is_static!: boolean;
  /**
   * Typescript extension.
   */
  accessibility!: Accessibility | undefined;
  /**
   * Typescript extension.
   */
  is_abstract!: boolean;
  is_optional!: boolean;
}

export class ClassProp extends Node {
  kind!: "ClassProp";
  parent!: Class;
  key!: Expr;
  value!: Expr | undefined;
  type_ann!: TsTypeAnn | undefined;
  is_static!: boolean;
  decorators!: Array<Decorator>;
  computed!: boolean;
  /**
   * Typescript extension.
   */
  accessibility!: Accessibility | undefined;
  /**
   * Typescript extension.
   */
  is_abstract!: boolean;
  is_optional!: boolean;
  readonly!: boolean;
  declare!: boolean;
  definite!: boolean;
}

export class ComputedPropName extends Node {
  kind!: "ComputedPropName";
  parent!: Node;
  expr!: Expr;
}

export class CondExpr extends Node {
  kind!: "CondExpr";
  parent!: Node;
  test!: Expr;
  cons!: Expr;
  alt!: Expr;
}

export class Constructor extends Node {
  kind!: "Constructor";
  parent!: Class;
  key!: PropName;
  params!: Array<ParamOrTsParamProp>;
  body!: BlockStmt | undefined;
  accessibility!: Accessibility | undefined;
  is_optional!: boolean;
}

export class ContinueStmt extends Node {
  kind!: "ContinueStmt";
  parent!: Node;
  label!: Ident | undefined;
}

export class DebuggerStmt extends Node {
  kind!: "DebuggerStmt";
  parent!: Node;
}

export class Decorator extends Node {
  kind!: "Decorator";
  parent!: Node;
  expr!: Expr;
}

export class DoWhileStmt extends Node {
  kind!: "DoWhileStmt";
  parent!: Node;
  test!: Expr;
  body!: Stmt;
}

export class EmptyStmt extends Node {
  kind!: "EmptyStmt";
  parent!: Node;
}

/**
 * `export * from 'mod'`
 */
export class ExportAll extends Node {
  kind!: "ExportAll";
  parent!: Module
    | TsModuleBlock;
  src!: Str;
  asserts!: ObjectLit | undefined;
}

export class ExportDecl extends Node {
  kind!: "ExportDecl";
  parent!: Module
    | TsModuleBlock;
  decl!: Decl;
}

export class ExportDefaultDecl extends Node {
  kind!: "ExportDefaultDecl";
  parent!: Module
    | TsModuleBlock;
  decl!: DefaultDecl;
}

export class ExportDefaultExpr extends Node {
  kind!: "ExportDefaultExpr";
  parent!: Module
    | TsModuleBlock;
  expr!: Expr;
}

export class ExportDefaultSpecifier extends Node {
  kind!: "ExportDefaultSpecifier";
  parent!: NamedExport;
  exported!: Ident;
}

export class ExportNamedSpecifier extends Node {
  kind!: "ExportNamedSpecifier";
  parent!: NamedExport;
  /**
   * `foo` in `export { foo as bar }`
   */
  orig!: Ident;
  /**
   * `Some(bar)` in `export { foo as bar }`
   */
  exported!: Ident | undefined;
}

/**
 * `export * as foo from 'src';`
 */
export class ExportNamespaceSpecifier extends Node {
  kind!: "ExportNamespaceSpecifier";
  parent!: NamedExport;
  name!: Ident;
}

export class ExprOrSpread extends Node {
  kind!: "ExprOrSpread";
  parent!: ArrayLit
    | CallExpr
    | NewExpr;
  spread!: Span | undefined;
  expr!: Expr;
}

export class ExprStmt extends Node {
  kind!: "ExprStmt";
  parent!: Node;
  expr!: Expr;
}

export class FnDecl extends Node {
  kind!: "FnDecl";
  parent!: Node;
  ident!: Ident;
  declare!: boolean;
  function!: Function;
}

/**
 * Function expression.
 */
export class FnExpr extends Node {
  kind!: "FnExpr";
  parent!: Node;
  ident!: Ident | undefined;
  function!: Function;
}

export class ForInStmt extends Node {
  kind!: "ForInStmt";
  parent!: Node;
  left!: VarDeclOrPat;
  right!: Expr;
  body!: Stmt;
}

export class ForOfStmt extends Node {
  kind!: "ForOfStmt";
  parent!: Node;
  /**
   * Span of the await token.
   * 
   * es2018
   * 
   * for-await-of statements, e.g., `for await (const x of xs) {`
   */
  await_token!: Span | undefined;
  left!: VarDeclOrPat;
  right!: Expr;
  body!: Stmt;
}

export class ForStmt extends Node {
  kind!: "ForStmt";
  parent!: Node;
  init!: VarDeclOrExpr | undefined;
  test!: Expr | undefined;
  update!: Expr | undefined;
  body!: Stmt;
}

/**
 * Common parts of function and method.
 */
export class Function extends Node {
  kind!: "Function";
  parent!: Node;
  params!: Array<Param>;
  decorators!: Array<Decorator>;
  body!: BlockStmt | undefined;
  /**
   * if it's a generator.
   */
  is_generator!: boolean;
  /**
   * if it's an async function.
   */
  is_async!: boolean;
  type_params!: TsTypeParamDecl | undefined;
  return_type!: TsTypeAnn | undefined;
}

export class GetterProp extends Node {
  kind!: "GetterProp";
  parent!: ObjectLit;
  key!: PropName;
  type_ann!: TsTypeAnn | undefined;
  body!: BlockStmt | undefined;
}

/**
 * Ident with span.
 */
export class Ident extends Node {
  kind!: "Ident";
  parent!: Node;
  sym!: JsWord;
  /**
   * TypeScript only. Used in case of an optional parameter.
   */
  optional!: boolean;
}

export class IfStmt extends Node {
  kind!: "IfStmt";
  parent!: Node;
  test!: Expr;
  cons!: Stmt;
  alt!: Stmt | undefined;
}

export class ImportDecl extends Node {
  kind!: "ImportDecl";
  parent!: Module
    | TsModuleBlock;
  specifiers!: Array<ImportSpecifier>;
  src!: Str;
  type_only!: boolean;
  asserts!: ObjectLit | undefined;
}

/**
 * e.g. `import foo from 'mod.js'`
 */
export class ImportDefaultSpecifier extends Node {
  kind!: "ImportDefaultSpecifier";
  parent!: ImportDecl;
  local!: Ident;
}

/**
 * e.g. local = foo, imported = None `import { foo } from 'mod.js'`
 * e.g. local = bar, imported = Some(foo) for `import { foo as bar } from
 * 'mod.js'`
 */
export class ImportNamedSpecifier extends Node {
  kind!: "ImportNamedSpecifier";
  parent!: ImportDecl;
  local!: Ident;
  imported!: Ident | undefined;
}

/**
 * e.g. `import * as foo from 'mod.js'`.
 */
export class ImportStarAsSpecifier extends Node {
  kind!: "ImportStarAsSpecifier";
  parent!: ImportDecl;
  local!: Ident;
}

/**
 * Represents a invalid node.
 */
export class Invalid extends Node {
  kind!: "Invalid";
  parent!: Node;
}

export class JSXAttr extends Node {
  kind!: "JSXAttr";
  parent!: JSXOpeningElement;
  name!: JSXAttrName;
  /**
   * Babel uses Expr instead of JSXAttrValue
   */
  value!: JSXAttrValue | undefined;
}

export class JSXClosingElement extends Node {
  kind!: "JSXClosingElement";
  parent!: JSXElement;
  name!: JSXElementName;
}

export class JSXClosingFragment extends Node {
  kind!: "JSXClosingFragment";
  parent!: JSXFragment;
}

export class JSXElement extends Node {
  kind!: "JSXElement";
  parent!: Node;
  opening!: JSXOpeningElement;
  children!: Array<JSXElementChild>;
  closing!: JSXClosingElement | undefined;
}

export class JSXEmptyExpr extends Node {
  kind!: "JSXEmptyExpr";
  parent!: Node;
}

export class JSXExprContainer extends Node {
  kind!: "JSXExprContainer";
  parent!: JSXAttr
    | JSXElement
    | JSXFragment;
  expr!: JSXExpr;
}

export class JSXFragment extends Node {
  kind!: "JSXFragment";
  parent!: Node;
  opening!: JSXOpeningFragment;
  children!: Array<JSXElementChild>;
  closing!: JSXClosingFragment;
}

export class JSXMemberExpr extends Node {
  kind!: "JSXMemberExpr";
  parent!: Node;
  obj!: JSXObject;
  prop!: Ident;
}

/**
 * XML-based namespace syntax:
 */
export class JSXNamespacedName extends Node {
  kind!: "JSXNamespacedName";
  parent!: Node;
  ns!: Ident;
  name!: Ident;
}

export class JSXOpeningElement extends Node {
  kind!: "JSXOpeningElement";
  parent!: JSXElement;
  name!: JSXElementName;
  attrs!: Array<JSXAttrOrSpread>;
  self_closing!: boolean;
  /**
   * Note: This field's name is different from one from babel because it is
   * misleading
   */
  type_args!: TsTypeParamInstantiation | undefined;
}

export class JSXOpeningFragment extends Node {
  kind!: "JSXOpeningFragment";
  parent!: JSXFragment;
}

export class JSXSpreadChild extends Node {
  kind!: "JSXSpreadChild";
  parent!: JSXElement
    | JSXFragment;
  expr!: Expr;
}

export class JSXText extends Node {
  kind!: "JSXText";
  parent!: Node;
  value!: JsWord;
  raw!: JsWord;
}

/**
 * `{key: value}`
 */
export class KeyValuePatProp extends Node {
  kind!: "KeyValuePatProp";
  parent!: ObjectPat;
  key!: PropName;
  value!: Pat;
}

export class KeyValueProp extends Node {
  kind!: "KeyValueProp";
  parent!: ObjectLit;
  key!: PropName;
  value!: Expr;
}

export class LabeledStmt extends Node {
  kind!: "LabeledStmt";
  parent!: Node;
  label!: Ident;
  body!: Stmt;
}

export class MemberExpr extends Node {
  kind!: "MemberExpr";
  parent!: Node;
  obj!: ExprOrSuper;
  prop!: Expr;
  computed!: boolean;
}

export class MetaPropExpr extends Node {
  kind!: "MetaPropExpr";
  parent!: Node;
  meta!: Ident;
  prop!: Ident;
}

export class MethodProp extends Node {
  kind!: "MethodProp";
  parent!: ObjectLit;
  key!: PropName;
  function!: Function;
}

export class Module extends Node {
  kind!: "Module";
  body!: Array<ModuleItem>;
  shebang!: JsWord | undefined;
}

/**
 * `export { foo } from 'mod'`
 * `export { foo as bar } from 'mod'`
 */
export class NamedExport extends Node {
  kind!: "NamedExport";
  parent!: Module
    | TsModuleBlock;
  specifiers!: Array<ExportSpecifier>;
  src!: Str | undefined;
  type_only!: boolean;
  asserts!: ObjectLit | undefined;
}

export class NewExpr extends Node {
  kind!: "NewExpr";
  parent!: Node;
  callee!: Expr;
  args!: Array<ExprOrSpread> | undefined;
  type_args!: TsTypeParamInstantiation | undefined;
}

export class Null extends Node {
  kind!: "Null";
  parent!: Node;
}

export class Number extends Node {
  kind!: "Number";
  parent!: Node;
  /**
   * **Note**: This should not be `NaN`. Use [crate::Ident] to represent NaN.
   * 
   * If you store `NaN` in this field, a hash map will behave strangely.
   */
  value!: number;
}

/**
 * Object literal.
 */
export class ObjectLit extends Node {
  kind!: "ObjectLit";
  parent!: Node;
  props!: Array<PropOrSpread>;
}

export class ObjectPat extends Node {
  kind!: "ObjectPat";
  parent!: Node;
  props!: Array<ObjectPatProp>;
  /**
   * Only in an ambient context
   */
  optional!: boolean;
  type_ann!: TsTypeAnn | undefined;
}

export class OptChainExpr extends Node {
  kind!: "OptChainExpr";
  parent!: Node;
  question_dot_token!: Span;
  expr!: Expr;
}

export class Param extends Node {
  kind!: "Param";
  parent!: Constructor
    | Function;
  decorators!: Array<Decorator>;
  pat!: Pat;
}

export class ParenExpr extends Node {
  kind!: "ParenExpr";
  parent!: Node;
  expr!: Expr;
}

export class PrivateMethod extends Node {
  kind!: "PrivateMethod";
  parent!: Class;
  key!: PrivateName;
  function!: Function;
  method_kind!: MethodKind;
  is_static!: boolean;
  /**
   * Typescript extension.
   */
  accessibility!: Accessibility | undefined;
  /**
   * Typescript extension.
   */
  is_abstract!: boolean;
  is_optional!: boolean;
}

export class PrivateName extends Node {
  kind!: "PrivateName";
  parent!: Node;
  id!: Ident;
}

export class PrivateProp extends Node {
  kind!: "PrivateProp";
  parent!: Class;
  key!: PrivateName;
  value!: Expr | undefined;
  type_ann!: TsTypeAnn | undefined;
  is_static!: boolean;
  decorators!: Array<Decorator>;
  computed!: boolean;
  /**
   * Typescript extension.
   */
  accessibility!: Accessibility | undefined;
  /**
   * Typescript extension.
   */
  is_abstract!: boolean;
  is_optional!: boolean;
  readonly!: boolean;
  definite!: boolean;
}

export class Regex extends Node {
  kind!: "Regex";
  parent!: Node;
  exp!: JsWord;
  flags!: JsWord;
}

/**
 * EsTree `RestElement`
 */
export class RestPat extends Node {
  kind!: "RestPat";
  parent!: Node;
  dot3_token!: Span;
  arg!: Pat;
  type_ann!: TsTypeAnn | undefined;
}

export class ReturnStmt extends Node {
  kind!: "ReturnStmt";
  parent!: Node;
  arg!: Expr | undefined;
}

export class Script extends Node {
  kind!: "Script";
  body!: Array<Stmt>;
  shebang!: JsWord | undefined;
}

export class SeqExpr extends Node {
  kind!: "SeqExpr";
  parent!: Node;
  exprs!: Array<Expr>;
}

export class SetterProp extends Node {
  kind!: "SetterProp";
  parent!: ObjectLit;
  key!: PropName;
  param!: Pat;
  body!: BlockStmt | undefined;
}

export class SpreadElement extends Node {
  kind!: "SpreadElement";
  parent!: JSXOpeningElement
    | ObjectLit;
  dot3_token!: Span;
  expr!: Expr;
}

export class Str extends Node {
  kind!: "Str";
  parent!: Node;
  value!: JsWord;
  /**
   * This includes line escape.
   */
  has_escape!: boolean;
  str_kind!: StrKind;
}

export class Super extends Node {
  kind!: "Super";
  parent!: CallExpr
    | MemberExpr;
}

export class SwitchCase extends Node {
  kind!: "SwitchCase";
  parent!: SwitchStmt;
  /**
   * None for `default:`
   */
  test!: Expr | undefined;
  cons!: Array<Stmt>;
}

export class SwitchStmt extends Node {
  kind!: "SwitchStmt";
  parent!: Node;
  discriminant!: Expr;
  cases!: Array<SwitchCase>;
}

export class TaggedTpl extends Node {
  kind!: "TaggedTpl";
  parent!: Node;
  tag!: Expr;
  exprs!: Array<Expr>;
  quasis!: Array<TplElement>;
  type_params!: TsTypeParamInstantiation | undefined;
}

export class ThisExpr extends Node {
  kind!: "ThisExpr";
  parent!: Node;
}

export class ThrowStmt extends Node {
  kind!: "ThrowStmt";
  parent!: Node;
  arg!: Expr;
}

export class Tpl extends Node {
  kind!: "Tpl";
  parent!: Node;
  exprs!: Array<Expr>;
  quasis!: Array<TplElement>;
}

export class TplElement extends Node {
  kind!: "TplElement";
  parent!: TaggedTpl
    | Tpl
    | TsTplLitType;
  tail!: boolean;
  cooked!: Str | undefined;
  raw!: Str;
}

export class TryStmt extends Node {
  kind!: "TryStmt";
  parent!: Node;
  block!: BlockStmt;
  handler!: CatchClause | undefined;
  finalizer!: BlockStmt | undefined;
}

export class TsArrayType extends Node {
  kind!: "TsArrayType";
  parent!: Node;
  elem_type!: TsType;
}

export class TsAsExpr extends Node {
  kind!: "TsAsExpr";
  parent!: Node;
  expr!: Expr;
  type_ann!: TsType;
}

export class TsCallSignatureDecl extends Node {
  kind!: "TsCallSignatureDecl";
  parent!: TsInterfaceBody
    | TsTypeLit;
  params!: Array<TsFnParam>;
  type_ann!: TsTypeAnn | undefined;
  type_params!: TsTypeParamDecl | undefined;
}

export class TsConditionalType extends Node {
  kind!: "TsConditionalType";
  parent!: Node;
  check_type!: TsType;
  extends_type!: TsType;
  true_type!: TsType;
  false_type!: TsType;
}

export class TsConstAssertion extends Node {
  kind!: "TsConstAssertion";
  parent!: Node;
  expr!: Expr;
}

export class TsConstructSignatureDecl extends Node {
  kind!: "TsConstructSignatureDecl";
  parent!: TsInterfaceBody
    | TsTypeLit;
  params!: Array<TsFnParam>;
  type_ann!: TsTypeAnn | undefined;
  type_params!: TsTypeParamDecl | undefined;
}

export class TsConstructorType extends Node {
  kind!: "TsConstructorType";
  parent!: Node;
  params!: Array<TsFnParam>;
  type_params!: TsTypeParamDecl | undefined;
  type_ann!: TsTypeAnn;
  is_abstract!: boolean;
}

export class TsEnumDecl extends Node {
  kind!: "TsEnumDecl";
  parent!: Node;
  declare!: boolean;
  is_const!: boolean;
  id!: Ident;
  members!: Array<TsEnumMember>;
}

export class TsEnumMember extends Node {
  kind!: "TsEnumMember";
  parent!: TsEnumDecl;
  id!: TsEnumMemberId;
  init!: Expr | undefined;
}

/**
 * TypeScript's own parser uses ExportAssignment for both `export default` and
 * `export =`. But for @babel/parser, `export default` is an ExportDefaultDecl,
 * so a TsExportAssignment is always `export =`.
 */
export class TsExportAssignment extends Node {
  kind!: "TsExportAssignment";
  parent!: Module
    | TsModuleBlock;
  expr!: Expr;
}

export class TsExprWithTypeArgs extends Node {
  kind!: "TsExprWithTypeArgs";
  parent!: Class
    | TsInterfaceDecl;
  expr!: TsEntityName;
  type_args!: TsTypeParamInstantiation | undefined;
}

export class TsExternalModuleRef extends Node {
  kind!: "TsExternalModuleRef";
  parent!: TsImportEqualsDecl;
  expr!: Str;
}

export class TsFnType extends Node {
  kind!: "TsFnType";
  parent!: Node;
  params!: Array<TsFnParam>;
  type_params!: TsTypeParamDecl | undefined;
  type_ann!: TsTypeAnn;
}

export class TsImportEqualsDecl extends Node {
  kind!: "TsImportEqualsDecl";
  parent!: Module
    | TsModuleBlock;
  declare!: boolean;
  is_export!: boolean;
  id!: Ident;
  module_ref!: TsModuleRef;
}

export class TsImportType extends Node {
  kind!: "TsImportType";
  parent!: Node;
  arg!: Str;
  qualifier!: TsEntityName | undefined;
  type_args!: TsTypeParamInstantiation | undefined;
}

export class TsIndexSignature extends Node {
  kind!: "TsIndexSignature";
  parent!: Class
    | TsInterfaceBody
    | TsTypeLit;
  params!: Array<TsFnParam>;
  type_ann!: TsTypeAnn | undefined;
  readonly!: boolean;
}

export class TsIndexedAccessType extends Node {
  kind!: "TsIndexedAccessType";
  parent!: Node;
  readonly!: boolean;
  obj_type!: TsType;
  index_type!: TsType;
}

export class TsInferType extends Node {
  kind!: "TsInferType";
  parent!: Node;
  type_param!: TsTypeParam;
}

export class TsInterfaceBody extends Node {
  kind!: "TsInterfaceBody";
  parent!: TsInterfaceDecl;
  body!: Array<TsTypeElement>;
}

export class TsInterfaceDecl extends Node {
  kind!: "TsInterfaceDecl";
  parent!: Node;
  id!: Ident;
  declare!: boolean;
  type_params!: TsTypeParamDecl | undefined;
  extends!: Array<TsExprWithTypeArgs>;
  body!: TsInterfaceBody;
}

export class TsIntersectionType extends Node {
  kind!: "TsIntersectionType";
  parent!: Node;
  types!: Array<TsType>;
}

export class TsKeywordType extends Node {
  kind!: "TsKeywordType";
  parent!: Node;
  keyword_kind!: TsKeywordTypeKind;
}

export class TsLitType extends Node {
  kind!: "TsLitType";
  parent!: Node;
  lit!: TsLit;
}

export class TsMappedType extends Node {
  kind!: "TsMappedType";
  parent!: Node;
  readonly!: TruePlusMinus | undefined;
  type_param!: TsTypeParam;
  name_type!: TsType | undefined;
  optional!: TruePlusMinus | undefined;
  type_ann!: TsType | undefined;
}

export class TsMethodSignature extends Node {
  kind!: "TsMethodSignature";
  parent!: TsInterfaceBody
    | TsTypeLit;
  readonly!: boolean;
  key!: Expr;
  computed!: boolean;
  optional!: boolean;
  params!: Array<TsFnParam>;
  type_ann!: TsTypeAnn | undefined;
  type_params!: TsTypeParamDecl | undefined;
}

export class TsModuleBlock extends Node {
  kind!: "TsModuleBlock";
  parent!: TsModuleDecl
    | TsNamespaceDecl;
  body!: Array<ModuleItem>;
}

export class TsModuleDecl extends Node {
  kind!: "TsModuleDecl";
  parent!: Node;
  declare!: boolean;
  /**
   * In TypeScript, this is only available through`node.flags`.
   */
  global!: boolean;
  id!: TsModuleName;
  body!: TsNamespaceBody | undefined;
}

export class TsNamespaceDecl extends Node {
  kind!: "TsNamespaceDecl";
  parent!: TsModuleDecl
    | TsNamespaceDecl;
  declare!: boolean;
  /**
   * In TypeScript, this is only available through`node.flags`.
   */
  global!: boolean;
  id!: Ident;
  body!: TsNamespaceBody;
}

export class TsNamespaceExportDecl extends Node {
  kind!: "TsNamespaceExportDecl";
  parent!: Module
    | TsModuleBlock;
  id!: Ident;
}

export class TsNonNullExpr extends Node {
  kind!: "TsNonNullExpr";
  parent!: Node;
  expr!: Expr;
}

export class TsOptionalType extends Node {
  kind!: "TsOptionalType";
  parent!: Node;
  type_ann!: TsType;
}

export class TsParamProp extends Node {
  kind!: "TsParamProp";
  parent!: Constructor;
  decorators!: Array<Decorator>;
  /**
   * At least one of `accessibility` or `readonly` must be set.
   */
  accessibility!: Accessibility | undefined;
  readonly!: boolean;
  param!: TsParamPropParam;
}

export class TsParenthesizedType extends Node {
  kind!: "TsParenthesizedType";
  parent!: Node;
  type_ann!: TsType;
}

export class TsPropertySignature extends Node {
  kind!: "TsPropertySignature";
  parent!: TsInterfaceBody
    | TsTypeLit;
  readonly!: boolean;
  key!: Expr;
  computed!: boolean;
  optional!: boolean;
  init!: Expr | undefined;
  params!: Array<TsFnParam>;
  type_ann!: TsTypeAnn | undefined;
  type_params!: TsTypeParamDecl | undefined;
}

export class TsQualifiedName extends Node {
  kind!: "TsQualifiedName";
  parent!: Node;
  left!: TsEntityName;
  right!: Ident;
}

export class TsRestType extends Node {
  kind!: "TsRestType";
  parent!: Node;
  type_ann!: TsType;
}

export class TsThisType extends Node {
  kind!: "TsThisType";
  parent!: Node;
}

export class TsTplLitType extends Node {
  kind!: "TsTplLitType";
  parent!: TsLitType;
  types!: Array<TsType>;
  quasis!: Array<TplElement>;
}

export class TsTupleElement extends Node {
  kind!: "TsTupleElement";
  parent!: TsTupleType;
  /**
   * `Ident` or `RestPat { arg: Ident }`
   */
  label!: Pat | undefined;
  ty!: TsType;
}

export class TsTupleType extends Node {
  kind!: "TsTupleType";
  parent!: Node;
  elem_types!: Array<TsTupleElement>;
}

export class TsTypeAliasDecl extends Node {
  kind!: "TsTypeAliasDecl";
  parent!: Node;
  declare!: boolean;
  id!: Ident;
  type_params!: TsTypeParamDecl | undefined;
  type_ann!: TsType;
}

export class TsTypeAnn extends Node {
  kind!: "TsTypeAnn";
  parent!: Node;
  type_ann!: TsType;
}

export class TsTypeAssertion extends Node {
  kind!: "TsTypeAssertion";
  parent!: Node;
  expr!: Expr;
  type_ann!: TsType;
}

export class TsTypeLit extends Node {
  kind!: "TsTypeLit";
  parent!: Node;
  members!: Array<TsTypeElement>;
}

export class TsTypeOperator extends Node {
  kind!: "TsTypeOperator";
  parent!: Node;
  op!: TsTypeOperatorOp;
  type_ann!: TsType;
}

export class TsTypeParam extends Node {
  kind!: "TsTypeParam";
  parent!: TsInferType
    | TsMappedType
    | TsTypeParamDecl;
  name!: Ident;
  constraint!: TsType | undefined;
  default!: TsType | undefined;
}

export class TsTypeParamDecl extends Node {
  kind!: "TsTypeParamDecl";
  parent!: Node;
  params!: Array<TsTypeParam>;
}

export class TsTypeParamInstantiation extends Node {
  kind!: "TsTypeParamInstantiation";
  parent!: Node;
  params!: Array<TsType>;
}

export class TsTypePredicate extends Node {
  kind!: "TsTypePredicate";
  parent!: Node;
  asserts!: boolean;
  param_name!: TsThisTypeOrIdent;
  type_ann!: TsTypeAnn | undefined;
}

/**
 * `typeof` operator
 */
export class TsTypeQuery extends Node {
  kind!: "TsTypeQuery";
  parent!: Node;
  expr_name!: TsTypeQueryExpr;
}

export class TsTypeRef extends Node {
  kind!: "TsTypeRef";
  parent!: Node;
  type_name!: TsEntityName;
  type_params!: TsTypeParamInstantiation | undefined;
}

export class TsUnionType extends Node {
  kind!: "TsUnionType";
  parent!: Node;
  types!: Array<TsType>;
}

export class UnaryExpr extends Node {
  kind!: "UnaryExpr";
  parent!: Node;
  op!: UnaryOp;
  arg!: Expr;
}

export class UpdateExpr extends Node {
  kind!: "UpdateExpr";
  parent!: Node;
  op!: UpdateOp;
  prefix!: boolean;
  arg!: Expr;
}

export class VarDecl extends Node {
  kind!: "VarDecl";
  parent!: Node;
  decl_kind!: VarDeclKind;
  declare!: boolean;
  decls!: Array<VarDeclarator>;
}

export class VarDeclarator extends Node {
  kind!: "VarDeclarator";
  parent!: VarDecl;
  name!: Pat;
  /**
   * Initialization expression.
   */
  init!: Expr | undefined;
  /**
   * Typescript only
   */
  definite!: boolean;
}

export class WhileStmt extends Node {
  kind!: "WhileStmt";
  parent!: Node;
  test!: Expr;
  body!: Stmt;
}

export class WithStmt extends Node {
  kind!: "WithStmt";
  parent!: Node;
  obj!: Expr;
  body!: Stmt;
}

export class YieldExpr extends Node {
  kind!: "YieldExpr";
  parent!: Node;
  arg!: Expr | undefined;
  delegate!: boolean;
}
