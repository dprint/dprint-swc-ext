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

/** Used for `obj` property of `JSXMemberExpr`. */
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
  /** `=` */
  Assign,
  /** `+=` */
  AddAssign,
  /** `-=` */
  SubAssign,
  /** `*=` */
  MulAssign,
  /** `/=` */
  DivAssign,
  /** `%=` */
  ModAssign,
  /** `<<=` */
  LShiftAssign,
  /** `>>=` */
  RShiftAssign,
  /** `>>>=` */
  ZeroFillRShiftAssign,
  /** `|=` */
  BitOrAssign,
  /** `^=` */
  BitXorAssign,
  /** `&=` */
  BitAndAssign,
  /** `**=` */
  ExpAssign,
  /** `&&=` */
  AndAssign,
  /** `||=` */
  OrAssign,
  /** `??=` */
  NullishAssign,
}

export enum BinaryOp {
  /** `==` */
  EqEq,
  /** `!=` */
  NotEq,
  /** `===` */
  EqEqEq,
  /** `!==` */
  NotEqEq,
  /** `<` */
  Lt,
  /** `<=` */
  LtEq,
  /** `>` */
  Gt,
  /** `>=` */
  GtEq,
  /** `<<` */
  LShift,
  /** `>>` */
  RShift,
  /** `>>>` */
  ZeroFillRShift,
  /** `+` */
  Add,
  /** `-` */
  Sub,
  /** `*` */
  Mul,
  /** `/` */
  Div,
  /** `%` */
  Mod,
  /** `|` */
  BitOr,
  /** `^` */
  BitXor,
  /** `&` */
  BitAnd,
  /** `||` */
  LogicalOr,
  /** `&&` */
  LogicalAnd,
  /** `in` */
  In,
  /** `instanceof` */
  InstanceOf,
  /** `**` */
  Exp,
  /** `??` */
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

/** THis enum determines how string literal should be printed. */
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
  /** `keyof` */
  KeyOf,
  /** `unique` */
  Unique,
  /** `readonly` */
  ReadOnly,
}

export enum UnaryOp {
  /** `-` */
  Minus,
  /** `+` */
  Plus,
  /** `!` */
  Bang,
  /** `~` */
  Tilde,
  /** `typeof` */
  TypeOf,
  /** `void` */
  Void,
  /** `delete` */
  Delete,
}

export enum UpdateOp {
  /** `++` */
  PlusPlus,
  /** `--` */
  MinusMinus,
}

export enum VarDeclKind {
  /** `var` */
  Var,
  /** `let` */
  Let,
  /** `const` */
  Const,
}

export enum BinOpToken {
  /** `==` */
  EqEq,
  /** `!=` */
  NotEq,
  /** `===` */
  EqEqEq,
  /** `!==` */
  NotEqEq,
  /** `<` */
  Lt,
  /** `<=` */
  LtEq,
  /** `>` */
  Gt,
  /** `>=` */
  GtEq,
  /** `<<` */
  LShift,
  /** `>>` */
  RShift,
  /** `>>>` */
  ZeroFillRShift,
  /** `+` */
  Add,
  /** `-` */
  Sub,
  /** `*` */
  Mul,
  /** `/` */
  Div,
  /** `%` */
  Mod,
  /** `|` */
  BitOr,
  /** `^` */
  BitXor,
  /** `&` */
  BitAnd,
  /** `**` */
  Exp,
  /** `||` */
  LogicalOr,
  /** `&&` */
  LogicalAnd,
  /** `??` */
  NullishCoalescing,
}

/** Keywords */
export enum Keyword {
  /** Spec says this might be identifier. */
  Await,
  Break,
  Case,
  Catch,
  Continue,
  Debugger,
  Default_,
  Do,
  Else,
  Finally,
  For,
  Function,
  If,
  Return,
  Switch,
  Throw,
  Try,
  Var,
  Let,
  Const,
  While,
  With,
  New,
  This,
  Super,
  Class,
  Extends,
  Export,
  Import,
  /** Spec says this might be identifier. */
  Yield,
  In,
  InstanceOf,
  TypeOf,
  Void,
  Delete,
}

export type Token =
  | TokenWord
  | TokenKind.Arrow
  | TokenKind.Hash
  | TokenKind.At
  | TokenKind.Dot
  | TokenKind.DotDotDot
  | TokenKind.Bang
  | TokenKind.LParen
  | TokenKind.RParen
  | TokenKind.LBracket
  | TokenKind.RBracket
  | TokenKind.LBrace
  | TokenKind.RBrace
  | TokenKind.Semi
  | TokenKind.Comma
  | TokenKind.BackQuote
  | TokenTemplate
  | TokenKind.Colon
  | TokenKind.ColonColon
  | TokenBinOp
  | TokenAssignOp
  | TokenKind.DollarLBrace
  | TokenKind.QuestionMark
  | TokenKind.PlusPlus
  | TokenKind.MinusMinus
  | TokenKind.Tilde
  | TokenStr
  | TokenRegex
  | TokenNum
  | TokenBigInt
  | TokenJSXName
  | TokenJSXText
  | TokenKind.JSXTagStart
  | TokenKind.JSXTagEnd
  | TokenShebang
  | TokenError;

export enum TokenKind {
  Word,
  /** '=>' */
  Arrow,
  /** '#' */
  Hash,
  /** '@' */
  At,
  /** '.' */
  Dot,
  /** '...' */
  DotDotDot,
  /** '!' */
  Bang,
  /** '(' */
  LParen,
  /** ')' */
  RParen,
  /** `[` */
  LBracket,
  /** ']' */
  RBracket,
  /** '{' */
  LBrace,
  /** '}' */
  RBrace,
  /** ';' */
  Semi,
  /** ',' */
  Comma,
  /** '`' */
  BackQuote,
  Template,
  /** ':' */
  Colon,
  /** '::' */
  ColonColon,
  BinOp,
  AssignOp,
  /** '${' */
  DollarLBrace,
  /** '?' */
  QuestionMark,
  /** `++` */
  PlusPlus,
  /** `--` */
  MinusMinus,
  /** `~` */
  Tilde,
  Str,
  Regex,
  Num,
  BigInt,
  JSXName,
  JSXText,
  JSXTagStart,
  JSXTagEnd,
  Shebang,
  Error,
}

/**
 * Identifier, "null", "true", "false".
 * 
 * Contains `null` and ``
 */
export interface TokenWord {
  kind: TokenKind.Word;
  inner: Word;
}

export interface TokenTemplate {
  kind: TokenKind.Template;
  raw: JsWord;
  cooked: JsWord | undefined;
  has_escape: boolean;
}

export interface TokenBinOp {
  kind: TokenKind.BinOp;
  inner: BinOpToken;
}

export interface TokenAssignOp {
  kind: TokenKind.AssignOp;
  inner: AssignOp;
}

/** String literal. Span of this token contains quote. */
export interface TokenStr {
  kind: TokenKind.Str;
  value: JsWord;
  has_escape: boolean;
}

/** Regexp literal. */
export interface TokenRegex {
  kind: TokenKind.Regex;
  inner: [JsWord, JsWord];
}

/** TODO: Make Num as enum and separate decimal, binary, ..etc */
export interface TokenNum {
  kind: TokenKind.Num;
  inner: number;
}

export interface TokenBigInt {
  kind: TokenKind.BigInt;
  inner: BigIntValue;
}

export interface TokenJSXName {
  kind: TokenKind.JSXName;
  name: JsWord;
}

export interface TokenJSXText {
  kind: TokenKind.JSXText;
  raw: JsWord;
}

export interface TokenShebang {
  kind: TokenKind.Shebang;
  inner: JsWord;
}

export interface TokenError {
  kind: TokenKind.Error;
  inner: Error;
}

export type Word =
  | WordKeyword
  | WordKind.Null
  | WordKind.True
  | WordKind.False
  | WordIdent;

export enum WordKind {
  Keyword,
  Null,
  True,
  False,
  Ident,
}

export interface WordKeyword {
  kind: WordKind.Keyword;
  inner: Keyword;
}

export interface WordIdent {
  kind: WordKind.Ident;
  inner: JsWord;
}

export enum NodeKind {
  ArrayLit,
  ArrayPat,
  ArrowExpr,
  AssignExpr,
  AssignPat,
  AssignPatProp,
  AssignProp,
  AwaitExpr,
  BigInt,
  BinExpr,
  BindingIdent,
  BlockStmt,
  Bool,
  BreakStmt,
  CallExpr,
  CatchClause,
  Class,
  ClassDecl,
  ClassExpr,
  ClassMethod,
  ClassProp,
  ComputedPropName,
  CondExpr,
  Constructor,
  ContinueStmt,
  DebuggerStmt,
  Decorator,
  DoWhileStmt,
  EmptyStmt,
  ExportAll,
  ExportDecl,
  ExportDefaultDecl,
  ExportDefaultExpr,
  ExportDefaultSpecifier,
  ExportNamedSpecifier,
  ExportNamespaceSpecifier,
  ExprOrSpread,
  ExprStmt,
  FnDecl,
  FnExpr,
  ForInStmt,
  ForOfStmt,
  ForStmt,
  Function,
  GetterProp,
  Ident,
  IfStmt,
  ImportDecl,
  ImportDefaultSpecifier,
  ImportNamedSpecifier,
  ImportStarAsSpecifier,
  Invalid,
  JSXAttr,
  JSXClosingElement,
  JSXClosingFragment,
  JSXElement,
  JSXEmptyExpr,
  JSXExprContainer,
  JSXFragment,
  JSXMemberExpr,
  JSXNamespacedName,
  JSXOpeningElement,
  JSXOpeningFragment,
  JSXSpreadChild,
  JSXText,
  KeyValuePatProp,
  KeyValueProp,
  LabeledStmt,
  MemberExpr,
  MetaPropExpr,
  MethodProp,
  Module,
  NamedExport,
  NewExpr,
  Null,
  Number,
  ObjectLit,
  ObjectPat,
  OptChainExpr,
  Param,
  ParenExpr,
  PrivateMethod,
  PrivateName,
  PrivateProp,
  Regex,
  RestPat,
  ReturnStmt,
  Script,
  SeqExpr,
  SetterProp,
  SpreadElement,
  Str,
  Super,
  SwitchCase,
  SwitchStmt,
  TaggedTpl,
  ThisExpr,
  ThrowStmt,
  Tpl,
  TplElement,
  TryStmt,
  TsArrayType,
  TsAsExpr,
  TsCallSignatureDecl,
  TsConditionalType,
  TsConstAssertion,
  TsConstructSignatureDecl,
  TsConstructorType,
  TsEnumDecl,
  TsEnumMember,
  TsExportAssignment,
  TsExprWithTypeArgs,
  TsExternalModuleRef,
  TsFnType,
  TsImportEqualsDecl,
  TsImportType,
  TsIndexSignature,
  TsIndexedAccessType,
  TsInferType,
  TsInterfaceBody,
  TsInterfaceDecl,
  TsIntersectionType,
  TsKeywordType,
  TsLitType,
  TsMappedType,
  TsMethodSignature,
  TsModuleBlock,
  TsModuleDecl,
  TsNamespaceDecl,
  TsNamespaceExportDecl,
  TsNonNullExpr,
  TsOptionalType,
  TsParamProp,
  TsParenthesizedType,
  TsPropertySignature,
  TsQualifiedName,
  TsRestType,
  TsThisType,
  TsTplLitType,
  TsTupleElement,
  TsTupleType,
  TsTypeAliasDecl,
  TsTypeAnn,
  TsTypeAssertion,
  TsTypeLit,
  TsTypeOperator,
  TsTypeParam,
  TsTypeParamDecl,
  TsTypeParamInstantiation,
  TsTypePredicate,
  TsTypeQuery,
  TsTypeRef,
  TsUnionType,
  UnaryExpr,
  UpdateExpr,
  VarDecl,
  VarDeclarator,
  WhileStmt,
  WithStmt,
  YieldExpr,
}

/** Array literal. */
export class ArrayLit extends Node {
  kind!: NodeKind.ArrayLit;
  parent!: Node;
  elems!: Array<ExprOrSpread | undefined>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.elems.length);
    let i = 0;
    for (const child of this.elems) {
      if (child != null) {
        children[i++] = child;
      }
    }
    return children;
  }
}

export class ArrayPat extends Node {
  kind!: NodeKind.ArrayPat;
  parent!: Node;
  elems!: Array<Pat | undefined>;
  /** Only in an ambient context */
  optional!: boolean;
  type_ann!: TsTypeAnn | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.elems.length + (this.type_ann == null ? 0 : 1));
    let i = 0;
    for (const child of this.elems) {
      if (child != null) {
        children[i++] = child;
      }
    }
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    return children;
  }
}

export class ArrowExpr extends Node {
  kind!: NodeKind.ArrowExpr;
  parent!: Node;
  params!: Array<Pat>;
  body!: BlockStmtOrExpr;
  is_async!: boolean;
  is_generator!: boolean;
  type_params!: TsTypeParamDecl | undefined;
  return_type!: TsTypeAnn | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.params.length + (this.type_params == null ? 0 : 1) + (this.return_type == null ? 0 : 1));
    let i = 0;
    for (const child of this.params) {
      children[i++] = child;
    }
    children[i++] = this.body;
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    if (this.return_type != null) {
      children[i++] = this.return_type;
    }
    return children;
  }
}

export class AssignExpr extends Node {
  kind!: NodeKind.AssignExpr;
  parent!: Node;
  op!: AssignOp;
  left!: PatOrExpr;
  right!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.left;
    children[i++] = this.right;
    return children;
  }
}

export class AssignPat extends Node {
  kind!: NodeKind.AssignPat;
  parent!: Node;
  left!: Pat;
  right!: Expr;
  type_ann!: TsTypeAnn | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(2 + (this.type_ann == null ? 0 : 1));
    let i = 0;
    children[i++] = this.left;
    children[i++] = this.right;
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    return children;
  }
}

/** `{key}` or `{key = value}` */
export class AssignPatProp extends Node {
  kind!: NodeKind.AssignPatProp;
  parent!: ObjectPat;
  key!: Ident;
  value!: Expr | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.value == null ? 0 : 1));
    let i = 0;
    children[i++] = this.key;
    if (this.value != null) {
      children[i++] = this.value;
    }
    return children;
  }
}

export class AssignProp extends Node {
  kind!: NodeKind.AssignProp;
  parent!: ObjectLit;
  key!: Ident;
  value!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.key;
    children[i++] = this.value;
    return children;
  }
}

export class AwaitExpr extends Node {
  kind!: NodeKind.AwaitExpr;
  parent!: Node;
  arg!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.arg;
    return children;
  }
}

export class BigInt extends Node {
  kind!: NodeKind.BigInt;
  parent!: Node;
  value!: BigIntValue;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class BinExpr extends Node {
  kind!: NodeKind.BinExpr;
  parent!: Node;
  op!: BinaryOp;
  left!: Expr;
  right!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.left;
    children[i++] = this.right;
    return children;
  }
}

/** Identifer used as a pattern. */
export class BindingIdent extends Node {
  kind!: NodeKind.BindingIdent;
  parent!: Node;
  id!: Ident;
  type_ann!: TsTypeAnn | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.type_ann == null ? 0 : 1));
    let i = 0;
    children[i++] = this.id;
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    return children;
  }
}

/** Use when only block statements are allowed. */
export class BlockStmt extends Node {
  kind!: NodeKind.BlockStmt;
  parent!: Node;
  stmts!: Array<Stmt>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.stmts.length);
    let i = 0;
    for (const child of this.stmts) {
      children[i++] = child;
    }
    return children;
  }
}

export class Bool extends Node {
  kind!: NodeKind.Bool;
  parent!: Node;
  value!: boolean;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class BreakStmt extends Node {
  kind!: NodeKind.BreakStmt;
  parent!: Node;
  label!: Ident | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array((this.label == null ? 0 : 1));
    let i = 0;
    if (this.label != null) {
      children[i++] = this.label;
    }
    return children;
  }
}

export class CallExpr extends Node {
  kind!: NodeKind.CallExpr;
  parent!: Node;
  callee!: ExprOrSuper;
  args!: Array<ExprOrSpread>;
  type_args!: TsTypeParamInstantiation | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.args.length + (this.type_args == null ? 0 : 1));
    let i = 0;
    children[i++] = this.callee;
    for (const child of this.args) {
      children[i++] = child;
    }
    if (this.type_args != null) {
      children[i++] = this.type_args;
    }
    return children;
  }
}

export class CatchClause extends Node {
  kind!: NodeKind.CatchClause;
  parent!: TryStmt;
  /**
   * es2019
   * 
   * The param is null if the catch binding is omitted. E.g., try { foo() }
   * catch { bar() }
   */
  param!: Pat | undefined;
  body!: BlockStmt;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.param == null ? 0 : 1));
    let i = 0;
    if (this.param != null) {
      children[i++] = this.param;
    }
    children[i++] = this.body;
    return children;
  }
}

export class Class extends Node {
  kind!: NodeKind.Class;
  parent!: ClassDecl
    | ClassExpr;
  decorators!: Array<Decorator>;
  body!: Array<ClassMember>;
  super_class!: Expr | undefined;
  is_abstract!: boolean;
  type_params!: TsTypeParamDecl | undefined;
  super_type_params!: TsTypeParamInstantiation | undefined;
  /** Typescript extension. */
  implements!: Array<TsExprWithTypeArgs>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.decorators.length + this.body.length + (this.super_class == null ? 0 : 1) + (this.type_params == null ? 0 : 1) + (this.super_type_params == null ? 0 : 1) + this.implements.length);
    let i = 0;
    for (const child of this.decorators) {
      children[i++] = child;
    }
    for (const child of this.body) {
      children[i++] = child;
    }
    if (this.super_class != null) {
      children[i++] = this.super_class;
    }
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    if (this.super_type_params != null) {
      children[i++] = this.super_type_params;
    }
    for (const child of this.implements) {
      children[i++] = child;
    }
    return children;
  }
}

export class ClassDecl extends Node {
  kind!: NodeKind.ClassDecl;
  parent!: Node;
  ident!: Ident;
  declare!: boolean;
  class!: Class;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.ident;
    children[i++] = this.class;
    return children;
  }
}

/** Class expression. */
export class ClassExpr extends Node {
  kind!: NodeKind.ClassExpr;
  parent!: Node;
  ident!: Ident | undefined;
  class!: Class;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.ident == null ? 0 : 1));
    let i = 0;
    if (this.ident != null) {
      children[i++] = this.ident;
    }
    children[i++] = this.class;
    return children;
  }
}

export class ClassMethod extends Node {
  kind!: NodeKind.ClassMethod;
  parent!: Class;
  key!: PropName;
  function!: Function;
  method_kind!: MethodKind;
  is_static!: boolean;
  /** Typescript extension. */
  accessibility!: Accessibility | undefined;
  /** Typescript extension. */
  is_abstract!: boolean;
  is_optional!: boolean;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.key;
    children[i++] = this.function;
    return children;
  }
}

export class ClassProp extends Node {
  kind!: NodeKind.ClassProp;
  parent!: Class;
  key!: Expr;
  value!: Expr | undefined;
  type_ann!: TsTypeAnn | undefined;
  is_static!: boolean;
  decorators!: Array<Decorator>;
  computed!: boolean;
  /** Typescript extension. */
  accessibility!: Accessibility | undefined;
  /** Typescript extension. */
  is_abstract!: boolean;
  is_optional!: boolean;
  readonly!: boolean;
  declare!: boolean;
  definite!: boolean;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.value == null ? 0 : 1) + (this.type_ann == null ? 0 : 1) + this.decorators.length);
    let i = 0;
    children[i++] = this.key;
    if (this.value != null) {
      children[i++] = this.value;
    }
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    for (const child of this.decorators) {
      children[i++] = child;
    }
    return children;
  }
}

export class ComputedPropName extends Node {
  kind!: NodeKind.ComputedPropName;
  parent!: Node;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class CondExpr extends Node {
  kind!: NodeKind.CondExpr;
  parent!: Node;
  test!: Expr;
  cons!: Expr;
  alt!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(3);
    let i = 0;
    children[i++] = this.test;
    children[i++] = this.cons;
    children[i++] = this.alt;
    return children;
  }
}

export class Constructor extends Node {
  kind!: NodeKind.Constructor;
  parent!: Class;
  key!: PropName;
  params!: Array<ParamOrTsParamProp>;
  body!: BlockStmt | undefined;
  accessibility!: Accessibility | undefined;
  is_optional!: boolean;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.params.length + (this.body == null ? 0 : 1));
    let i = 0;
    children[i++] = this.key;
    for (const child of this.params) {
      children[i++] = child;
    }
    if (this.body != null) {
      children[i++] = this.body;
    }
    return children;
  }
}

export class ContinueStmt extends Node {
  kind!: NodeKind.ContinueStmt;
  parent!: Node;
  label!: Ident | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array((this.label == null ? 0 : 1));
    let i = 0;
    if (this.label != null) {
      children[i++] = this.label;
    }
    return children;
  }
}

export class DebuggerStmt extends Node {
  kind!: NodeKind.DebuggerStmt;
  parent!: Node;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class Decorator extends Node {
  kind!: NodeKind.Decorator;
  parent!: Node;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class DoWhileStmt extends Node {
  kind!: NodeKind.DoWhileStmt;
  parent!: Node;
  test!: Expr;
  body!: Stmt;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.test;
    children[i++] = this.body;
    return children;
  }
}

export class EmptyStmt extends Node {
  kind!: NodeKind.EmptyStmt;
  parent!: Node;

  getChildren(): Node[] {
    return new Array(0);
  }
}

/** `export * from 'mod'` */
export class ExportAll extends Node {
  kind!: NodeKind.ExportAll;
  parent!: Module
    | TsModuleBlock;
  src!: Str;
  asserts!: ObjectLit | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.asserts == null ? 0 : 1));
    let i = 0;
    children[i++] = this.src;
    if (this.asserts != null) {
      children[i++] = this.asserts;
    }
    return children;
  }
}

export class ExportDecl extends Node {
  kind!: NodeKind.ExportDecl;
  parent!: Module
    | TsModuleBlock;
  decl!: Decl;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.decl;
    return children;
  }
}

export class ExportDefaultDecl extends Node {
  kind!: NodeKind.ExportDefaultDecl;
  parent!: Module
    | TsModuleBlock;
  decl!: DefaultDecl;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.decl;
    return children;
  }
}

export class ExportDefaultExpr extends Node {
  kind!: NodeKind.ExportDefaultExpr;
  parent!: Module
    | TsModuleBlock;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class ExportDefaultSpecifier extends Node {
  kind!: NodeKind.ExportDefaultSpecifier;
  parent!: NamedExport;
  exported!: Ident;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.exported;
    return children;
  }
}

export class ExportNamedSpecifier extends Node {
  kind!: NodeKind.ExportNamedSpecifier;
  parent!: NamedExport;
  /** `foo` in `export { foo as bar }` */
  orig!: Ident;
  /** `Some(bar)` in `export { foo as bar }` */
  exported!: Ident | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.exported == null ? 0 : 1));
    let i = 0;
    children[i++] = this.orig;
    if (this.exported != null) {
      children[i++] = this.exported;
    }
    return children;
  }
}

/** `export * as foo from 'src';` */
export class ExportNamespaceSpecifier extends Node {
  kind!: NodeKind.ExportNamespaceSpecifier;
  parent!: NamedExport;
  name!: Ident;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.name;
    return children;
  }
}

export class ExprOrSpread extends Node {
  kind!: NodeKind.ExprOrSpread;
  parent!: ArrayLit
    | CallExpr
    | NewExpr;
  spread!: Span | undefined;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class ExprStmt extends Node {
  kind!: NodeKind.ExprStmt;
  parent!: Node;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class FnDecl extends Node {
  kind!: NodeKind.FnDecl;
  parent!: Node;
  ident!: Ident;
  declare!: boolean;
  function!: Function;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.ident;
    children[i++] = this.function;
    return children;
  }
}

/** Function expression. */
export class FnExpr extends Node {
  kind!: NodeKind.FnExpr;
  parent!: Node;
  ident!: Ident | undefined;
  function!: Function;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.ident == null ? 0 : 1));
    let i = 0;
    if (this.ident != null) {
      children[i++] = this.ident;
    }
    children[i++] = this.function;
    return children;
  }
}

export class ForInStmt extends Node {
  kind!: NodeKind.ForInStmt;
  parent!: Node;
  left!: VarDeclOrPat;
  right!: Expr;
  body!: Stmt;

  getChildren(): Node[] {
    const children: Node[] = new Array(3);
    let i = 0;
    children[i++] = this.left;
    children[i++] = this.right;
    children[i++] = this.body;
    return children;
  }
}

export class ForOfStmt extends Node {
  kind!: NodeKind.ForOfStmt;
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

  getChildren(): Node[] {
    const children: Node[] = new Array(3);
    let i = 0;
    children[i++] = this.left;
    children[i++] = this.right;
    children[i++] = this.body;
    return children;
  }
}

export class ForStmt extends Node {
  kind!: NodeKind.ForStmt;
  parent!: Node;
  init!: VarDeclOrExpr | undefined;
  test!: Expr | undefined;
  update!: Expr | undefined;
  body!: Stmt;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.init == null ? 0 : 1) + (this.test == null ? 0 : 1) + (this.update == null ? 0 : 1));
    let i = 0;
    if (this.init != null) {
      children[i++] = this.init;
    }
    if (this.test != null) {
      children[i++] = this.test;
    }
    if (this.update != null) {
      children[i++] = this.update;
    }
    children[i++] = this.body;
    return children;
  }
}

/** Common parts of function and method. */
export class Function extends Node {
  kind!: NodeKind.Function;
  parent!: Node;
  params!: Array<Param>;
  decorators!: Array<Decorator>;
  body!: BlockStmt | undefined;
  /** if it's a generator. */
  is_generator!: boolean;
  /** if it's an async function. */
  is_async!: boolean;
  type_params!: TsTypeParamDecl | undefined;
  return_type!: TsTypeAnn | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.params.length + this.decorators.length + (this.body == null ? 0 : 1) + (this.type_params == null ? 0 : 1) + (this.return_type == null ? 0 : 1));
    let i = 0;
    for (const child of this.params) {
      children[i++] = child;
    }
    for (const child of this.decorators) {
      children[i++] = child;
    }
    if (this.body != null) {
      children[i++] = this.body;
    }
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    if (this.return_type != null) {
      children[i++] = this.return_type;
    }
    return children;
  }
}

export class GetterProp extends Node {
  kind!: NodeKind.GetterProp;
  parent!: ObjectLit;
  key!: PropName;
  type_ann!: TsTypeAnn | undefined;
  body!: BlockStmt | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.type_ann == null ? 0 : 1) + (this.body == null ? 0 : 1));
    let i = 0;
    children[i++] = this.key;
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    if (this.body != null) {
      children[i++] = this.body;
    }
    return children;
  }
}

/** Ident with span. */
export class Ident extends Node {
  kind!: NodeKind.Ident;
  parent!: Node;
  sym!: JsWord;
  /** TypeScript only. Used in case of an optional parameter. */
  optional!: boolean;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class IfStmt extends Node {
  kind!: NodeKind.IfStmt;
  parent!: Node;
  test!: Expr;
  cons!: Stmt;
  alt!: Stmt | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(2 + (this.alt == null ? 0 : 1));
    let i = 0;
    children[i++] = this.test;
    children[i++] = this.cons;
    if (this.alt != null) {
      children[i++] = this.alt;
    }
    return children;
  }
}

export class ImportDecl extends Node {
  kind!: NodeKind.ImportDecl;
  parent!: Module
    | TsModuleBlock;
  specifiers!: Array<ImportSpecifier>;
  src!: Str;
  type_only!: boolean;
  asserts!: ObjectLit | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.specifiers.length + (this.asserts == null ? 0 : 1));
    let i = 0;
    for (const child of this.specifiers) {
      children[i++] = child;
    }
    children[i++] = this.src;
    if (this.asserts != null) {
      children[i++] = this.asserts;
    }
    return children;
  }
}

/** e.g. `import foo from 'mod.js'` */
export class ImportDefaultSpecifier extends Node {
  kind!: NodeKind.ImportDefaultSpecifier;
  parent!: ImportDecl;
  local!: Ident;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.local;
    return children;
  }
}

/**
 * e.g. local = foo, imported = None `import { foo } from 'mod.js'`
 * e.g. local = bar, imported = Some(foo) for `import { foo as bar } from
 * 'mod.js'`
 */
export class ImportNamedSpecifier extends Node {
  kind!: NodeKind.ImportNamedSpecifier;
  parent!: ImportDecl;
  local!: Ident;
  imported!: Ident | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.imported == null ? 0 : 1));
    let i = 0;
    children[i++] = this.local;
    if (this.imported != null) {
      children[i++] = this.imported;
    }
    return children;
  }
}

/** e.g. `import * as foo from 'mod.js'`. */
export class ImportStarAsSpecifier extends Node {
  kind!: NodeKind.ImportStarAsSpecifier;
  parent!: ImportDecl;
  local!: Ident;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.local;
    return children;
  }
}

/** Represents a invalid node. */
export class Invalid extends Node {
  kind!: NodeKind.Invalid;
  parent!: Node;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class JSXAttr extends Node {
  kind!: NodeKind.JSXAttr;
  parent!: JSXOpeningElement;
  name!: JSXAttrName;
  /** Babel uses Expr instead of JSXAttrValue */
  value!: JSXAttrValue | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.value == null ? 0 : 1));
    let i = 0;
    children[i++] = this.name;
    if (this.value != null) {
      children[i++] = this.value;
    }
    return children;
  }
}

export class JSXClosingElement extends Node {
  kind!: NodeKind.JSXClosingElement;
  parent!: JSXElement;
  name!: JSXElementName;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.name;
    return children;
  }
}

export class JSXClosingFragment extends Node {
  kind!: NodeKind.JSXClosingFragment;
  parent!: JSXFragment;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class JSXElement extends Node {
  kind!: NodeKind.JSXElement;
  parent!: Node;
  opening!: JSXOpeningElement;
  children!: Array<JSXElementChild>;
  closing!: JSXClosingElement | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.children.length + (this.closing == null ? 0 : 1));
    let i = 0;
    children[i++] = this.opening;
    for (const child of this.children) {
      children[i++] = child;
    }
    if (this.closing != null) {
      children[i++] = this.closing;
    }
    return children;
  }
}

export class JSXEmptyExpr extends Node {
  kind!: NodeKind.JSXEmptyExpr;
  parent!: Node;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class JSXExprContainer extends Node {
  kind!: NodeKind.JSXExprContainer;
  parent!: JSXAttr
    | JSXElement
    | JSXFragment;
  expr!: JSXExpr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class JSXFragment extends Node {
  kind!: NodeKind.JSXFragment;
  parent!: Node;
  opening!: JSXOpeningFragment;
  children!: Array<JSXElementChild>;
  closing!: JSXClosingFragment;

  getChildren(): Node[] {
    const children: Node[] = new Array(2 + this.children.length);
    let i = 0;
    children[i++] = this.opening;
    for (const child of this.children) {
      children[i++] = child;
    }
    children[i++] = this.closing;
    return children;
  }
}

export class JSXMemberExpr extends Node {
  kind!: NodeKind.JSXMemberExpr;
  parent!: Node;
  obj!: JSXObject;
  prop!: Ident;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.obj;
    children[i++] = this.prop;
    return children;
  }
}

/** XML-based namespace syntax: */
export class JSXNamespacedName extends Node {
  kind!: NodeKind.JSXNamespacedName;
  parent!: Node;
  ns!: Ident;
  name!: Ident;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.ns;
    children[i++] = this.name;
    return children;
  }
}

export class JSXOpeningElement extends Node {
  kind!: NodeKind.JSXOpeningElement;
  parent!: JSXElement;
  name!: JSXElementName;
  attrs!: Array<JSXAttrOrSpread>;
  self_closing!: boolean;
  /**
   * Note: This field's name is different from one from babel because it is
   * misleading
   */
  type_args!: TsTypeParamInstantiation | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.attrs.length + (this.type_args == null ? 0 : 1));
    let i = 0;
    children[i++] = this.name;
    for (const child of this.attrs) {
      children[i++] = child;
    }
    if (this.type_args != null) {
      children[i++] = this.type_args;
    }
    return children;
  }
}

export class JSXOpeningFragment extends Node {
  kind!: NodeKind.JSXOpeningFragment;
  parent!: JSXFragment;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class JSXSpreadChild extends Node {
  kind!: NodeKind.JSXSpreadChild;
  parent!: JSXElement
    | JSXFragment;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class JSXText extends Node {
  kind!: NodeKind.JSXText;
  parent!: Node;
  value!: JsWord;
  raw!: JsWord;

  getChildren(): Node[] {
    return new Array(0);
  }
}

/** `{key: value}` */
export class KeyValuePatProp extends Node {
  kind!: NodeKind.KeyValuePatProp;
  parent!: ObjectPat;
  key!: PropName;
  value!: Pat;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.key;
    children[i++] = this.value;
    return children;
  }
}

export class KeyValueProp extends Node {
  kind!: NodeKind.KeyValueProp;
  parent!: ObjectLit;
  key!: PropName;
  value!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.key;
    children[i++] = this.value;
    return children;
  }
}

export class LabeledStmt extends Node {
  kind!: NodeKind.LabeledStmt;
  parent!: Node;
  label!: Ident;
  body!: Stmt;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.label;
    children[i++] = this.body;
    return children;
  }
}

export class MemberExpr extends Node {
  kind!: NodeKind.MemberExpr;
  parent!: Node;
  obj!: ExprOrSuper;
  prop!: Expr;
  computed!: boolean;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.obj;
    children[i++] = this.prop;
    return children;
  }
}

export class MetaPropExpr extends Node {
  kind!: NodeKind.MetaPropExpr;
  parent!: Node;
  meta!: Ident;
  prop!: Ident;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.meta;
    children[i++] = this.prop;
    return children;
  }
}

export class MethodProp extends Node {
  kind!: NodeKind.MethodProp;
  parent!: ObjectLit;
  key!: PropName;
  function!: Function;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.key;
    children[i++] = this.function;
    return children;
  }
}

export class Module extends Node {
  kind!: NodeKind.Module;
  body!: Array<ModuleItem>;
  shebang!: JsWord | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.body.length);
    let i = 0;
    for (const child of this.body) {
      children[i++] = child;
    }
    return children;
  }
}

/**
 * `export { foo } from 'mod'`
 * `export { foo as bar } from 'mod'`
 */
export class NamedExport extends Node {
  kind!: NodeKind.NamedExport;
  parent!: Module
    | TsModuleBlock;
  specifiers!: Array<ExportSpecifier>;
  src!: Str | undefined;
  type_only!: boolean;
  asserts!: ObjectLit | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.specifiers.length + (this.src == null ? 0 : 1) + (this.asserts == null ? 0 : 1));
    let i = 0;
    for (const child of this.specifiers) {
      children[i++] = child;
    }
    if (this.src != null) {
      children[i++] = this.src;
    }
    if (this.asserts != null) {
      children[i++] = this.asserts;
    }
    return children;
  }
}

export class NewExpr extends Node {
  kind!: NodeKind.NewExpr;
  parent!: Node;
  callee!: Expr;
  args!: Array<ExprOrSpread> | undefined;
  type_args!: TsTypeParamInstantiation | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.args == null ? 0 : this.args.length) + (this.type_args == null ? 0 : 1));
    let i = 0;
    children[i++] = this.callee;
    if (this.args != null) {
      for (const child of this.args) {
        children[i++] = child;
      }
    }
    if (this.type_args != null) {
      children[i++] = this.type_args;
    }
    return children;
  }
}

export class Null extends Node {
  kind!: NodeKind.Null;
  parent!: Node;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class Number extends Node {
  kind!: NodeKind.Number;
  parent!: Node;
  /**
   * **Note**: This should not be `NaN`. Use [crate::Ident] to represent NaN.
   * 
   * If you store `NaN` in this field, a hash map will behave strangely.
   */
  value!: number;

  getChildren(): Node[] {
    return new Array(0);
  }
}

/** Object literal. */
export class ObjectLit extends Node {
  kind!: NodeKind.ObjectLit;
  parent!: Node;
  props!: Array<PropOrSpread>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.props.length);
    let i = 0;
    for (const child of this.props) {
      children[i++] = child;
    }
    return children;
  }
}

export class ObjectPat extends Node {
  kind!: NodeKind.ObjectPat;
  parent!: Node;
  props!: Array<ObjectPatProp>;
  /** Only in an ambient context */
  optional!: boolean;
  type_ann!: TsTypeAnn | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.props.length + (this.type_ann == null ? 0 : 1));
    let i = 0;
    for (const child of this.props) {
      children[i++] = child;
    }
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    return children;
  }
}

export class OptChainExpr extends Node {
  kind!: NodeKind.OptChainExpr;
  parent!: Node;
  question_dot_token!: Span;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class Param extends Node {
  kind!: NodeKind.Param;
  parent!: Constructor
    | Function;
  decorators!: Array<Decorator>;
  pat!: Pat;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.decorators.length);
    let i = 0;
    for (const child of this.decorators) {
      children[i++] = child;
    }
    children[i++] = this.pat;
    return children;
  }
}

export class ParenExpr extends Node {
  kind!: NodeKind.ParenExpr;
  parent!: Node;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class PrivateMethod extends Node {
  kind!: NodeKind.PrivateMethod;
  parent!: Class;
  key!: PrivateName;
  function!: Function;
  method_kind!: MethodKind;
  is_static!: boolean;
  /** Typescript extension. */
  accessibility!: Accessibility | undefined;
  /** Typescript extension. */
  is_abstract!: boolean;
  is_optional!: boolean;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.key;
    children[i++] = this.function;
    return children;
  }
}

export class PrivateName extends Node {
  kind!: NodeKind.PrivateName;
  parent!: Node;
  id!: Ident;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.id;
    return children;
  }
}

export class PrivateProp extends Node {
  kind!: NodeKind.PrivateProp;
  parent!: Class;
  key!: PrivateName;
  value!: Expr | undefined;
  type_ann!: TsTypeAnn | undefined;
  is_static!: boolean;
  decorators!: Array<Decorator>;
  computed!: boolean;
  /** Typescript extension. */
  accessibility!: Accessibility | undefined;
  /** Typescript extension. */
  is_abstract!: boolean;
  is_optional!: boolean;
  readonly!: boolean;
  definite!: boolean;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.value == null ? 0 : 1) + (this.type_ann == null ? 0 : 1) + this.decorators.length);
    let i = 0;
    children[i++] = this.key;
    if (this.value != null) {
      children[i++] = this.value;
    }
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    for (const child of this.decorators) {
      children[i++] = child;
    }
    return children;
  }
}

export class Regex extends Node {
  kind!: NodeKind.Regex;
  parent!: Node;
  exp!: JsWord;
  flags!: JsWord;

  getChildren(): Node[] {
    return new Array(0);
  }
}

/** EsTree `RestElement` */
export class RestPat extends Node {
  kind!: NodeKind.RestPat;
  parent!: Node;
  dot3_token!: Span;
  arg!: Pat;
  type_ann!: TsTypeAnn | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.type_ann == null ? 0 : 1));
    let i = 0;
    children[i++] = this.arg;
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    return children;
  }
}

export class ReturnStmt extends Node {
  kind!: NodeKind.ReturnStmt;
  parent!: Node;
  arg!: Expr | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array((this.arg == null ? 0 : 1));
    let i = 0;
    if (this.arg != null) {
      children[i++] = this.arg;
    }
    return children;
  }
}

export class Script extends Node {
  kind!: NodeKind.Script;
  body!: Array<Stmt>;
  shebang!: JsWord | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.body.length);
    let i = 0;
    for (const child of this.body) {
      children[i++] = child;
    }
    return children;
  }
}

export class SeqExpr extends Node {
  kind!: NodeKind.SeqExpr;
  parent!: Node;
  exprs!: Array<Expr>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.exprs.length);
    let i = 0;
    for (const child of this.exprs) {
      children[i++] = child;
    }
    return children;
  }
}

export class SetterProp extends Node {
  kind!: NodeKind.SetterProp;
  parent!: ObjectLit;
  key!: PropName;
  param!: Pat;
  body!: BlockStmt | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(2 + (this.body == null ? 0 : 1));
    let i = 0;
    children[i++] = this.key;
    children[i++] = this.param;
    if (this.body != null) {
      children[i++] = this.body;
    }
    return children;
  }
}

export class SpreadElement extends Node {
  kind!: NodeKind.SpreadElement;
  parent!: JSXOpeningElement
    | ObjectLit;
  dot3_token!: Span;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class Str extends Node {
  kind!: NodeKind.Str;
  parent!: Node;
  value!: JsWord;
  /** This includes line escape. */
  has_escape!: boolean;
  str_kind!: StrKind;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class Super extends Node {
  kind!: NodeKind.Super;
  parent!: CallExpr
    | MemberExpr;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class SwitchCase extends Node {
  kind!: NodeKind.SwitchCase;
  parent!: SwitchStmt;
  /** None for `default:` */
  test!: Expr | undefined;
  cons!: Array<Stmt>;

  getChildren(): Node[] {
    const children: Node[] = new Array((this.test == null ? 0 : 1) + this.cons.length);
    let i = 0;
    if (this.test != null) {
      children[i++] = this.test;
    }
    for (const child of this.cons) {
      children[i++] = child;
    }
    return children;
  }
}

export class SwitchStmt extends Node {
  kind!: NodeKind.SwitchStmt;
  parent!: Node;
  discriminant!: Expr;
  cases!: Array<SwitchCase>;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.cases.length);
    let i = 0;
    children[i++] = this.discriminant;
    for (const child of this.cases) {
      children[i++] = child;
    }
    return children;
  }
}

export class TaggedTpl extends Node {
  kind!: NodeKind.TaggedTpl;
  parent!: Node;
  tag!: Expr;
  exprs!: Array<Expr>;
  quasis!: Array<TplElement>;
  type_params!: TsTypeParamInstantiation | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.exprs.length + this.quasis.length + (this.type_params == null ? 0 : 1));
    let i = 0;
    children[i++] = this.tag;
    for (const child of this.exprs) {
      children[i++] = child;
    }
    for (const child of this.quasis) {
      children[i++] = child;
    }
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    return children;
  }
}

export class ThisExpr extends Node {
  kind!: NodeKind.ThisExpr;
  parent!: Node;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class ThrowStmt extends Node {
  kind!: NodeKind.ThrowStmt;
  parent!: Node;
  arg!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.arg;
    return children;
  }
}

export class Tpl extends Node {
  kind!: NodeKind.Tpl;
  parent!: Node;
  exprs!: Array<Expr>;
  quasis!: Array<TplElement>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.exprs.length + this.quasis.length);
    let i = 0;
    for (const child of this.exprs) {
      children[i++] = child;
    }
    for (const child of this.quasis) {
      children[i++] = child;
    }
    return children;
  }
}

export class TplElement extends Node {
  kind!: NodeKind.TplElement;
  parent!: TaggedTpl
    | Tpl
    | TsTplLitType;
  tail!: boolean;
  cooked!: Str | undefined;
  raw!: Str;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.cooked == null ? 0 : 1));
    let i = 0;
    if (this.cooked != null) {
      children[i++] = this.cooked;
    }
    children[i++] = this.raw;
    return children;
  }
}

export class TryStmt extends Node {
  kind!: NodeKind.TryStmt;
  parent!: Node;
  block!: BlockStmt;
  handler!: CatchClause | undefined;
  finalizer!: BlockStmt | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.handler == null ? 0 : 1) + (this.finalizer == null ? 0 : 1));
    let i = 0;
    children[i++] = this.block;
    if (this.handler != null) {
      children[i++] = this.handler;
    }
    if (this.finalizer != null) {
      children[i++] = this.finalizer;
    }
    return children;
  }
}

export class TsArrayType extends Node {
  kind!: NodeKind.TsArrayType;
  parent!: Node;
  elem_type!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.elem_type;
    return children;
  }
}

export class TsAsExpr extends Node {
  kind!: NodeKind.TsAsExpr;
  parent!: Node;
  expr!: Expr;
  type_ann!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.expr;
    children[i++] = this.type_ann;
    return children;
  }
}

export class TsCallSignatureDecl extends Node {
  kind!: NodeKind.TsCallSignatureDecl;
  parent!: TsInterfaceBody
    | TsTypeLit;
  params!: Array<TsFnParam>;
  type_ann!: TsTypeAnn | undefined;
  type_params!: TsTypeParamDecl | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.params.length + (this.type_ann == null ? 0 : 1) + (this.type_params == null ? 0 : 1));
    let i = 0;
    for (const child of this.params) {
      children[i++] = child;
    }
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    return children;
  }
}

export class TsConditionalType extends Node {
  kind!: NodeKind.TsConditionalType;
  parent!: Node;
  check_type!: TsType;
  extends_type!: TsType;
  true_type!: TsType;
  false_type!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(4);
    let i = 0;
    children[i++] = this.check_type;
    children[i++] = this.extends_type;
    children[i++] = this.true_type;
    children[i++] = this.false_type;
    return children;
  }
}

export class TsConstAssertion extends Node {
  kind!: NodeKind.TsConstAssertion;
  parent!: Node;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class TsConstructSignatureDecl extends Node {
  kind!: NodeKind.TsConstructSignatureDecl;
  parent!: TsInterfaceBody
    | TsTypeLit;
  params!: Array<TsFnParam>;
  type_ann!: TsTypeAnn | undefined;
  type_params!: TsTypeParamDecl | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.params.length + (this.type_ann == null ? 0 : 1) + (this.type_params == null ? 0 : 1));
    let i = 0;
    for (const child of this.params) {
      children[i++] = child;
    }
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    return children;
  }
}

export class TsConstructorType extends Node {
  kind!: NodeKind.TsConstructorType;
  parent!: Node;
  params!: Array<TsFnParam>;
  type_params!: TsTypeParamDecl | undefined;
  type_ann!: TsTypeAnn;
  is_abstract!: boolean;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.params.length + (this.type_params == null ? 0 : 1));
    let i = 0;
    for (const child of this.params) {
      children[i++] = child;
    }
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    children[i++] = this.type_ann;
    return children;
  }
}

export class TsEnumDecl extends Node {
  kind!: NodeKind.TsEnumDecl;
  parent!: Node;
  declare!: boolean;
  is_const!: boolean;
  id!: Ident;
  members!: Array<TsEnumMember>;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.members.length);
    let i = 0;
    children[i++] = this.id;
    for (const child of this.members) {
      children[i++] = child;
    }
    return children;
  }
}

export class TsEnumMember extends Node {
  kind!: NodeKind.TsEnumMember;
  parent!: TsEnumDecl;
  id!: TsEnumMemberId;
  init!: Expr | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.init == null ? 0 : 1));
    let i = 0;
    children[i++] = this.id;
    if (this.init != null) {
      children[i++] = this.init;
    }
    return children;
  }
}

/**
 * TypeScript's own parser uses ExportAssignment for both `export default` and
 * `export =`. But for @babel/parser, `export default` is an ExportDefaultDecl,
 * so a TsExportAssignment is always `export =`.
 */
export class TsExportAssignment extends Node {
  kind!: NodeKind.TsExportAssignment;
  parent!: Module
    | TsModuleBlock;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class TsExprWithTypeArgs extends Node {
  kind!: NodeKind.TsExprWithTypeArgs;
  parent!: Class
    | TsInterfaceDecl;
  expr!: TsEntityName;
  type_args!: TsTypeParamInstantiation | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.type_args == null ? 0 : 1));
    let i = 0;
    children[i++] = this.expr;
    if (this.type_args != null) {
      children[i++] = this.type_args;
    }
    return children;
  }
}

export class TsExternalModuleRef extends Node {
  kind!: NodeKind.TsExternalModuleRef;
  parent!: TsImportEqualsDecl;
  expr!: Str;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class TsFnType extends Node {
  kind!: NodeKind.TsFnType;
  parent!: Node;
  params!: Array<TsFnParam>;
  type_params!: TsTypeParamDecl | undefined;
  type_ann!: TsTypeAnn;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.params.length + (this.type_params == null ? 0 : 1));
    let i = 0;
    for (const child of this.params) {
      children[i++] = child;
    }
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    children[i++] = this.type_ann;
    return children;
  }
}

export class TsImportEqualsDecl extends Node {
  kind!: NodeKind.TsImportEqualsDecl;
  parent!: Module
    | TsModuleBlock;
  declare!: boolean;
  is_export!: boolean;
  id!: Ident;
  module_ref!: TsModuleRef;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.id;
    children[i++] = this.module_ref;
    return children;
  }
}

export class TsImportType extends Node {
  kind!: NodeKind.TsImportType;
  parent!: Node;
  arg!: Str;
  qualifier!: TsEntityName | undefined;
  type_args!: TsTypeParamInstantiation | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.qualifier == null ? 0 : 1) + (this.type_args == null ? 0 : 1));
    let i = 0;
    children[i++] = this.arg;
    if (this.qualifier != null) {
      children[i++] = this.qualifier;
    }
    if (this.type_args != null) {
      children[i++] = this.type_args;
    }
    return children;
  }
}

export class TsIndexSignature extends Node {
  kind!: NodeKind.TsIndexSignature;
  parent!: Class
    | TsInterfaceBody
    | TsTypeLit;
  params!: Array<TsFnParam>;
  type_ann!: TsTypeAnn | undefined;
  readonly!: boolean;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.params.length + (this.type_ann == null ? 0 : 1));
    let i = 0;
    for (const child of this.params) {
      children[i++] = child;
    }
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    return children;
  }
}

export class TsIndexedAccessType extends Node {
  kind!: NodeKind.TsIndexedAccessType;
  parent!: Node;
  readonly!: boolean;
  obj_type!: TsType;
  index_type!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.obj_type;
    children[i++] = this.index_type;
    return children;
  }
}

export class TsInferType extends Node {
  kind!: NodeKind.TsInferType;
  parent!: Node;
  type_param!: TsTypeParam;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.type_param;
    return children;
  }
}

export class TsInterfaceBody extends Node {
  kind!: NodeKind.TsInterfaceBody;
  parent!: TsInterfaceDecl;
  body!: Array<TsTypeElement>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.body.length);
    let i = 0;
    for (const child of this.body) {
      children[i++] = child;
    }
    return children;
  }
}

export class TsInterfaceDecl extends Node {
  kind!: NodeKind.TsInterfaceDecl;
  parent!: Node;
  id!: Ident;
  declare!: boolean;
  type_params!: TsTypeParamDecl | undefined;
  extends!: Array<TsExprWithTypeArgs>;
  body!: TsInterfaceBody;

  getChildren(): Node[] {
    const children: Node[] = new Array(2 + (this.type_params == null ? 0 : 1) + this.extends.length);
    let i = 0;
    children[i++] = this.id;
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    for (const child of this.extends) {
      children[i++] = child;
    }
    children[i++] = this.body;
    return children;
  }
}

export class TsIntersectionType extends Node {
  kind!: NodeKind.TsIntersectionType;
  parent!: Node;
  types!: Array<TsType>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.types.length);
    let i = 0;
    for (const child of this.types) {
      children[i++] = child;
    }
    return children;
  }
}

export class TsKeywordType extends Node {
  kind!: NodeKind.TsKeywordType;
  parent!: Node;
  keyword_kind!: TsKeywordTypeKind;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class TsLitType extends Node {
  kind!: NodeKind.TsLitType;
  parent!: Node;
  lit!: TsLit;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.lit;
    return children;
  }
}

export class TsMappedType extends Node {
  kind!: NodeKind.TsMappedType;
  parent!: Node;
  readonly!: TruePlusMinus | undefined;
  type_param!: TsTypeParam;
  name_type!: TsType | undefined;
  optional!: TruePlusMinus | undefined;
  type_ann!: TsType | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.name_type == null ? 0 : 1) + (this.type_ann == null ? 0 : 1));
    let i = 0;
    children[i++] = this.type_param;
    if (this.name_type != null) {
      children[i++] = this.name_type;
    }
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    return children;
  }
}

export class TsMethodSignature extends Node {
  kind!: NodeKind.TsMethodSignature;
  parent!: TsInterfaceBody
    | TsTypeLit;
  readonly!: boolean;
  key!: Expr;
  computed!: boolean;
  optional!: boolean;
  params!: Array<TsFnParam>;
  type_ann!: TsTypeAnn | undefined;
  type_params!: TsTypeParamDecl | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.params.length + (this.type_ann == null ? 0 : 1) + (this.type_params == null ? 0 : 1));
    let i = 0;
    children[i++] = this.key;
    for (const child of this.params) {
      children[i++] = child;
    }
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    return children;
  }
}

export class TsModuleBlock extends Node {
  kind!: NodeKind.TsModuleBlock;
  parent!: TsModuleDecl
    | TsNamespaceDecl;
  body!: Array<ModuleItem>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.body.length);
    let i = 0;
    for (const child of this.body) {
      children[i++] = child;
    }
    return children;
  }
}

export class TsModuleDecl extends Node {
  kind!: NodeKind.TsModuleDecl;
  parent!: Node;
  declare!: boolean;
  /** In TypeScript, this is only available through`node.flags`. */
  global!: boolean;
  id!: TsModuleName;
  body!: TsNamespaceBody | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.body == null ? 0 : 1));
    let i = 0;
    children[i++] = this.id;
    if (this.body != null) {
      children[i++] = this.body;
    }
    return children;
  }
}

export class TsNamespaceDecl extends Node {
  kind!: NodeKind.TsNamespaceDecl;
  parent!: TsModuleDecl
    | TsNamespaceDecl;
  declare!: boolean;
  /** In TypeScript, this is only available through`node.flags`. */
  global!: boolean;
  id!: Ident;
  body!: TsNamespaceBody;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.id;
    children[i++] = this.body;
    return children;
  }
}

export class TsNamespaceExportDecl extends Node {
  kind!: NodeKind.TsNamespaceExportDecl;
  parent!: Module
    | TsModuleBlock;
  id!: Ident;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.id;
    return children;
  }
}

export class TsNonNullExpr extends Node {
  kind!: NodeKind.TsNonNullExpr;
  parent!: Node;
  expr!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr;
    return children;
  }
}

export class TsOptionalType extends Node {
  kind!: NodeKind.TsOptionalType;
  parent!: Node;
  type_ann!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.type_ann;
    return children;
  }
}

export class TsParamProp extends Node {
  kind!: NodeKind.TsParamProp;
  parent!: Constructor;
  decorators!: Array<Decorator>;
  /** At least one of `accessibility` or `readonly` must be set. */
  accessibility!: Accessibility | undefined;
  readonly!: boolean;
  param!: TsParamPropParam;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + this.decorators.length);
    let i = 0;
    for (const child of this.decorators) {
      children[i++] = child;
    }
    children[i++] = this.param;
    return children;
  }
}

export class TsParenthesizedType extends Node {
  kind!: NodeKind.TsParenthesizedType;
  parent!: Node;
  type_ann!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.type_ann;
    return children;
  }
}

export class TsPropertySignature extends Node {
  kind!: NodeKind.TsPropertySignature;
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

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.init == null ? 0 : 1) + this.params.length + (this.type_ann == null ? 0 : 1) + (this.type_params == null ? 0 : 1));
    let i = 0;
    children[i++] = this.key;
    if (this.init != null) {
      children[i++] = this.init;
    }
    for (const child of this.params) {
      children[i++] = child;
    }
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    return children;
  }
}

export class TsQualifiedName extends Node {
  kind!: NodeKind.TsQualifiedName;
  parent!: Node;
  left!: TsEntityName;
  right!: Ident;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.left;
    children[i++] = this.right;
    return children;
  }
}

export class TsRestType extends Node {
  kind!: NodeKind.TsRestType;
  parent!: Node;
  type_ann!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.type_ann;
    return children;
  }
}

export class TsThisType extends Node {
  kind!: NodeKind.TsThisType;
  parent!: Node;

  getChildren(): Node[] {
    return new Array(0);
  }
}

export class TsTplLitType extends Node {
  kind!: NodeKind.TsTplLitType;
  parent!: TsLitType;
  types!: Array<TsType>;
  quasis!: Array<TplElement>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.types.length + this.quasis.length);
    let i = 0;
    for (const child of this.types) {
      children[i++] = child;
    }
    for (const child of this.quasis) {
      children[i++] = child;
    }
    return children;
  }
}

export class TsTupleElement extends Node {
  kind!: NodeKind.TsTupleElement;
  parent!: TsTupleType;
  /** `Ident` or `RestPat { arg: Ident }` */
  label!: Pat | undefined;
  ty!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.label == null ? 0 : 1));
    let i = 0;
    if (this.label != null) {
      children[i++] = this.label;
    }
    children[i++] = this.ty;
    return children;
  }
}

export class TsTupleType extends Node {
  kind!: NodeKind.TsTupleType;
  parent!: Node;
  elem_types!: Array<TsTupleElement>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.elem_types.length);
    let i = 0;
    for (const child of this.elem_types) {
      children[i++] = child;
    }
    return children;
  }
}

export class TsTypeAliasDecl extends Node {
  kind!: NodeKind.TsTypeAliasDecl;
  parent!: Node;
  declare!: boolean;
  id!: Ident;
  type_params!: TsTypeParamDecl | undefined;
  type_ann!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(2 + (this.type_params == null ? 0 : 1));
    let i = 0;
    children[i++] = this.id;
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    children[i++] = this.type_ann;
    return children;
  }
}

export class TsTypeAnn extends Node {
  kind!: NodeKind.TsTypeAnn;
  parent!: Node;
  type_ann!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.type_ann;
    return children;
  }
}

export class TsTypeAssertion extends Node {
  kind!: NodeKind.TsTypeAssertion;
  parent!: Node;
  expr!: Expr;
  type_ann!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.expr;
    children[i++] = this.type_ann;
    return children;
  }
}

export class TsTypeLit extends Node {
  kind!: NodeKind.TsTypeLit;
  parent!: Node;
  members!: Array<TsTypeElement>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.members.length);
    let i = 0;
    for (const child of this.members) {
      children[i++] = child;
    }
    return children;
  }
}

export class TsTypeOperator extends Node {
  kind!: NodeKind.TsTypeOperator;
  parent!: Node;
  op!: TsTypeOperatorOp;
  type_ann!: TsType;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.type_ann;
    return children;
  }
}

export class TsTypeParam extends Node {
  kind!: NodeKind.TsTypeParam;
  parent!: TsInferType
    | TsMappedType
    | TsTypeParamDecl;
  name!: Ident;
  constraint!: TsType | undefined;
  default!: TsType | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.constraint == null ? 0 : 1) + (this.default == null ? 0 : 1));
    let i = 0;
    children[i++] = this.name;
    if (this.constraint != null) {
      children[i++] = this.constraint;
    }
    if (this.default != null) {
      children[i++] = this.default;
    }
    return children;
  }
}

export class TsTypeParamDecl extends Node {
  kind!: NodeKind.TsTypeParamDecl;
  parent!: Node;
  params!: Array<TsTypeParam>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.params.length);
    let i = 0;
    for (const child of this.params) {
      children[i++] = child;
    }
    return children;
  }
}

export class TsTypeParamInstantiation extends Node {
  kind!: NodeKind.TsTypeParamInstantiation;
  parent!: Node;
  params!: Array<TsType>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.params.length);
    let i = 0;
    for (const child of this.params) {
      children[i++] = child;
    }
    return children;
  }
}

export class TsTypePredicate extends Node {
  kind!: NodeKind.TsTypePredicate;
  parent!: Node;
  asserts!: boolean;
  param_name!: TsThisTypeOrIdent;
  type_ann!: TsTypeAnn | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.type_ann == null ? 0 : 1));
    let i = 0;
    children[i++] = this.param_name;
    if (this.type_ann != null) {
      children[i++] = this.type_ann;
    }
    return children;
  }
}

/** `typeof` operator */
export class TsTypeQuery extends Node {
  kind!: NodeKind.TsTypeQuery;
  parent!: Node;
  expr_name!: TsTypeQueryExpr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.expr_name;
    return children;
  }
}

export class TsTypeRef extends Node {
  kind!: NodeKind.TsTypeRef;
  parent!: Node;
  type_name!: TsEntityName;
  type_params!: TsTypeParamInstantiation | undefined;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.type_params == null ? 0 : 1));
    let i = 0;
    children[i++] = this.type_name;
    if (this.type_params != null) {
      children[i++] = this.type_params;
    }
    return children;
  }
}

export class TsUnionType extends Node {
  kind!: NodeKind.TsUnionType;
  parent!: Node;
  types!: Array<TsType>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.types.length);
    let i = 0;
    for (const child of this.types) {
      children[i++] = child;
    }
    return children;
  }
}

export class UnaryExpr extends Node {
  kind!: NodeKind.UnaryExpr;
  parent!: Node;
  op!: UnaryOp;
  arg!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.arg;
    return children;
  }
}

export class UpdateExpr extends Node {
  kind!: NodeKind.UpdateExpr;
  parent!: Node;
  op!: UpdateOp;
  prefix!: boolean;
  arg!: Expr;

  getChildren(): Node[] {
    const children: Node[] = new Array(1);
    let i = 0;
    children[i++] = this.arg;
    return children;
  }
}

export class VarDecl extends Node {
  kind!: NodeKind.VarDecl;
  parent!: Node;
  decl_kind!: VarDeclKind;
  declare!: boolean;
  decls!: Array<VarDeclarator>;

  getChildren(): Node[] {
    const children: Node[] = new Array(this.decls.length);
    let i = 0;
    for (const child of this.decls) {
      children[i++] = child;
    }
    return children;
  }
}

export class VarDeclarator extends Node {
  kind!: NodeKind.VarDeclarator;
  parent!: VarDecl;
  name!: Pat;
  /** Initialization expression. */
  init!: Expr | undefined;
  /** Typescript only */
  definite!: boolean;

  getChildren(): Node[] {
    const children: Node[] = new Array(1 + (this.init == null ? 0 : 1));
    let i = 0;
    children[i++] = this.name;
    if (this.init != null) {
      children[i++] = this.init;
    }
    return children;
  }
}

export class WhileStmt extends Node {
  kind!: NodeKind.WhileStmt;
  parent!: Node;
  test!: Expr;
  body!: Stmt;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.test;
    children[i++] = this.body;
    return children;
  }
}

export class WithStmt extends Node {
  kind!: NodeKind.WithStmt;
  parent!: Node;
  obj!: Expr;
  body!: Stmt;

  getChildren(): Node[] {
    const children: Node[] = new Array(2);
    let i = 0;
    children[i++] = this.obj;
    children[i++] = this.body;
    return children;
  }
}

export class YieldExpr extends Node {
  kind!: NodeKind.YieldExpr;
  parent!: Node;
  arg!: Expr | undefined;
  delegate!: boolean;

  getChildren(): Node[] {
    const children: Node[] = new Array((this.arg == null ? 0 : 1));
    let i = 0;
    if (this.arg != null) {
      children[i++] = this.arg;
    }
    return children;
  }
}
