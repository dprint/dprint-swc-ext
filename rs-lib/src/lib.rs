// This code is code generated.
// Run `deno run -A generation/main.ts` from the root directory to regenerate it.
use std::mem::{self, MaybeUninit};
use swc_common::{Span, Spanned};
use swc_ecma_ast::{VarDeclKind, TsTypeOperatorOp, TsKeywordTypeKind, BinaryOp, AssignOp, UpdateOp, Accessibility, MethodKind, UnaryOp, TruePlusMinus};

pub fn get_reference_view(module: &swc_ecma_ast::Module) -> Module {
  let module = unsafe { mem::transmute::<&swc_ecma_ast::Module, &'static swc_ecma_ast::Module>(&module) };
  get_view_for_module(module)
}

pub trait NodeTrait {
  fn parent(&self) -> Option<&Node>;
  fn children(&self) -> Vec<Node>;
}

impl From<& Box<Expr>> for Node {
  fn from(boxed_node: &Box<Expr>) -> Node {
    (&**boxed_node).into()}
}

impl From<& Box<JSXElement>> for Node {
  fn from(boxed_node: &Box<JSXElement>) -> Node {
    (&**boxed_node).into()}
}

impl From<& Box<JSXMemberExpr>> for Node {
  fn from(boxed_node: &Box<JSXMemberExpr>) -> Node {
    (&**boxed_node).into()}
}

impl From<& Box<Pat>> for Node {
  fn from(boxed_node: &Box<Pat>) -> Node {
    (&**boxed_node).into()}
}

impl From<& Box<Prop>> for Node {
  fn from(boxed_node: &Box<Prop>) -> Node {
    (&**boxed_node).into()}
}

impl From<& Box<Stmt>> for Node {
  fn from(boxed_node: &Box<Stmt>) -> Node {
    (&**boxed_node).into()}
}

impl From<& Box<TsNamespaceBody>> for Node {
  fn from(boxed_node: &Box<TsNamespaceBody>) -> Node {
    (&**boxed_node).into()}
}

impl From<& Box<TsQualifiedName>> for Node {
  fn from(boxed_node: &Box<TsQualifiedName>) -> Node {
    (&**boxed_node).into()}
}

impl From<& Box<TsType>> for Node {
  fn from(boxed_node: &Box<TsType>) -> Node {
    (&**boxed_node).into()}
}

#[derive(Clone)]
pub enum Node {
  SwitchCase(&'static SwitchCase),
  ThrowStmt(&'static ThrowStmt),
  JSXClosingFragment(&'static JSXClosingFragment),
  BigInt(&'static BigInt),
  ExportDefaultSpecifier(&'static ExportDefaultSpecifier),
  TsTypeParam(&'static TsTypeParam),
  WithStmt(&'static WithStmt),
  Regex(&'static Regex),
  TsMethodSignature(&'static TsMethodSignature),
  UpdateExpr(&'static UpdateExpr),
  SetterProp(&'static SetterProp),
  TaggedTpl(&'static TaggedTpl),
  ExportAll(&'static ExportAll),
  TsModuleBlock(&'static TsModuleBlock),
  SwitchStmt(&'static SwitchStmt),
  TsEnumMember(&'static TsEnumMember),
  TsIndexedAccessType(&'static TsIndexedAccessType),
  TsRestType(&'static TsRestType),
  ExprStmt(&'static ExprStmt),
  TsOptionalType(&'static TsOptionalType),
  Tpl(&'static Tpl),
  Invalid(&'static Invalid),
  ComputedPropName(&'static ComputedPropName),
  TsFnType(&'static TsFnType),
  BlockStmt(&'static BlockStmt),
  TsTypeAliasDecl(&'static TsTypeAliasDecl),
  MemberExpr(&'static MemberExpr),
  Function(&'static Function),
  ImportDecl(&'static ImportDecl),
  TsTypePredicate(&'static TsTypePredicate),
  YieldExpr(&'static YieldExpr),
  KeyValueProp(&'static KeyValueProp),
  Param(&'static Param),
  JSXFragment(&'static JSXFragment),
  ImportDefaultSpecifier(&'static ImportDefaultSpecifier),
  Number(&'static Number),
  JSXAttr(&'static JSXAttr),
  ParenExpr(&'static ParenExpr),
  Super(&'static Super),
  TsConstructorType(&'static TsConstructorType),
  Class(&'static Class),
  RestPat(&'static RestPat),
  TsNamespaceExportDecl(&'static TsNamespaceExportDecl),
  JSXOpeningFragment(&'static JSXOpeningFragment),
  NewExpr(&'static NewExpr),
  FnExpr(&'static FnExpr),
  IfStmt(&'static IfStmt),
  TsParenthesizedType(&'static TsParenthesizedType),
  AssignPatProp(&'static AssignPatProp),
  TsImportType(&'static TsImportType),
  Bool(&'static Bool),
  TsImportEqualsDecl(&'static TsImportEqualsDecl),
  AssignProp(&'static AssignProp),
  TsInterfaceDecl(&'static TsInterfaceDecl),
  JSXEmptyExpr(&'static JSXEmptyExpr),
  TsQualifiedName(&'static TsQualifiedName),
  ExportDecl(&'static ExportDecl),
  CatchClause(&'static CatchClause),
  LabeledStmt(&'static LabeledStmt),
  ContinueStmt(&'static ContinueStmt),
  TsConstructSignatureDecl(&'static TsConstructSignatureDecl),
  TsEnumDecl(&'static TsEnumDecl),
  OptChainExpr(&'static OptChainExpr),
  TsNamespaceDecl(&'static TsNamespaceDecl),
  SeqExpr(&'static SeqExpr),
  TsExternalModuleRef(&'static TsExternalModuleRef),
  TsTypeParamInstantiation(&'static TsTypeParamInstantiation),
  ReturnStmt(&'static ReturnStmt),
  TsTplLitType(&'static TsTplLitType),
  ExportDefaultExpr(&'static ExportDefaultExpr),
  TsCallSignatureDecl(&'static TsCallSignatureDecl),
  AwaitExpr(&'static AwaitExpr),
  ClassMethod(&'static ClassMethod),
  TsParamProp(&'static TsParamProp),
  ClassProp(&'static ClassProp),
  TsTypeAnn(&'static TsTypeAnn),
  ForStmt(&'static ForStmt),
  ObjectPat(&'static ObjectPat),
  TsTypeQuery(&'static TsTypeQuery),
  ThisExpr(&'static ThisExpr),
  DebuggerStmt(&'static DebuggerStmt),
  TsTypeParamDecl(&'static TsTypeParamDecl),
  TsTypeAssertion(&'static TsTypeAssertion),
  TplElement(&'static TplElement),
  TsKeywordType(&'static TsKeywordType),
  JSXSpreadChild(&'static JSXSpreadChild),
  TsIntersectionType(&'static TsIntersectionType),
  MetaPropExpr(&'static MetaPropExpr),
  ExprOrSpread(&'static ExprOrSpread),
  TsArrayType(&'static TsArrayType),
  TsTypeRef(&'static TsTypeRef),
  TsThisType(&'static TsThisType),
  TryStmt(&'static TryStmt),
  CallExpr(&'static CallExpr),
  TsMappedType(&'static TsMappedType),
  JSXExprContainer(&'static JSXExprContainer),
  PrivateProp(&'static PrivateProp),
  TsExportAssignment(&'static TsExportAssignment),
  TsInterfaceBody(&'static TsInterfaceBody),
  TsTupleElement(&'static TsTupleElement),
  VarDeclarator(&'static VarDeclarator),
  JSXMemberExpr(&'static JSXMemberExpr),
  TsConstAssertion(&'static TsConstAssertion),
  ExportNamespaceSpecifier(&'static ExportNamespaceSpecifier),
  ObjectLit(&'static ObjectLit),
  Module(&'static Module),
  TsIndexSignature(&'static TsIndexSignature),
  TsTypeCastExpr(&'static TsTypeCastExpr),
  TsTupleType(&'static TsTupleType),
  Null(&'static Null),
  TsTypeOperator(&'static TsTypeOperator),
  JSXClosingElement(&'static JSXClosingElement),
  BinExpr(&'static BinExpr),
  UnaryExpr(&'static UnaryExpr),
  TsPropertySignature(&'static TsPropertySignature),
  Constructor(&'static Constructor),
  FnDecl(&'static FnDecl),
  TsNonNullExpr(&'static TsNonNullExpr),
  ClassExpr(&'static ClassExpr),
  ForInStmt(&'static ForInStmt),
  EmptyStmt(&'static EmptyStmt),
  WhileStmt(&'static WhileStmt),
  Str(&'static Str),
  TsExprWithTypeArgs(&'static TsExprWithTypeArgs),
  AssignPat(&'static AssignPat),
  ExportNamedSpecifier(&'static ExportNamedSpecifier),
  TsConditionalType(&'static TsConditionalType),
  TsTypeLit(&'static TsTypeLit),
  BreakStmt(&'static BreakStmt),
  ImportStarAsSpecifier(&'static ImportStarAsSpecifier),
  TsInferType(&'static TsInferType),
  PrivateMethod(&'static PrivateMethod),
  ForOfStmt(&'static ForOfStmt),
  TsUnionType(&'static TsUnionType),
  TsModuleDecl(&'static TsModuleDecl),
  GetterProp(&'static GetterProp),
  CondExpr(&'static CondExpr),
  ImportNamedSpecifier(&'static ImportNamedSpecifier),
  NamedExport(&'static NamedExport),
  JSXElement(&'static JSXElement),
  ClassDecl(&'static ClassDecl),
  ArrayPat(&'static ArrayPat),
  DoWhileStmt(&'static DoWhileStmt),
  JSXText(&'static JSXText),
  VarDecl(&'static VarDecl),
  PrivateName(&'static PrivateName),
  JSXNamespacedName(&'static JSXNamespacedName),
  JSXOpeningElement(&'static JSXOpeningElement),
  SpreadElement(&'static SpreadElement),
  ExportDefaultDecl(&'static ExportDefaultDecl),
  ArrowExpr(&'static ArrowExpr),
  TsAsExpr(&'static TsAsExpr),
  KeyValuePatProp(&'static KeyValuePatProp),
  TsLitType(&'static TsLitType),
  AssignExpr(&'static AssignExpr),
  ArrayLit(&'static ArrayLit),
  Decorator(&'static Decorator),
  Ident(&'static Ident),
  MethodProp(&'static MethodProp),
}

impl Spanned for Node {
  fn span(&self) -> Span {
    match self {
      Node::SwitchCase(node) => node.span(),
      Node::ThrowStmt(node) => node.span(),
      Node::JSXClosingFragment(node) => node.span(),
      Node::BigInt(node) => node.span(),
      Node::ExportDefaultSpecifier(node) => node.span(),
      Node::TsTypeParam(node) => node.span(),
      Node::WithStmt(node) => node.span(),
      Node::Regex(node) => node.span(),
      Node::TsMethodSignature(node) => node.span(),
      Node::UpdateExpr(node) => node.span(),
      Node::SetterProp(node) => node.span(),
      Node::TaggedTpl(node) => node.span(),
      Node::ExportAll(node) => node.span(),
      Node::TsModuleBlock(node) => node.span(),
      Node::SwitchStmt(node) => node.span(),
      Node::TsEnumMember(node) => node.span(),
      Node::TsIndexedAccessType(node) => node.span(),
      Node::TsRestType(node) => node.span(),
      Node::ExprStmt(node) => node.span(),
      Node::TsOptionalType(node) => node.span(),
      Node::Tpl(node) => node.span(),
      Node::Invalid(node) => node.span(),
      Node::ComputedPropName(node) => node.span(),
      Node::TsFnType(node) => node.span(),
      Node::BlockStmt(node) => node.span(),
      Node::TsTypeAliasDecl(node) => node.span(),
      Node::MemberExpr(node) => node.span(),
      Node::Function(node) => node.span(),
      Node::ImportDecl(node) => node.span(),
      Node::TsTypePredicate(node) => node.span(),
      Node::YieldExpr(node) => node.span(),
      Node::KeyValueProp(node) => node.span(),
      Node::Param(node) => node.span(),
      Node::JSXFragment(node) => node.span(),
      Node::ImportDefaultSpecifier(node) => node.span(),
      Node::Number(node) => node.span(),
      Node::JSXAttr(node) => node.span(),
      Node::ParenExpr(node) => node.span(),
      Node::Super(node) => node.span(),
      Node::TsConstructorType(node) => node.span(),
      Node::Class(node) => node.span(),
      Node::RestPat(node) => node.span(),
      Node::TsNamespaceExportDecl(node) => node.span(),
      Node::JSXOpeningFragment(node) => node.span(),
      Node::NewExpr(node) => node.span(),
      Node::FnExpr(node) => node.span(),
      Node::IfStmt(node) => node.span(),
      Node::TsParenthesizedType(node) => node.span(),
      Node::AssignPatProp(node) => node.span(),
      Node::TsImportType(node) => node.span(),
      Node::Bool(node) => node.span(),
      Node::TsImportEqualsDecl(node) => node.span(),
      Node::AssignProp(node) => node.span(),
      Node::TsInterfaceDecl(node) => node.span(),
      Node::JSXEmptyExpr(node) => node.span(),
      Node::TsQualifiedName(node) => node.span(),
      Node::ExportDecl(node) => node.span(),
      Node::CatchClause(node) => node.span(),
      Node::LabeledStmt(node) => node.span(),
      Node::ContinueStmt(node) => node.span(),
      Node::TsConstructSignatureDecl(node) => node.span(),
      Node::TsEnumDecl(node) => node.span(),
      Node::OptChainExpr(node) => node.span(),
      Node::TsNamespaceDecl(node) => node.span(),
      Node::SeqExpr(node) => node.span(),
      Node::TsExternalModuleRef(node) => node.span(),
      Node::TsTypeParamInstantiation(node) => node.span(),
      Node::ReturnStmt(node) => node.span(),
      Node::TsTplLitType(node) => node.span(),
      Node::ExportDefaultExpr(node) => node.span(),
      Node::TsCallSignatureDecl(node) => node.span(),
      Node::AwaitExpr(node) => node.span(),
      Node::ClassMethod(node) => node.span(),
      Node::TsParamProp(node) => node.span(),
      Node::ClassProp(node) => node.span(),
      Node::TsTypeAnn(node) => node.span(),
      Node::ForStmt(node) => node.span(),
      Node::ObjectPat(node) => node.span(),
      Node::TsTypeQuery(node) => node.span(),
      Node::ThisExpr(node) => node.span(),
      Node::DebuggerStmt(node) => node.span(),
      Node::TsTypeParamDecl(node) => node.span(),
      Node::TsTypeAssertion(node) => node.span(),
      Node::TplElement(node) => node.span(),
      Node::TsKeywordType(node) => node.span(),
      Node::JSXSpreadChild(node) => node.span(),
      Node::TsIntersectionType(node) => node.span(),
      Node::MetaPropExpr(node) => node.span(),
      Node::ExprOrSpread(node) => node.span(),
      Node::TsArrayType(node) => node.span(),
      Node::TsTypeRef(node) => node.span(),
      Node::TsThisType(node) => node.span(),
      Node::TryStmt(node) => node.span(),
      Node::CallExpr(node) => node.span(),
      Node::TsMappedType(node) => node.span(),
      Node::JSXExprContainer(node) => node.span(),
      Node::PrivateProp(node) => node.span(),
      Node::TsExportAssignment(node) => node.span(),
      Node::TsInterfaceBody(node) => node.span(),
      Node::TsTupleElement(node) => node.span(),
      Node::VarDeclarator(node) => node.span(),
      Node::JSXMemberExpr(node) => node.span(),
      Node::TsConstAssertion(node) => node.span(),
      Node::ExportNamespaceSpecifier(node) => node.span(),
      Node::ObjectLit(node) => node.span(),
      Node::Module(node) => node.span(),
      Node::TsIndexSignature(node) => node.span(),
      Node::TsTypeCastExpr(node) => node.span(),
      Node::TsTupleType(node) => node.span(),
      Node::Null(node) => node.span(),
      Node::TsTypeOperator(node) => node.span(),
      Node::JSXClosingElement(node) => node.span(),
      Node::BinExpr(node) => node.span(),
      Node::UnaryExpr(node) => node.span(),
      Node::TsPropertySignature(node) => node.span(),
      Node::Constructor(node) => node.span(),
      Node::FnDecl(node) => node.span(),
      Node::TsNonNullExpr(node) => node.span(),
      Node::ClassExpr(node) => node.span(),
      Node::ForInStmt(node) => node.span(),
      Node::EmptyStmt(node) => node.span(),
      Node::WhileStmt(node) => node.span(),
      Node::Str(node) => node.span(),
      Node::TsExprWithTypeArgs(node) => node.span(),
      Node::AssignPat(node) => node.span(),
      Node::ExportNamedSpecifier(node) => node.span(),
      Node::TsConditionalType(node) => node.span(),
      Node::TsTypeLit(node) => node.span(),
      Node::BreakStmt(node) => node.span(),
      Node::ImportStarAsSpecifier(node) => node.span(),
      Node::TsInferType(node) => node.span(),
      Node::PrivateMethod(node) => node.span(),
      Node::ForOfStmt(node) => node.span(),
      Node::TsUnionType(node) => node.span(),
      Node::TsModuleDecl(node) => node.span(),
      Node::GetterProp(node) => node.span(),
      Node::CondExpr(node) => node.span(),
      Node::ImportNamedSpecifier(node) => node.span(),
      Node::NamedExport(node) => node.span(),
      Node::JSXElement(node) => node.span(),
      Node::ClassDecl(node) => node.span(),
      Node::ArrayPat(node) => node.span(),
      Node::DoWhileStmt(node) => node.span(),
      Node::JSXText(node) => node.span(),
      Node::VarDecl(node) => node.span(),
      Node::PrivateName(node) => node.span(),
      Node::JSXNamespacedName(node) => node.span(),
      Node::JSXOpeningElement(node) => node.span(),
      Node::SpreadElement(node) => node.span(),
      Node::ExportDefaultDecl(node) => node.span(),
      Node::ArrowExpr(node) => node.span(),
      Node::TsAsExpr(node) => node.span(),
      Node::KeyValuePatProp(node) => node.span(),
      Node::TsLitType(node) => node.span(),
      Node::AssignExpr(node) => node.span(),
      Node::ArrayLit(node) => node.span(),
      Node::Decorator(node) => node.span(),
      Node::Ident(node) => node.span(),
      Node::MethodProp(node) => node.span(),
    }
  }
}
impl NodeTrait for Node {
  fn parent(&self) -> Option<&Node> {
    match self {
      Node::SwitchCase(node) => node.parent(),
      Node::ThrowStmt(node) => node.parent(),
      Node::JSXClosingFragment(node) => node.parent(),
      Node::BigInt(node) => node.parent(),
      Node::ExportDefaultSpecifier(node) => node.parent(),
      Node::TsTypeParam(node) => node.parent(),
      Node::WithStmt(node) => node.parent(),
      Node::Regex(node) => node.parent(),
      Node::TsMethodSignature(node) => node.parent(),
      Node::UpdateExpr(node) => node.parent(),
      Node::SetterProp(node) => node.parent(),
      Node::TaggedTpl(node) => node.parent(),
      Node::ExportAll(node) => node.parent(),
      Node::TsModuleBlock(node) => node.parent(),
      Node::SwitchStmt(node) => node.parent(),
      Node::TsEnumMember(node) => node.parent(),
      Node::TsIndexedAccessType(node) => node.parent(),
      Node::TsRestType(node) => node.parent(),
      Node::ExprStmt(node) => node.parent(),
      Node::TsOptionalType(node) => node.parent(),
      Node::Tpl(node) => node.parent(),
      Node::Invalid(node) => node.parent(),
      Node::ComputedPropName(node) => node.parent(),
      Node::TsFnType(node) => node.parent(),
      Node::BlockStmt(node) => node.parent(),
      Node::TsTypeAliasDecl(node) => node.parent(),
      Node::MemberExpr(node) => node.parent(),
      Node::Function(node) => node.parent(),
      Node::ImportDecl(node) => node.parent(),
      Node::TsTypePredicate(node) => node.parent(),
      Node::YieldExpr(node) => node.parent(),
      Node::KeyValueProp(node) => node.parent(),
      Node::Param(node) => node.parent(),
      Node::JSXFragment(node) => node.parent(),
      Node::ImportDefaultSpecifier(node) => node.parent(),
      Node::Number(node) => node.parent(),
      Node::JSXAttr(node) => node.parent(),
      Node::ParenExpr(node) => node.parent(),
      Node::Super(node) => node.parent(),
      Node::TsConstructorType(node) => node.parent(),
      Node::Class(node) => node.parent(),
      Node::RestPat(node) => node.parent(),
      Node::TsNamespaceExportDecl(node) => node.parent(),
      Node::JSXOpeningFragment(node) => node.parent(),
      Node::NewExpr(node) => node.parent(),
      Node::FnExpr(node) => node.parent(),
      Node::IfStmt(node) => node.parent(),
      Node::TsParenthesizedType(node) => node.parent(),
      Node::AssignPatProp(node) => node.parent(),
      Node::TsImportType(node) => node.parent(),
      Node::Bool(node) => node.parent(),
      Node::TsImportEqualsDecl(node) => node.parent(),
      Node::AssignProp(node) => node.parent(),
      Node::TsInterfaceDecl(node) => node.parent(),
      Node::JSXEmptyExpr(node) => node.parent(),
      Node::TsQualifiedName(node) => node.parent(),
      Node::ExportDecl(node) => node.parent(),
      Node::CatchClause(node) => node.parent(),
      Node::LabeledStmt(node) => node.parent(),
      Node::ContinueStmt(node) => node.parent(),
      Node::TsConstructSignatureDecl(node) => node.parent(),
      Node::TsEnumDecl(node) => node.parent(),
      Node::OptChainExpr(node) => node.parent(),
      Node::TsNamespaceDecl(node) => node.parent(),
      Node::SeqExpr(node) => node.parent(),
      Node::TsExternalModuleRef(node) => node.parent(),
      Node::TsTypeParamInstantiation(node) => node.parent(),
      Node::ReturnStmt(node) => node.parent(),
      Node::TsTplLitType(node) => node.parent(),
      Node::ExportDefaultExpr(node) => node.parent(),
      Node::TsCallSignatureDecl(node) => node.parent(),
      Node::AwaitExpr(node) => node.parent(),
      Node::ClassMethod(node) => node.parent(),
      Node::TsParamProp(node) => node.parent(),
      Node::ClassProp(node) => node.parent(),
      Node::TsTypeAnn(node) => node.parent(),
      Node::ForStmt(node) => node.parent(),
      Node::ObjectPat(node) => node.parent(),
      Node::TsTypeQuery(node) => node.parent(),
      Node::ThisExpr(node) => node.parent(),
      Node::DebuggerStmt(node) => node.parent(),
      Node::TsTypeParamDecl(node) => node.parent(),
      Node::TsTypeAssertion(node) => node.parent(),
      Node::TplElement(node) => node.parent(),
      Node::TsKeywordType(node) => node.parent(),
      Node::JSXSpreadChild(node) => node.parent(),
      Node::TsIntersectionType(node) => node.parent(),
      Node::MetaPropExpr(node) => node.parent(),
      Node::ExprOrSpread(node) => node.parent(),
      Node::TsArrayType(node) => node.parent(),
      Node::TsTypeRef(node) => node.parent(),
      Node::TsThisType(node) => node.parent(),
      Node::TryStmt(node) => node.parent(),
      Node::CallExpr(node) => node.parent(),
      Node::TsMappedType(node) => node.parent(),
      Node::JSXExprContainer(node) => node.parent(),
      Node::PrivateProp(node) => node.parent(),
      Node::TsExportAssignment(node) => node.parent(),
      Node::TsInterfaceBody(node) => node.parent(),
      Node::TsTupleElement(node) => node.parent(),
      Node::VarDeclarator(node) => node.parent(),
      Node::JSXMemberExpr(node) => node.parent(),
      Node::TsConstAssertion(node) => node.parent(),
      Node::ExportNamespaceSpecifier(node) => node.parent(),
      Node::ObjectLit(node) => node.parent(),
      Node::Module(node) => node.parent(),
      Node::TsIndexSignature(node) => node.parent(),
      Node::TsTypeCastExpr(node) => node.parent(),
      Node::TsTupleType(node) => node.parent(),
      Node::Null(node) => node.parent(),
      Node::TsTypeOperator(node) => node.parent(),
      Node::JSXClosingElement(node) => node.parent(),
      Node::BinExpr(node) => node.parent(),
      Node::UnaryExpr(node) => node.parent(),
      Node::TsPropertySignature(node) => node.parent(),
      Node::Constructor(node) => node.parent(),
      Node::FnDecl(node) => node.parent(),
      Node::TsNonNullExpr(node) => node.parent(),
      Node::ClassExpr(node) => node.parent(),
      Node::ForInStmt(node) => node.parent(),
      Node::EmptyStmt(node) => node.parent(),
      Node::WhileStmt(node) => node.parent(),
      Node::Str(node) => node.parent(),
      Node::TsExprWithTypeArgs(node) => node.parent(),
      Node::AssignPat(node) => node.parent(),
      Node::ExportNamedSpecifier(node) => node.parent(),
      Node::TsConditionalType(node) => node.parent(),
      Node::TsTypeLit(node) => node.parent(),
      Node::BreakStmt(node) => node.parent(),
      Node::ImportStarAsSpecifier(node) => node.parent(),
      Node::TsInferType(node) => node.parent(),
      Node::PrivateMethod(node) => node.parent(),
      Node::ForOfStmt(node) => node.parent(),
      Node::TsUnionType(node) => node.parent(),
      Node::TsModuleDecl(node) => node.parent(),
      Node::GetterProp(node) => node.parent(),
      Node::CondExpr(node) => node.parent(),
      Node::ImportNamedSpecifier(node) => node.parent(),
      Node::NamedExport(node) => node.parent(),
      Node::JSXElement(node) => node.parent(),
      Node::ClassDecl(node) => node.parent(),
      Node::ArrayPat(node) => node.parent(),
      Node::DoWhileStmt(node) => node.parent(),
      Node::JSXText(node) => node.parent(),
      Node::VarDecl(node) => node.parent(),
      Node::PrivateName(node) => node.parent(),
      Node::JSXNamespacedName(node) => node.parent(),
      Node::JSXOpeningElement(node) => node.parent(),
      Node::SpreadElement(node) => node.parent(),
      Node::ExportDefaultDecl(node) => node.parent(),
      Node::ArrowExpr(node) => node.parent(),
      Node::TsAsExpr(node) => node.parent(),
      Node::KeyValuePatProp(node) => node.parent(),
      Node::TsLitType(node) => node.parent(),
      Node::AssignExpr(node) => node.parent(),
      Node::ArrayLit(node) => node.parent(),
      Node::Decorator(node) => node.parent(),
      Node::Ident(node) => node.parent(),
      Node::MethodProp(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      Node::SwitchCase(node) => node.children(),
      Node::ThrowStmt(node) => node.children(),
      Node::JSXClosingFragment(node) => node.children(),
      Node::BigInt(node) => node.children(),
      Node::ExportDefaultSpecifier(node) => node.children(),
      Node::TsTypeParam(node) => node.children(),
      Node::WithStmt(node) => node.children(),
      Node::Regex(node) => node.children(),
      Node::TsMethodSignature(node) => node.children(),
      Node::UpdateExpr(node) => node.children(),
      Node::SetterProp(node) => node.children(),
      Node::TaggedTpl(node) => node.children(),
      Node::ExportAll(node) => node.children(),
      Node::TsModuleBlock(node) => node.children(),
      Node::SwitchStmt(node) => node.children(),
      Node::TsEnumMember(node) => node.children(),
      Node::TsIndexedAccessType(node) => node.children(),
      Node::TsRestType(node) => node.children(),
      Node::ExprStmt(node) => node.children(),
      Node::TsOptionalType(node) => node.children(),
      Node::Tpl(node) => node.children(),
      Node::Invalid(node) => node.children(),
      Node::ComputedPropName(node) => node.children(),
      Node::TsFnType(node) => node.children(),
      Node::BlockStmt(node) => node.children(),
      Node::TsTypeAliasDecl(node) => node.children(),
      Node::MemberExpr(node) => node.children(),
      Node::Function(node) => node.children(),
      Node::ImportDecl(node) => node.children(),
      Node::TsTypePredicate(node) => node.children(),
      Node::YieldExpr(node) => node.children(),
      Node::KeyValueProp(node) => node.children(),
      Node::Param(node) => node.children(),
      Node::JSXFragment(node) => node.children(),
      Node::ImportDefaultSpecifier(node) => node.children(),
      Node::Number(node) => node.children(),
      Node::JSXAttr(node) => node.children(),
      Node::ParenExpr(node) => node.children(),
      Node::Super(node) => node.children(),
      Node::TsConstructorType(node) => node.children(),
      Node::Class(node) => node.children(),
      Node::RestPat(node) => node.children(),
      Node::TsNamespaceExportDecl(node) => node.children(),
      Node::JSXOpeningFragment(node) => node.children(),
      Node::NewExpr(node) => node.children(),
      Node::FnExpr(node) => node.children(),
      Node::IfStmt(node) => node.children(),
      Node::TsParenthesizedType(node) => node.children(),
      Node::AssignPatProp(node) => node.children(),
      Node::TsImportType(node) => node.children(),
      Node::Bool(node) => node.children(),
      Node::TsImportEqualsDecl(node) => node.children(),
      Node::AssignProp(node) => node.children(),
      Node::TsInterfaceDecl(node) => node.children(),
      Node::JSXEmptyExpr(node) => node.children(),
      Node::TsQualifiedName(node) => node.children(),
      Node::ExportDecl(node) => node.children(),
      Node::CatchClause(node) => node.children(),
      Node::LabeledStmt(node) => node.children(),
      Node::ContinueStmt(node) => node.children(),
      Node::TsConstructSignatureDecl(node) => node.children(),
      Node::TsEnumDecl(node) => node.children(),
      Node::OptChainExpr(node) => node.children(),
      Node::TsNamespaceDecl(node) => node.children(),
      Node::SeqExpr(node) => node.children(),
      Node::TsExternalModuleRef(node) => node.children(),
      Node::TsTypeParamInstantiation(node) => node.children(),
      Node::ReturnStmt(node) => node.children(),
      Node::TsTplLitType(node) => node.children(),
      Node::ExportDefaultExpr(node) => node.children(),
      Node::TsCallSignatureDecl(node) => node.children(),
      Node::AwaitExpr(node) => node.children(),
      Node::ClassMethod(node) => node.children(),
      Node::TsParamProp(node) => node.children(),
      Node::ClassProp(node) => node.children(),
      Node::TsTypeAnn(node) => node.children(),
      Node::ForStmt(node) => node.children(),
      Node::ObjectPat(node) => node.children(),
      Node::TsTypeQuery(node) => node.children(),
      Node::ThisExpr(node) => node.children(),
      Node::DebuggerStmt(node) => node.children(),
      Node::TsTypeParamDecl(node) => node.children(),
      Node::TsTypeAssertion(node) => node.children(),
      Node::TplElement(node) => node.children(),
      Node::TsKeywordType(node) => node.children(),
      Node::JSXSpreadChild(node) => node.children(),
      Node::TsIntersectionType(node) => node.children(),
      Node::MetaPropExpr(node) => node.children(),
      Node::ExprOrSpread(node) => node.children(),
      Node::TsArrayType(node) => node.children(),
      Node::TsTypeRef(node) => node.children(),
      Node::TsThisType(node) => node.children(),
      Node::TryStmt(node) => node.children(),
      Node::CallExpr(node) => node.children(),
      Node::TsMappedType(node) => node.children(),
      Node::JSXExprContainer(node) => node.children(),
      Node::PrivateProp(node) => node.children(),
      Node::TsExportAssignment(node) => node.children(),
      Node::TsInterfaceBody(node) => node.children(),
      Node::TsTupleElement(node) => node.children(),
      Node::VarDeclarator(node) => node.children(),
      Node::JSXMemberExpr(node) => node.children(),
      Node::TsConstAssertion(node) => node.children(),
      Node::ExportNamespaceSpecifier(node) => node.children(),
      Node::ObjectLit(node) => node.children(),
      Node::Module(node) => node.children(),
      Node::TsIndexSignature(node) => node.children(),
      Node::TsTypeCastExpr(node) => node.children(),
      Node::TsTupleType(node) => node.children(),
      Node::Null(node) => node.children(),
      Node::TsTypeOperator(node) => node.children(),
      Node::JSXClosingElement(node) => node.children(),
      Node::BinExpr(node) => node.children(),
      Node::UnaryExpr(node) => node.children(),
      Node::TsPropertySignature(node) => node.children(),
      Node::Constructor(node) => node.children(),
      Node::FnDecl(node) => node.children(),
      Node::TsNonNullExpr(node) => node.children(),
      Node::ClassExpr(node) => node.children(),
      Node::ForInStmt(node) => node.children(),
      Node::EmptyStmt(node) => node.children(),
      Node::WhileStmt(node) => node.children(),
      Node::Str(node) => node.children(),
      Node::TsExprWithTypeArgs(node) => node.children(),
      Node::AssignPat(node) => node.children(),
      Node::ExportNamedSpecifier(node) => node.children(),
      Node::TsConditionalType(node) => node.children(),
      Node::TsTypeLit(node) => node.children(),
      Node::BreakStmt(node) => node.children(),
      Node::ImportStarAsSpecifier(node) => node.children(),
      Node::TsInferType(node) => node.children(),
      Node::PrivateMethod(node) => node.children(),
      Node::ForOfStmt(node) => node.children(),
      Node::TsUnionType(node) => node.children(),
      Node::TsModuleDecl(node) => node.children(),
      Node::GetterProp(node) => node.children(),
      Node::CondExpr(node) => node.children(),
      Node::ImportNamedSpecifier(node) => node.children(),
      Node::NamedExport(node) => node.children(),
      Node::JSXElement(node) => node.children(),
      Node::ClassDecl(node) => node.children(),
      Node::ArrayPat(node) => node.children(),
      Node::DoWhileStmt(node) => node.children(),
      Node::JSXText(node) => node.children(),
      Node::VarDecl(node) => node.children(),
      Node::PrivateName(node) => node.children(),
      Node::JSXNamespacedName(node) => node.children(),
      Node::JSXOpeningElement(node) => node.children(),
      Node::SpreadElement(node) => node.children(),
      Node::ExportDefaultDecl(node) => node.children(),
      Node::ArrowExpr(node) => node.children(),
      Node::TsAsExpr(node) => node.children(),
      Node::KeyValuePatProp(node) => node.children(),
      Node::TsLitType(node) => node.children(),
      Node::AssignExpr(node) => node.children(),
      Node::ArrayLit(node) => node.children(),
      Node::Decorator(node) => node.children(),
      Node::Ident(node) => node.children(),
      Node::MethodProp(node) => node.children(),
    }
  }
}

pub enum JSXAttrValue {
  Lit(Lit),
  JSXExprContainer(JSXExprContainer),
  JSXElement(Box<JSXElement>),
  JSXFragment(JSXFragment),
}

impl Spanned for JSXAttrValue {
  fn span(&self) -> Span {
    match self {
      JSXAttrValue::Lit(node) => node.span(),
      JSXAttrValue::JSXExprContainer(node) => node.span(),
      JSXAttrValue::JSXElement(node) => node.span(),
      JSXAttrValue::JSXFragment(node) => node.span(),
    }
  }
}

impl NodeTrait for JSXAttrValue {
  fn parent(&self) -> Option<&Node> {
    match self {
      JSXAttrValue::Lit(node) => node.parent(),
      JSXAttrValue::JSXExprContainer(node) => node.parent(),
      JSXAttrValue::JSXElement(node) => node.parent(),
      JSXAttrValue::JSXFragment(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      JSXAttrValue::Lit(node) => node.children(),
      JSXAttrValue::JSXExprContainer(node) => node.children(),
      JSXAttrValue::JSXElement(node) => node.children(),
      JSXAttrValue::JSXFragment(node) => node.children(),
    }
  }
}
impl From<&JSXAttrValue> for Node {
  fn from(node: &JSXAttrValue) -> Node {
    match node {
      JSXAttrValue::Lit(node) => node.into(),
      JSXAttrValue::JSXExprContainer(node) => node.into(),
      JSXAttrValue::JSXElement(node) => node.into(),
      JSXAttrValue::JSXFragment(node) => node.into(),
    }
  }
}

fn get_view_for_jsxattr_value(ref_node: &'static swc_ecma_ast::JSXAttrValue) -> JSXAttrValue {
  match ref_node {
    swc_ecma_ast::JSXAttrValue::Lit(value) => JSXAttrValue::Lit(get_view_for_lit(value)),
    swc_ecma_ast::JSXAttrValue::JSXExprContainer(value) => JSXAttrValue::JSXExprContainer(get_view_for_jsxexpr_container(value)),
    swc_ecma_ast::JSXAttrValue::JSXElement(value) => JSXAttrValue::JSXElement(Box::new(get_view_for_jsxelement(value))),
    swc_ecma_ast::JSXAttrValue::JSXFragment(value) => JSXAttrValue::JSXFragment(get_view_for_jsxfragment(value)),
  }
}

fn set_parent_for_jsxattr_value(node: &mut JSXAttrValue, parent: Node) {
  match node {
    JSXAttrValue::Lit(node) => {
      set_parent_for_lit(node, parent);
    },
    JSXAttrValue::JSXExprContainer(node) => {
      node.parent = parent;
    },
    JSXAttrValue::JSXElement(node) => {
      node.parent = parent;
    },
    JSXAttrValue::JSXFragment(node) => {
      node.parent = parent;
    },
  }
}

pub enum PropOrSpread {
  /// Spread properties, e.g., `{a: 1, ...obj, b: 2}`.
  Spread(SpreadElement),
  Prop(Box<Prop>),
}

impl Spanned for PropOrSpread {
  fn span(&self) -> Span {
    match self {
      PropOrSpread::Spread(node) => node.span(),
      PropOrSpread::Prop(node) => node.span(),
    }
  }
}

impl NodeTrait for PropOrSpread {
  fn parent(&self) -> Option<&Node> {
    match self {
      PropOrSpread::Spread(node) => node.parent(),
      PropOrSpread::Prop(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      PropOrSpread::Spread(node) => node.children(),
      PropOrSpread::Prop(node) => node.children(),
    }
  }
}
impl From<&PropOrSpread> for Node {
  fn from(node: &PropOrSpread) -> Node {
    match node {
      PropOrSpread::Spread(node) => node.into(),
      PropOrSpread::Prop(node) => node.into(),
    }
  }
}

fn get_view_for_prop_or_spread(ref_node: &'static swc_ecma_ast::PropOrSpread) -> PropOrSpread {
  match ref_node {
    swc_ecma_ast::PropOrSpread::Spread(value) => PropOrSpread::Spread(get_view_for_spread_element(value)),
    swc_ecma_ast::PropOrSpread::Prop(value) => PropOrSpread::Prop(Box::new(get_view_for_prop(value))),
  }
}

fn set_parent_for_prop_or_spread(node: &mut PropOrSpread, parent: Node) {
  match node {
    PropOrSpread::Spread(node) => {
      node.parent = parent;
    },
    PropOrSpread::Prop(node) => {
      set_parent_for_prop(node, parent);
    },
  }
}

pub enum VarDeclOrExpr {
  VarDecl(VarDecl),
  Expr(Box<Expr>),
}

impl Spanned for VarDeclOrExpr {
  fn span(&self) -> Span {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.span(),
      VarDeclOrExpr::Expr(node) => node.span(),
    }
  }
}

impl NodeTrait for VarDeclOrExpr {
  fn parent(&self) -> Option<&Node> {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.parent(),
      VarDeclOrExpr::Expr(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.children(),
      VarDeclOrExpr::Expr(node) => node.children(),
    }
  }
}
impl From<&VarDeclOrExpr> for Node {
  fn from(node: &VarDeclOrExpr) -> Node {
    match node {
      VarDeclOrExpr::VarDecl(node) => node.into(),
      VarDeclOrExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_var_decl_or_expr(ref_node: &'static swc_ecma_ast::VarDeclOrExpr) -> VarDeclOrExpr {
  match ref_node {
    swc_ecma_ast::VarDeclOrExpr::VarDecl(value) => VarDeclOrExpr::VarDecl(get_view_for_var_decl(value)),
    swc_ecma_ast::VarDeclOrExpr::Expr(value) => VarDeclOrExpr::Expr(Box::new(get_view_for_expr(value))),
  }
}

fn set_parent_for_var_decl_or_expr(node: &mut VarDeclOrExpr, parent: Node) {
  match node {
    VarDeclOrExpr::VarDecl(node) => {
      node.parent = parent;
    },
    VarDeclOrExpr::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
  }
}

pub enum TsThisTypeOrIdent {
  TsThisType(TsThisType),
  Ident(Ident),
}

impl Spanned for TsThisTypeOrIdent {
  fn span(&self) -> Span {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.span(),
      TsThisTypeOrIdent::Ident(node) => node.span(),
    }
  }
}

impl NodeTrait for TsThisTypeOrIdent {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.parent(),
      TsThisTypeOrIdent::Ident(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.children(),
      TsThisTypeOrIdent::Ident(node) => node.children(),
    }
  }
}
impl From<&TsThisTypeOrIdent> for Node {
  fn from(node: &TsThisTypeOrIdent) -> Node {
    match node {
      TsThisTypeOrIdent::TsThisType(node) => node.into(),
      TsThisTypeOrIdent::Ident(node) => node.into(),
    }
  }
}

fn get_view_for_ts_this_type_or_ident(ref_node: &'static swc_ecma_ast::TsThisTypeOrIdent) -> TsThisTypeOrIdent {
  match ref_node {
    swc_ecma_ast::TsThisTypeOrIdent::TsThisType(value) => TsThisTypeOrIdent::TsThisType(get_view_for_ts_this_type(value)),
    swc_ecma_ast::TsThisTypeOrIdent::Ident(value) => TsThisTypeOrIdent::Ident(get_view_for_ident(value)),
  }
}

fn set_parent_for_ts_this_type_or_ident(node: &mut TsThisTypeOrIdent, parent: Node) {
  match node {
    TsThisTypeOrIdent::TsThisType(node) => {
      node.parent = parent;
    },
    TsThisTypeOrIdent::Ident(node) => {
      node.parent = parent;
    },
  }
}

pub enum Prop {
  /// `a` in `{ a, }`
  Shorthand(Ident),
  /// `key: value` in `{ key: value, }`
  KeyValue(KeyValueProp),
  /// This is **invalid** for object literal.
  Assign(AssignProp),
  Getter(GetterProp),
  Setter(SetterProp),
  Method(MethodProp),
}

impl Spanned for Prop {
  fn span(&self) -> Span {
    match self {
      Prop::Shorthand(node) => node.span(),
      Prop::KeyValue(node) => node.span(),
      Prop::Assign(node) => node.span(),
      Prop::Getter(node) => node.span(),
      Prop::Setter(node) => node.span(),
      Prop::Method(node) => node.span(),
    }
  }
}

impl NodeTrait for Prop {
  fn parent(&self) -> Option<&Node> {
    match self {
      Prop::Shorthand(node) => node.parent(),
      Prop::KeyValue(node) => node.parent(),
      Prop::Assign(node) => node.parent(),
      Prop::Getter(node) => node.parent(),
      Prop::Setter(node) => node.parent(),
      Prop::Method(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      Prop::Shorthand(node) => node.children(),
      Prop::KeyValue(node) => node.children(),
      Prop::Assign(node) => node.children(),
      Prop::Getter(node) => node.children(),
      Prop::Setter(node) => node.children(),
      Prop::Method(node) => node.children(),
    }
  }
}
impl From<&Prop> for Node {
  fn from(node: &Prop) -> Node {
    match node {
      Prop::Shorthand(node) => node.into(),
      Prop::KeyValue(node) => node.into(),
      Prop::Assign(node) => node.into(),
      Prop::Getter(node) => node.into(),
      Prop::Setter(node) => node.into(),
      Prop::Method(node) => node.into(),
    }
  }
}

fn get_view_for_prop(ref_node: &'static swc_ecma_ast::Prop) -> Prop {
  match ref_node {
    swc_ecma_ast::Prop::Shorthand(value) => Prop::Shorthand(get_view_for_ident(value)),
    swc_ecma_ast::Prop::KeyValue(value) => Prop::KeyValue(get_view_for_key_value_prop(value)),
    swc_ecma_ast::Prop::Assign(value) => Prop::Assign(get_view_for_assign_prop(value)),
    swc_ecma_ast::Prop::Getter(value) => Prop::Getter(get_view_for_getter_prop(value)),
    swc_ecma_ast::Prop::Setter(value) => Prop::Setter(get_view_for_setter_prop(value)),
    swc_ecma_ast::Prop::Method(value) => Prop::Method(get_view_for_method_prop(value)),
  }
}

fn set_parent_for_prop(node: &mut Prop, parent: Node) {
  match node {
    Prop::Shorthand(node) => {
      node.parent = parent;
    },
    Prop::KeyValue(node) => {
      node.parent = parent;
    },
    Prop::Assign(node) => {
      node.parent = parent;
    },
    Prop::Getter(node) => {
      node.parent = parent;
    },
    Prop::Setter(node) => {
      node.parent = parent;
    },
    Prop::Method(node) => {
      node.parent = parent;
    },
  }
}

pub enum TsTypeQueryExpr {
  TsEntityName(TsEntityName),
  Import(TsImportType),
}

impl Spanned for TsTypeQueryExpr {
  fn span(&self) -> Span {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.span(),
      TsTypeQueryExpr::Import(node) => node.span(),
    }
  }
}

impl NodeTrait for TsTypeQueryExpr {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.parent(),
      TsTypeQueryExpr::Import(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.children(),
      TsTypeQueryExpr::Import(node) => node.children(),
    }
  }
}
impl From<&TsTypeQueryExpr> for Node {
  fn from(node: &TsTypeQueryExpr) -> Node {
    match node {
      TsTypeQueryExpr::TsEntityName(node) => node.into(),
      TsTypeQueryExpr::Import(node) => node.into(),
    }
  }
}

fn get_view_for_ts_type_query_expr(ref_node: &'static swc_ecma_ast::TsTypeQueryExpr) -> TsTypeQueryExpr {
  match ref_node {
    swc_ecma_ast::TsTypeQueryExpr::TsEntityName(value) => TsTypeQueryExpr::TsEntityName(get_view_for_ts_entity_name(value)),
    swc_ecma_ast::TsTypeQueryExpr::Import(value) => TsTypeQueryExpr::Import(get_view_for_ts_import_type(value)),
  }
}

fn set_parent_for_ts_type_query_expr(node: &mut TsTypeQueryExpr, parent: Node) {
  match node {
    TsTypeQueryExpr::TsEntityName(node) => {
      set_parent_for_ts_entity_name(node, parent);
    },
    TsTypeQueryExpr::Import(node) => {
      node.parent = parent;
    },
  }
}

/// `namespace A.B { }` is a namespace named `A` with another TsNamespaceDecl as
/// its body.
pub enum TsNamespaceBody {
  TsModuleBlock(TsModuleBlock),
  TsNamespaceDecl(TsNamespaceDecl),
}

impl Spanned for TsNamespaceBody {
  fn span(&self) -> Span {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.span(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.span(),
    }
  }
}

impl NodeTrait for TsNamespaceBody {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.parent(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.children(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.children(),
    }
  }
}
impl From<&TsNamespaceBody> for Node {
  fn from(node: &TsNamespaceBody) -> Node {
    match node {
      TsNamespaceBody::TsModuleBlock(node) => node.into(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.into(),
    }
  }
}

fn get_view_for_ts_namespace_body(ref_node: &'static swc_ecma_ast::TsNamespaceBody) -> TsNamespaceBody {
  match ref_node {
    swc_ecma_ast::TsNamespaceBody::TsModuleBlock(value) => TsNamespaceBody::TsModuleBlock(get_view_for_ts_module_block(value)),
    swc_ecma_ast::TsNamespaceBody::TsNamespaceDecl(value) => TsNamespaceBody::TsNamespaceDecl(get_view_for_ts_namespace_decl(value)),
  }
}

fn set_parent_for_ts_namespace_body(node: &mut TsNamespaceBody, parent: Node) {
  match node {
    TsNamespaceBody::TsModuleBlock(node) => {
      node.parent = parent;
    },
    TsNamespaceBody::TsNamespaceDecl(node) => {
      node.parent = parent;
    },
  }
}

pub enum Lit {
  Str(Str),
  Bool(Bool),
  Null(Null),
  Num(Number),
  BigInt(BigInt),
  Regex(Regex),
  JSXText(JSXText),
}

impl Spanned for Lit {
  fn span(&self) -> Span {
    match self {
      Lit::Str(node) => node.span(),
      Lit::Bool(node) => node.span(),
      Lit::Null(node) => node.span(),
      Lit::Num(node) => node.span(),
      Lit::BigInt(node) => node.span(),
      Lit::Regex(node) => node.span(),
      Lit::JSXText(node) => node.span(),
    }
  }
}

impl NodeTrait for Lit {
  fn parent(&self) -> Option<&Node> {
    match self {
      Lit::Str(node) => node.parent(),
      Lit::Bool(node) => node.parent(),
      Lit::Null(node) => node.parent(),
      Lit::Num(node) => node.parent(),
      Lit::BigInt(node) => node.parent(),
      Lit::Regex(node) => node.parent(),
      Lit::JSXText(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      Lit::Str(node) => node.children(),
      Lit::Bool(node) => node.children(),
      Lit::Null(node) => node.children(),
      Lit::Num(node) => node.children(),
      Lit::BigInt(node) => node.children(),
      Lit::Regex(node) => node.children(),
      Lit::JSXText(node) => node.children(),
    }
  }
}
impl From<&Lit> for Node {
  fn from(node: &Lit) -> Node {
    match node {
      Lit::Str(node) => node.into(),
      Lit::Bool(node) => node.into(),
      Lit::Null(node) => node.into(),
      Lit::Num(node) => node.into(),
      Lit::BigInt(node) => node.into(),
      Lit::Regex(node) => node.into(),
      Lit::JSXText(node) => node.into(),
    }
  }
}

fn get_view_for_lit(ref_node: &'static swc_ecma_ast::Lit) -> Lit {
  match ref_node {
    swc_ecma_ast::Lit::Str(value) => Lit::Str(get_view_for_str(value)),
    swc_ecma_ast::Lit::Bool(value) => Lit::Bool(get_view_for_bool(value)),
    swc_ecma_ast::Lit::Null(value) => Lit::Null(get_view_for_null(value)),
    swc_ecma_ast::Lit::Num(value) => Lit::Num(get_view_for_number(value)),
    swc_ecma_ast::Lit::BigInt(value) => Lit::BigInt(get_view_for_big_int(value)),
    swc_ecma_ast::Lit::Regex(value) => Lit::Regex(get_view_for_regex(value)),
    swc_ecma_ast::Lit::JSXText(value) => Lit::JSXText(get_view_for_jsxtext(value)),
  }
}

fn set_parent_for_lit(node: &mut Lit, parent: Node) {
  match node {
    Lit::Str(node) => {
      node.parent = parent;
    },
    Lit::Bool(node) => {
      node.parent = parent;
    },
    Lit::Null(node) => {
      node.parent = parent;
    },
    Lit::Num(node) => {
      node.parent = parent;
    },
    Lit::BigInt(node) => {
      node.parent = parent;
    },
    Lit::Regex(node) => {
      node.parent = parent;
    },
    Lit::JSXText(node) => {
      node.parent = parent;
    },
  }
}

pub enum ImportSpecifier {
  Named(ImportNamedSpecifier),
  Default(ImportDefaultSpecifier),
  Namespace(ImportStarAsSpecifier),
}

impl Spanned for ImportSpecifier {
  fn span(&self) -> Span {
    match self {
      ImportSpecifier::Named(node) => node.span(),
      ImportSpecifier::Default(node) => node.span(),
      ImportSpecifier::Namespace(node) => node.span(),
    }
  }
}

impl NodeTrait for ImportSpecifier {
  fn parent(&self) -> Option<&Node> {
    match self {
      ImportSpecifier::Named(node) => node.parent(),
      ImportSpecifier::Default(node) => node.parent(),
      ImportSpecifier::Namespace(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      ImportSpecifier::Named(node) => node.children(),
      ImportSpecifier::Default(node) => node.children(),
      ImportSpecifier::Namespace(node) => node.children(),
    }
  }
}
impl From<&ImportSpecifier> for Node {
  fn from(node: &ImportSpecifier) -> Node {
    match node {
      ImportSpecifier::Named(node) => node.into(),
      ImportSpecifier::Default(node) => node.into(),
      ImportSpecifier::Namespace(node) => node.into(),
    }
  }
}

fn get_view_for_import_specifier(ref_node: &'static swc_ecma_ast::ImportSpecifier) -> ImportSpecifier {
  match ref_node {
    swc_ecma_ast::ImportSpecifier::Named(value) => ImportSpecifier::Named(get_view_for_import_named_specifier(value)),
    swc_ecma_ast::ImportSpecifier::Default(value) => ImportSpecifier::Default(get_view_for_import_default_specifier(value)),
    swc_ecma_ast::ImportSpecifier::Namespace(value) => ImportSpecifier::Namespace(get_view_for_import_star_as_specifier(value)),
  }
}

fn set_parent_for_import_specifier(node: &mut ImportSpecifier, parent: Node) {
  match node {
    ImportSpecifier::Named(node) => {
      node.parent = parent;
    },
    ImportSpecifier::Default(node) => {
      node.parent = parent;
    },
    ImportSpecifier::Namespace(node) => {
      node.parent = parent;
    },
  }
}

pub enum ExportSpecifier {
  Namespace(ExportNamespaceSpecifier),
  Default(ExportDefaultSpecifier),
  Named(ExportNamedSpecifier),
}

impl Spanned for ExportSpecifier {
  fn span(&self) -> Span {
    match self {
      ExportSpecifier::Namespace(node) => node.span(),
      ExportSpecifier::Default(node) => node.span(),
      ExportSpecifier::Named(node) => node.span(),
    }
  }
}

impl NodeTrait for ExportSpecifier {
  fn parent(&self) -> Option<&Node> {
    match self {
      ExportSpecifier::Namespace(node) => node.parent(),
      ExportSpecifier::Default(node) => node.parent(),
      ExportSpecifier::Named(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      ExportSpecifier::Namespace(node) => node.children(),
      ExportSpecifier::Default(node) => node.children(),
      ExportSpecifier::Named(node) => node.children(),
    }
  }
}
impl From<&ExportSpecifier> for Node {
  fn from(node: &ExportSpecifier) -> Node {
    match node {
      ExportSpecifier::Namespace(node) => node.into(),
      ExportSpecifier::Default(node) => node.into(),
      ExportSpecifier::Named(node) => node.into(),
    }
  }
}

fn get_view_for_export_specifier(ref_node: &'static swc_ecma_ast::ExportSpecifier) -> ExportSpecifier {
  match ref_node {
    swc_ecma_ast::ExportSpecifier::Namespace(value) => ExportSpecifier::Namespace(get_view_for_export_namespace_specifier(value)),
    swc_ecma_ast::ExportSpecifier::Default(value) => ExportSpecifier::Default(get_view_for_export_default_specifier(value)),
    swc_ecma_ast::ExportSpecifier::Named(value) => ExportSpecifier::Named(get_view_for_export_named_specifier(value)),
  }
}

fn set_parent_for_export_specifier(node: &mut ExportSpecifier, parent: Node) {
  match node {
    ExportSpecifier::Namespace(node) => {
      node.parent = parent;
    },
    ExportSpecifier::Default(node) => {
      node.parent = parent;
    },
    ExportSpecifier::Named(node) => {
      node.parent = parent;
    },
  }
}

pub enum Stmt {
  Block(BlockStmt),
  Empty(EmptyStmt),
  Debugger(DebuggerStmt),
  With(WithStmt),
  Return(ReturnStmt),
  Labeled(LabeledStmt),
  Break(BreakStmt),
  Continue(ContinueStmt),
  If(IfStmt),
  Switch(SwitchStmt),
  Throw(ThrowStmt),
  /// A try statement. If handler is null then finalizer must be a BlockStmt.
  Try(TryStmt),
  While(WhileStmt),
  DoWhile(DoWhileStmt),
  For(ForStmt),
  ForIn(ForInStmt),
  ForOf(ForOfStmt),
  Decl(Decl),
  Expr(ExprStmt),
}

impl Spanned for Stmt {
  fn span(&self) -> Span {
    match self {
      Stmt::Block(node) => node.span(),
      Stmt::Empty(node) => node.span(),
      Stmt::Debugger(node) => node.span(),
      Stmt::With(node) => node.span(),
      Stmt::Return(node) => node.span(),
      Stmt::Labeled(node) => node.span(),
      Stmt::Break(node) => node.span(),
      Stmt::Continue(node) => node.span(),
      Stmt::If(node) => node.span(),
      Stmt::Switch(node) => node.span(),
      Stmt::Throw(node) => node.span(),
      Stmt::Try(node) => node.span(),
      Stmt::While(node) => node.span(),
      Stmt::DoWhile(node) => node.span(),
      Stmt::For(node) => node.span(),
      Stmt::ForIn(node) => node.span(),
      Stmt::ForOf(node) => node.span(),
      Stmt::Decl(node) => node.span(),
      Stmt::Expr(node) => node.span(),
    }
  }
}

impl NodeTrait for Stmt {
  fn parent(&self) -> Option<&Node> {
    match self {
      Stmt::Block(node) => node.parent(),
      Stmt::Empty(node) => node.parent(),
      Stmt::Debugger(node) => node.parent(),
      Stmt::With(node) => node.parent(),
      Stmt::Return(node) => node.parent(),
      Stmt::Labeled(node) => node.parent(),
      Stmt::Break(node) => node.parent(),
      Stmt::Continue(node) => node.parent(),
      Stmt::If(node) => node.parent(),
      Stmt::Switch(node) => node.parent(),
      Stmt::Throw(node) => node.parent(),
      Stmt::Try(node) => node.parent(),
      Stmt::While(node) => node.parent(),
      Stmt::DoWhile(node) => node.parent(),
      Stmt::For(node) => node.parent(),
      Stmt::ForIn(node) => node.parent(),
      Stmt::ForOf(node) => node.parent(),
      Stmt::Decl(node) => node.parent(),
      Stmt::Expr(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      Stmt::Block(node) => node.children(),
      Stmt::Empty(node) => node.children(),
      Stmt::Debugger(node) => node.children(),
      Stmt::With(node) => node.children(),
      Stmt::Return(node) => node.children(),
      Stmt::Labeled(node) => node.children(),
      Stmt::Break(node) => node.children(),
      Stmt::Continue(node) => node.children(),
      Stmt::If(node) => node.children(),
      Stmt::Switch(node) => node.children(),
      Stmt::Throw(node) => node.children(),
      Stmt::Try(node) => node.children(),
      Stmt::While(node) => node.children(),
      Stmt::DoWhile(node) => node.children(),
      Stmt::For(node) => node.children(),
      Stmt::ForIn(node) => node.children(),
      Stmt::ForOf(node) => node.children(),
      Stmt::Decl(node) => node.children(),
      Stmt::Expr(node) => node.children(),
    }
  }
}
impl From<&Stmt> for Node {
  fn from(node: &Stmt) -> Node {
    match node {
      Stmt::Block(node) => node.into(),
      Stmt::Empty(node) => node.into(),
      Stmt::Debugger(node) => node.into(),
      Stmt::With(node) => node.into(),
      Stmt::Return(node) => node.into(),
      Stmt::Labeled(node) => node.into(),
      Stmt::Break(node) => node.into(),
      Stmt::Continue(node) => node.into(),
      Stmt::If(node) => node.into(),
      Stmt::Switch(node) => node.into(),
      Stmt::Throw(node) => node.into(),
      Stmt::Try(node) => node.into(),
      Stmt::While(node) => node.into(),
      Stmt::DoWhile(node) => node.into(),
      Stmt::For(node) => node.into(),
      Stmt::ForIn(node) => node.into(),
      Stmt::ForOf(node) => node.into(),
      Stmt::Decl(node) => node.into(),
      Stmt::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_stmt(ref_node: &'static swc_ecma_ast::Stmt) -> Stmt {
  match ref_node {
    swc_ecma_ast::Stmt::Block(value) => Stmt::Block(get_view_for_block_stmt(value)),
    swc_ecma_ast::Stmt::Empty(value) => Stmt::Empty(get_view_for_empty_stmt(value)),
    swc_ecma_ast::Stmt::Debugger(value) => Stmt::Debugger(get_view_for_debugger_stmt(value)),
    swc_ecma_ast::Stmt::With(value) => Stmt::With(get_view_for_with_stmt(value)),
    swc_ecma_ast::Stmt::Return(value) => Stmt::Return(get_view_for_return_stmt(value)),
    swc_ecma_ast::Stmt::Labeled(value) => Stmt::Labeled(get_view_for_labeled_stmt(value)),
    swc_ecma_ast::Stmt::Break(value) => Stmt::Break(get_view_for_break_stmt(value)),
    swc_ecma_ast::Stmt::Continue(value) => Stmt::Continue(get_view_for_continue_stmt(value)),
    swc_ecma_ast::Stmt::If(value) => Stmt::If(get_view_for_if_stmt(value)),
    swc_ecma_ast::Stmt::Switch(value) => Stmt::Switch(get_view_for_switch_stmt(value)),
    swc_ecma_ast::Stmt::Throw(value) => Stmt::Throw(get_view_for_throw_stmt(value)),
    swc_ecma_ast::Stmt::Try(value) => Stmt::Try(get_view_for_try_stmt(value)),
    swc_ecma_ast::Stmt::While(value) => Stmt::While(get_view_for_while_stmt(value)),
    swc_ecma_ast::Stmt::DoWhile(value) => Stmt::DoWhile(get_view_for_do_while_stmt(value)),
    swc_ecma_ast::Stmt::For(value) => Stmt::For(get_view_for_for_stmt(value)),
    swc_ecma_ast::Stmt::ForIn(value) => Stmt::ForIn(get_view_for_for_in_stmt(value)),
    swc_ecma_ast::Stmt::ForOf(value) => Stmt::ForOf(get_view_for_for_of_stmt(value)),
    swc_ecma_ast::Stmt::Decl(value) => Stmt::Decl(get_view_for_decl(value)),
    swc_ecma_ast::Stmt::Expr(value) => Stmt::Expr(get_view_for_expr_stmt(value)),
  }
}

fn set_parent_for_stmt(node: &mut Stmt, parent: Node) {
  match node {
    Stmt::Block(node) => {
      node.parent = parent;
    },
    Stmt::Empty(node) => {
      node.parent = parent;
    },
    Stmt::Debugger(node) => {
      node.parent = parent;
    },
    Stmt::With(node) => {
      node.parent = parent;
    },
    Stmt::Return(node) => {
      node.parent = parent;
    },
    Stmt::Labeled(node) => {
      node.parent = parent;
    },
    Stmt::Break(node) => {
      node.parent = parent;
    },
    Stmt::Continue(node) => {
      node.parent = parent;
    },
    Stmt::If(node) => {
      node.parent = parent;
    },
    Stmt::Switch(node) => {
      node.parent = parent;
    },
    Stmt::Throw(node) => {
      node.parent = parent;
    },
    Stmt::Try(node) => {
      node.parent = parent;
    },
    Stmt::While(node) => {
      node.parent = parent;
    },
    Stmt::DoWhile(node) => {
      node.parent = parent;
    },
    Stmt::For(node) => {
      node.parent = parent;
    },
    Stmt::ForIn(node) => {
      node.parent = parent;
    },
    Stmt::ForOf(node) => {
      node.parent = parent;
    },
    Stmt::Decl(node) => {
      set_parent_for_decl(node, parent);
    },
    Stmt::Expr(node) => {
      node.parent = parent;
    },
  }
}

pub enum Pat {
  Ident(Ident),
  Array(ArrayPat),
  Rest(RestPat),
  Object(ObjectPat),
  Assign(AssignPat),
  Invalid(Invalid),
  /// Only for for-in / for-of loops. This is *syntatically* valid.
  Expr(Box<Expr>),
}

impl Spanned for Pat {
  fn span(&self) -> Span {
    match self {
      Pat::Ident(node) => node.span(),
      Pat::Array(node) => node.span(),
      Pat::Rest(node) => node.span(),
      Pat::Object(node) => node.span(),
      Pat::Assign(node) => node.span(),
      Pat::Invalid(node) => node.span(),
      Pat::Expr(node) => node.span(),
    }
  }
}

impl NodeTrait for Pat {
  fn parent(&self) -> Option<&Node> {
    match self {
      Pat::Ident(node) => node.parent(),
      Pat::Array(node) => node.parent(),
      Pat::Rest(node) => node.parent(),
      Pat::Object(node) => node.parent(),
      Pat::Assign(node) => node.parent(),
      Pat::Invalid(node) => node.parent(),
      Pat::Expr(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      Pat::Ident(node) => node.children(),
      Pat::Array(node) => node.children(),
      Pat::Rest(node) => node.children(),
      Pat::Object(node) => node.children(),
      Pat::Assign(node) => node.children(),
      Pat::Invalid(node) => node.children(),
      Pat::Expr(node) => node.children(),
    }
  }
}
impl From<&Pat> for Node {
  fn from(node: &Pat) -> Node {
    match node {
      Pat::Ident(node) => node.into(),
      Pat::Array(node) => node.into(),
      Pat::Rest(node) => node.into(),
      Pat::Object(node) => node.into(),
      Pat::Assign(node) => node.into(),
      Pat::Invalid(node) => node.into(),
      Pat::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_pat(ref_node: &'static swc_ecma_ast::Pat) -> Pat {
  match ref_node {
    swc_ecma_ast::Pat::Ident(value) => Pat::Ident(get_view_for_ident(value)),
    swc_ecma_ast::Pat::Array(value) => Pat::Array(get_view_for_array_pat(value)),
    swc_ecma_ast::Pat::Rest(value) => Pat::Rest(get_view_for_rest_pat(value)),
    swc_ecma_ast::Pat::Object(value) => Pat::Object(get_view_for_object_pat(value)),
    swc_ecma_ast::Pat::Assign(value) => Pat::Assign(get_view_for_assign_pat(value)),
    swc_ecma_ast::Pat::Invalid(value) => Pat::Invalid(get_view_for_invalid(value)),
    swc_ecma_ast::Pat::Expr(value) => Pat::Expr(Box::new(get_view_for_expr(value))),
  }
}

fn set_parent_for_pat(node: &mut Pat, parent: Node) {
  match node {
    Pat::Ident(node) => {
      node.parent = parent;
    },
    Pat::Array(node) => {
      node.parent = parent;
    },
    Pat::Rest(node) => {
      node.parent = parent;
    },
    Pat::Object(node) => {
      node.parent = parent;
    },
    Pat::Assign(node) => {
      node.parent = parent;
    },
    Pat::Invalid(node) => {
      node.parent = parent;
    },
    Pat::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
  }
}

pub enum TsModuleName {
  Ident(Ident),
  Str(Str),
}

impl Spanned for TsModuleName {
  fn span(&self) -> Span {
    match self {
      TsModuleName::Ident(node) => node.span(),
      TsModuleName::Str(node) => node.span(),
    }
  }
}

impl NodeTrait for TsModuleName {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsModuleName::Ident(node) => node.parent(),
      TsModuleName::Str(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsModuleName::Ident(node) => node.children(),
      TsModuleName::Str(node) => node.children(),
    }
  }
}
impl From<&TsModuleName> for Node {
  fn from(node: &TsModuleName) -> Node {
    match node {
      TsModuleName::Ident(node) => node.into(),
      TsModuleName::Str(node) => node.into(),
    }
  }
}

fn get_view_for_ts_module_name(ref_node: &'static swc_ecma_ast::TsModuleName) -> TsModuleName {
  match ref_node {
    swc_ecma_ast::TsModuleName::Ident(value) => TsModuleName::Ident(get_view_for_ident(value)),
    swc_ecma_ast::TsModuleName::Str(value) => TsModuleName::Str(get_view_for_str(value)),
  }
}

fn set_parent_for_ts_module_name(node: &mut TsModuleName, parent: Node) {
  match node {
    TsModuleName::Ident(node) => {
      node.parent = parent;
    },
    TsModuleName::Str(node) => {
      node.parent = parent;
    },
  }
}

pub enum TsFnParam {
  Ident(Ident),
  Array(ArrayPat),
  Rest(RestPat),
  Object(ObjectPat),
}

impl Spanned for TsFnParam {
  fn span(&self) -> Span {
    match self {
      TsFnParam::Ident(node) => node.span(),
      TsFnParam::Array(node) => node.span(),
      TsFnParam::Rest(node) => node.span(),
      TsFnParam::Object(node) => node.span(),
    }
  }
}

impl NodeTrait for TsFnParam {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsFnParam::Ident(node) => node.parent(),
      TsFnParam::Array(node) => node.parent(),
      TsFnParam::Rest(node) => node.parent(),
      TsFnParam::Object(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsFnParam::Ident(node) => node.children(),
      TsFnParam::Array(node) => node.children(),
      TsFnParam::Rest(node) => node.children(),
      TsFnParam::Object(node) => node.children(),
    }
  }
}
impl From<&TsFnParam> for Node {
  fn from(node: &TsFnParam) -> Node {
    match node {
      TsFnParam::Ident(node) => node.into(),
      TsFnParam::Array(node) => node.into(),
      TsFnParam::Rest(node) => node.into(),
      TsFnParam::Object(node) => node.into(),
    }
  }
}

fn get_view_for_ts_fn_param(ref_node: &'static swc_ecma_ast::TsFnParam) -> TsFnParam {
  match ref_node {
    swc_ecma_ast::TsFnParam::Ident(value) => TsFnParam::Ident(get_view_for_ident(value)),
    swc_ecma_ast::TsFnParam::Array(value) => TsFnParam::Array(get_view_for_array_pat(value)),
    swc_ecma_ast::TsFnParam::Rest(value) => TsFnParam::Rest(get_view_for_rest_pat(value)),
    swc_ecma_ast::TsFnParam::Object(value) => TsFnParam::Object(get_view_for_object_pat(value)),
  }
}

fn set_parent_for_ts_fn_param(node: &mut TsFnParam, parent: Node) {
  match node {
    TsFnParam::Ident(node) => {
      node.parent = parent;
    },
    TsFnParam::Array(node) => {
      node.parent = parent;
    },
    TsFnParam::Rest(node) => {
      node.parent = parent;
    },
    TsFnParam::Object(node) => {
      node.parent = parent;
    },
  }
}

pub enum ClassMember {
  Constructor(Constructor),
  /// `es2015`
  Method(ClassMethod),
  PrivateMethod(PrivateMethod),
  /// stage 0 / Typescript
  ClassProp(ClassProp),
  PrivateProp(PrivateProp),
  TsIndexSignature(TsIndexSignature),
  Empty(EmptyStmt),
}

impl Spanned for ClassMember {
  fn span(&self) -> Span {
    match self {
      ClassMember::Constructor(node) => node.span(),
      ClassMember::Method(node) => node.span(),
      ClassMember::PrivateMethod(node) => node.span(),
      ClassMember::ClassProp(node) => node.span(),
      ClassMember::PrivateProp(node) => node.span(),
      ClassMember::TsIndexSignature(node) => node.span(),
      ClassMember::Empty(node) => node.span(),
    }
  }
}

impl NodeTrait for ClassMember {
  fn parent(&self) -> Option<&Node> {
    match self {
      ClassMember::Constructor(node) => node.parent(),
      ClassMember::Method(node) => node.parent(),
      ClassMember::PrivateMethod(node) => node.parent(),
      ClassMember::ClassProp(node) => node.parent(),
      ClassMember::PrivateProp(node) => node.parent(),
      ClassMember::TsIndexSignature(node) => node.parent(),
      ClassMember::Empty(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      ClassMember::Constructor(node) => node.children(),
      ClassMember::Method(node) => node.children(),
      ClassMember::PrivateMethod(node) => node.children(),
      ClassMember::ClassProp(node) => node.children(),
      ClassMember::PrivateProp(node) => node.children(),
      ClassMember::TsIndexSignature(node) => node.children(),
      ClassMember::Empty(node) => node.children(),
    }
  }
}
impl From<&ClassMember> for Node {
  fn from(node: &ClassMember) -> Node {
    match node {
      ClassMember::Constructor(node) => node.into(),
      ClassMember::Method(node) => node.into(),
      ClassMember::PrivateMethod(node) => node.into(),
      ClassMember::ClassProp(node) => node.into(),
      ClassMember::PrivateProp(node) => node.into(),
      ClassMember::TsIndexSignature(node) => node.into(),
      ClassMember::Empty(node) => node.into(),
    }
  }
}

fn get_view_for_class_member(ref_node: &'static swc_ecma_ast::ClassMember) -> ClassMember {
  match ref_node {
    swc_ecma_ast::ClassMember::Constructor(value) => ClassMember::Constructor(get_view_for_constructor(value)),
    swc_ecma_ast::ClassMember::Method(value) => ClassMember::Method(get_view_for_class_method(value)),
    swc_ecma_ast::ClassMember::PrivateMethod(value) => ClassMember::PrivateMethod(get_view_for_private_method(value)),
    swc_ecma_ast::ClassMember::ClassProp(value) => ClassMember::ClassProp(get_view_for_class_prop(value)),
    swc_ecma_ast::ClassMember::PrivateProp(value) => ClassMember::PrivateProp(get_view_for_private_prop(value)),
    swc_ecma_ast::ClassMember::TsIndexSignature(value) => ClassMember::TsIndexSignature(get_view_for_ts_index_signature(value)),
    swc_ecma_ast::ClassMember::Empty(value) => ClassMember::Empty(get_view_for_empty_stmt(value)),
  }
}

fn set_parent_for_class_member(node: &mut ClassMember, parent: Node) {
  match node {
    ClassMember::Constructor(node) => {
      node.parent = parent;
    },
    ClassMember::Method(node) => {
      node.parent = parent;
    },
    ClassMember::PrivateMethod(node) => {
      node.parent = parent;
    },
    ClassMember::ClassProp(node) => {
      node.parent = parent;
    },
    ClassMember::PrivateProp(node) => {
      node.parent = parent;
    },
    ClassMember::TsIndexSignature(node) => {
      node.parent = parent;
    },
    ClassMember::Empty(node) => {
      node.parent = parent;
    },
  }
}

pub enum VarDeclOrPat {
  VarDecl(VarDecl),
  Pat(Pat),
}

impl Spanned for VarDeclOrPat {
  fn span(&self) -> Span {
    match self {
      VarDeclOrPat::VarDecl(node) => node.span(),
      VarDeclOrPat::Pat(node) => node.span(),
    }
  }
}

impl NodeTrait for VarDeclOrPat {
  fn parent(&self) -> Option<&Node> {
    match self {
      VarDeclOrPat::VarDecl(node) => node.parent(),
      VarDeclOrPat::Pat(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      VarDeclOrPat::VarDecl(node) => node.children(),
      VarDeclOrPat::Pat(node) => node.children(),
    }
  }
}
impl From<&VarDeclOrPat> for Node {
  fn from(node: &VarDeclOrPat) -> Node {
    match node {
      VarDeclOrPat::VarDecl(node) => node.into(),
      VarDeclOrPat::Pat(node) => node.into(),
    }
  }
}

fn get_view_for_var_decl_or_pat(ref_node: &'static swc_ecma_ast::VarDeclOrPat) -> VarDeclOrPat {
  match ref_node {
    swc_ecma_ast::VarDeclOrPat::VarDecl(value) => VarDeclOrPat::VarDecl(get_view_for_var_decl(value)),
    swc_ecma_ast::VarDeclOrPat::Pat(value) => VarDeclOrPat::Pat(get_view_for_pat(value)),
  }
}

fn set_parent_for_var_decl_or_pat(node: &mut VarDeclOrPat, parent: Node) {
  match node {
    VarDeclOrPat::VarDecl(node) => {
      node.parent = parent;
    },
    VarDeclOrPat::Pat(node) => {
      set_parent_for_pat(node, parent);
    },
  }
}

pub enum TsModuleRef {
  TsEntityName(TsEntityName),
  TsExternalModuleRef(TsExternalModuleRef),
}

impl Spanned for TsModuleRef {
  fn span(&self) -> Span {
    match self {
      TsModuleRef::TsEntityName(node) => node.span(),
      TsModuleRef::TsExternalModuleRef(node) => node.span(),
    }
  }
}

impl NodeTrait for TsModuleRef {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsModuleRef::TsEntityName(node) => node.parent(),
      TsModuleRef::TsExternalModuleRef(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsModuleRef::TsEntityName(node) => node.children(),
      TsModuleRef::TsExternalModuleRef(node) => node.children(),
    }
  }
}
impl From<&TsModuleRef> for Node {
  fn from(node: &TsModuleRef) -> Node {
    match node {
      TsModuleRef::TsEntityName(node) => node.into(),
      TsModuleRef::TsExternalModuleRef(node) => node.into(),
    }
  }
}

fn get_view_for_ts_module_ref(ref_node: &'static swc_ecma_ast::TsModuleRef) -> TsModuleRef {
  match ref_node {
    swc_ecma_ast::TsModuleRef::TsEntityName(value) => TsModuleRef::TsEntityName(get_view_for_ts_entity_name(value)),
    swc_ecma_ast::TsModuleRef::TsExternalModuleRef(value) => TsModuleRef::TsExternalModuleRef(get_view_for_ts_external_module_ref(value)),
  }
}

fn set_parent_for_ts_module_ref(node: &mut TsModuleRef, parent: Node) {
  match node {
    TsModuleRef::TsEntityName(node) => {
      set_parent_for_ts_entity_name(node, parent);
    },
    TsModuleRef::TsExternalModuleRef(node) => {
      node.parent = parent;
    },
  }
}

pub enum JSXAttrOrSpread {
  JSXAttr(JSXAttr),
  SpreadElement(SpreadElement),
}

impl Spanned for JSXAttrOrSpread {
  fn span(&self) -> Span {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.span(),
      JSXAttrOrSpread::SpreadElement(node) => node.span(),
    }
  }
}

impl NodeTrait for JSXAttrOrSpread {
  fn parent(&self) -> Option<&Node> {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.parent(),
      JSXAttrOrSpread::SpreadElement(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.children(),
      JSXAttrOrSpread::SpreadElement(node) => node.children(),
    }
  }
}
impl From<&JSXAttrOrSpread> for Node {
  fn from(node: &JSXAttrOrSpread) -> Node {
    match node {
      JSXAttrOrSpread::JSXAttr(node) => node.into(),
      JSXAttrOrSpread::SpreadElement(node) => node.into(),
    }
  }
}

fn get_view_for_jsxattr_or_spread(ref_node: &'static swc_ecma_ast::JSXAttrOrSpread) -> JSXAttrOrSpread {
  match ref_node {
    swc_ecma_ast::JSXAttrOrSpread::JSXAttr(value) => JSXAttrOrSpread::JSXAttr(get_view_for_jsxattr(value)),
    swc_ecma_ast::JSXAttrOrSpread::SpreadElement(value) => JSXAttrOrSpread::SpreadElement(get_view_for_spread_element(value)),
  }
}

fn set_parent_for_jsxattr_or_spread(node: &mut JSXAttrOrSpread, parent: Node) {
  match node {
    JSXAttrOrSpread::JSXAttr(node) => {
      node.parent = parent;
    },
    JSXAttrOrSpread::SpreadElement(node) => {
      node.parent = parent;
    },
  }
}

pub enum ParamOrTsParamProp {
  TsParamProp(TsParamProp),
  Param(Param),
}

impl Spanned for ParamOrTsParamProp {
  fn span(&self) -> Span {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.span(),
      ParamOrTsParamProp::Param(node) => node.span(),
    }
  }
}

impl NodeTrait for ParamOrTsParamProp {
  fn parent(&self) -> Option<&Node> {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.parent(),
      ParamOrTsParamProp::Param(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.children(),
      ParamOrTsParamProp::Param(node) => node.children(),
    }
  }
}
impl From<&ParamOrTsParamProp> for Node {
  fn from(node: &ParamOrTsParamProp) -> Node {
    match node {
      ParamOrTsParamProp::TsParamProp(node) => node.into(),
      ParamOrTsParamProp::Param(node) => node.into(),
    }
  }
}

fn get_view_for_param_or_ts_param_prop(ref_node: &'static swc_ecma_ast::ParamOrTsParamProp) -> ParamOrTsParamProp {
  match ref_node {
    swc_ecma_ast::ParamOrTsParamProp::TsParamProp(value) => ParamOrTsParamProp::TsParamProp(get_view_for_ts_param_prop(value)),
    swc_ecma_ast::ParamOrTsParamProp::Param(value) => ParamOrTsParamProp::Param(get_view_for_param(value)),
  }
}

fn set_parent_for_param_or_ts_param_prop(node: &mut ParamOrTsParamProp, parent: Node) {
  match node {
    ParamOrTsParamProp::TsParamProp(node) => {
      node.parent = parent;
    },
    ParamOrTsParamProp::Param(node) => {
      node.parent = parent;
    },
  }
}

pub enum ExprOrSuper {
  Super(Super),
  Expr(Box<Expr>),
}

impl Spanned for ExprOrSuper {
  fn span(&self) -> Span {
    match self {
      ExprOrSuper::Super(node) => node.span(),
      ExprOrSuper::Expr(node) => node.span(),
    }
  }
}

impl NodeTrait for ExprOrSuper {
  fn parent(&self) -> Option<&Node> {
    match self {
      ExprOrSuper::Super(node) => node.parent(),
      ExprOrSuper::Expr(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      ExprOrSuper::Super(node) => node.children(),
      ExprOrSuper::Expr(node) => node.children(),
    }
  }
}
impl From<&ExprOrSuper> for Node {
  fn from(node: &ExprOrSuper) -> Node {
    match node {
      ExprOrSuper::Super(node) => node.into(),
      ExprOrSuper::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_expr_or_super(ref_node: &'static swc_ecma_ast::ExprOrSuper) -> ExprOrSuper {
  match ref_node {
    swc_ecma_ast::ExprOrSuper::Super(value) => ExprOrSuper::Super(get_view_for_super(value)),
    swc_ecma_ast::ExprOrSuper::Expr(value) => ExprOrSuper::Expr(Box::new(get_view_for_expr(value))),
  }
}

fn set_parent_for_expr_or_super(node: &mut ExprOrSuper, parent: Node) {
  match node {
    ExprOrSuper::Super(node) => {
      node.parent = parent;
    },
    ExprOrSuper::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
  }
}

pub enum TsTypeElement {
  TsCallSignatureDecl(TsCallSignatureDecl),
  TsConstructSignatureDecl(TsConstructSignatureDecl),
  TsPropertySignature(TsPropertySignature),
  TsMethodSignature(TsMethodSignature),
  TsIndexSignature(TsIndexSignature),
}

impl Spanned for TsTypeElement {
  fn span(&self) -> Span {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.span(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.span(),
      TsTypeElement::TsPropertySignature(node) => node.span(),
      TsTypeElement::TsMethodSignature(node) => node.span(),
      TsTypeElement::TsIndexSignature(node) => node.span(),
    }
  }
}

impl NodeTrait for TsTypeElement {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.parent(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.parent(),
      TsTypeElement::TsPropertySignature(node) => node.parent(),
      TsTypeElement::TsMethodSignature(node) => node.parent(),
      TsTypeElement::TsIndexSignature(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.children(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.children(),
      TsTypeElement::TsPropertySignature(node) => node.children(),
      TsTypeElement::TsMethodSignature(node) => node.children(),
      TsTypeElement::TsIndexSignature(node) => node.children(),
    }
  }
}
impl From<&TsTypeElement> for Node {
  fn from(node: &TsTypeElement) -> Node {
    match node {
      TsTypeElement::TsCallSignatureDecl(node) => node.into(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.into(),
      TsTypeElement::TsPropertySignature(node) => node.into(),
      TsTypeElement::TsMethodSignature(node) => node.into(),
      TsTypeElement::TsIndexSignature(node) => node.into(),
    }
  }
}

fn get_view_for_ts_type_element(ref_node: &'static swc_ecma_ast::TsTypeElement) -> TsTypeElement {
  match ref_node {
    swc_ecma_ast::TsTypeElement::TsCallSignatureDecl(value) => TsTypeElement::TsCallSignatureDecl(get_view_for_ts_call_signature_decl(value)),
    swc_ecma_ast::TsTypeElement::TsConstructSignatureDecl(value) => TsTypeElement::TsConstructSignatureDecl(get_view_for_ts_construct_signature_decl(value)),
    swc_ecma_ast::TsTypeElement::TsPropertySignature(value) => TsTypeElement::TsPropertySignature(get_view_for_ts_property_signature(value)),
    swc_ecma_ast::TsTypeElement::TsMethodSignature(value) => TsTypeElement::TsMethodSignature(get_view_for_ts_method_signature(value)),
    swc_ecma_ast::TsTypeElement::TsIndexSignature(value) => TsTypeElement::TsIndexSignature(get_view_for_ts_index_signature(value)),
  }
}

fn set_parent_for_ts_type_element(node: &mut TsTypeElement, parent: Node) {
  match node {
    TsTypeElement::TsCallSignatureDecl(node) => {
      node.parent = parent;
    },
    TsTypeElement::TsConstructSignatureDecl(node) => {
      node.parent = parent;
    },
    TsTypeElement::TsPropertySignature(node) => {
      node.parent = parent;
    },
    TsTypeElement::TsMethodSignature(node) => {
      node.parent = parent;
    },
    TsTypeElement::TsIndexSignature(node) => {
      node.parent = parent;
    },
  }
}

pub enum BlockStmtOrExpr {
  BlockStmt(BlockStmt),
  Expr(Box<Expr>),
}

impl Spanned for BlockStmtOrExpr {
  fn span(&self) -> Span {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.span(),
      BlockStmtOrExpr::Expr(node) => node.span(),
    }
  }
}

impl NodeTrait for BlockStmtOrExpr {
  fn parent(&self) -> Option<&Node> {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.parent(),
      BlockStmtOrExpr::Expr(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.children(),
      BlockStmtOrExpr::Expr(node) => node.children(),
    }
  }
}
impl From<&BlockStmtOrExpr> for Node {
  fn from(node: &BlockStmtOrExpr) -> Node {
    match node {
      BlockStmtOrExpr::BlockStmt(node) => node.into(),
      BlockStmtOrExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_block_stmt_or_expr(ref_node: &'static swc_ecma_ast::BlockStmtOrExpr) -> BlockStmtOrExpr {
  match ref_node {
    swc_ecma_ast::BlockStmtOrExpr::BlockStmt(value) => BlockStmtOrExpr::BlockStmt(get_view_for_block_stmt(value)),
    swc_ecma_ast::BlockStmtOrExpr::Expr(value) => BlockStmtOrExpr::Expr(Box::new(get_view_for_expr(value))),
  }
}

fn set_parent_for_block_stmt_or_expr(node: &mut BlockStmtOrExpr, parent: Node) {
  match node {
    BlockStmtOrExpr::BlockStmt(node) => {
      node.parent = parent;
    },
    BlockStmtOrExpr::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
  }
}

pub enum TsUnionOrIntersectionType {
  TsUnionType(TsUnionType),
  TsIntersectionType(TsIntersectionType),
}

impl Spanned for TsUnionOrIntersectionType {
  fn span(&self) -> Span {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.span(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.span(),
    }
  }
}

impl NodeTrait for TsUnionOrIntersectionType {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.parent(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.children(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.children(),
    }
  }
}
impl From<&TsUnionOrIntersectionType> for Node {
  fn from(node: &TsUnionOrIntersectionType) -> Node {
    match node {
      TsUnionOrIntersectionType::TsUnionType(node) => node.into(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.into(),
    }
  }
}

fn get_view_for_ts_union_or_intersection_type(ref_node: &'static swc_ecma_ast::TsUnionOrIntersectionType) -> TsUnionOrIntersectionType {
  match ref_node {
    swc_ecma_ast::TsUnionOrIntersectionType::TsUnionType(value) => TsUnionOrIntersectionType::TsUnionType(get_view_for_ts_union_type(value)),
    swc_ecma_ast::TsUnionOrIntersectionType::TsIntersectionType(value) => TsUnionOrIntersectionType::TsIntersectionType(get_view_for_ts_intersection_type(value)),
  }
}

fn set_parent_for_ts_union_or_intersection_type(node: &mut TsUnionOrIntersectionType, parent: Node) {
  match node {
    TsUnionOrIntersectionType::TsUnionType(node) => {
      node.parent = parent;
    },
    TsUnionOrIntersectionType::TsIntersectionType(node) => {
      node.parent = parent;
    },
  }
}

pub enum DefaultDecl {
  Class(ClassExpr),
  Fn(FnExpr),
  TsInterfaceDecl(TsInterfaceDecl),
}

impl Spanned for DefaultDecl {
  fn span(&self) -> Span {
    match self {
      DefaultDecl::Class(node) => node.span(),
      DefaultDecl::Fn(node) => node.span(),
      DefaultDecl::TsInterfaceDecl(node) => node.span(),
    }
  }
}

impl NodeTrait for DefaultDecl {
  fn parent(&self) -> Option<&Node> {
    match self {
      DefaultDecl::Class(node) => node.parent(),
      DefaultDecl::Fn(node) => node.parent(),
      DefaultDecl::TsInterfaceDecl(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      DefaultDecl::Class(node) => node.children(),
      DefaultDecl::Fn(node) => node.children(),
      DefaultDecl::TsInterfaceDecl(node) => node.children(),
    }
  }
}
impl From<&DefaultDecl> for Node {
  fn from(node: &DefaultDecl) -> Node {
    match node {
      DefaultDecl::Class(node) => node.into(),
      DefaultDecl::Fn(node) => node.into(),
      DefaultDecl::TsInterfaceDecl(node) => node.into(),
    }
  }
}

fn get_view_for_default_decl(ref_node: &'static swc_ecma_ast::DefaultDecl) -> DefaultDecl {
  match ref_node {
    swc_ecma_ast::DefaultDecl::Class(value) => DefaultDecl::Class(get_view_for_class_expr(value)),
    swc_ecma_ast::DefaultDecl::Fn(value) => DefaultDecl::Fn(get_view_for_fn_expr(value)),
    swc_ecma_ast::DefaultDecl::TsInterfaceDecl(value) => DefaultDecl::TsInterfaceDecl(get_view_for_ts_interface_decl(value)),
  }
}

fn set_parent_for_default_decl(node: &mut DefaultDecl, parent: Node) {
  match node {
    DefaultDecl::Class(node) => {
      node.parent = parent;
    },
    DefaultDecl::Fn(node) => {
      node.parent = parent;
    },
    DefaultDecl::TsInterfaceDecl(node) => {
      node.parent = parent;
    },
  }
}

/// 
/// - Invalid: [Ident] with empty symbol.
pub enum TsEnumMemberId {
  Ident(Ident),
  Str(Str),
}

impl Spanned for TsEnumMemberId {
  fn span(&self) -> Span {
    match self {
      TsEnumMemberId::Ident(node) => node.span(),
      TsEnumMemberId::Str(node) => node.span(),
    }
  }
}

impl NodeTrait for TsEnumMemberId {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsEnumMemberId::Ident(node) => node.parent(),
      TsEnumMemberId::Str(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsEnumMemberId::Ident(node) => node.children(),
      TsEnumMemberId::Str(node) => node.children(),
    }
  }
}
impl From<&TsEnumMemberId> for Node {
  fn from(node: &TsEnumMemberId) -> Node {
    match node {
      TsEnumMemberId::Ident(node) => node.into(),
      TsEnumMemberId::Str(node) => node.into(),
    }
  }
}

fn get_view_for_ts_enum_member_id(ref_node: &'static swc_ecma_ast::TsEnumMemberId) -> TsEnumMemberId {
  match ref_node {
    swc_ecma_ast::TsEnumMemberId::Ident(value) => TsEnumMemberId::Ident(get_view_for_ident(value)),
    swc_ecma_ast::TsEnumMemberId::Str(value) => TsEnumMemberId::Str(get_view_for_str(value)),
  }
}

fn set_parent_for_ts_enum_member_id(node: &mut TsEnumMemberId, parent: Node) {
  match node {
    TsEnumMemberId::Ident(node) => {
      node.parent = parent;
    },
    TsEnumMemberId::Str(node) => {
      node.parent = parent;
    },
  }
}

pub enum TsParamPropParam {
  Ident(Ident),
  Assign(AssignPat),
}

impl Spanned for TsParamPropParam {
  fn span(&self) -> Span {
    match self {
      TsParamPropParam::Ident(node) => node.span(),
      TsParamPropParam::Assign(node) => node.span(),
    }
  }
}

impl NodeTrait for TsParamPropParam {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsParamPropParam::Ident(node) => node.parent(),
      TsParamPropParam::Assign(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsParamPropParam::Ident(node) => node.children(),
      TsParamPropParam::Assign(node) => node.children(),
    }
  }
}
impl From<&TsParamPropParam> for Node {
  fn from(node: &TsParamPropParam) -> Node {
    match node {
      TsParamPropParam::Ident(node) => node.into(),
      TsParamPropParam::Assign(node) => node.into(),
    }
  }
}

fn get_view_for_ts_param_prop_param(ref_node: &'static swc_ecma_ast::TsParamPropParam) -> TsParamPropParam {
  match ref_node {
    swc_ecma_ast::TsParamPropParam::Ident(value) => TsParamPropParam::Ident(get_view_for_ident(value)),
    swc_ecma_ast::TsParamPropParam::Assign(value) => TsParamPropParam::Assign(get_view_for_assign_pat(value)),
  }
}

fn set_parent_for_ts_param_prop_param(node: &mut TsParamPropParam, parent: Node) {
  match node {
    TsParamPropParam::Ident(node) => {
      node.parent = parent;
    },
    TsParamPropParam::Assign(node) => {
      node.parent = parent;
    },
  }
}

pub enum JSXElementChild {
  JSXText(JSXText),
  JSXExprContainer(JSXExprContainer),
  JSXSpreadChild(JSXSpreadChild),
  JSXElement(Box<JSXElement>),
  JSXFragment(JSXFragment),
}

impl Spanned for JSXElementChild {
  fn span(&self) -> Span {
    match self {
      JSXElementChild::JSXText(node) => node.span(),
      JSXElementChild::JSXExprContainer(node) => node.span(),
      JSXElementChild::JSXSpreadChild(node) => node.span(),
      JSXElementChild::JSXElement(node) => node.span(),
      JSXElementChild::JSXFragment(node) => node.span(),
    }
  }
}

impl NodeTrait for JSXElementChild {
  fn parent(&self) -> Option<&Node> {
    match self {
      JSXElementChild::JSXText(node) => node.parent(),
      JSXElementChild::JSXExprContainer(node) => node.parent(),
      JSXElementChild::JSXSpreadChild(node) => node.parent(),
      JSXElementChild::JSXElement(node) => node.parent(),
      JSXElementChild::JSXFragment(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      JSXElementChild::JSXText(node) => node.children(),
      JSXElementChild::JSXExprContainer(node) => node.children(),
      JSXElementChild::JSXSpreadChild(node) => node.children(),
      JSXElementChild::JSXElement(node) => node.children(),
      JSXElementChild::JSXFragment(node) => node.children(),
    }
  }
}
impl From<&JSXElementChild> for Node {
  fn from(node: &JSXElementChild) -> Node {
    match node {
      JSXElementChild::JSXText(node) => node.into(),
      JSXElementChild::JSXExprContainer(node) => node.into(),
      JSXElementChild::JSXSpreadChild(node) => node.into(),
      JSXElementChild::JSXElement(node) => node.into(),
      JSXElementChild::JSXFragment(node) => node.into(),
    }
  }
}

fn get_view_for_jsxelement_child(ref_node: &'static swc_ecma_ast::JSXElementChild) -> JSXElementChild {
  match ref_node {
    swc_ecma_ast::JSXElementChild::JSXText(value) => JSXElementChild::JSXText(get_view_for_jsxtext(value)),
    swc_ecma_ast::JSXElementChild::JSXExprContainer(value) => JSXElementChild::JSXExprContainer(get_view_for_jsxexpr_container(value)),
    swc_ecma_ast::JSXElementChild::JSXSpreadChild(value) => JSXElementChild::JSXSpreadChild(get_view_for_jsxspread_child(value)),
    swc_ecma_ast::JSXElementChild::JSXElement(value) => JSXElementChild::JSXElement(Box::new(get_view_for_jsxelement(value))),
    swc_ecma_ast::JSXElementChild::JSXFragment(value) => JSXElementChild::JSXFragment(get_view_for_jsxfragment(value)),
  }
}

fn set_parent_for_jsxelement_child(node: &mut JSXElementChild, parent: Node) {
  match node {
    JSXElementChild::JSXText(node) => {
      node.parent = parent;
    },
    JSXElementChild::JSXExprContainer(node) => {
      node.parent = parent;
    },
    JSXElementChild::JSXSpreadChild(node) => {
      node.parent = parent;
    },
    JSXElementChild::JSXElement(node) => {
      node.parent = parent;
    },
    JSXElementChild::JSXFragment(node) => {
      node.parent = parent;
    },
  }
}

pub enum ModuleItem {
  ModuleDecl(ModuleDecl),
  Stmt(Stmt),
}

impl Spanned for ModuleItem {
  fn span(&self) -> Span {
    match self {
      ModuleItem::ModuleDecl(node) => node.span(),
      ModuleItem::Stmt(node) => node.span(),
    }
  }
}

impl NodeTrait for ModuleItem {
  fn parent(&self) -> Option<&Node> {
    match self {
      ModuleItem::ModuleDecl(node) => node.parent(),
      ModuleItem::Stmt(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      ModuleItem::ModuleDecl(node) => node.children(),
      ModuleItem::Stmt(node) => node.children(),
    }
  }
}
impl From<&ModuleItem> for Node {
  fn from(node: &ModuleItem) -> Node {
    match node {
      ModuleItem::ModuleDecl(node) => node.into(),
      ModuleItem::Stmt(node) => node.into(),
    }
  }
}

fn get_view_for_module_item(ref_node: &'static swc_ecma_ast::ModuleItem) -> ModuleItem {
  match ref_node {
    swc_ecma_ast::ModuleItem::ModuleDecl(value) => ModuleItem::ModuleDecl(get_view_for_module_decl(value)),
    swc_ecma_ast::ModuleItem::Stmt(value) => ModuleItem::Stmt(get_view_for_stmt(value)),
  }
}

fn set_parent_for_module_item(node: &mut ModuleItem, parent: Node) {
  match node {
    ModuleItem::ModuleDecl(node) => {
      set_parent_for_module_decl(node, parent);
    },
    ModuleItem::Stmt(node) => {
      set_parent_for_stmt(node, parent);
    },
  }
}

pub enum PropName {
  Ident(Ident),
  /// String literal.
  Str(Str),
  /// Numeric literal.
  Num(Number),
  Computed(ComputedPropName),
  BigInt(BigInt),
}

impl Spanned for PropName {
  fn span(&self) -> Span {
    match self {
      PropName::Ident(node) => node.span(),
      PropName::Str(node) => node.span(),
      PropName::Num(node) => node.span(),
      PropName::Computed(node) => node.span(),
      PropName::BigInt(node) => node.span(),
    }
  }
}

impl NodeTrait for PropName {
  fn parent(&self) -> Option<&Node> {
    match self {
      PropName::Ident(node) => node.parent(),
      PropName::Str(node) => node.parent(),
      PropName::Num(node) => node.parent(),
      PropName::Computed(node) => node.parent(),
      PropName::BigInt(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      PropName::Ident(node) => node.children(),
      PropName::Str(node) => node.children(),
      PropName::Num(node) => node.children(),
      PropName::Computed(node) => node.children(),
      PropName::BigInt(node) => node.children(),
    }
  }
}
impl From<&PropName> for Node {
  fn from(node: &PropName) -> Node {
    match node {
      PropName::Ident(node) => node.into(),
      PropName::Str(node) => node.into(),
      PropName::Num(node) => node.into(),
      PropName::Computed(node) => node.into(),
      PropName::BigInt(node) => node.into(),
    }
  }
}

fn get_view_for_prop_name(ref_node: &'static swc_ecma_ast::PropName) -> PropName {
  match ref_node {
    swc_ecma_ast::PropName::Ident(value) => PropName::Ident(get_view_for_ident(value)),
    swc_ecma_ast::PropName::Str(value) => PropName::Str(get_view_for_str(value)),
    swc_ecma_ast::PropName::Num(value) => PropName::Num(get_view_for_number(value)),
    swc_ecma_ast::PropName::Computed(value) => PropName::Computed(get_view_for_computed_prop_name(value)),
    swc_ecma_ast::PropName::BigInt(value) => PropName::BigInt(get_view_for_big_int(value)),
  }
}

fn set_parent_for_prop_name(node: &mut PropName, parent: Node) {
  match node {
    PropName::Ident(node) => {
      node.parent = parent;
    },
    PropName::Str(node) => {
      node.parent = parent;
    },
    PropName::Num(node) => {
      node.parent = parent;
    },
    PropName::Computed(node) => {
      node.parent = parent;
    },
    PropName::BigInt(node) => {
      node.parent = parent;
    },
  }
}

pub enum JSXAttrName {
  Ident(Ident),
  JSXNamespacedName(JSXNamespacedName),
}

impl Spanned for JSXAttrName {
  fn span(&self) -> Span {
    match self {
      JSXAttrName::Ident(node) => node.span(),
      JSXAttrName::JSXNamespacedName(node) => node.span(),
    }
  }
}

impl NodeTrait for JSXAttrName {
  fn parent(&self) -> Option<&Node> {
    match self {
      JSXAttrName::Ident(node) => node.parent(),
      JSXAttrName::JSXNamespacedName(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      JSXAttrName::Ident(node) => node.children(),
      JSXAttrName::JSXNamespacedName(node) => node.children(),
    }
  }
}
impl From<&JSXAttrName> for Node {
  fn from(node: &JSXAttrName) -> Node {
    match node {
      JSXAttrName::Ident(node) => node.into(),
      JSXAttrName::JSXNamespacedName(node) => node.into(),
    }
  }
}

fn get_view_for_jsxattr_name(ref_node: &'static swc_ecma_ast::JSXAttrName) -> JSXAttrName {
  match ref_node {
    swc_ecma_ast::JSXAttrName::Ident(value) => JSXAttrName::Ident(get_view_for_ident(value)),
    swc_ecma_ast::JSXAttrName::JSXNamespacedName(value) => JSXAttrName::JSXNamespacedName(get_view_for_jsxnamespaced_name(value)),
  }
}

fn set_parent_for_jsxattr_name(node: &mut JSXAttrName, parent: Node) {
  match node {
    JSXAttrName::Ident(node) => {
      node.parent = parent;
    },
    JSXAttrName::JSXNamespacedName(node) => {
      node.parent = parent;
    },
  }
}

pub enum Decl {
  Class(ClassDecl),
  Fn(FnDecl),
  Var(VarDecl),
  TsInterface(TsInterfaceDecl),
  TsTypeAlias(TsTypeAliasDecl),
  TsEnum(TsEnumDecl),
  TsModule(TsModuleDecl),
}

impl Spanned for Decl {
  fn span(&self) -> Span {
    match self {
      Decl::Class(node) => node.span(),
      Decl::Fn(node) => node.span(),
      Decl::Var(node) => node.span(),
      Decl::TsInterface(node) => node.span(),
      Decl::TsTypeAlias(node) => node.span(),
      Decl::TsEnum(node) => node.span(),
      Decl::TsModule(node) => node.span(),
    }
  }
}

impl NodeTrait for Decl {
  fn parent(&self) -> Option<&Node> {
    match self {
      Decl::Class(node) => node.parent(),
      Decl::Fn(node) => node.parent(),
      Decl::Var(node) => node.parent(),
      Decl::TsInterface(node) => node.parent(),
      Decl::TsTypeAlias(node) => node.parent(),
      Decl::TsEnum(node) => node.parent(),
      Decl::TsModule(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      Decl::Class(node) => node.children(),
      Decl::Fn(node) => node.children(),
      Decl::Var(node) => node.children(),
      Decl::TsInterface(node) => node.children(),
      Decl::TsTypeAlias(node) => node.children(),
      Decl::TsEnum(node) => node.children(),
      Decl::TsModule(node) => node.children(),
    }
  }
}
impl From<&Decl> for Node {
  fn from(node: &Decl) -> Node {
    match node {
      Decl::Class(node) => node.into(),
      Decl::Fn(node) => node.into(),
      Decl::Var(node) => node.into(),
      Decl::TsInterface(node) => node.into(),
      Decl::TsTypeAlias(node) => node.into(),
      Decl::TsEnum(node) => node.into(),
      Decl::TsModule(node) => node.into(),
    }
  }
}

fn get_view_for_decl(ref_node: &'static swc_ecma_ast::Decl) -> Decl {
  match ref_node {
    swc_ecma_ast::Decl::Class(value) => Decl::Class(get_view_for_class_decl(value)),
    swc_ecma_ast::Decl::Fn(value) => Decl::Fn(get_view_for_fn_decl(value)),
    swc_ecma_ast::Decl::Var(value) => Decl::Var(get_view_for_var_decl(value)),
    swc_ecma_ast::Decl::TsInterface(value) => Decl::TsInterface(get_view_for_ts_interface_decl(value)),
    swc_ecma_ast::Decl::TsTypeAlias(value) => Decl::TsTypeAlias(get_view_for_ts_type_alias_decl(value)),
    swc_ecma_ast::Decl::TsEnum(value) => Decl::TsEnum(get_view_for_ts_enum_decl(value)),
    swc_ecma_ast::Decl::TsModule(value) => Decl::TsModule(get_view_for_ts_module_decl(value)),
  }
}

fn set_parent_for_decl(node: &mut Decl, parent: Node) {
  match node {
    Decl::Class(node) => {
      node.parent = parent;
    },
    Decl::Fn(node) => {
      node.parent = parent;
    },
    Decl::Var(node) => {
      node.parent = parent;
    },
    Decl::TsInterface(node) => {
      node.parent = parent;
    },
    Decl::TsTypeAlias(node) => {
      node.parent = parent;
    },
    Decl::TsEnum(node) => {
      node.parent = parent;
    },
    Decl::TsModule(node) => {
      node.parent = parent;
    },
  }
}

pub enum TsLit {
  Number(Number),
  Str(Str),
  Bool(Bool),
  BigInt(BigInt),
  Tpl(TsTplLitType),
}

impl Spanned for TsLit {
  fn span(&self) -> Span {
    match self {
      TsLit::Number(node) => node.span(),
      TsLit::Str(node) => node.span(),
      TsLit::Bool(node) => node.span(),
      TsLit::BigInt(node) => node.span(),
      TsLit::Tpl(node) => node.span(),
    }
  }
}

impl NodeTrait for TsLit {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsLit::Number(node) => node.parent(),
      TsLit::Str(node) => node.parent(),
      TsLit::Bool(node) => node.parent(),
      TsLit::BigInt(node) => node.parent(),
      TsLit::Tpl(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsLit::Number(node) => node.children(),
      TsLit::Str(node) => node.children(),
      TsLit::Bool(node) => node.children(),
      TsLit::BigInt(node) => node.children(),
      TsLit::Tpl(node) => node.children(),
    }
  }
}
impl From<&TsLit> for Node {
  fn from(node: &TsLit) -> Node {
    match node {
      TsLit::Number(node) => node.into(),
      TsLit::Str(node) => node.into(),
      TsLit::Bool(node) => node.into(),
      TsLit::BigInt(node) => node.into(),
      TsLit::Tpl(node) => node.into(),
    }
  }
}

fn get_view_for_ts_lit(ref_node: &'static swc_ecma_ast::TsLit) -> TsLit {
  match ref_node {
    swc_ecma_ast::TsLit::Number(value) => TsLit::Number(get_view_for_number(value)),
    swc_ecma_ast::TsLit::Str(value) => TsLit::Str(get_view_for_str(value)),
    swc_ecma_ast::TsLit::Bool(value) => TsLit::Bool(get_view_for_bool(value)),
    swc_ecma_ast::TsLit::BigInt(value) => TsLit::BigInt(get_view_for_big_int(value)),
    swc_ecma_ast::TsLit::Tpl(value) => TsLit::Tpl(get_view_for_ts_tpl_lit_type(value)),
  }
}

fn set_parent_for_ts_lit(node: &mut TsLit, parent: Node) {
  match node {
    TsLit::Number(node) => {
      node.parent = parent;
    },
    TsLit::Str(node) => {
      node.parent = parent;
    },
    TsLit::Bool(node) => {
      node.parent = parent;
    },
    TsLit::BigInt(node) => {
      node.parent = parent;
    },
    TsLit::Tpl(node) => {
      node.parent = parent;
    },
  }
}

pub enum TsEntityName {
  TsQualifiedName(Box<TsQualifiedName>),
  Ident(Ident),
}

impl Spanned for TsEntityName {
  fn span(&self) -> Span {
    match self {
      TsEntityName::TsQualifiedName(node) => node.span(),
      TsEntityName::Ident(node) => node.span(),
    }
  }
}

impl NodeTrait for TsEntityName {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsEntityName::TsQualifiedName(node) => node.parent(),
      TsEntityName::Ident(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsEntityName::TsQualifiedName(node) => node.children(),
      TsEntityName::Ident(node) => node.children(),
    }
  }
}
impl From<&TsEntityName> for Node {
  fn from(node: &TsEntityName) -> Node {
    match node {
      TsEntityName::TsQualifiedName(node) => node.into(),
      TsEntityName::Ident(node) => node.into(),
    }
  }
}

fn get_view_for_ts_entity_name(ref_node: &'static swc_ecma_ast::TsEntityName) -> TsEntityName {
  match ref_node {
    swc_ecma_ast::TsEntityName::TsQualifiedName(value) => TsEntityName::TsQualifiedName(Box::new(get_view_for_ts_qualified_name(value))),
    swc_ecma_ast::TsEntityName::Ident(value) => TsEntityName::Ident(get_view_for_ident(value)),
  }
}

fn set_parent_for_ts_entity_name(node: &mut TsEntityName, parent: Node) {
  match node {
    TsEntityName::TsQualifiedName(node) => {
      node.parent = parent;
    },
    TsEntityName::Ident(node) => {
      node.parent = parent;
    },
  }
}

pub enum Expr {
  This(ThisExpr),
  Array(ArrayLit),
  Object(ObjectLit),
  Fn(FnExpr),
  Unary(UnaryExpr),
  /// `++v`, `--v`, `v++`, `v--`
  Update(UpdateExpr),
  Bin(BinExpr),
  Assign(AssignExpr),
  /// A member expression. If computed is true, the node corresponds to a
  /// computed (a[b]) member expression and property is an Expression. If
  /// computed is false, the node corresponds to a static (a.b) member
  /// expression and property is an Identifier.
  Member(MemberExpr),
  /// true ? 'a' : 'b'
  Cond(CondExpr),
  Call(CallExpr),
  /// `new Cat()`
  New(NewExpr),
  Seq(SeqExpr),
  Ident(Ident),
  Lit(Lit),
  Tpl(Tpl),
  TaggedTpl(TaggedTpl),
  Arrow(ArrowExpr),
  Class(ClassExpr),
  Yield(YieldExpr),
  MetaProp(MetaPropExpr),
  Await(AwaitExpr),
  Paren(ParenExpr),
  JSXMember(JSXMemberExpr),
  JSXNamespacedName(JSXNamespacedName),
  JSXEmpty(JSXEmptyExpr),
  JSXElement(Box<JSXElement>),
  JSXFragment(JSXFragment),
  TsTypeAssertion(TsTypeAssertion),
  TsConstAssertion(TsConstAssertion),
  TsNonNull(TsNonNullExpr),
  TsTypeCast(TsTypeCastExpr),
  TsAs(TsAsExpr),
  PrivateName(PrivateName),
  OptChain(OptChainExpr),
  Invalid(Invalid),
}

impl Spanned for Expr {
  fn span(&self) -> Span {
    match self {
      Expr::This(node) => node.span(),
      Expr::Array(node) => node.span(),
      Expr::Object(node) => node.span(),
      Expr::Fn(node) => node.span(),
      Expr::Unary(node) => node.span(),
      Expr::Update(node) => node.span(),
      Expr::Bin(node) => node.span(),
      Expr::Assign(node) => node.span(),
      Expr::Member(node) => node.span(),
      Expr::Cond(node) => node.span(),
      Expr::Call(node) => node.span(),
      Expr::New(node) => node.span(),
      Expr::Seq(node) => node.span(),
      Expr::Ident(node) => node.span(),
      Expr::Lit(node) => node.span(),
      Expr::Tpl(node) => node.span(),
      Expr::TaggedTpl(node) => node.span(),
      Expr::Arrow(node) => node.span(),
      Expr::Class(node) => node.span(),
      Expr::Yield(node) => node.span(),
      Expr::MetaProp(node) => node.span(),
      Expr::Await(node) => node.span(),
      Expr::Paren(node) => node.span(),
      Expr::JSXMember(node) => node.span(),
      Expr::JSXNamespacedName(node) => node.span(),
      Expr::JSXEmpty(node) => node.span(),
      Expr::JSXElement(node) => node.span(),
      Expr::JSXFragment(node) => node.span(),
      Expr::TsTypeAssertion(node) => node.span(),
      Expr::TsConstAssertion(node) => node.span(),
      Expr::TsNonNull(node) => node.span(),
      Expr::TsTypeCast(node) => node.span(),
      Expr::TsAs(node) => node.span(),
      Expr::PrivateName(node) => node.span(),
      Expr::OptChain(node) => node.span(),
      Expr::Invalid(node) => node.span(),
    }
  }
}

impl NodeTrait for Expr {
  fn parent(&self) -> Option<&Node> {
    match self {
      Expr::This(node) => node.parent(),
      Expr::Array(node) => node.parent(),
      Expr::Object(node) => node.parent(),
      Expr::Fn(node) => node.parent(),
      Expr::Unary(node) => node.parent(),
      Expr::Update(node) => node.parent(),
      Expr::Bin(node) => node.parent(),
      Expr::Assign(node) => node.parent(),
      Expr::Member(node) => node.parent(),
      Expr::Cond(node) => node.parent(),
      Expr::Call(node) => node.parent(),
      Expr::New(node) => node.parent(),
      Expr::Seq(node) => node.parent(),
      Expr::Ident(node) => node.parent(),
      Expr::Lit(node) => node.parent(),
      Expr::Tpl(node) => node.parent(),
      Expr::TaggedTpl(node) => node.parent(),
      Expr::Arrow(node) => node.parent(),
      Expr::Class(node) => node.parent(),
      Expr::Yield(node) => node.parent(),
      Expr::MetaProp(node) => node.parent(),
      Expr::Await(node) => node.parent(),
      Expr::Paren(node) => node.parent(),
      Expr::JSXMember(node) => node.parent(),
      Expr::JSXNamespacedName(node) => node.parent(),
      Expr::JSXEmpty(node) => node.parent(),
      Expr::JSXElement(node) => node.parent(),
      Expr::JSXFragment(node) => node.parent(),
      Expr::TsTypeAssertion(node) => node.parent(),
      Expr::TsConstAssertion(node) => node.parent(),
      Expr::TsNonNull(node) => node.parent(),
      Expr::TsTypeCast(node) => node.parent(),
      Expr::TsAs(node) => node.parent(),
      Expr::PrivateName(node) => node.parent(),
      Expr::OptChain(node) => node.parent(),
      Expr::Invalid(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      Expr::This(node) => node.children(),
      Expr::Array(node) => node.children(),
      Expr::Object(node) => node.children(),
      Expr::Fn(node) => node.children(),
      Expr::Unary(node) => node.children(),
      Expr::Update(node) => node.children(),
      Expr::Bin(node) => node.children(),
      Expr::Assign(node) => node.children(),
      Expr::Member(node) => node.children(),
      Expr::Cond(node) => node.children(),
      Expr::Call(node) => node.children(),
      Expr::New(node) => node.children(),
      Expr::Seq(node) => node.children(),
      Expr::Ident(node) => node.children(),
      Expr::Lit(node) => node.children(),
      Expr::Tpl(node) => node.children(),
      Expr::TaggedTpl(node) => node.children(),
      Expr::Arrow(node) => node.children(),
      Expr::Class(node) => node.children(),
      Expr::Yield(node) => node.children(),
      Expr::MetaProp(node) => node.children(),
      Expr::Await(node) => node.children(),
      Expr::Paren(node) => node.children(),
      Expr::JSXMember(node) => node.children(),
      Expr::JSXNamespacedName(node) => node.children(),
      Expr::JSXEmpty(node) => node.children(),
      Expr::JSXElement(node) => node.children(),
      Expr::JSXFragment(node) => node.children(),
      Expr::TsTypeAssertion(node) => node.children(),
      Expr::TsConstAssertion(node) => node.children(),
      Expr::TsNonNull(node) => node.children(),
      Expr::TsTypeCast(node) => node.children(),
      Expr::TsAs(node) => node.children(),
      Expr::PrivateName(node) => node.children(),
      Expr::OptChain(node) => node.children(),
      Expr::Invalid(node) => node.children(),
    }
  }
}
impl From<&Expr> for Node {
  fn from(node: &Expr) -> Node {
    match node {
      Expr::This(node) => node.into(),
      Expr::Array(node) => node.into(),
      Expr::Object(node) => node.into(),
      Expr::Fn(node) => node.into(),
      Expr::Unary(node) => node.into(),
      Expr::Update(node) => node.into(),
      Expr::Bin(node) => node.into(),
      Expr::Assign(node) => node.into(),
      Expr::Member(node) => node.into(),
      Expr::Cond(node) => node.into(),
      Expr::Call(node) => node.into(),
      Expr::New(node) => node.into(),
      Expr::Seq(node) => node.into(),
      Expr::Ident(node) => node.into(),
      Expr::Lit(node) => node.into(),
      Expr::Tpl(node) => node.into(),
      Expr::TaggedTpl(node) => node.into(),
      Expr::Arrow(node) => node.into(),
      Expr::Class(node) => node.into(),
      Expr::Yield(node) => node.into(),
      Expr::MetaProp(node) => node.into(),
      Expr::Await(node) => node.into(),
      Expr::Paren(node) => node.into(),
      Expr::JSXMember(node) => node.into(),
      Expr::JSXNamespacedName(node) => node.into(),
      Expr::JSXEmpty(node) => node.into(),
      Expr::JSXElement(node) => node.into(),
      Expr::JSXFragment(node) => node.into(),
      Expr::TsTypeAssertion(node) => node.into(),
      Expr::TsConstAssertion(node) => node.into(),
      Expr::TsNonNull(node) => node.into(),
      Expr::TsTypeCast(node) => node.into(),
      Expr::TsAs(node) => node.into(),
      Expr::PrivateName(node) => node.into(),
      Expr::OptChain(node) => node.into(),
      Expr::Invalid(node) => node.into(),
    }
  }
}

fn get_view_for_expr(ref_node: &'static swc_ecma_ast::Expr) -> Expr {
  match ref_node {
    swc_ecma_ast::Expr::This(value) => Expr::This(get_view_for_this_expr(value)),
    swc_ecma_ast::Expr::Array(value) => Expr::Array(get_view_for_array_lit(value)),
    swc_ecma_ast::Expr::Object(value) => Expr::Object(get_view_for_object_lit(value)),
    swc_ecma_ast::Expr::Fn(value) => Expr::Fn(get_view_for_fn_expr(value)),
    swc_ecma_ast::Expr::Unary(value) => Expr::Unary(get_view_for_unary_expr(value)),
    swc_ecma_ast::Expr::Update(value) => Expr::Update(get_view_for_update_expr(value)),
    swc_ecma_ast::Expr::Bin(value) => Expr::Bin(get_view_for_bin_expr(value)),
    swc_ecma_ast::Expr::Assign(value) => Expr::Assign(get_view_for_assign_expr(value)),
    swc_ecma_ast::Expr::Member(value) => Expr::Member(get_view_for_member_expr(value)),
    swc_ecma_ast::Expr::Cond(value) => Expr::Cond(get_view_for_cond_expr(value)),
    swc_ecma_ast::Expr::Call(value) => Expr::Call(get_view_for_call_expr(value)),
    swc_ecma_ast::Expr::New(value) => Expr::New(get_view_for_new_expr(value)),
    swc_ecma_ast::Expr::Seq(value) => Expr::Seq(get_view_for_seq_expr(value)),
    swc_ecma_ast::Expr::Ident(value) => Expr::Ident(get_view_for_ident(value)),
    swc_ecma_ast::Expr::Lit(value) => Expr::Lit(get_view_for_lit(value)),
    swc_ecma_ast::Expr::Tpl(value) => Expr::Tpl(get_view_for_tpl(value)),
    swc_ecma_ast::Expr::TaggedTpl(value) => Expr::TaggedTpl(get_view_for_tagged_tpl(value)),
    swc_ecma_ast::Expr::Arrow(value) => Expr::Arrow(get_view_for_arrow_expr(value)),
    swc_ecma_ast::Expr::Class(value) => Expr::Class(get_view_for_class_expr(value)),
    swc_ecma_ast::Expr::Yield(value) => Expr::Yield(get_view_for_yield_expr(value)),
    swc_ecma_ast::Expr::MetaProp(value) => Expr::MetaProp(get_view_for_meta_prop_expr(value)),
    swc_ecma_ast::Expr::Await(value) => Expr::Await(get_view_for_await_expr(value)),
    swc_ecma_ast::Expr::Paren(value) => Expr::Paren(get_view_for_paren_expr(value)),
    swc_ecma_ast::Expr::JSXMember(value) => Expr::JSXMember(get_view_for_jsxmember_expr(value)),
    swc_ecma_ast::Expr::JSXNamespacedName(value) => Expr::JSXNamespacedName(get_view_for_jsxnamespaced_name(value)),
    swc_ecma_ast::Expr::JSXEmpty(value) => Expr::JSXEmpty(get_view_for_jsxempty_expr(value)),
    swc_ecma_ast::Expr::JSXElement(value) => Expr::JSXElement(Box::new(get_view_for_jsxelement(value))),
    swc_ecma_ast::Expr::JSXFragment(value) => Expr::JSXFragment(get_view_for_jsxfragment(value)),
    swc_ecma_ast::Expr::TsTypeAssertion(value) => Expr::TsTypeAssertion(get_view_for_ts_type_assertion(value)),
    swc_ecma_ast::Expr::TsConstAssertion(value) => Expr::TsConstAssertion(get_view_for_ts_const_assertion(value)),
    swc_ecma_ast::Expr::TsNonNull(value) => Expr::TsNonNull(get_view_for_ts_non_null_expr(value)),
    swc_ecma_ast::Expr::TsTypeCast(value) => Expr::TsTypeCast(get_view_for_ts_type_cast_expr(value)),
    swc_ecma_ast::Expr::TsAs(value) => Expr::TsAs(get_view_for_ts_as_expr(value)),
    swc_ecma_ast::Expr::PrivateName(value) => Expr::PrivateName(get_view_for_private_name(value)),
    swc_ecma_ast::Expr::OptChain(value) => Expr::OptChain(get_view_for_opt_chain_expr(value)),
    swc_ecma_ast::Expr::Invalid(value) => Expr::Invalid(get_view_for_invalid(value)),
  }
}

fn set_parent_for_expr(node: &mut Expr, parent: Node) {
  match node {
    Expr::This(node) => {
      node.parent = parent;
    },
    Expr::Array(node) => {
      node.parent = parent;
    },
    Expr::Object(node) => {
      node.parent = parent;
    },
    Expr::Fn(node) => {
      node.parent = parent;
    },
    Expr::Unary(node) => {
      node.parent = parent;
    },
    Expr::Update(node) => {
      node.parent = parent;
    },
    Expr::Bin(node) => {
      node.parent = parent;
    },
    Expr::Assign(node) => {
      node.parent = parent;
    },
    Expr::Member(node) => {
      node.parent = parent;
    },
    Expr::Cond(node) => {
      node.parent = parent;
    },
    Expr::Call(node) => {
      node.parent = parent;
    },
    Expr::New(node) => {
      node.parent = parent;
    },
    Expr::Seq(node) => {
      node.parent = parent;
    },
    Expr::Ident(node) => {
      node.parent = parent;
    },
    Expr::Lit(node) => {
      set_parent_for_lit(node, parent);
    },
    Expr::Tpl(node) => {
      node.parent = parent;
    },
    Expr::TaggedTpl(node) => {
      node.parent = parent;
    },
    Expr::Arrow(node) => {
      node.parent = parent;
    },
    Expr::Class(node) => {
      node.parent = parent;
    },
    Expr::Yield(node) => {
      node.parent = parent;
    },
    Expr::MetaProp(node) => {
      node.parent = parent;
    },
    Expr::Await(node) => {
      node.parent = parent;
    },
    Expr::Paren(node) => {
      node.parent = parent;
    },
    Expr::JSXMember(node) => {
      node.parent = parent;
    },
    Expr::JSXNamespacedName(node) => {
      node.parent = parent;
    },
    Expr::JSXEmpty(node) => {
      node.parent = parent;
    },
    Expr::JSXElement(node) => {
      node.parent = parent;
    },
    Expr::JSXFragment(node) => {
      node.parent = parent;
    },
    Expr::TsTypeAssertion(node) => {
      node.parent = parent;
    },
    Expr::TsConstAssertion(node) => {
      node.parent = parent;
    },
    Expr::TsNonNull(node) => {
      node.parent = parent;
    },
    Expr::TsTypeCast(node) => {
      node.parent = parent;
    },
    Expr::TsAs(node) => {
      node.parent = parent;
    },
    Expr::PrivateName(node) => {
      node.parent = parent;
    },
    Expr::OptChain(node) => {
      node.parent = parent;
    },
    Expr::Invalid(node) => {
      node.parent = parent;
    },
  }
}

/// Used for `obj` property of `JSXMemberExpr`.
pub enum JSXObject {
  JSXMemberExpr(Box<JSXMemberExpr>),
  Ident(Ident),
}

impl Spanned for JSXObject {
  fn span(&self) -> Span {
    match self {
      JSXObject::JSXMemberExpr(node) => node.span(),
      JSXObject::Ident(node) => node.span(),
    }
  }
}

impl NodeTrait for JSXObject {
  fn parent(&self) -> Option<&Node> {
    match self {
      JSXObject::JSXMemberExpr(node) => node.parent(),
      JSXObject::Ident(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      JSXObject::JSXMemberExpr(node) => node.children(),
      JSXObject::Ident(node) => node.children(),
    }
  }
}
impl From<&JSXObject> for Node {
  fn from(node: &JSXObject) -> Node {
    match node {
      JSXObject::JSXMemberExpr(node) => node.into(),
      JSXObject::Ident(node) => node.into(),
    }
  }
}

fn get_view_for_jsxobject(ref_node: &'static swc_ecma_ast::JSXObject) -> JSXObject {
  match ref_node {
    swc_ecma_ast::JSXObject::JSXMemberExpr(value) => JSXObject::JSXMemberExpr(Box::new(get_view_for_jsxmember_expr(value))),
    swc_ecma_ast::JSXObject::Ident(value) => JSXObject::Ident(get_view_for_ident(value)),
  }
}

fn set_parent_for_jsxobject(node: &mut JSXObject, parent: Node) {
  match node {
    JSXObject::JSXMemberExpr(node) => {
      node.parent = parent;
    },
    JSXObject::Ident(node) => {
      node.parent = parent;
    },
  }
}

pub enum PatOrExpr {
  Expr(Box<Expr>),
  Pat(Box<Pat>),
}

impl Spanned for PatOrExpr {
  fn span(&self) -> Span {
    match self {
      PatOrExpr::Expr(node) => node.span(),
      PatOrExpr::Pat(node) => node.span(),
    }
  }
}

impl NodeTrait for PatOrExpr {
  fn parent(&self) -> Option<&Node> {
    match self {
      PatOrExpr::Expr(node) => node.parent(),
      PatOrExpr::Pat(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      PatOrExpr::Expr(node) => node.children(),
      PatOrExpr::Pat(node) => node.children(),
    }
  }
}
impl From<&PatOrExpr> for Node {
  fn from(node: &PatOrExpr) -> Node {
    match node {
      PatOrExpr::Expr(node) => node.into(),
      PatOrExpr::Pat(node) => node.into(),
    }
  }
}

fn get_view_for_pat_or_expr(ref_node: &'static swc_ecma_ast::PatOrExpr) -> PatOrExpr {
  match ref_node {
    swc_ecma_ast::PatOrExpr::Expr(value) => PatOrExpr::Expr(Box::new(get_view_for_expr(value))),
    swc_ecma_ast::PatOrExpr::Pat(value) => PatOrExpr::Pat(Box::new(get_view_for_pat(value))),
  }
}

fn set_parent_for_pat_or_expr(node: &mut PatOrExpr, parent: Node) {
  match node {
    PatOrExpr::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
    PatOrExpr::Pat(node) => {
      set_parent_for_pat(node, parent);
    },
  }
}

pub enum ModuleDecl {
  Import(ImportDecl),
  ExportDecl(ExportDecl),
  ExportNamed(NamedExport),
  ExportDefaultDecl(ExportDefaultDecl),
  ExportDefaultExpr(ExportDefaultExpr),
  ExportAll(ExportAll),
  TsImportEquals(TsImportEqualsDecl),
  TsExportAssignment(TsExportAssignment),
  TsNamespaceExport(TsNamespaceExportDecl),
}

impl Spanned for ModuleDecl {
  fn span(&self) -> Span {
    match self {
      ModuleDecl::Import(node) => node.span(),
      ModuleDecl::ExportDecl(node) => node.span(),
      ModuleDecl::ExportNamed(node) => node.span(),
      ModuleDecl::ExportDefaultDecl(node) => node.span(),
      ModuleDecl::ExportDefaultExpr(node) => node.span(),
      ModuleDecl::ExportAll(node) => node.span(),
      ModuleDecl::TsImportEquals(node) => node.span(),
      ModuleDecl::TsExportAssignment(node) => node.span(),
      ModuleDecl::TsNamespaceExport(node) => node.span(),
    }
  }
}

impl NodeTrait for ModuleDecl {
  fn parent(&self) -> Option<&Node> {
    match self {
      ModuleDecl::Import(node) => node.parent(),
      ModuleDecl::ExportDecl(node) => node.parent(),
      ModuleDecl::ExportNamed(node) => node.parent(),
      ModuleDecl::ExportDefaultDecl(node) => node.parent(),
      ModuleDecl::ExportDefaultExpr(node) => node.parent(),
      ModuleDecl::ExportAll(node) => node.parent(),
      ModuleDecl::TsImportEquals(node) => node.parent(),
      ModuleDecl::TsExportAssignment(node) => node.parent(),
      ModuleDecl::TsNamespaceExport(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      ModuleDecl::Import(node) => node.children(),
      ModuleDecl::ExportDecl(node) => node.children(),
      ModuleDecl::ExportNamed(node) => node.children(),
      ModuleDecl::ExportDefaultDecl(node) => node.children(),
      ModuleDecl::ExportDefaultExpr(node) => node.children(),
      ModuleDecl::ExportAll(node) => node.children(),
      ModuleDecl::TsImportEquals(node) => node.children(),
      ModuleDecl::TsExportAssignment(node) => node.children(),
      ModuleDecl::TsNamespaceExport(node) => node.children(),
    }
  }
}
impl From<&ModuleDecl> for Node {
  fn from(node: &ModuleDecl) -> Node {
    match node {
      ModuleDecl::Import(node) => node.into(),
      ModuleDecl::ExportDecl(node) => node.into(),
      ModuleDecl::ExportNamed(node) => node.into(),
      ModuleDecl::ExportDefaultDecl(node) => node.into(),
      ModuleDecl::ExportDefaultExpr(node) => node.into(),
      ModuleDecl::ExportAll(node) => node.into(),
      ModuleDecl::TsImportEquals(node) => node.into(),
      ModuleDecl::TsExportAssignment(node) => node.into(),
      ModuleDecl::TsNamespaceExport(node) => node.into(),
    }
  }
}

fn get_view_for_module_decl(ref_node: &'static swc_ecma_ast::ModuleDecl) -> ModuleDecl {
  match ref_node {
    swc_ecma_ast::ModuleDecl::Import(value) => ModuleDecl::Import(get_view_for_import_decl(value)),
    swc_ecma_ast::ModuleDecl::ExportDecl(value) => ModuleDecl::ExportDecl(get_view_for_export_decl(value)),
    swc_ecma_ast::ModuleDecl::ExportNamed(value) => ModuleDecl::ExportNamed(get_view_for_named_export(value)),
    swc_ecma_ast::ModuleDecl::ExportDefaultDecl(value) => ModuleDecl::ExportDefaultDecl(get_view_for_export_default_decl(value)),
    swc_ecma_ast::ModuleDecl::ExportDefaultExpr(value) => ModuleDecl::ExportDefaultExpr(get_view_for_export_default_expr(value)),
    swc_ecma_ast::ModuleDecl::ExportAll(value) => ModuleDecl::ExportAll(get_view_for_export_all(value)),
    swc_ecma_ast::ModuleDecl::TsImportEquals(value) => ModuleDecl::TsImportEquals(get_view_for_ts_import_equals_decl(value)),
    swc_ecma_ast::ModuleDecl::TsExportAssignment(value) => ModuleDecl::TsExportAssignment(get_view_for_ts_export_assignment(value)),
    swc_ecma_ast::ModuleDecl::TsNamespaceExport(value) => ModuleDecl::TsNamespaceExport(get_view_for_ts_namespace_export_decl(value)),
  }
}

fn set_parent_for_module_decl(node: &mut ModuleDecl, parent: Node) {
  match node {
    ModuleDecl::Import(node) => {
      node.parent = parent;
    },
    ModuleDecl::ExportDecl(node) => {
      node.parent = parent;
    },
    ModuleDecl::ExportNamed(node) => {
      node.parent = parent;
    },
    ModuleDecl::ExportDefaultDecl(node) => {
      node.parent = parent;
    },
    ModuleDecl::ExportDefaultExpr(node) => {
      node.parent = parent;
    },
    ModuleDecl::ExportAll(node) => {
      node.parent = parent;
    },
    ModuleDecl::TsImportEquals(node) => {
      node.parent = parent;
    },
    ModuleDecl::TsExportAssignment(node) => {
      node.parent = parent;
    },
    ModuleDecl::TsNamespaceExport(node) => {
      node.parent = parent;
    },
  }
}

pub enum JSXElementName {
  Ident(Ident),
  JSXMemberExpr(JSXMemberExpr),
  JSXNamespacedName(JSXNamespacedName),
}

impl Spanned for JSXElementName {
  fn span(&self) -> Span {
    match self {
      JSXElementName::Ident(node) => node.span(),
      JSXElementName::JSXMemberExpr(node) => node.span(),
      JSXElementName::JSXNamespacedName(node) => node.span(),
    }
  }
}

impl NodeTrait for JSXElementName {
  fn parent(&self) -> Option<&Node> {
    match self {
      JSXElementName::Ident(node) => node.parent(),
      JSXElementName::JSXMemberExpr(node) => node.parent(),
      JSXElementName::JSXNamespacedName(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      JSXElementName::Ident(node) => node.children(),
      JSXElementName::JSXMemberExpr(node) => node.children(),
      JSXElementName::JSXNamespacedName(node) => node.children(),
    }
  }
}
impl From<&JSXElementName> for Node {
  fn from(node: &JSXElementName) -> Node {
    match node {
      JSXElementName::Ident(node) => node.into(),
      JSXElementName::JSXMemberExpr(node) => node.into(),
      JSXElementName::JSXNamespacedName(node) => node.into(),
    }
  }
}

fn get_view_for_jsxelement_name(ref_node: &'static swc_ecma_ast::JSXElementName) -> JSXElementName {
  match ref_node {
    swc_ecma_ast::JSXElementName::Ident(value) => JSXElementName::Ident(get_view_for_ident(value)),
    swc_ecma_ast::JSXElementName::JSXMemberExpr(value) => JSXElementName::JSXMemberExpr(get_view_for_jsxmember_expr(value)),
    swc_ecma_ast::JSXElementName::JSXNamespacedName(value) => JSXElementName::JSXNamespacedName(get_view_for_jsxnamespaced_name(value)),
  }
}

fn set_parent_for_jsxelement_name(node: &mut JSXElementName, parent: Node) {
  match node {
    JSXElementName::Ident(node) => {
      node.parent = parent;
    },
    JSXElementName::JSXMemberExpr(node) => {
      node.parent = parent;
    },
    JSXElementName::JSXNamespacedName(node) => {
      node.parent = parent;
    },
  }
}

pub enum JSXExpr {
  JSXEmptyExpr(JSXEmptyExpr),
  Expr(Box<Expr>),
}

impl Spanned for JSXExpr {
  fn span(&self) -> Span {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.span(),
      JSXExpr::Expr(node) => node.span(),
    }
  }
}

impl NodeTrait for JSXExpr {
  fn parent(&self) -> Option<&Node> {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.parent(),
      JSXExpr::Expr(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.children(),
      JSXExpr::Expr(node) => node.children(),
    }
  }
}
impl From<&JSXExpr> for Node {
  fn from(node: &JSXExpr) -> Node {
    match node {
      JSXExpr::JSXEmptyExpr(node) => node.into(),
      JSXExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_jsxexpr(ref_node: &'static swc_ecma_ast::JSXExpr) -> JSXExpr {
  match ref_node {
    swc_ecma_ast::JSXExpr::JSXEmptyExpr(value) => JSXExpr::JSXEmptyExpr(get_view_for_jsxempty_expr(value)),
    swc_ecma_ast::JSXExpr::Expr(value) => JSXExpr::Expr(Box::new(get_view_for_expr(value))),
  }
}

fn set_parent_for_jsxexpr(node: &mut JSXExpr, parent: Node) {
  match node {
    JSXExpr::JSXEmptyExpr(node) => {
      node.parent = parent;
    },
    JSXExpr::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
  }
}

pub enum TsType {
  TsKeywordType(TsKeywordType),
  TsThisType(TsThisType),
  TsFnOrConstructorType(TsFnOrConstructorType),
  TsTypeRef(TsTypeRef),
  TsTypeQuery(TsTypeQuery),
  TsTypeLit(TsTypeLit),
  TsArrayType(TsArrayType),
  TsTupleType(TsTupleType),
  TsOptionalType(TsOptionalType),
  TsRestType(TsRestType),
  TsUnionOrIntersectionType(TsUnionOrIntersectionType),
  TsConditionalType(TsConditionalType),
  TsInferType(TsInferType),
  TsParenthesizedType(TsParenthesizedType),
  TsTypeOperator(TsTypeOperator),
  TsIndexedAccessType(TsIndexedAccessType),
  TsMappedType(TsMappedType),
  TsLitType(TsLitType),
  TsTypePredicate(TsTypePredicate),
  TsImportType(TsImportType),
}

impl Spanned for TsType {
  fn span(&self) -> Span {
    match self {
      TsType::TsKeywordType(node) => node.span(),
      TsType::TsThisType(node) => node.span(),
      TsType::TsFnOrConstructorType(node) => node.span(),
      TsType::TsTypeRef(node) => node.span(),
      TsType::TsTypeQuery(node) => node.span(),
      TsType::TsTypeLit(node) => node.span(),
      TsType::TsArrayType(node) => node.span(),
      TsType::TsTupleType(node) => node.span(),
      TsType::TsOptionalType(node) => node.span(),
      TsType::TsRestType(node) => node.span(),
      TsType::TsUnionOrIntersectionType(node) => node.span(),
      TsType::TsConditionalType(node) => node.span(),
      TsType::TsInferType(node) => node.span(),
      TsType::TsParenthesizedType(node) => node.span(),
      TsType::TsTypeOperator(node) => node.span(),
      TsType::TsIndexedAccessType(node) => node.span(),
      TsType::TsMappedType(node) => node.span(),
      TsType::TsLitType(node) => node.span(),
      TsType::TsTypePredicate(node) => node.span(),
      TsType::TsImportType(node) => node.span(),
    }
  }
}

impl NodeTrait for TsType {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsType::TsKeywordType(node) => node.parent(),
      TsType::TsThisType(node) => node.parent(),
      TsType::TsFnOrConstructorType(node) => node.parent(),
      TsType::TsTypeRef(node) => node.parent(),
      TsType::TsTypeQuery(node) => node.parent(),
      TsType::TsTypeLit(node) => node.parent(),
      TsType::TsArrayType(node) => node.parent(),
      TsType::TsTupleType(node) => node.parent(),
      TsType::TsOptionalType(node) => node.parent(),
      TsType::TsRestType(node) => node.parent(),
      TsType::TsUnionOrIntersectionType(node) => node.parent(),
      TsType::TsConditionalType(node) => node.parent(),
      TsType::TsInferType(node) => node.parent(),
      TsType::TsParenthesizedType(node) => node.parent(),
      TsType::TsTypeOperator(node) => node.parent(),
      TsType::TsIndexedAccessType(node) => node.parent(),
      TsType::TsMappedType(node) => node.parent(),
      TsType::TsLitType(node) => node.parent(),
      TsType::TsTypePredicate(node) => node.parent(),
      TsType::TsImportType(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsType::TsKeywordType(node) => node.children(),
      TsType::TsThisType(node) => node.children(),
      TsType::TsFnOrConstructorType(node) => node.children(),
      TsType::TsTypeRef(node) => node.children(),
      TsType::TsTypeQuery(node) => node.children(),
      TsType::TsTypeLit(node) => node.children(),
      TsType::TsArrayType(node) => node.children(),
      TsType::TsTupleType(node) => node.children(),
      TsType::TsOptionalType(node) => node.children(),
      TsType::TsRestType(node) => node.children(),
      TsType::TsUnionOrIntersectionType(node) => node.children(),
      TsType::TsConditionalType(node) => node.children(),
      TsType::TsInferType(node) => node.children(),
      TsType::TsParenthesizedType(node) => node.children(),
      TsType::TsTypeOperator(node) => node.children(),
      TsType::TsIndexedAccessType(node) => node.children(),
      TsType::TsMappedType(node) => node.children(),
      TsType::TsLitType(node) => node.children(),
      TsType::TsTypePredicate(node) => node.children(),
      TsType::TsImportType(node) => node.children(),
    }
  }
}
impl From<&TsType> for Node {
  fn from(node: &TsType) -> Node {
    match node {
      TsType::TsKeywordType(node) => node.into(),
      TsType::TsThisType(node) => node.into(),
      TsType::TsFnOrConstructorType(node) => node.into(),
      TsType::TsTypeRef(node) => node.into(),
      TsType::TsTypeQuery(node) => node.into(),
      TsType::TsTypeLit(node) => node.into(),
      TsType::TsArrayType(node) => node.into(),
      TsType::TsTupleType(node) => node.into(),
      TsType::TsOptionalType(node) => node.into(),
      TsType::TsRestType(node) => node.into(),
      TsType::TsUnionOrIntersectionType(node) => node.into(),
      TsType::TsConditionalType(node) => node.into(),
      TsType::TsInferType(node) => node.into(),
      TsType::TsParenthesizedType(node) => node.into(),
      TsType::TsTypeOperator(node) => node.into(),
      TsType::TsIndexedAccessType(node) => node.into(),
      TsType::TsMappedType(node) => node.into(),
      TsType::TsLitType(node) => node.into(),
      TsType::TsTypePredicate(node) => node.into(),
      TsType::TsImportType(node) => node.into(),
    }
  }
}

fn get_view_for_ts_type(ref_node: &'static swc_ecma_ast::TsType) -> TsType {
  match ref_node {
    swc_ecma_ast::TsType::TsKeywordType(value) => TsType::TsKeywordType(get_view_for_ts_keyword_type(value)),
    swc_ecma_ast::TsType::TsThisType(value) => TsType::TsThisType(get_view_for_ts_this_type(value)),
    swc_ecma_ast::TsType::TsFnOrConstructorType(value) => TsType::TsFnOrConstructorType(get_view_for_ts_fn_or_constructor_type(value)),
    swc_ecma_ast::TsType::TsTypeRef(value) => TsType::TsTypeRef(get_view_for_ts_type_ref(value)),
    swc_ecma_ast::TsType::TsTypeQuery(value) => TsType::TsTypeQuery(get_view_for_ts_type_query(value)),
    swc_ecma_ast::TsType::TsTypeLit(value) => TsType::TsTypeLit(get_view_for_ts_type_lit(value)),
    swc_ecma_ast::TsType::TsArrayType(value) => TsType::TsArrayType(get_view_for_ts_array_type(value)),
    swc_ecma_ast::TsType::TsTupleType(value) => TsType::TsTupleType(get_view_for_ts_tuple_type(value)),
    swc_ecma_ast::TsType::TsOptionalType(value) => TsType::TsOptionalType(get_view_for_ts_optional_type(value)),
    swc_ecma_ast::TsType::TsRestType(value) => TsType::TsRestType(get_view_for_ts_rest_type(value)),
    swc_ecma_ast::TsType::TsUnionOrIntersectionType(value) => TsType::TsUnionOrIntersectionType(get_view_for_ts_union_or_intersection_type(value)),
    swc_ecma_ast::TsType::TsConditionalType(value) => TsType::TsConditionalType(get_view_for_ts_conditional_type(value)),
    swc_ecma_ast::TsType::TsInferType(value) => TsType::TsInferType(get_view_for_ts_infer_type(value)),
    swc_ecma_ast::TsType::TsParenthesizedType(value) => TsType::TsParenthesizedType(get_view_for_ts_parenthesized_type(value)),
    swc_ecma_ast::TsType::TsTypeOperator(value) => TsType::TsTypeOperator(get_view_for_ts_type_operator(value)),
    swc_ecma_ast::TsType::TsIndexedAccessType(value) => TsType::TsIndexedAccessType(get_view_for_ts_indexed_access_type(value)),
    swc_ecma_ast::TsType::TsMappedType(value) => TsType::TsMappedType(get_view_for_ts_mapped_type(value)),
    swc_ecma_ast::TsType::TsLitType(value) => TsType::TsLitType(get_view_for_ts_lit_type(value)),
    swc_ecma_ast::TsType::TsTypePredicate(value) => TsType::TsTypePredicate(get_view_for_ts_type_predicate(value)),
    swc_ecma_ast::TsType::TsImportType(value) => TsType::TsImportType(get_view_for_ts_import_type(value)),
  }
}

fn set_parent_for_ts_type(node: &mut TsType, parent: Node) {
  match node {
    TsType::TsKeywordType(node) => {
      node.parent = parent;
    },
    TsType::TsThisType(node) => {
      node.parent = parent;
    },
    TsType::TsFnOrConstructorType(node) => {
      set_parent_for_ts_fn_or_constructor_type(node, parent);
    },
    TsType::TsTypeRef(node) => {
      node.parent = parent;
    },
    TsType::TsTypeQuery(node) => {
      node.parent = parent;
    },
    TsType::TsTypeLit(node) => {
      node.parent = parent;
    },
    TsType::TsArrayType(node) => {
      node.parent = parent;
    },
    TsType::TsTupleType(node) => {
      node.parent = parent;
    },
    TsType::TsOptionalType(node) => {
      node.parent = parent;
    },
    TsType::TsRestType(node) => {
      node.parent = parent;
    },
    TsType::TsUnionOrIntersectionType(node) => {
      set_parent_for_ts_union_or_intersection_type(node, parent);
    },
    TsType::TsConditionalType(node) => {
      node.parent = parent;
    },
    TsType::TsInferType(node) => {
      node.parent = parent;
    },
    TsType::TsParenthesizedType(node) => {
      node.parent = parent;
    },
    TsType::TsTypeOperator(node) => {
      node.parent = parent;
    },
    TsType::TsIndexedAccessType(node) => {
      node.parent = parent;
    },
    TsType::TsMappedType(node) => {
      node.parent = parent;
    },
    TsType::TsLitType(node) => {
      node.parent = parent;
    },
    TsType::TsTypePredicate(node) => {
      node.parent = parent;
    },
    TsType::TsImportType(node) => {
      node.parent = parent;
    },
  }
}

pub enum ObjectPatProp {
  KeyValue(KeyValuePatProp),
  Assign(AssignPatProp),
  Rest(RestPat),
}

impl Spanned for ObjectPatProp {
  fn span(&self) -> Span {
    match self {
      ObjectPatProp::KeyValue(node) => node.span(),
      ObjectPatProp::Assign(node) => node.span(),
      ObjectPatProp::Rest(node) => node.span(),
    }
  }
}

impl NodeTrait for ObjectPatProp {
  fn parent(&self) -> Option<&Node> {
    match self {
      ObjectPatProp::KeyValue(node) => node.parent(),
      ObjectPatProp::Assign(node) => node.parent(),
      ObjectPatProp::Rest(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      ObjectPatProp::KeyValue(node) => node.children(),
      ObjectPatProp::Assign(node) => node.children(),
      ObjectPatProp::Rest(node) => node.children(),
    }
  }
}
impl From<&ObjectPatProp> for Node {
  fn from(node: &ObjectPatProp) -> Node {
    match node {
      ObjectPatProp::KeyValue(node) => node.into(),
      ObjectPatProp::Assign(node) => node.into(),
      ObjectPatProp::Rest(node) => node.into(),
    }
  }
}

fn get_view_for_object_pat_prop(ref_node: &'static swc_ecma_ast::ObjectPatProp) -> ObjectPatProp {
  match ref_node {
    swc_ecma_ast::ObjectPatProp::KeyValue(value) => ObjectPatProp::KeyValue(get_view_for_key_value_pat_prop(value)),
    swc_ecma_ast::ObjectPatProp::Assign(value) => ObjectPatProp::Assign(get_view_for_assign_pat_prop(value)),
    swc_ecma_ast::ObjectPatProp::Rest(value) => ObjectPatProp::Rest(get_view_for_rest_pat(value)),
  }
}

fn set_parent_for_object_pat_prop(node: &mut ObjectPatProp, parent: Node) {
  match node {
    ObjectPatProp::KeyValue(node) => {
      node.parent = parent;
    },
    ObjectPatProp::Assign(node) => {
      node.parent = parent;
    },
    ObjectPatProp::Rest(node) => {
      node.parent = parent;
    },
  }
}

pub enum TsFnOrConstructorType {
  TsFnType(TsFnType),
  TsConstructorType(TsConstructorType),
}

impl Spanned for TsFnOrConstructorType {
  fn span(&self) -> Span {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.span(),
      TsFnOrConstructorType::TsConstructorType(node) => node.span(),
    }
  }
}

impl NodeTrait for TsFnOrConstructorType {
  fn parent(&self) -> Option<&Node> {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.parent(),
      TsFnOrConstructorType::TsConstructorType(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node> {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.children(),
      TsFnOrConstructorType::TsConstructorType(node) => node.children(),
    }
  }
}
impl From<&TsFnOrConstructorType> for Node {
  fn from(node: &TsFnOrConstructorType) -> Node {
    match node {
      TsFnOrConstructorType::TsFnType(node) => node.into(),
      TsFnOrConstructorType::TsConstructorType(node) => node.into(),
    }
  }
}

fn get_view_for_ts_fn_or_constructor_type(ref_node: &'static swc_ecma_ast::TsFnOrConstructorType) -> TsFnOrConstructorType {
  match ref_node {
    swc_ecma_ast::TsFnOrConstructorType::TsFnType(value) => TsFnOrConstructorType::TsFnType(get_view_for_ts_fn_type(value)),
    swc_ecma_ast::TsFnOrConstructorType::TsConstructorType(value) => TsFnOrConstructorType::TsConstructorType(get_view_for_ts_constructor_type(value)),
  }
}

fn set_parent_for_ts_fn_or_constructor_type(node: &mut TsFnOrConstructorType, parent: Node) {
  match node {
    TsFnOrConstructorType::TsFnType(node) => {
      node.parent = parent;
    },
    TsFnOrConstructorType::TsConstructorType(node) => {
      node.parent = parent;
    },
  }
}

pub struct SwitchCase {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::SwitchCase,
  /// None for `default:`
  pub test: Option<Box<Expr>>,
  pub cons: Vec<Stmt>,
}

impl Spanned for SwitchCase {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&SwitchCase> for Node {
  fn from(node: &SwitchCase) -> Node {
    let static_ref = unsafe { mem::transmute::<&SwitchCase, &'static SwitchCase>(&node) };
    Node::SwitchCase(static_ref)
  }
}

impl NodeTrait for SwitchCase {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(match &self.test { Some(_value) => 1, None => 0, } + self.cons.len());
    if let Some(child) = &self.test {
      children.push(child.into());
    }
    for child in self.cons.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_switch_case(ref_node: &'static swc_ecma_ast::SwitchCase) -> SwitchCase {
  let value = &ref_node.test;
  let field_test = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let value = &ref_node.cons;
  let field_cons = value.iter().map(|value| get_view_for_stmt(value)).collect();
  let mut node = SwitchCase {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    test: field_test,
    cons: field_cons,
  };
  let child_parent_ref = unsafe { mem::transmute::<&SwitchCase, &'static SwitchCase>(&node) };
  let parent = Node::SwitchCase(child_parent_ref);
  if let Some(node) = node.test.as_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  for node in node.cons.iter_mut() {
    set_parent_for_stmt(node, parent.clone());
  }
  node
}

pub struct ThrowStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ThrowStmt,
  pub arg: Box<Expr>,
}

impl Spanned for ThrowStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ThrowStmt> for Node {
  fn from(node: &ThrowStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&ThrowStmt, &'static ThrowStmt>(&node) };
    Node::ThrowStmt(static_ref)
  }
}

impl NodeTrait for ThrowStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }
}

fn get_view_for_throw_stmt(ref_node: &'static swc_ecma_ast::ThrowStmt) -> ThrowStmt {
  let value = &ref_node.arg;
  let field_arg = Box::new(get_view_for_expr(value));
  let mut node = ThrowStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ThrowStmt, &'static ThrowStmt>(&node) };
  let parent = Node::ThrowStmt(child_parent_ref);
  set_parent_for_expr(&mut node.arg, parent);
  node
}

pub struct JSXClosingFragment {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXClosingFragment,
}

impl Spanned for JSXClosingFragment {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXClosingFragment> for Node {
  fn from(node: &JSXClosingFragment) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXClosingFragment, &'static JSXClosingFragment>(&node) };
    Node::JSXClosingFragment(static_ref)
  }
}

impl NodeTrait for JSXClosingFragment {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_jsxclosing_fragment(ref_node: &'static swc_ecma_ast::JSXClosingFragment) -> JSXClosingFragment {
  let node = JSXClosingFragment {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct BigInt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::BigInt,
}

impl BigInt {
  pub fn value(&self) -> &num_bigint::BigInt {
    &self.inner.value
  }
}

impl Spanned for BigInt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&BigInt> for Node {
  fn from(node: &BigInt) -> Node {
    let static_ref = unsafe { mem::transmute::<&BigInt, &'static BigInt>(&node) };
    Node::BigInt(static_ref)
  }
}

impl NodeTrait for BigInt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_big_int(ref_node: &'static swc_ecma_ast::BigInt) -> BigInt {
  let node = BigInt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct ExportDefaultSpecifier {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ExportDefaultSpecifier,
  pub exported: Ident,
}

impl Spanned for ExportDefaultSpecifier {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ExportDefaultSpecifier> for Node {
  fn from(node: &ExportDefaultSpecifier) -> Node {
    let static_ref = unsafe { mem::transmute::<&ExportDefaultSpecifier, &'static ExportDefaultSpecifier>(&node) };
    Node::ExportDefaultSpecifier(static_ref)
  }
}

impl NodeTrait for ExportDefaultSpecifier {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.exported).into());
    children
  }
}

fn get_view_for_export_default_specifier(ref_node: &'static swc_ecma_ast::ExportDefaultSpecifier) -> ExportDefaultSpecifier {
  let value = &ref_node.exported;
  let field_exported = get_view_for_ident(value);
  let mut node = ExportDefaultSpecifier {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    exported: field_exported,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportDefaultSpecifier, &'static ExportDefaultSpecifier>(&node) };
  let parent = Node::ExportDefaultSpecifier(child_parent_ref);
  node.exported.parent = parent;
  node
}

pub struct TsTypeParam {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeParam,
  pub name: Ident,
  pub constraint: Option<Box<TsType>>,
  pub default: Option<Box<TsType>>,
}

impl Spanned for TsTypeParam {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeParam> for Node {
  fn from(node: &TsTypeParam) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeParam, &'static TsTypeParam>(&node) };
    Node::TsTypeParam(static_ref)
  }
}

impl NodeTrait for TsTypeParam {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.constraint { Some(_value) => 1, None => 0, } + match &self.default { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    if let Some(child) = &self.constraint {
      children.push(child.into());
    }
    if let Some(child) = &self.default {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_type_param(ref_node: &'static swc_ecma_ast::TsTypeParam) -> TsTypeParam {
  let value = &ref_node.name;
  let field_name = get_view_for_ident(value);
  let value = &ref_node.constraint;
  let field_constraint = match value {
    Some(value) => Some(Box::new(get_view_for_ts_type(value))),
    None => None,
  };
  let value = &ref_node.default;
  let field_default = match value {
    Some(value) => Some(Box::new(get_view_for_ts_type(value))),
    None => None,
  };
  let mut node = TsTypeParam {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    name: field_name,
    constraint: field_constraint,
    default: field_default,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeParam, &'static TsTypeParam>(&node) };
  let parent = Node::TsTypeParam(child_parent_ref);
  node.name.parent = parent.clone();
  if let Some(node) = node.constraint.as_mut() {
    set_parent_for_ts_type(node, parent.clone());
  }
  if let Some(node) = node.default.as_mut() {
    set_parent_for_ts_type(node, parent);
  }
  node
}

pub struct WithStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::WithStmt,
  pub obj: Box<Expr>,
  pub body: Box<Stmt>,
}

impl Spanned for WithStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&WithStmt> for Node {
  fn from(node: &WithStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&WithStmt, &'static WithStmt>(&node) };
    Node::WithStmt(static_ref)
  }
}

impl NodeTrait for WithStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push((&self.body).into());
    children
  }
}

fn get_view_for_with_stmt(ref_node: &'static swc_ecma_ast::WithStmt) -> WithStmt {
  let value = &ref_node.obj;
  let field_obj = Box::new(get_view_for_expr(value));
  let value = &ref_node.body;
  let field_body = Box::new(get_view_for_stmt(value));
  let mut node = WithStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    obj: field_obj,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&WithStmt, &'static WithStmt>(&node) };
  let parent = Node::WithStmt(child_parent_ref);
  set_parent_for_expr(&mut node.obj, parent.clone());
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct Regex {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Regex,
}

impl Regex {
  pub fn exp(&self) -> &swc_atoms::JsWord {
    &self.inner.exp
  }

  pub fn flags(&self) -> &swc_atoms::JsWord {
    &self.inner.flags
  }
}

impl Spanned for Regex {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Regex> for Node {
  fn from(node: &Regex) -> Node {
    let static_ref = unsafe { mem::transmute::<&Regex, &'static Regex>(&node) };
    Node::Regex(static_ref)
  }
}

impl NodeTrait for Regex {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_regex(ref_node: &'static swc_ecma_ast::Regex) -> Regex {
  let node = Regex {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsMethodSignature {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsMethodSignature,
  pub key: Box<Expr>,
  pub params: Vec<TsFnParam>,
  pub type_ann: Option<TsTypeAnn>,
  pub type_params: Option<TsTypeParamDecl>,
}

impl TsMethodSignature {
  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }

  pub fn computed(&self) -> bool {
    self.inner.computed
  }

  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl Spanned for TsMethodSignature {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsMethodSignature> for Node {
  fn from(node: &TsMethodSignature) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsMethodSignature, &'static TsMethodSignature>(&node) };
    Node::TsMethodSignature(static_ref)
  }
}

impl NodeTrait for TsMethodSignature {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.params.len() + match &self.type_ann { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_method_signature(ref_node: &'static swc_ecma_ast::TsMethodSignature) -> TsMethodSignature {
  let value = &ref_node.key;
  let field_key = Box::new(get_view_for_expr(value));
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_ts_fn_param(value)).collect();
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let mut node = TsMethodSignature {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    params: field_params,
    type_ann: field_type_ann,
    type_params: field_type_params,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsMethodSignature, &'static TsMethodSignature>(&node) };
  let parent = Node::TsMethodSignature(child_parent_ref);
  set_parent_for_expr(&mut node.key, parent.clone());
  for node in node.params.iter_mut() {
    set_parent_for_ts_fn_param(node, parent.clone());
  }
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct UpdateExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::UpdateExpr,
  pub arg: Box<Expr>,
}

impl UpdateExpr {
  pub fn op(&self) -> &UpdateOp {
    &self.inner.op
  }

  pub fn prefix(&self) -> bool {
    self.inner.prefix
  }
}

impl Spanned for UpdateExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&UpdateExpr> for Node {
  fn from(node: &UpdateExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&UpdateExpr, &'static UpdateExpr>(&node) };
    Node::UpdateExpr(static_ref)
  }
}

impl NodeTrait for UpdateExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }
}

fn get_view_for_update_expr(ref_node: &'static swc_ecma_ast::UpdateExpr) -> UpdateExpr {
  let value = &ref_node.arg;
  let field_arg = Box::new(get_view_for_expr(value));
  let mut node = UpdateExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
  };
  let child_parent_ref = unsafe { mem::transmute::<&UpdateExpr, &'static UpdateExpr>(&node) };
  let parent = Node::UpdateExpr(child_parent_ref);
  set_parent_for_expr(&mut node.arg, parent);
  node
}

pub struct SetterProp {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::SetterProp,
  pub key: PropName,
  pub param: Pat,
  pub body: Option<BlockStmt>,
}

impl Spanned for SetterProp {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&SetterProp> for Node {
  fn from(node: &SetterProp) -> Node {
    let static_ref = unsafe { mem::transmute::<&SetterProp, &'static SetterProp>(&node) };
    Node::SetterProp(static_ref)
  }
}

impl NodeTrait for SetterProp {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2 + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    children.push((&self.param).into());
    if let Some(child) = &self.body {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_setter_prop(ref_node: &'static swc_ecma_ast::SetterProp) -> SetterProp {
  let value = &ref_node.key;
  let field_key = get_view_for_prop_name(value);
  let value = &ref_node.param;
  let field_param = get_view_for_pat(value);
  let value = &ref_node.body;
  let field_body = match value {
    Some(value) => Some(get_view_for_block_stmt(value)),
    None => None,
  };
  let mut node = SetterProp {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    param: field_param,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&SetterProp, &'static SetterProp>(&node) };
  let parent = Node::SetterProp(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  set_parent_for_pat(&mut node.param, parent.clone());
  if let Some(node) = node.body.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TaggedTpl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TaggedTpl,
  pub tag: Box<Expr>,
  pub exprs: Vec<Box<Expr>>,
  pub quasis: Vec<TplElement>,
  pub type_params: Option<TsTypeParamInstantiation>,
}

impl Spanned for TaggedTpl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TaggedTpl> for Node {
  fn from(node: &TaggedTpl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TaggedTpl, &'static TaggedTpl>(&node) };
    Node::TaggedTpl(static_ref)
  }
}

impl NodeTrait for TaggedTpl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.exprs.len() + self.quasis.len() + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.tag).into());
    for child in self.exprs.iter() {
      children.push(child.into());
    }
    for child in self.quasis.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_tagged_tpl(ref_node: &'static swc_ecma_ast::TaggedTpl) -> TaggedTpl {
  let value = &ref_node.tag;
  let field_tag = Box::new(get_view_for_expr(value));
  let value = &ref_node.exprs;
  let field_exprs = value.iter().map(|value| Box::new(get_view_for_expr(value))).collect();
  let value = &ref_node.quasis;
  let field_quasis = value.iter().map(|value| get_view_for_tpl_element(value)).collect();
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value)),
    None => None,
  };
  let mut node = TaggedTpl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    tag: field_tag,
    exprs: field_exprs,
    quasis: field_quasis,
    type_params: field_type_params,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TaggedTpl, &'static TaggedTpl>(&node) };
  let parent = Node::TaggedTpl(child_parent_ref);
  set_parent_for_expr(&mut node.tag, parent.clone());
  for node in node.exprs.iter_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  for node in node.quasis.iter_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent;
  }
  node
}

/// `export * from 'mod'`
pub struct ExportAll {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ExportAll,
  pub src: Str,
}

impl Spanned for ExportAll {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ExportAll> for Node {
  fn from(node: &ExportAll) -> Node {
    let static_ref = unsafe { mem::transmute::<&ExportAll, &'static ExportAll>(&node) };
    Node::ExportAll(static_ref)
  }
}

impl NodeTrait for ExportAll {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.src).into());
    children
  }
}

fn get_view_for_export_all(ref_node: &'static swc_ecma_ast::ExportAll) -> ExportAll {
  let value = &ref_node.src;
  let field_src = get_view_for_str(value);
  let mut node = ExportAll {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    src: field_src,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportAll, &'static ExportAll>(&node) };
  let parent = Node::ExportAll(child_parent_ref);
  node.src.parent = parent;
  node
}

pub struct TsModuleBlock {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsModuleBlock,
  pub body: Vec<ModuleItem>,
}

impl Spanned for TsModuleBlock {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsModuleBlock> for Node {
  fn from(node: &TsModuleBlock) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsModuleBlock, &'static TsModuleBlock>(&node) };
    Node::TsModuleBlock(static_ref)
  }
}

impl NodeTrait for TsModuleBlock {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.body.len());
    for child in self.body.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_module_block(ref_node: &'static swc_ecma_ast::TsModuleBlock) -> TsModuleBlock {
  let value = &ref_node.body;
  let field_body = value.iter().map(|value| get_view_for_module_item(value)).collect();
  let mut node = TsModuleBlock {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsModuleBlock, &'static TsModuleBlock>(&node) };
  let parent = Node::TsModuleBlock(child_parent_ref);
  for node in node.body.iter_mut() {
    set_parent_for_module_item(node, parent.clone());
  }
  node
}

pub struct SwitchStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::SwitchStmt,
  pub discriminant: Box<Expr>,
  pub cases: Vec<SwitchCase>,
}

impl Spanned for SwitchStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&SwitchStmt> for Node {
  fn from(node: &SwitchStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&SwitchStmt, &'static SwitchStmt>(&node) };
    Node::SwitchStmt(static_ref)
  }
}

impl NodeTrait for SwitchStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.cases.len());
    children.push((&self.discriminant).into());
    for child in self.cases.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_switch_stmt(ref_node: &'static swc_ecma_ast::SwitchStmt) -> SwitchStmt {
  let value = &ref_node.discriminant;
  let field_discriminant = Box::new(get_view_for_expr(value));
  let value = &ref_node.cases;
  let field_cases = value.iter().map(|value| get_view_for_switch_case(value)).collect();
  let mut node = SwitchStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    discriminant: field_discriminant,
    cases: field_cases,
  };
  let child_parent_ref = unsafe { mem::transmute::<&SwitchStmt, &'static SwitchStmt>(&node) };
  let parent = Node::SwitchStmt(child_parent_ref);
  set_parent_for_expr(&mut node.discriminant, parent.clone());
  for node in node.cases.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

pub struct TsEnumMember {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsEnumMember,
  pub id: TsEnumMemberId,
  pub init: Option<Box<Expr>>,
}

impl Spanned for TsEnumMember {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsEnumMember> for Node {
  fn from(node: &TsEnumMember) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsEnumMember, &'static TsEnumMember>(&node) };
    Node::TsEnumMember(static_ref)
  }
}

impl NodeTrait for TsEnumMember {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, });
    children.push((&self.id).into());
    if let Some(child) = &self.init {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_enum_member(ref_node: &'static swc_ecma_ast::TsEnumMember) -> TsEnumMember {
  let value = &ref_node.id;
  let field_id = get_view_for_ts_enum_member_id(value);
  let value = &ref_node.init;
  let field_init = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let mut node = TsEnumMember {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
    init: field_init,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsEnumMember, &'static TsEnumMember>(&node) };
  let parent = Node::TsEnumMember(child_parent_ref);
  set_parent_for_ts_enum_member_id(&mut node.id, parent.clone());
  if let Some(node) = node.init.as_mut() {
    set_parent_for_expr(node, parent);
  }
  node
}

pub struct TsIndexedAccessType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsIndexedAccessType,
  pub obj_type: Box<TsType>,
  pub index_type: Box<TsType>,
}

impl TsIndexedAccessType {
  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }
}

impl Spanned for TsIndexedAccessType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsIndexedAccessType> for Node {
  fn from(node: &TsIndexedAccessType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsIndexedAccessType, &'static TsIndexedAccessType>(&node) };
    Node::TsIndexedAccessType(static_ref)
  }
}

impl NodeTrait for TsIndexedAccessType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj_type).into());
    children.push((&self.index_type).into());
    children
  }
}

fn get_view_for_ts_indexed_access_type(ref_node: &'static swc_ecma_ast::TsIndexedAccessType) -> TsIndexedAccessType {
  let value = &ref_node.obj_type;
  let field_obj_type = Box::new(get_view_for_ts_type(value));
  let value = &ref_node.index_type;
  let field_index_type = Box::new(get_view_for_ts_type(value));
  let mut node = TsIndexedAccessType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    obj_type: field_obj_type,
    index_type: field_index_type,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsIndexedAccessType, &'static TsIndexedAccessType>(&node) };
  let parent = Node::TsIndexedAccessType(child_parent_ref);
  set_parent_for_ts_type(&mut node.obj_type, parent.clone());
  set_parent_for_ts_type(&mut node.index_type, parent);
  node
}

pub struct TsRestType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsRestType,
  pub type_ann: Box<TsType>,
}

impl Spanned for TsRestType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsRestType> for Node {
  fn from(node: &TsRestType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsRestType, &'static TsRestType>(&node) };
    Node::TsRestType(static_ref)
  }
}

impl NodeTrait for TsRestType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_rest_type(ref_node: &'static swc_ecma_ast::TsRestType) -> TsRestType {
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsRestType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsRestType, &'static TsRestType>(&node) };
  let parent = Node::TsRestType(child_parent_ref);
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct ExprStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ExprStmt,
  pub expr: Box<Expr>,
}

impl Spanned for ExprStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ExprStmt> for Node {
  fn from(node: &ExprStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&ExprStmt, &'static ExprStmt>(&node) };
    Node::ExprStmt(static_ref)
  }
}

impl NodeTrait for ExprStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_expr_stmt(ref_node: &'static swc_ecma_ast::ExprStmt) -> ExprStmt {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = ExprStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExprStmt, &'static ExprStmt>(&node) };
  let parent = Node::ExprStmt(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsOptionalType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsOptionalType,
  pub type_ann: Box<TsType>,
}

impl Spanned for TsOptionalType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsOptionalType> for Node {
  fn from(node: &TsOptionalType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsOptionalType, &'static TsOptionalType>(&node) };
    Node::TsOptionalType(static_ref)
  }
}

impl NodeTrait for TsOptionalType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_optional_type(ref_node: &'static swc_ecma_ast::TsOptionalType) -> TsOptionalType {
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsOptionalType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsOptionalType, &'static TsOptionalType>(&node) };
  let parent = Node::TsOptionalType(child_parent_ref);
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct Tpl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Tpl,
  pub exprs: Vec<Box<Expr>>,
  pub quasis: Vec<TplElement>,
}

impl Spanned for Tpl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Tpl> for Node {
  fn from(node: &Tpl) -> Node {
    let static_ref = unsafe { mem::transmute::<&Tpl, &'static Tpl>(&node) };
    Node::Tpl(static_ref)
  }
}

impl NodeTrait for Tpl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.exprs.len() + self.quasis.len());
    for child in self.exprs.iter() {
      children.push(child.into());
    }
    for child in self.quasis.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_tpl(ref_node: &'static swc_ecma_ast::Tpl) -> Tpl {
  let value = &ref_node.exprs;
  let field_exprs = value.iter().map(|value| Box::new(get_view_for_expr(value))).collect();
  let value = &ref_node.quasis;
  let field_quasis = value.iter().map(|value| get_view_for_tpl_element(value)).collect();
  let mut node = Tpl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    exprs: field_exprs,
    quasis: field_quasis,
  };
  let child_parent_ref = unsafe { mem::transmute::<&Tpl, &'static Tpl>(&node) };
  let parent = Node::Tpl(child_parent_ref);
  for node in node.exprs.iter_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  for node in node.quasis.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

/// Represents a invalid node.
pub struct Invalid {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Invalid,
}

impl Spanned for Invalid {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Invalid> for Node {
  fn from(node: &Invalid) -> Node {
    let static_ref = unsafe { mem::transmute::<&Invalid, &'static Invalid>(&node) };
    Node::Invalid(static_ref)
  }
}

impl NodeTrait for Invalid {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_invalid(ref_node: &'static swc_ecma_ast::Invalid) -> Invalid {
  let node = Invalid {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct ComputedPropName {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ComputedPropName,
  pub expr: Box<Expr>,
}

impl Spanned for ComputedPropName {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ComputedPropName> for Node {
  fn from(node: &ComputedPropName) -> Node {
    let static_ref = unsafe { mem::transmute::<&ComputedPropName, &'static ComputedPropName>(&node) };
    Node::ComputedPropName(static_ref)
  }
}

impl NodeTrait for ComputedPropName {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_computed_prop_name(ref_node: &'static swc_ecma_ast::ComputedPropName) -> ComputedPropName {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = ComputedPropName {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ComputedPropName, &'static ComputedPropName>(&node) };
  let parent = Node::ComputedPropName(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsFnType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsFnType,
  pub params: Vec<TsFnParam>,
  pub type_params: Option<TsTypeParamDecl>,
  pub type_ann: TsTypeAnn,
}

impl Spanned for TsFnType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsFnType> for Node {
  fn from(node: &TsFnType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsFnType, &'static TsFnType>(&node) };
    Node::TsFnType(static_ref)
  }
}

impl NodeTrait for TsFnType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.params.len() + match &self.type_params { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_fn_type(ref_node: &'static swc_ecma_ast::TsFnType) -> TsFnType {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_ts_fn_param(value)).collect();
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let value = &ref_node.type_ann;
  let field_type_ann = get_view_for_ts_type_ann(value);
  let mut node = TsFnType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
    type_params: field_type_params,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsFnType, &'static TsFnType>(&node) };
  let parent = Node::TsFnType(child_parent_ref);
  for node in node.params.iter_mut() {
    set_parent_for_ts_fn_param(node, parent.clone());
  }
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent.clone();
  }
  node.type_ann.parent = parent;
  node
}

/// Use when only block statements are allowed.
pub struct BlockStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::BlockStmt,
  pub stmts: Vec<Stmt>,
}

impl Spanned for BlockStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&BlockStmt> for Node {
  fn from(node: &BlockStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&BlockStmt, &'static BlockStmt>(&node) };
    Node::BlockStmt(static_ref)
  }
}

impl NodeTrait for BlockStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.stmts.len());
    for child in self.stmts.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_block_stmt(ref_node: &'static swc_ecma_ast::BlockStmt) -> BlockStmt {
  let value = &ref_node.stmts;
  let field_stmts = value.iter().map(|value| get_view_for_stmt(value)).collect();
  let mut node = BlockStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    stmts: field_stmts,
  };
  let child_parent_ref = unsafe { mem::transmute::<&BlockStmt, &'static BlockStmt>(&node) };
  let parent = Node::BlockStmt(child_parent_ref);
  for node in node.stmts.iter_mut() {
    set_parent_for_stmt(node, parent.clone());
  }
  node
}

pub struct TsTypeAliasDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeAliasDecl,
  pub id: Ident,
  pub type_params: Option<TsTypeParamDecl>,
  pub type_ann: Box<TsType>,
}

impl TsTypeAliasDecl {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl Spanned for TsTypeAliasDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeAliasDecl> for Node {
  fn from(node: &TsTypeAliasDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeAliasDecl, &'static TsTypeAliasDecl>(&node) };
    Node::TsTypeAliasDecl(static_ref)
  }
}

impl NodeTrait for TsTypeAliasDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2 + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.id).into());
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_type_alias_decl(ref_node: &'static swc_ecma_ast::TsTypeAliasDecl) -> TsTypeAliasDecl {
  let value = &ref_node.id;
  let field_id = get_view_for_ident(value);
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsTypeAliasDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
    type_params: field_type_params,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeAliasDecl, &'static TsTypeAliasDecl>(&node) };
  let parent = Node::TsTypeAliasDecl(child_parent_ref);
  node.id.parent = parent.clone();
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent.clone();
  }
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct MemberExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::MemberExpr,
  pub obj: ExprOrSuper,
  pub prop: Box<Expr>,
}

impl MemberExpr {
  pub fn computed(&self) -> bool {
    self.inner.computed
  }
}

impl Spanned for MemberExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&MemberExpr> for Node {
  fn from(node: &MemberExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&MemberExpr, &'static MemberExpr>(&node) };
    Node::MemberExpr(static_ref)
  }
}

impl NodeTrait for MemberExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push((&self.prop).into());
    children
  }
}

fn get_view_for_member_expr(ref_node: &'static swc_ecma_ast::MemberExpr) -> MemberExpr {
  let value = &ref_node.obj;
  let field_obj = get_view_for_expr_or_super(value);
  let value = &ref_node.prop;
  let field_prop = Box::new(get_view_for_expr(value));
  let mut node = MemberExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    obj: field_obj,
    prop: field_prop,
  };
  let child_parent_ref = unsafe { mem::transmute::<&MemberExpr, &'static MemberExpr>(&node) };
  let parent = Node::MemberExpr(child_parent_ref);
  set_parent_for_expr_or_super(&mut node.obj, parent.clone());
  set_parent_for_expr(&mut node.prop, parent);
  node
}

/// Common parts of function and method.
pub struct Function {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Function,
  pub params: Vec<Param>,
  pub decorators: Vec<Decorator>,
  pub body: Option<BlockStmt>,
  pub type_params: Option<TsTypeParamDecl>,
  pub return_type: Option<TsTypeAnn>,
}

impl Function {
  /// if it's a generator.
  pub fn is_generator(&self) -> bool {
    self.inner.is_generator
  }

  /// if it's an async function.
  pub fn is_async(&self) -> bool {
    self.inner.is_async
  }
}

impl Spanned for Function {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Function> for Node {
  fn from(node: &Function) -> Node {
    let static_ref = unsafe { mem::transmute::<&Function, &'static Function>(&node) };
    Node::Function(static_ref)
  }
}

impl NodeTrait for Function {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.params.len() + self.decorators.len() + match &self.body { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, } + match &self.return_type { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    for child in self.decorators.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.body {
      children.push(child.into());
    }
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    if let Some(child) = &self.return_type {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_function(ref_node: &'static swc_ecma_ast::Function) -> Function {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_param(value)).collect();
  let value = &ref_node.decorators;
  let field_decorators = value.iter().map(|value| get_view_for_decorator(value)).collect();
  let value = &ref_node.body;
  let field_body = match value {
    Some(value) => Some(get_view_for_block_stmt(value)),
    None => None,
  };
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let value = &ref_node.return_type;
  let field_return_type = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let mut node = Function {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
    decorators: field_decorators,
    body: field_body,
    type_params: field_type_params,
    return_type: field_return_type,
  };
  let child_parent_ref = unsafe { mem::transmute::<&Function, &'static Function>(&node) };
  let parent = Node::Function(child_parent_ref);
  for node in node.params.iter_mut() {
    node.parent = parent.clone();
  }
  for node in node.decorators.iter_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.body.as_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.return_type.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct ImportDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ImportDecl,
  pub specifiers: Vec<ImportSpecifier>,
  pub src: Str,
  pub asserts: Option<ObjectLit>,
}

impl ImportDecl {
  pub fn type_only(&self) -> bool {
    self.inner.type_only
  }
}

impl Spanned for ImportDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ImportDecl> for Node {
  fn from(node: &ImportDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&ImportDecl, &'static ImportDecl>(&node) };
    Node::ImportDecl(static_ref)
  }
}

impl NodeTrait for ImportDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.specifiers.len() + match &self.asserts { Some(_value) => 1, None => 0, });
    for child in self.specifiers.iter() {
      children.push(child.into());
    }
    children.push((&self.src).into());
    if let Some(child) = &self.asserts {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_import_decl(ref_node: &'static swc_ecma_ast::ImportDecl) -> ImportDecl {
  let value = &ref_node.specifiers;
  let field_specifiers = value.iter().map(|value| get_view_for_import_specifier(value)).collect();
  let value = &ref_node.src;
  let field_src = get_view_for_str(value);
  let value = &ref_node.asserts;
  let field_asserts = match value {
    Some(value) => Some(get_view_for_object_lit(value)),
    None => None,
  };
  let mut node = ImportDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    specifiers: field_specifiers,
    src: field_src,
    asserts: field_asserts,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ImportDecl, &'static ImportDecl>(&node) };
  let parent = Node::ImportDecl(child_parent_ref);
  for node in node.specifiers.iter_mut() {
    set_parent_for_import_specifier(node, parent.clone());
  }
  node.src.parent = parent.clone();
  if let Some(node) = node.asserts.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsTypePredicate {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypePredicate,
  pub param_name: TsThisTypeOrIdent,
  pub type_ann: Option<TsTypeAnn>,
}

impl TsTypePredicate {
  pub fn asserts(&self) -> bool {
    self.inner.asserts
  }
}

impl Spanned for TsTypePredicate {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypePredicate> for Node {
  fn from(node: &TsTypePredicate) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypePredicate, &'static TsTypePredicate>(&node) };
    Node::TsTypePredicate(static_ref)
  }
}

impl NodeTrait for TsTypePredicate {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.param_name).into());
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_type_predicate(ref_node: &'static swc_ecma_ast::TsTypePredicate) -> TsTypePredicate {
  let value = &ref_node.param_name;
  let field_param_name = get_view_for_ts_this_type_or_ident(value);
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let mut node = TsTypePredicate {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    param_name: field_param_name,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypePredicate, &'static TsTypePredicate>(&node) };
  let parent = Node::TsTypePredicate(child_parent_ref);
  set_parent_for_ts_this_type_or_ident(&mut node.param_name, parent.clone());
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct YieldExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::YieldExpr,
  pub arg: Option<Box<Expr>>,
}

impl YieldExpr {
  pub fn delegate(&self) -> bool {
    self.inner.delegate
  }
}

impl Spanned for YieldExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&YieldExpr> for Node {
  fn from(node: &YieldExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&YieldExpr, &'static YieldExpr>(&node) };
    Node::YieldExpr(static_ref)
  }
}

impl NodeTrait for YieldExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(match &self.arg { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.arg {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_yield_expr(ref_node: &'static swc_ecma_ast::YieldExpr) -> YieldExpr {
  let value = &ref_node.arg;
  let field_arg = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let mut node = YieldExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
  };
  let child_parent_ref = unsafe { mem::transmute::<&YieldExpr, &'static YieldExpr>(&node) };
  let parent = Node::YieldExpr(child_parent_ref);
  if let Some(node) = node.arg.as_mut() {
    set_parent_for_expr(node, parent);
  }
  node
}

pub struct KeyValueProp {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::KeyValueProp,
  pub key: PropName,
  pub value: Box<Expr>,
}

impl Spanned for KeyValueProp {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&KeyValueProp> for Node {
  fn from(node: &KeyValueProp) -> Node {
    let static_ref = unsafe { mem::transmute::<&KeyValueProp, &'static KeyValueProp>(&node) };
    Node::KeyValueProp(static_ref)
  }
}

impl NodeTrait for KeyValueProp {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.value).into());
    children
  }
}

fn get_view_for_key_value_prop(ref_node: &'static swc_ecma_ast::KeyValueProp) -> KeyValueProp {
  let value = &ref_node.key;
  let field_key = get_view_for_prop_name(value);
  let value = &ref_node.value;
  let field_value = Box::new(get_view_for_expr(value));
  let mut node = KeyValueProp {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    value: field_value,
  };
  let child_parent_ref = unsafe { mem::transmute::<&KeyValueProp, &'static KeyValueProp>(&node) };
  let parent = Node::KeyValueProp(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  set_parent_for_expr(&mut node.value, parent);
  node
}

pub struct Param {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Param,
  pub decorators: Vec<Decorator>,
  pub pat: Pat,
}

impl Spanned for Param {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Param> for Node {
  fn from(node: &Param) -> Node {
    let static_ref = unsafe { mem::transmute::<&Param, &'static Param>(&node) };
    Node::Param(static_ref)
  }
}

impl NodeTrait for Param {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.decorators.len());
    for child in self.decorators.iter() {
      children.push(child.into());
    }
    children.push((&self.pat).into());
    children
  }
}

fn get_view_for_param(ref_node: &'static swc_ecma_ast::Param) -> Param {
  let value = &ref_node.decorators;
  let field_decorators = value.iter().map(|value| get_view_for_decorator(value)).collect();
  let value = &ref_node.pat;
  let field_pat = get_view_for_pat(value);
  let mut node = Param {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    decorators: field_decorators,
    pat: field_pat,
  };
  let child_parent_ref = unsafe { mem::transmute::<&Param, &'static Param>(&node) };
  let parent = Node::Param(child_parent_ref);
  for node in node.decorators.iter_mut() {
    node.parent = parent.clone();
  }
  set_parent_for_pat(&mut node.pat, parent);
  node
}

pub struct JSXFragment {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXFragment,
  pub opening: JSXOpeningFragment,
  pub children: Vec<JSXElementChild>,
  pub closing: JSXClosingFragment,
}

impl Spanned for JSXFragment {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXFragment> for Node {
  fn from(node: &JSXFragment) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXFragment, &'static JSXFragment>(&node) };
    Node::JSXFragment(static_ref)
  }
}

impl NodeTrait for JSXFragment {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2 + self.children.len());
    children.push((&self.opening).into());
    for child in self.children.iter() {
      children.push(child.into());
    }
    children.push((&self.closing).into());
    children
  }
}

fn get_view_for_jsxfragment(ref_node: &'static swc_ecma_ast::JSXFragment) -> JSXFragment {
  let value = &ref_node.opening;
  let field_opening = get_view_for_jsxopening_fragment(value);
  let value = &ref_node.children;
  let field_children = value.iter().map(|value| get_view_for_jsxelement_child(value)).collect();
  let value = &ref_node.closing;
  let field_closing = get_view_for_jsxclosing_fragment(value);
  let mut node = JSXFragment {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    opening: field_opening,
    children: field_children,
    closing: field_closing,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXFragment, &'static JSXFragment>(&node) };
  let parent = Node::JSXFragment(child_parent_ref);
  node.opening.parent = parent.clone();
  for node in node.children.iter_mut() {
    set_parent_for_jsxelement_child(node, parent.clone());
  }
  node.closing.parent = parent;
  node
}

/// e.g. `import foo from 'mod.js'`
pub struct ImportDefaultSpecifier {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ImportDefaultSpecifier,
  pub local: Ident,
}

impl Spanned for ImportDefaultSpecifier {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ImportDefaultSpecifier> for Node {
  fn from(node: &ImportDefaultSpecifier) -> Node {
    let static_ref = unsafe { mem::transmute::<&ImportDefaultSpecifier, &'static ImportDefaultSpecifier>(&node) };
    Node::ImportDefaultSpecifier(static_ref)
  }
}

impl NodeTrait for ImportDefaultSpecifier {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.local).into());
    children
  }
}

fn get_view_for_import_default_specifier(ref_node: &'static swc_ecma_ast::ImportDefaultSpecifier) -> ImportDefaultSpecifier {
  let value = &ref_node.local;
  let field_local = get_view_for_ident(value);
  let mut node = ImportDefaultSpecifier {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    local: field_local,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ImportDefaultSpecifier, &'static ImportDefaultSpecifier>(&node) };
  let parent = Node::ImportDefaultSpecifier(child_parent_ref);
  node.local.parent = parent;
  node
}

pub struct Number {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Number,
}

impl Number {
  /// **Note**: This should not be `NaN`. Use [crate::Ident] to represent NaN.
  /// 
  /// If you store `NaN` in this field, a hash map will behave strangely.
  pub fn value(&self) -> f64 {
    self.inner.value
  }
}

impl Spanned for Number {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Number> for Node {
  fn from(node: &Number) -> Node {
    let static_ref = unsafe { mem::transmute::<&Number, &'static Number>(&node) };
    Node::Number(static_ref)
  }
}

impl NodeTrait for Number {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_number(ref_node: &'static swc_ecma_ast::Number) -> Number {
  let node = Number {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct JSXAttr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXAttr,
  pub name: JSXAttrName,
  /// Babel uses Expr instead of JSXAttrValue
  pub value: Option<JSXAttrValue>,
}

impl Spanned for JSXAttr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXAttr> for Node {
  fn from(node: &JSXAttr) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXAttr, &'static JSXAttr>(&node) };
    Node::JSXAttr(static_ref)
  }
}

impl NodeTrait for JSXAttr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    if let Some(child) = &self.value {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_jsxattr(ref_node: &'static swc_ecma_ast::JSXAttr) -> JSXAttr {
  let value = &ref_node.name;
  let field_name = get_view_for_jsxattr_name(value);
  let value = &ref_node.value;
  let field_value = match value {
    Some(value) => Some(get_view_for_jsxattr_value(value)),
    None => None,
  };
  let mut node = JSXAttr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    name: field_name,
    value: field_value,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXAttr, &'static JSXAttr>(&node) };
  let parent = Node::JSXAttr(child_parent_ref);
  set_parent_for_jsxattr_name(&mut node.name, parent.clone());
  if let Some(node) = node.value.as_mut() {
    set_parent_for_jsxattr_value(node, parent);
  }
  node
}

pub struct ParenExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ParenExpr,
  pub expr: Box<Expr>,
}

impl Spanned for ParenExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ParenExpr> for Node {
  fn from(node: &ParenExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&ParenExpr, &'static ParenExpr>(&node) };
    Node::ParenExpr(static_ref)
  }
}

impl NodeTrait for ParenExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_paren_expr(ref_node: &'static swc_ecma_ast::ParenExpr) -> ParenExpr {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = ParenExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ParenExpr, &'static ParenExpr>(&node) };
  let parent = Node::ParenExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct Super {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Super,
}

impl Spanned for Super {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Super> for Node {
  fn from(node: &Super) -> Node {
    let static_ref = unsafe { mem::transmute::<&Super, &'static Super>(&node) };
    Node::Super(static_ref)
  }
}

impl NodeTrait for Super {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_super(ref_node: &'static swc_ecma_ast::Super) -> Super {
  let node = Super {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsConstructorType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsConstructorType,
  pub params: Vec<TsFnParam>,
  pub type_params: Option<TsTypeParamDecl>,
  pub type_ann: TsTypeAnn,
}

impl Spanned for TsConstructorType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsConstructorType> for Node {
  fn from(node: &TsConstructorType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsConstructorType, &'static TsConstructorType>(&node) };
    Node::TsConstructorType(static_ref)
  }
}

impl NodeTrait for TsConstructorType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.params.len() + match &self.type_params { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_constructor_type(ref_node: &'static swc_ecma_ast::TsConstructorType) -> TsConstructorType {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_ts_fn_param(value)).collect();
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let value = &ref_node.type_ann;
  let field_type_ann = get_view_for_ts_type_ann(value);
  let mut node = TsConstructorType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
    type_params: field_type_params,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsConstructorType, &'static TsConstructorType>(&node) };
  let parent = Node::TsConstructorType(child_parent_ref);
  for node in node.params.iter_mut() {
    set_parent_for_ts_fn_param(node, parent.clone());
  }
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent.clone();
  }
  node.type_ann.parent = parent;
  node
}

pub struct Class {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Class,
  pub decorators: Vec<Decorator>,
  pub body: Vec<ClassMember>,
  pub super_class: Option<Box<Expr>>,
  pub type_params: Option<TsTypeParamDecl>,
  pub super_type_params: Option<TsTypeParamInstantiation>,
  /// Typescript extension.
  pub implements: Vec<TsExprWithTypeArgs>,
}

impl Class {
  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }
}

impl Spanned for Class {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Class> for Node {
  fn from(node: &Class) -> Node {
    let static_ref = unsafe { mem::transmute::<&Class, &'static Class>(&node) };
    Node::Class(static_ref)
  }
}

impl NodeTrait for Class {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.decorators.len() + self.body.len() + match &self.super_class { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, } + match &self.super_type_params { Some(_value) => 1, None => 0, } + self.implements.len());
    for child in self.decorators.iter() {
      children.push(child.into());
    }
    for child in self.body.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.super_class {
      children.push(child.into());
    }
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    if let Some(child) = &self.super_type_params {
      children.push(child.into());
    }
    for child in self.implements.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_class(ref_node: &'static swc_ecma_ast::Class) -> Class {
  let value = &ref_node.decorators;
  let field_decorators = value.iter().map(|value| get_view_for_decorator(value)).collect();
  let value = &ref_node.body;
  let field_body = value.iter().map(|value| get_view_for_class_member(value)).collect();
  let value = &ref_node.super_class;
  let field_super_class = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let value = &ref_node.super_type_params;
  let field_super_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value)),
    None => None,
  };
  let value = &ref_node.implements;
  let field_implements = value.iter().map(|value| get_view_for_ts_expr_with_type_args(value)).collect();
  let mut node = Class {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    decorators: field_decorators,
    body: field_body,
    super_class: field_super_class,
    type_params: field_type_params,
    super_type_params: field_super_type_params,
    implements: field_implements,
  };
  let child_parent_ref = unsafe { mem::transmute::<&Class, &'static Class>(&node) };
  let parent = Node::Class(child_parent_ref);
  for node in node.decorators.iter_mut() {
    node.parent = parent.clone();
  }
  for node in node.body.iter_mut() {
    set_parent_for_class_member(node, parent.clone());
  }
  if let Some(node) = node.super_class.as_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.super_type_params.as_mut() {
    node.parent = parent.clone();
  }
  for node in node.implements.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

/// EsTree `RestElement`
pub struct RestPat {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::RestPat,
  pub arg: Box<Pat>,
  pub type_ann: Option<TsTypeAnn>,
}

impl RestPat {
  pub fn dot3_token(&self) -> &swc_common::Span {
    &self.inner.dot3_token
  }
}

impl Spanned for RestPat {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&RestPat> for Node {
  fn from(node: &RestPat) -> Node {
    let static_ref = unsafe { mem::transmute::<&RestPat, &'static RestPat>(&node) };
    Node::RestPat(static_ref)
  }
}

impl NodeTrait for RestPat {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.arg).into());
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_rest_pat(ref_node: &'static swc_ecma_ast::RestPat) -> RestPat {
  let value = &ref_node.arg;
  let field_arg = Box::new(get_view_for_pat(value));
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let mut node = RestPat {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&RestPat, &'static RestPat>(&node) };
  let parent = Node::RestPat(child_parent_ref);
  set_parent_for_pat(&mut node.arg, parent.clone());
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsNamespaceExportDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsNamespaceExportDecl,
  pub id: Ident,
}

impl Spanned for TsNamespaceExportDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsNamespaceExportDecl> for Node {
  fn from(node: &TsNamespaceExportDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsNamespaceExportDecl, &'static TsNamespaceExportDecl>(&node) };
    Node::TsNamespaceExportDecl(static_ref)
  }
}

impl NodeTrait for TsNamespaceExportDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.id).into());
    children
  }
}

fn get_view_for_ts_namespace_export_decl(ref_node: &'static swc_ecma_ast::TsNamespaceExportDecl) -> TsNamespaceExportDecl {
  let value = &ref_node.id;
  let field_id = get_view_for_ident(value);
  let mut node = TsNamespaceExportDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsNamespaceExportDecl, &'static TsNamespaceExportDecl>(&node) };
  let parent = Node::TsNamespaceExportDecl(child_parent_ref);
  node.id.parent = parent;
  node
}

pub struct JSXOpeningFragment {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXOpeningFragment,
}

impl Spanned for JSXOpeningFragment {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXOpeningFragment> for Node {
  fn from(node: &JSXOpeningFragment) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXOpeningFragment, &'static JSXOpeningFragment>(&node) };
    Node::JSXOpeningFragment(static_ref)
  }
}

impl NodeTrait for JSXOpeningFragment {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_jsxopening_fragment(ref_node: &'static swc_ecma_ast::JSXOpeningFragment) -> JSXOpeningFragment {
  let node = JSXOpeningFragment {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct NewExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::NewExpr,
  pub callee: Box<Expr>,
  pub args: Option<Vec<ExprOrSpread>>,
  pub type_args: Option<TsTypeParamInstantiation>,
}

impl Spanned for NewExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&NewExpr> for Node {
  fn from(node: &NewExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&NewExpr, &'static NewExpr>(&node) };
    Node::NewExpr(static_ref)
  }
}

impl NodeTrait for NewExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.args { Some(_value) => _value.len(), None => 0, } + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.callee).into());
    if let Some(child) = &self.args {
      for child in child.iter() {
        children.push(child.into());
      }
    }
    if let Some(child) = &self.type_args {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_new_expr(ref_node: &'static swc_ecma_ast::NewExpr) -> NewExpr {
  let value = &ref_node.callee;
  let field_callee = Box::new(get_view_for_expr(value));
  let value = &ref_node.args;
  let field_args = match value {
    Some(value) => Some(value.iter().map(|value| get_view_for_expr_or_spread(value)).collect()),
    None => None,
  };
  let value = &ref_node.type_args;
  let field_type_args = match value {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value)),
    None => None,
  };
  let mut node = NewExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    callee: field_callee,
    args: field_args,
    type_args: field_type_args,
  };
  let child_parent_ref = unsafe { mem::transmute::<&NewExpr, &'static NewExpr>(&node) };
  let parent = Node::NewExpr(child_parent_ref);
  set_parent_for_expr(&mut node.callee, parent.clone());
  if let Some(node) = node.args.as_mut() {
    for node in node.iter_mut() {
      node.parent = parent.clone();
    }
  }
  if let Some(node) = node.type_args.as_mut() {
    node.parent = parent;
  }
  node
}

/// Function expression.
pub struct FnExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::FnExpr,
  pub ident: Option<Ident>,
  pub function: Function,
}

impl Spanned for FnExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&FnExpr> for Node {
  fn from(node: &FnExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&FnExpr, &'static FnExpr>(&node) };
    Node::FnExpr(static_ref)
  }
}

impl NodeTrait for FnExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.ident { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.ident {
      children.push(child.into());
    }
    children.push((&self.function).into());
    children
  }
}

fn get_view_for_fn_expr(ref_node: &'static swc_ecma_ast::FnExpr) -> FnExpr {
  let value = &ref_node.ident;
  let field_ident = match value {
    Some(value) => Some(get_view_for_ident(value)),
    None => None,
  };
  let value = &ref_node.function;
  let field_function = get_view_for_function(value);
  let mut node = FnExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    ident: field_ident,
    function: field_function,
  };
  let child_parent_ref = unsafe { mem::transmute::<&FnExpr, &'static FnExpr>(&node) };
  let parent = Node::FnExpr(child_parent_ref);
  if let Some(node) = node.ident.as_mut() {
    node.parent = parent.clone();
  }
  node.function.parent = parent;
  node
}

pub struct IfStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::IfStmt,
  pub test: Box<Expr>,
  pub cons: Box<Stmt>,
  pub alt: Option<Box<Stmt>>,
}

impl Spanned for IfStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&IfStmt> for Node {
  fn from(node: &IfStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&IfStmt, &'static IfStmt>(&node) };
    Node::IfStmt(static_ref)
  }
}

impl NodeTrait for IfStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2 + match &self.alt { Some(_value) => 1, None => 0, });
    children.push((&self.test).into());
    children.push((&self.cons).into());
    if let Some(child) = &self.alt {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_if_stmt(ref_node: &'static swc_ecma_ast::IfStmt) -> IfStmt {
  let value = &ref_node.test;
  let field_test = Box::new(get_view_for_expr(value));
  let value = &ref_node.cons;
  let field_cons = Box::new(get_view_for_stmt(value));
  let value = &ref_node.alt;
  let field_alt = match value {
    Some(value) => Some(Box::new(get_view_for_stmt(value))),
    None => None,
  };
  let mut node = IfStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    test: field_test,
    cons: field_cons,
    alt: field_alt,
  };
  let child_parent_ref = unsafe { mem::transmute::<&IfStmt, &'static IfStmt>(&node) };
  let parent = Node::IfStmt(child_parent_ref);
  set_parent_for_expr(&mut node.test, parent.clone());
  set_parent_for_stmt(&mut node.cons, parent.clone());
  if let Some(node) = node.alt.as_mut() {
    set_parent_for_stmt(node, parent);
  }
  node
}

pub struct TsParenthesizedType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsParenthesizedType,
  pub type_ann: Box<TsType>,
}

impl Spanned for TsParenthesizedType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsParenthesizedType> for Node {
  fn from(node: &TsParenthesizedType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsParenthesizedType, &'static TsParenthesizedType>(&node) };
    Node::TsParenthesizedType(static_ref)
  }
}

impl NodeTrait for TsParenthesizedType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_parenthesized_type(ref_node: &'static swc_ecma_ast::TsParenthesizedType) -> TsParenthesizedType {
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsParenthesizedType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsParenthesizedType, &'static TsParenthesizedType>(&node) };
  let parent = Node::TsParenthesizedType(child_parent_ref);
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

/// `{key}` or `{key = value}`
pub struct AssignPatProp {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::AssignPatProp,
  pub key: Ident,
  pub value: Option<Box<Expr>>,
}

impl Spanned for AssignPatProp {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&AssignPatProp> for Node {
  fn from(node: &AssignPatProp) -> Node {
    let static_ref = unsafe { mem::transmute::<&AssignPatProp, &'static AssignPatProp>(&node) };
    Node::AssignPatProp(static_ref)
  }
}

impl NodeTrait for AssignPatProp {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    if let Some(child) = &self.value {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_assign_pat_prop(ref_node: &'static swc_ecma_ast::AssignPatProp) -> AssignPatProp {
  let value = &ref_node.key;
  let field_key = get_view_for_ident(value);
  let value = &ref_node.value;
  let field_value = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let mut node = AssignPatProp {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    value: field_value,
  };
  let child_parent_ref = unsafe { mem::transmute::<&AssignPatProp, &'static AssignPatProp>(&node) };
  let parent = Node::AssignPatProp(child_parent_ref);
  node.key.parent = parent.clone();
  if let Some(node) = node.value.as_mut() {
    set_parent_for_expr(node, parent);
  }
  node
}

pub struct TsImportType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsImportType,
  pub arg: Str,
  pub qualifier: Option<TsEntityName>,
  pub type_args: Option<TsTypeParamInstantiation>,
}

impl Spanned for TsImportType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsImportType> for Node {
  fn from(node: &TsImportType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsImportType, &'static TsImportType>(&node) };
    Node::TsImportType(static_ref)
  }
}

impl NodeTrait for TsImportType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.qualifier { Some(_value) => 1, None => 0, } + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.arg).into());
    if let Some(child) = &self.qualifier {
      children.push(child.into());
    }
    if let Some(child) = &self.type_args {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_import_type(ref_node: &'static swc_ecma_ast::TsImportType) -> TsImportType {
  let value = &ref_node.arg;
  let field_arg = get_view_for_str(value);
  let value = &ref_node.qualifier;
  let field_qualifier = match value {
    Some(value) => Some(get_view_for_ts_entity_name(value)),
    None => None,
  };
  let value = &ref_node.type_args;
  let field_type_args = match value {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value)),
    None => None,
  };
  let mut node = TsImportType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
    qualifier: field_qualifier,
    type_args: field_type_args,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsImportType, &'static TsImportType>(&node) };
  let parent = Node::TsImportType(child_parent_ref);
  node.arg.parent = parent.clone();
  if let Some(node) = node.qualifier.as_mut() {
    set_parent_for_ts_entity_name(node, parent.clone());
  }
  if let Some(node) = node.type_args.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct Bool {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Bool,
}

impl Bool {
  pub fn value(&self) -> bool {
    self.inner.value
  }
}

impl Spanned for Bool {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Bool> for Node {
  fn from(node: &Bool) -> Node {
    let static_ref = unsafe { mem::transmute::<&Bool, &'static Bool>(&node) };
    Node::Bool(static_ref)
  }
}

impl NodeTrait for Bool {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_bool(ref_node: &'static swc_ecma_ast::Bool) -> Bool {
  let node = Bool {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsImportEqualsDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsImportEqualsDecl,
  pub id: Ident,
  pub module_ref: TsModuleRef,
}

impl TsImportEqualsDecl {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  pub fn is_export(&self) -> bool {
    self.inner.is_export
  }
}

impl Spanned for TsImportEqualsDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsImportEqualsDecl> for Node {
  fn from(node: &TsImportEqualsDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsImportEqualsDecl, &'static TsImportEqualsDecl>(&node) };
    Node::TsImportEqualsDecl(static_ref)
  }
}

impl NodeTrait for TsImportEqualsDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.id).into());
    children.push((&self.module_ref).into());
    children
  }
}

fn get_view_for_ts_import_equals_decl(ref_node: &'static swc_ecma_ast::TsImportEqualsDecl) -> TsImportEqualsDecl {
  let value = &ref_node.id;
  let field_id = get_view_for_ident(value);
  let value = &ref_node.module_ref;
  let field_module_ref = get_view_for_ts_module_ref(value);
  let mut node = TsImportEqualsDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
    module_ref: field_module_ref,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsImportEqualsDecl, &'static TsImportEqualsDecl>(&node) };
  let parent = Node::TsImportEqualsDecl(child_parent_ref);
  node.id.parent = parent.clone();
  set_parent_for_ts_module_ref(&mut node.module_ref, parent);
  node
}

pub struct AssignProp {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::AssignProp,
  pub key: Ident,
  pub value: Box<Expr>,
}

impl Spanned for AssignProp {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&AssignProp> for Node {
  fn from(node: &AssignProp) -> Node {
    let static_ref = unsafe { mem::transmute::<&AssignProp, &'static AssignProp>(&node) };
    Node::AssignProp(static_ref)
  }
}

impl NodeTrait for AssignProp {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.value).into());
    children
  }
}

fn get_view_for_assign_prop(ref_node: &'static swc_ecma_ast::AssignProp) -> AssignProp {
  let value = &ref_node.key;
  let field_key = get_view_for_ident(value);
  let value = &ref_node.value;
  let field_value = Box::new(get_view_for_expr(value));
  let mut node = AssignProp {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    value: field_value,
  };
  let child_parent_ref = unsafe { mem::transmute::<&AssignProp, &'static AssignProp>(&node) };
  let parent = Node::AssignProp(child_parent_ref);
  node.key.parent = parent.clone();
  set_parent_for_expr(&mut node.value, parent);
  node
}

pub struct TsInterfaceDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsInterfaceDecl,
  pub id: Ident,
  pub type_params: Option<TsTypeParamDecl>,
  pub extends: Vec<TsExprWithTypeArgs>,
  pub body: TsInterfaceBody,
}

impl TsInterfaceDecl {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl Spanned for TsInterfaceDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsInterfaceDecl> for Node {
  fn from(node: &TsInterfaceDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsInterfaceDecl, &'static TsInterfaceDecl>(&node) };
    Node::TsInterfaceDecl(static_ref)
  }
}

impl NodeTrait for TsInterfaceDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2 + match &self.type_params { Some(_value) => 1, None => 0, } + self.extends.len());
    children.push((&self.id).into());
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    for child in self.extends.iter() {
      children.push(child.into());
    }
    children.push((&self.body).into());
    children
  }
}

fn get_view_for_ts_interface_decl(ref_node: &'static swc_ecma_ast::TsInterfaceDecl) -> TsInterfaceDecl {
  let value = &ref_node.id;
  let field_id = get_view_for_ident(value);
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let value = &ref_node.extends;
  let field_extends = value.iter().map(|value| get_view_for_ts_expr_with_type_args(value)).collect();
  let value = &ref_node.body;
  let field_body = get_view_for_ts_interface_body(value);
  let mut node = TsInterfaceDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
    type_params: field_type_params,
    extends: field_extends,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsInterfaceDecl, &'static TsInterfaceDecl>(&node) };
  let parent = Node::TsInterfaceDecl(child_parent_ref);
  node.id.parent = parent.clone();
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent.clone();
  }
  for node in node.extends.iter_mut() {
    node.parent = parent.clone();
  }
  node.body.parent = parent;
  node
}

pub struct JSXEmptyExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXEmptyExpr,
}

impl Spanned for JSXEmptyExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXEmptyExpr> for Node {
  fn from(node: &JSXEmptyExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXEmptyExpr, &'static JSXEmptyExpr>(&node) };
    Node::JSXEmptyExpr(static_ref)
  }
}

impl NodeTrait for JSXEmptyExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_jsxempty_expr(ref_node: &'static swc_ecma_ast::JSXEmptyExpr) -> JSXEmptyExpr {
  let node = JSXEmptyExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsQualifiedName {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsQualifiedName,
  pub left: TsEntityName,
  pub right: Ident,
}

impl Spanned for TsQualifiedName {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsQualifiedName> for Node {
  fn from(node: &TsQualifiedName) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsQualifiedName, &'static TsQualifiedName>(&node) };
    Node::TsQualifiedName(static_ref)
  }
}

impl NodeTrait for TsQualifiedName {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children
  }
}

fn get_view_for_ts_qualified_name(ref_node: &'static swc_ecma_ast::TsQualifiedName) -> TsQualifiedName {
  let value = &ref_node.left;
  let field_left = get_view_for_ts_entity_name(value);
  let value = &ref_node.right;
  let field_right = get_view_for_ident(value);
  let mut node = TsQualifiedName {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    left: field_left,
    right: field_right,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsQualifiedName, &'static TsQualifiedName>(&node) };
  let parent = Node::TsQualifiedName(child_parent_ref);
  set_parent_for_ts_entity_name(&mut node.left, parent.clone());
  node.right.parent = parent;
  node
}

pub struct ExportDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ExportDecl,
  pub decl: Decl,
}

impl Spanned for ExportDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ExportDecl> for Node {
  fn from(node: &ExportDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&ExportDecl, &'static ExportDecl>(&node) };
    Node::ExportDecl(static_ref)
  }
}

impl NodeTrait for ExportDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.decl).into());
    children
  }
}

fn get_view_for_export_decl(ref_node: &'static swc_ecma_ast::ExportDecl) -> ExportDecl {
  let value = &ref_node.decl;
  let field_decl = get_view_for_decl(value);
  let mut node = ExportDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    decl: field_decl,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportDecl, &'static ExportDecl>(&node) };
  let parent = Node::ExportDecl(child_parent_ref);
  set_parent_for_decl(&mut node.decl, parent);
  node
}

pub struct CatchClause {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::CatchClause,
  /// es2019
  /// 
  /// The param is null if the catch binding is omitted. E.g., try { foo() }
  /// catch { bar() }
  pub param: Option<Pat>,
  pub body: BlockStmt,
}

impl Spanned for CatchClause {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&CatchClause> for Node {
  fn from(node: &CatchClause) -> Node {
    let static_ref = unsafe { mem::transmute::<&CatchClause, &'static CatchClause>(&node) };
    Node::CatchClause(static_ref)
  }
}

impl NodeTrait for CatchClause {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.param { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.param {
      children.push(child.into());
    }
    children.push((&self.body).into());
    children
  }
}

fn get_view_for_catch_clause(ref_node: &'static swc_ecma_ast::CatchClause) -> CatchClause {
  let value = &ref_node.param;
  let field_param = match value {
    Some(value) => Some(get_view_for_pat(value)),
    None => None,
  };
  let value = &ref_node.body;
  let field_body = get_view_for_block_stmt(value);
  let mut node = CatchClause {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    param: field_param,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&CatchClause, &'static CatchClause>(&node) };
  let parent = Node::CatchClause(child_parent_ref);
  if let Some(node) = node.param.as_mut() {
    set_parent_for_pat(node, parent.clone());
  }
  node.body.parent = parent;
  node
}

pub struct LabeledStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::LabeledStmt,
  pub label: Ident,
  pub body: Box<Stmt>,
}

impl Spanned for LabeledStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&LabeledStmt> for Node {
  fn from(node: &LabeledStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&LabeledStmt, &'static LabeledStmt>(&node) };
    Node::LabeledStmt(static_ref)
  }
}

impl NodeTrait for LabeledStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.label).into());
    children.push((&self.body).into());
    children
  }
}

fn get_view_for_labeled_stmt(ref_node: &'static swc_ecma_ast::LabeledStmt) -> LabeledStmt {
  let value = &ref_node.label;
  let field_label = get_view_for_ident(value);
  let value = &ref_node.body;
  let field_body = Box::new(get_view_for_stmt(value));
  let mut node = LabeledStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    label: field_label,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&LabeledStmt, &'static LabeledStmt>(&node) };
  let parent = Node::LabeledStmt(child_parent_ref);
  node.label.parent = parent.clone();
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct ContinueStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ContinueStmt,
  pub label: Option<Ident>,
}

impl Spanned for ContinueStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ContinueStmt> for Node {
  fn from(node: &ContinueStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&ContinueStmt, &'static ContinueStmt>(&node) };
    Node::ContinueStmt(static_ref)
  }
}

impl NodeTrait for ContinueStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.label {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_continue_stmt(ref_node: &'static swc_ecma_ast::ContinueStmt) -> ContinueStmt {
  let value = &ref_node.label;
  let field_label = match value {
    Some(value) => Some(get_view_for_ident(value)),
    None => None,
  };
  let mut node = ContinueStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    label: field_label,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ContinueStmt, &'static ContinueStmt>(&node) };
  let parent = Node::ContinueStmt(child_parent_ref);
  if let Some(node) = node.label.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsConstructSignatureDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsConstructSignatureDecl,
  pub params: Vec<TsFnParam>,
  pub type_ann: Option<TsTypeAnn>,
  pub type_params: Option<TsTypeParamDecl>,
}

impl Spanned for TsConstructSignatureDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsConstructSignatureDecl> for Node {
  fn from(node: &TsConstructSignatureDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsConstructSignatureDecl, &'static TsConstructSignatureDecl>(&node) };
    Node::TsConstructSignatureDecl(static_ref)
  }
}

impl NodeTrait for TsConstructSignatureDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.params.len() + match &self.type_ann { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_construct_signature_decl(ref_node: &'static swc_ecma_ast::TsConstructSignatureDecl) -> TsConstructSignatureDecl {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_ts_fn_param(value)).collect();
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let mut node = TsConstructSignatureDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
    type_ann: field_type_ann,
    type_params: field_type_params,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsConstructSignatureDecl, &'static TsConstructSignatureDecl>(&node) };
  let parent = Node::TsConstructSignatureDecl(child_parent_ref);
  for node in node.params.iter_mut() {
    set_parent_for_ts_fn_param(node, parent.clone());
  }
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsEnumDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsEnumDecl,
  pub id: Ident,
  pub members: Vec<TsEnumMember>,
}

impl TsEnumDecl {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  pub fn is_const(&self) -> bool {
    self.inner.is_const
  }
}

impl Spanned for TsEnumDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsEnumDecl> for Node {
  fn from(node: &TsEnumDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsEnumDecl, &'static TsEnumDecl>(&node) };
    Node::TsEnumDecl(static_ref)
  }
}

impl NodeTrait for TsEnumDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.members.len());
    children.push((&self.id).into());
    for child in self.members.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_enum_decl(ref_node: &'static swc_ecma_ast::TsEnumDecl) -> TsEnumDecl {
  let value = &ref_node.id;
  let field_id = get_view_for_ident(value);
  let value = &ref_node.members;
  let field_members = value.iter().map(|value| get_view_for_ts_enum_member(value)).collect();
  let mut node = TsEnumDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
    members: field_members,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsEnumDecl, &'static TsEnumDecl>(&node) };
  let parent = Node::TsEnumDecl(child_parent_ref);
  node.id.parent = parent.clone();
  for node in node.members.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

pub struct OptChainExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::OptChainExpr,
  pub expr: Box<Expr>,
}

impl OptChainExpr {
  pub fn question_dot_token(&self) -> &swc_common::Span {
    &self.inner.question_dot_token
  }
}

impl Spanned for OptChainExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&OptChainExpr> for Node {
  fn from(node: &OptChainExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&OptChainExpr, &'static OptChainExpr>(&node) };
    Node::OptChainExpr(static_ref)
  }
}

impl NodeTrait for OptChainExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_opt_chain_expr(ref_node: &'static swc_ecma_ast::OptChainExpr) -> OptChainExpr {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = OptChainExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&OptChainExpr, &'static OptChainExpr>(&node) };
  let parent = Node::OptChainExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsNamespaceDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsNamespaceDecl,
  pub id: Ident,
  pub body: Box<TsNamespaceBody>,
}

impl TsNamespaceDecl {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  /// In TypeScript, this is only available through`node.flags`.
  pub fn global(&self) -> bool {
    self.inner.global
  }
}

impl Spanned for TsNamespaceDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsNamespaceDecl> for Node {
  fn from(node: &TsNamespaceDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsNamespaceDecl, &'static TsNamespaceDecl>(&node) };
    Node::TsNamespaceDecl(static_ref)
  }
}

impl NodeTrait for TsNamespaceDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.id).into());
    children.push((&self.body).into());
    children
  }
}

fn get_view_for_ts_namespace_decl(ref_node: &'static swc_ecma_ast::TsNamespaceDecl) -> TsNamespaceDecl {
  let value = &ref_node.id;
  let field_id = get_view_for_ident(value);
  let value = &ref_node.body;
  let field_body = Box::new(get_view_for_ts_namespace_body(value));
  let mut node = TsNamespaceDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsNamespaceDecl, &'static TsNamespaceDecl>(&node) };
  let parent = Node::TsNamespaceDecl(child_parent_ref);
  node.id.parent = parent.clone();
  set_parent_for_ts_namespace_body(&mut node.body, parent);
  node
}

pub struct SeqExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::SeqExpr,
  pub exprs: Vec<Box<Expr>>,
}

impl Spanned for SeqExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&SeqExpr> for Node {
  fn from(node: &SeqExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&SeqExpr, &'static SeqExpr>(&node) };
    Node::SeqExpr(static_ref)
  }
}

impl NodeTrait for SeqExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.exprs.len());
    for child in self.exprs.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_seq_expr(ref_node: &'static swc_ecma_ast::SeqExpr) -> SeqExpr {
  let value = &ref_node.exprs;
  let field_exprs = value.iter().map(|value| Box::new(get_view_for_expr(value))).collect();
  let mut node = SeqExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    exprs: field_exprs,
  };
  let child_parent_ref = unsafe { mem::transmute::<&SeqExpr, &'static SeqExpr>(&node) };
  let parent = Node::SeqExpr(child_parent_ref);
  for node in node.exprs.iter_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  node
}

pub struct TsExternalModuleRef {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsExternalModuleRef,
  pub expr: Str,
}

impl Spanned for TsExternalModuleRef {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsExternalModuleRef> for Node {
  fn from(node: &TsExternalModuleRef) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsExternalModuleRef, &'static TsExternalModuleRef>(&node) };
    Node::TsExternalModuleRef(static_ref)
  }
}

impl NodeTrait for TsExternalModuleRef {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_ts_external_module_ref(ref_node: &'static swc_ecma_ast::TsExternalModuleRef) -> TsExternalModuleRef {
  let value = &ref_node.expr;
  let field_expr = get_view_for_str(value);
  let mut node = TsExternalModuleRef {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsExternalModuleRef, &'static TsExternalModuleRef>(&node) };
  let parent = Node::TsExternalModuleRef(child_parent_ref);
  node.expr.parent = parent;
  node
}

pub struct TsTypeParamInstantiation {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeParamInstantiation,
  pub params: Vec<Box<TsType>>,
}

impl Spanned for TsTypeParamInstantiation {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeParamInstantiation> for Node {
  fn from(node: &TsTypeParamInstantiation) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeParamInstantiation, &'static TsTypeParamInstantiation>(&node) };
    Node::TsTypeParamInstantiation(static_ref)
  }
}

impl NodeTrait for TsTypeParamInstantiation {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.params.len());
    for child in self.params.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_type_param_instantiation(ref_node: &'static swc_ecma_ast::TsTypeParamInstantiation) -> TsTypeParamInstantiation {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| Box::new(get_view_for_ts_type(value))).collect();
  let mut node = TsTypeParamInstantiation {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeParamInstantiation, &'static TsTypeParamInstantiation>(&node) };
  let parent = Node::TsTypeParamInstantiation(child_parent_ref);
  for node in node.params.iter_mut() {
    set_parent_for_ts_type(node, parent.clone());
  }
  node
}

pub struct ReturnStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ReturnStmt,
  pub arg: Option<Box<Expr>>,
}

impl Spanned for ReturnStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ReturnStmt> for Node {
  fn from(node: &ReturnStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&ReturnStmt, &'static ReturnStmt>(&node) };
    Node::ReturnStmt(static_ref)
  }
}

impl NodeTrait for ReturnStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(match &self.arg { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.arg {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_return_stmt(ref_node: &'static swc_ecma_ast::ReturnStmt) -> ReturnStmt {
  let value = &ref_node.arg;
  let field_arg = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let mut node = ReturnStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ReturnStmt, &'static ReturnStmt>(&node) };
  let parent = Node::ReturnStmt(child_parent_ref);
  if let Some(node) = node.arg.as_mut() {
    set_parent_for_expr(node, parent);
  }
  node
}

pub struct TsTplLitType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTplLitType,
  pub types: Vec<Box<TsType>>,
  pub quasis: Vec<TplElement>,
}

impl Spanned for TsTplLitType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTplLitType> for Node {
  fn from(node: &TsTplLitType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTplLitType, &'static TsTplLitType>(&node) };
    Node::TsTplLitType(static_ref)
  }
}

impl NodeTrait for TsTplLitType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.types.len() + self.quasis.len());
    for child in self.types.iter() {
      children.push(child.into());
    }
    for child in self.quasis.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_tpl_lit_type(ref_node: &'static swc_ecma_ast::TsTplLitType) -> TsTplLitType {
  let value = &ref_node.types;
  let field_types = value.iter().map(|value| Box::new(get_view_for_ts_type(value))).collect();
  let value = &ref_node.quasis;
  let field_quasis = value.iter().map(|value| get_view_for_tpl_element(value)).collect();
  let mut node = TsTplLitType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    types: field_types,
    quasis: field_quasis,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTplLitType, &'static TsTplLitType>(&node) };
  let parent = Node::TsTplLitType(child_parent_ref);
  for node in node.types.iter_mut() {
    set_parent_for_ts_type(node, parent.clone());
  }
  for node in node.quasis.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

pub struct ExportDefaultExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ExportDefaultExpr,
  pub expr: Box<Expr>,
}

impl Spanned for ExportDefaultExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ExportDefaultExpr> for Node {
  fn from(node: &ExportDefaultExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&ExportDefaultExpr, &'static ExportDefaultExpr>(&node) };
    Node::ExportDefaultExpr(static_ref)
  }
}

impl NodeTrait for ExportDefaultExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_export_default_expr(ref_node: &'static swc_ecma_ast::ExportDefaultExpr) -> ExportDefaultExpr {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = ExportDefaultExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportDefaultExpr, &'static ExportDefaultExpr>(&node) };
  let parent = Node::ExportDefaultExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsCallSignatureDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsCallSignatureDecl,
  pub params: Vec<TsFnParam>,
  pub type_ann: Option<TsTypeAnn>,
  pub type_params: Option<TsTypeParamDecl>,
}

impl Spanned for TsCallSignatureDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsCallSignatureDecl> for Node {
  fn from(node: &TsCallSignatureDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsCallSignatureDecl, &'static TsCallSignatureDecl>(&node) };
    Node::TsCallSignatureDecl(static_ref)
  }
}

impl NodeTrait for TsCallSignatureDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.params.len() + match &self.type_ann { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_call_signature_decl(ref_node: &'static swc_ecma_ast::TsCallSignatureDecl) -> TsCallSignatureDecl {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_ts_fn_param(value)).collect();
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let mut node = TsCallSignatureDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
    type_ann: field_type_ann,
    type_params: field_type_params,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsCallSignatureDecl, &'static TsCallSignatureDecl>(&node) };
  let parent = Node::TsCallSignatureDecl(child_parent_ref);
  for node in node.params.iter_mut() {
    set_parent_for_ts_fn_param(node, parent.clone());
  }
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct AwaitExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::AwaitExpr,
  pub arg: Box<Expr>,
}

impl Spanned for AwaitExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&AwaitExpr> for Node {
  fn from(node: &AwaitExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&AwaitExpr, &'static AwaitExpr>(&node) };
    Node::AwaitExpr(static_ref)
  }
}

impl NodeTrait for AwaitExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }
}

fn get_view_for_await_expr(ref_node: &'static swc_ecma_ast::AwaitExpr) -> AwaitExpr {
  let value = &ref_node.arg;
  let field_arg = Box::new(get_view_for_expr(value));
  let mut node = AwaitExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
  };
  let child_parent_ref = unsafe { mem::transmute::<&AwaitExpr, &'static AwaitExpr>(&node) };
  let parent = Node::AwaitExpr(child_parent_ref);
  set_parent_for_expr(&mut node.arg, parent);
  node
}

pub struct ClassMethod {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ClassMethod,
  pub key: PropName,
  pub function: Function,
}

impl ClassMethod {
  pub fn kind(&self) -> &MethodKind {
    &self.inner.kind
  }

  pub fn is_static(&self) -> bool {
    self.inner.is_static
  }

  /// Typescript extension.
  pub fn accessibility(&self) -> &Option<Accessibility> {
    &self.inner.accessibility
  }

  /// Typescript extension.
  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }
}

impl Spanned for ClassMethod {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ClassMethod> for Node {
  fn from(node: &ClassMethod) -> Node {
    let static_ref = unsafe { mem::transmute::<&ClassMethod, &'static ClassMethod>(&node) };
    Node::ClassMethod(static_ref)
  }
}

impl NodeTrait for ClassMethod {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.function).into());
    children
  }
}

fn get_view_for_class_method(ref_node: &'static swc_ecma_ast::ClassMethod) -> ClassMethod {
  let value = &ref_node.key;
  let field_key = get_view_for_prop_name(value);
  let value = &ref_node.function;
  let field_function = get_view_for_function(value);
  let mut node = ClassMethod {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    function: field_function,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ClassMethod, &'static ClassMethod>(&node) };
  let parent = Node::ClassMethod(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  node.function.parent = parent;
  node
}

pub struct TsParamProp {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsParamProp,
  pub decorators: Vec<Decorator>,
  pub param: TsParamPropParam,
}

impl TsParamProp {
  /// At least one of `accessibility` or `readonly` must be set.
  pub fn accessibility(&self) -> &Option<Accessibility> {
    &self.inner.accessibility
  }

  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }
}

impl Spanned for TsParamProp {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsParamProp> for Node {
  fn from(node: &TsParamProp) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsParamProp, &'static TsParamProp>(&node) };
    Node::TsParamProp(static_ref)
  }
}

impl NodeTrait for TsParamProp {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.decorators.len());
    for child in self.decorators.iter() {
      children.push(child.into());
    }
    children.push((&self.param).into());
    children
  }
}

fn get_view_for_ts_param_prop(ref_node: &'static swc_ecma_ast::TsParamProp) -> TsParamProp {
  let value = &ref_node.decorators;
  let field_decorators = value.iter().map(|value| get_view_for_decorator(value)).collect();
  let value = &ref_node.param;
  let field_param = get_view_for_ts_param_prop_param(value);
  let mut node = TsParamProp {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    decorators: field_decorators,
    param: field_param,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsParamProp, &'static TsParamProp>(&node) };
  let parent = Node::TsParamProp(child_parent_ref);
  for node in node.decorators.iter_mut() {
    node.parent = parent.clone();
  }
  set_parent_for_ts_param_prop_param(&mut node.param, parent);
  node
}

pub struct ClassProp {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ClassProp,
  pub key: Box<Expr>,
  pub value: Option<Box<Expr>>,
  pub type_ann: Option<TsTypeAnn>,
  pub decorators: Vec<Decorator>,
}

impl ClassProp {
  pub fn is_static(&self) -> bool {
    self.inner.is_static
  }

  pub fn computed(&self) -> bool {
    self.inner.computed
  }

  /// Typescript extension.
  pub fn accessibility(&self) -> &Option<Accessibility> {
    &self.inner.accessibility
  }

  /// Typescript extension.
  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }

  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  pub fn definite(&self) -> bool {
    self.inner.definite
  }
}

impl Spanned for ClassProp {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ClassProp> for Node {
  fn from(node: &ClassProp) -> Node {
    let static_ref = unsafe { mem::transmute::<&ClassProp, &'static ClassProp>(&node) };
    Node::ClassProp(static_ref)
  }
}

impl NodeTrait for ClassProp {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, } + match &self.type_ann { Some(_value) => 1, None => 0, } + self.decorators.len());
    children.push((&self.key).into());
    if let Some(child) = &self.value {
      children.push(child.into());
    }
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    for child in self.decorators.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_class_prop(ref_node: &'static swc_ecma_ast::ClassProp) -> ClassProp {
  let value = &ref_node.key;
  let field_key = Box::new(get_view_for_expr(value));
  let value = &ref_node.value;
  let field_value = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let value = &ref_node.decorators;
  let field_decorators = value.iter().map(|value| get_view_for_decorator(value)).collect();
  let mut node = ClassProp {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    value: field_value,
    type_ann: field_type_ann,
    decorators: field_decorators,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ClassProp, &'static ClassProp>(&node) };
  let parent = Node::ClassProp(child_parent_ref);
  set_parent_for_expr(&mut node.key, parent.clone());
  if let Some(node) = node.value.as_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent.clone();
  }
  for node in node.decorators.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

pub struct TsTypeAnn {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeAnn,
  pub type_ann: Box<TsType>,
}

impl Spanned for TsTypeAnn {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeAnn> for Node {
  fn from(node: &TsTypeAnn) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeAnn, &'static TsTypeAnn>(&node) };
    Node::TsTypeAnn(static_ref)
  }
}

impl NodeTrait for TsTypeAnn {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_type_ann(ref_node: &'static swc_ecma_ast::TsTypeAnn) -> TsTypeAnn {
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsTypeAnn {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeAnn, &'static TsTypeAnn>(&node) };
  let parent = Node::TsTypeAnn(child_parent_ref);
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct ForStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ForStmt,
  pub init: Option<VarDeclOrExpr>,
  pub test: Option<Box<Expr>>,
  pub update: Option<Box<Expr>>,
  pub body: Box<Stmt>,
}

impl Spanned for ForStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ForStmt> for Node {
  fn from(node: &ForStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&ForStmt, &'static ForStmt>(&node) };
    Node::ForStmt(static_ref)
  }
}

impl NodeTrait for ForStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, } + match &self.test { Some(_value) => 1, None => 0, } + match &self.update { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.init {
      children.push(child.into());
    }
    if let Some(child) = &self.test {
      children.push(child.into());
    }
    if let Some(child) = &self.update {
      children.push(child.into());
    }
    children.push((&self.body).into());
    children
  }
}

fn get_view_for_for_stmt(ref_node: &'static swc_ecma_ast::ForStmt) -> ForStmt {
  let value = &ref_node.init;
  let field_init = match value {
    Some(value) => Some(get_view_for_var_decl_or_expr(value)),
    None => None,
  };
  let value = &ref_node.test;
  let field_test = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let value = &ref_node.update;
  let field_update = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let value = &ref_node.body;
  let field_body = Box::new(get_view_for_stmt(value));
  let mut node = ForStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    init: field_init,
    test: field_test,
    update: field_update,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ForStmt, &'static ForStmt>(&node) };
  let parent = Node::ForStmt(child_parent_ref);
  if let Some(node) = node.init.as_mut() {
    set_parent_for_var_decl_or_expr(node, parent.clone());
  }
  if let Some(node) = node.test.as_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  if let Some(node) = node.update.as_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct ObjectPat {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ObjectPat,
  pub props: Vec<ObjectPatProp>,
  pub type_ann: Option<TsTypeAnn>,
}

impl ObjectPat {
  /// Only in an ambient context
  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl Spanned for ObjectPat {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ObjectPat> for Node {
  fn from(node: &ObjectPat) -> Node {
    let static_ref = unsafe { mem::transmute::<&ObjectPat, &'static ObjectPat>(&node) };
    Node::ObjectPat(static_ref)
  }
}

impl NodeTrait for ObjectPat {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.props.len() + match &self.type_ann { Some(_value) => 1, None => 0, });
    for child in self.props.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_object_pat(ref_node: &'static swc_ecma_ast::ObjectPat) -> ObjectPat {
  let value = &ref_node.props;
  let field_props = value.iter().map(|value| get_view_for_object_pat_prop(value)).collect();
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let mut node = ObjectPat {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    props: field_props,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ObjectPat, &'static ObjectPat>(&node) };
  let parent = Node::ObjectPat(child_parent_ref);
  for node in node.props.iter_mut() {
    set_parent_for_object_pat_prop(node, parent.clone());
  }
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

/// `typeof` operator
pub struct TsTypeQuery {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeQuery,
  pub expr_name: TsTypeQueryExpr,
}

impl Spanned for TsTypeQuery {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeQuery> for Node {
  fn from(node: &TsTypeQuery) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeQuery, &'static TsTypeQuery>(&node) };
    Node::TsTypeQuery(static_ref)
  }
}

impl NodeTrait for TsTypeQuery {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr_name).into());
    children
  }
}

fn get_view_for_ts_type_query(ref_node: &'static swc_ecma_ast::TsTypeQuery) -> TsTypeQuery {
  let value = &ref_node.expr_name;
  let field_expr_name = get_view_for_ts_type_query_expr(value);
  let mut node = TsTypeQuery {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr_name: field_expr_name,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeQuery, &'static TsTypeQuery>(&node) };
  let parent = Node::TsTypeQuery(child_parent_ref);
  set_parent_for_ts_type_query_expr(&mut node.expr_name, parent);
  node
}

pub struct ThisExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ThisExpr,
}

impl Spanned for ThisExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ThisExpr> for Node {
  fn from(node: &ThisExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&ThisExpr, &'static ThisExpr>(&node) };
    Node::ThisExpr(static_ref)
  }
}

impl NodeTrait for ThisExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_this_expr(ref_node: &'static swc_ecma_ast::ThisExpr) -> ThisExpr {
  let node = ThisExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct DebuggerStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::DebuggerStmt,
}

impl Spanned for DebuggerStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&DebuggerStmt> for Node {
  fn from(node: &DebuggerStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&DebuggerStmt, &'static DebuggerStmt>(&node) };
    Node::DebuggerStmt(static_ref)
  }
}

impl NodeTrait for DebuggerStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_debugger_stmt(ref_node: &'static swc_ecma_ast::DebuggerStmt) -> DebuggerStmt {
  let node = DebuggerStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsTypeParamDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeParamDecl,
  pub params: Vec<TsTypeParam>,
}

impl Spanned for TsTypeParamDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeParamDecl> for Node {
  fn from(node: &TsTypeParamDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeParamDecl, &'static TsTypeParamDecl>(&node) };
    Node::TsTypeParamDecl(static_ref)
  }
}

impl NodeTrait for TsTypeParamDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.params.len());
    for child in self.params.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_type_param_decl(ref_node: &'static swc_ecma_ast::TsTypeParamDecl) -> TsTypeParamDecl {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_ts_type_param(value)).collect();
  let mut node = TsTypeParamDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeParamDecl, &'static TsTypeParamDecl>(&node) };
  let parent = Node::TsTypeParamDecl(child_parent_ref);
  for node in node.params.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

pub struct TsTypeAssertion {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeAssertion,
  pub expr: Box<Expr>,
  pub type_ann: Box<TsType>,
}

impl Spanned for TsTypeAssertion {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeAssertion> for Node {
  fn from(node: &TsTypeAssertion) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeAssertion, &'static TsTypeAssertion>(&node) };
    Node::TsTypeAssertion(static_ref)
  }
}

impl NodeTrait for TsTypeAssertion {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_type_assertion(ref_node: &'static swc_ecma_ast::TsTypeAssertion) -> TsTypeAssertion {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsTypeAssertion {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeAssertion, &'static TsTypeAssertion>(&node) };
  let parent = Node::TsTypeAssertion(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent.clone());
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct TplElement {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TplElement,
  pub cooked: Option<Str>,
  pub raw: Str,
}

impl TplElement {
  pub fn tail(&self) -> bool {
    self.inner.tail
  }
}

impl Spanned for TplElement {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TplElement> for Node {
  fn from(node: &TplElement) -> Node {
    let static_ref = unsafe { mem::transmute::<&TplElement, &'static TplElement>(&node) };
    Node::TplElement(static_ref)
  }
}

impl NodeTrait for TplElement {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.cooked { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.cooked {
      children.push(child.into());
    }
    children.push((&self.raw).into());
    children
  }
}

fn get_view_for_tpl_element(ref_node: &'static swc_ecma_ast::TplElement) -> TplElement {
  let value = &ref_node.cooked;
  let field_cooked = match value {
    Some(value) => Some(get_view_for_str(value)),
    None => None,
  };
  let value = &ref_node.raw;
  let field_raw = get_view_for_str(value);
  let mut node = TplElement {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    cooked: field_cooked,
    raw: field_raw,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TplElement, &'static TplElement>(&node) };
  let parent = Node::TplElement(child_parent_ref);
  if let Some(node) = node.cooked.as_mut() {
    node.parent = parent.clone();
  }
  node.raw.parent = parent;
  node
}

pub struct TsKeywordType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsKeywordType,
}

impl TsKeywordType {
  pub fn kind(&self) -> &TsKeywordTypeKind {
    &self.inner.kind
  }
}

impl Spanned for TsKeywordType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsKeywordType> for Node {
  fn from(node: &TsKeywordType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsKeywordType, &'static TsKeywordType>(&node) };
    Node::TsKeywordType(static_ref)
  }
}

impl NodeTrait for TsKeywordType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_ts_keyword_type(ref_node: &'static swc_ecma_ast::TsKeywordType) -> TsKeywordType {
  let node = TsKeywordType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct JSXSpreadChild {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXSpreadChild,
  pub expr: Box<Expr>,
}

impl Spanned for JSXSpreadChild {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXSpreadChild> for Node {
  fn from(node: &JSXSpreadChild) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXSpreadChild, &'static JSXSpreadChild>(&node) };
    Node::JSXSpreadChild(static_ref)
  }
}

impl NodeTrait for JSXSpreadChild {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_jsxspread_child(ref_node: &'static swc_ecma_ast::JSXSpreadChild) -> JSXSpreadChild {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = JSXSpreadChild {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXSpreadChild, &'static JSXSpreadChild>(&node) };
  let parent = Node::JSXSpreadChild(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsIntersectionType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsIntersectionType,
  pub types: Vec<Box<TsType>>,
}

impl Spanned for TsIntersectionType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsIntersectionType> for Node {
  fn from(node: &TsIntersectionType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsIntersectionType, &'static TsIntersectionType>(&node) };
    Node::TsIntersectionType(static_ref)
  }
}

impl NodeTrait for TsIntersectionType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.types.len());
    for child in self.types.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_intersection_type(ref_node: &'static swc_ecma_ast::TsIntersectionType) -> TsIntersectionType {
  let value = &ref_node.types;
  let field_types = value.iter().map(|value| Box::new(get_view_for_ts_type(value))).collect();
  let mut node = TsIntersectionType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    types: field_types,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsIntersectionType, &'static TsIntersectionType>(&node) };
  let parent = Node::TsIntersectionType(child_parent_ref);
  for node in node.types.iter_mut() {
    set_parent_for_ts_type(node, parent.clone());
  }
  node
}

pub struct MetaPropExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::MetaPropExpr,
  pub meta: Ident,
  pub prop: Ident,
}

impl Spanned for MetaPropExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&MetaPropExpr> for Node {
  fn from(node: &MetaPropExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&MetaPropExpr, &'static MetaPropExpr>(&node) };
    Node::MetaPropExpr(static_ref)
  }
}

impl NodeTrait for MetaPropExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.meta).into());
    children.push((&self.prop).into());
    children
  }
}

fn get_view_for_meta_prop_expr(ref_node: &'static swc_ecma_ast::MetaPropExpr) -> MetaPropExpr {
  let value = &ref_node.meta;
  let field_meta = get_view_for_ident(value);
  let value = &ref_node.prop;
  let field_prop = get_view_for_ident(value);
  let mut node = MetaPropExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    meta: field_meta,
    prop: field_prop,
  };
  let child_parent_ref = unsafe { mem::transmute::<&MetaPropExpr, &'static MetaPropExpr>(&node) };
  let parent = Node::MetaPropExpr(child_parent_ref);
  node.meta.parent = parent.clone();
  node.prop.parent = parent;
  node
}

pub struct ExprOrSpread {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ExprOrSpread,
  pub expr: Box<Expr>,
}

impl ExprOrSpread {
  pub fn spread(&self) -> &Option<swc_common::Span> {
    &self.inner.spread
  }
}

impl Spanned for ExprOrSpread {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ExprOrSpread> for Node {
  fn from(node: &ExprOrSpread) -> Node {
    let static_ref = unsafe { mem::transmute::<&ExprOrSpread, &'static ExprOrSpread>(&node) };
    Node::ExprOrSpread(static_ref)
  }
}

impl NodeTrait for ExprOrSpread {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_expr_or_spread(ref_node: &'static swc_ecma_ast::ExprOrSpread) -> ExprOrSpread {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = ExprOrSpread {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExprOrSpread, &'static ExprOrSpread>(&node) };
  let parent = Node::ExprOrSpread(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsArrayType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsArrayType,
  pub elem_type: Box<TsType>,
}

impl Spanned for TsArrayType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsArrayType> for Node {
  fn from(node: &TsArrayType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsArrayType, &'static TsArrayType>(&node) };
    Node::TsArrayType(static_ref)
  }
}

impl NodeTrait for TsArrayType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.elem_type).into());
    children
  }
}

fn get_view_for_ts_array_type(ref_node: &'static swc_ecma_ast::TsArrayType) -> TsArrayType {
  let value = &ref_node.elem_type;
  let field_elem_type = Box::new(get_view_for_ts_type(value));
  let mut node = TsArrayType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    elem_type: field_elem_type,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsArrayType, &'static TsArrayType>(&node) };
  let parent = Node::TsArrayType(child_parent_ref);
  set_parent_for_ts_type(&mut node.elem_type, parent);
  node
}

pub struct TsTypeRef {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeRef,
  pub type_name: TsEntityName,
  pub type_params: Option<TsTypeParamInstantiation>,
}

impl Spanned for TsTypeRef {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeRef> for Node {
  fn from(node: &TsTypeRef) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeRef, &'static TsTypeRef>(&node) };
    Node::TsTypeRef(static_ref)
  }
}

impl NodeTrait for TsTypeRef {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.type_name).into());
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_type_ref(ref_node: &'static swc_ecma_ast::TsTypeRef) -> TsTypeRef {
  let value = &ref_node.type_name;
  let field_type_name = get_view_for_ts_entity_name(value);
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value)),
    None => None,
  };
  let mut node = TsTypeRef {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_name: field_type_name,
    type_params: field_type_params,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeRef, &'static TsTypeRef>(&node) };
  let parent = Node::TsTypeRef(child_parent_ref);
  set_parent_for_ts_entity_name(&mut node.type_name, parent.clone());
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsThisType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsThisType,
}

impl Spanned for TsThisType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsThisType> for Node {
  fn from(node: &TsThisType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsThisType, &'static TsThisType>(&node) };
    Node::TsThisType(static_ref)
  }
}

impl NodeTrait for TsThisType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_ts_this_type(ref_node: &'static swc_ecma_ast::TsThisType) -> TsThisType {
  let node = TsThisType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TryStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TryStmt,
  pub block: BlockStmt,
  pub handler: Option<CatchClause>,
  pub finalizer: Option<BlockStmt>,
}

impl Spanned for TryStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TryStmt> for Node {
  fn from(node: &TryStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&TryStmt, &'static TryStmt>(&node) };
    Node::TryStmt(static_ref)
  }
}

impl NodeTrait for TryStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.handler { Some(_value) => 1, None => 0, } + match &self.finalizer { Some(_value) => 1, None => 0, });
    children.push((&self.block).into());
    if let Some(child) = &self.handler {
      children.push(child.into());
    }
    if let Some(child) = &self.finalizer {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_try_stmt(ref_node: &'static swc_ecma_ast::TryStmt) -> TryStmt {
  let value = &ref_node.block;
  let field_block = get_view_for_block_stmt(value);
  let value = &ref_node.handler;
  let field_handler = match value {
    Some(value) => Some(get_view_for_catch_clause(value)),
    None => None,
  };
  let value = &ref_node.finalizer;
  let field_finalizer = match value {
    Some(value) => Some(get_view_for_block_stmt(value)),
    None => None,
  };
  let mut node = TryStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    block: field_block,
    handler: field_handler,
    finalizer: field_finalizer,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TryStmt, &'static TryStmt>(&node) };
  let parent = Node::TryStmt(child_parent_ref);
  node.block.parent = parent.clone();
  if let Some(node) = node.handler.as_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.finalizer.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct CallExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::CallExpr,
  pub callee: ExprOrSuper,
  pub args: Vec<ExprOrSpread>,
  pub type_args: Option<TsTypeParamInstantiation>,
}

impl Spanned for CallExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&CallExpr> for Node {
  fn from(node: &CallExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&CallExpr, &'static CallExpr>(&node) };
    Node::CallExpr(static_ref)
  }
}

impl NodeTrait for CallExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.args.len() + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.callee).into());
    for child in self.args.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_args {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_call_expr(ref_node: &'static swc_ecma_ast::CallExpr) -> CallExpr {
  let value = &ref_node.callee;
  let field_callee = get_view_for_expr_or_super(value);
  let value = &ref_node.args;
  let field_args = value.iter().map(|value| get_view_for_expr_or_spread(value)).collect();
  let value = &ref_node.type_args;
  let field_type_args = match value {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value)),
    None => None,
  };
  let mut node = CallExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    callee: field_callee,
    args: field_args,
    type_args: field_type_args,
  };
  let child_parent_ref = unsafe { mem::transmute::<&CallExpr, &'static CallExpr>(&node) };
  let parent = Node::CallExpr(child_parent_ref);
  set_parent_for_expr_or_super(&mut node.callee, parent.clone());
  for node in node.args.iter_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.type_args.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsMappedType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsMappedType,
  pub type_param: TsTypeParam,
  pub name_type: Option<Box<TsType>>,
  pub type_ann: Option<Box<TsType>>,
}

impl TsMappedType {
  pub fn readonly(&self) -> &Option<TruePlusMinus> {
    &self.inner.readonly
  }

  pub fn optional(&self) -> &Option<TruePlusMinus> {
    &self.inner.optional
  }
}

impl Spanned for TsMappedType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsMappedType> for Node {
  fn from(node: &TsMappedType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsMappedType, &'static TsMappedType>(&node) };
    Node::TsMappedType(static_ref)
  }
}

impl NodeTrait for TsMappedType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.name_type { Some(_value) => 1, None => 0, } + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.type_param).into());
    if let Some(child) = &self.name_type {
      children.push(child.into());
    }
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_mapped_type(ref_node: &'static swc_ecma_ast::TsMappedType) -> TsMappedType {
  let value = &ref_node.type_param;
  let field_type_param = get_view_for_ts_type_param(value);
  let value = &ref_node.name_type;
  let field_name_type = match value {
    Some(value) => Some(Box::new(get_view_for_ts_type(value))),
    None => None,
  };
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(Box::new(get_view_for_ts_type(value))),
    None => None,
  };
  let mut node = TsMappedType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_param: field_type_param,
    name_type: field_name_type,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsMappedType, &'static TsMappedType>(&node) };
  let parent = Node::TsMappedType(child_parent_ref);
  node.type_param.parent = parent.clone();
  if let Some(node) = node.name_type.as_mut() {
    set_parent_for_ts_type(node, parent.clone());
  }
  if let Some(node) = node.type_ann.as_mut() {
    set_parent_for_ts_type(node, parent);
  }
  node
}

pub struct JSXExprContainer {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXExprContainer,
  pub expr: JSXExpr,
}

impl Spanned for JSXExprContainer {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXExprContainer> for Node {
  fn from(node: &JSXExprContainer) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXExprContainer, &'static JSXExprContainer>(&node) };
    Node::JSXExprContainer(static_ref)
  }
}

impl NodeTrait for JSXExprContainer {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_jsxexpr_container(ref_node: &'static swc_ecma_ast::JSXExprContainer) -> JSXExprContainer {
  let value = &ref_node.expr;
  let field_expr = get_view_for_jsxexpr(value);
  let mut node = JSXExprContainer {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXExprContainer, &'static JSXExprContainer>(&node) };
  let parent = Node::JSXExprContainer(child_parent_ref);
  set_parent_for_jsxexpr(&mut node.expr, parent);
  node
}

pub struct PrivateProp {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::PrivateProp,
  pub key: PrivateName,
  pub value: Option<Box<Expr>>,
  pub type_ann: Option<TsTypeAnn>,
  pub decorators: Vec<Decorator>,
}

impl PrivateProp {
  pub fn is_static(&self) -> bool {
    self.inner.is_static
  }

  pub fn computed(&self) -> bool {
    self.inner.computed
  }

  /// Typescript extension.
  pub fn accessibility(&self) -> &Option<Accessibility> {
    &self.inner.accessibility
  }

  /// Typescript extension.
  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }

  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }

  pub fn definite(&self) -> bool {
    self.inner.definite
  }
}

impl Spanned for PrivateProp {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&PrivateProp> for Node {
  fn from(node: &PrivateProp) -> Node {
    let static_ref = unsafe { mem::transmute::<&PrivateProp, &'static PrivateProp>(&node) };
    Node::PrivateProp(static_ref)
  }
}

impl NodeTrait for PrivateProp {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, } + match &self.type_ann { Some(_value) => 1, None => 0, } + self.decorators.len());
    children.push((&self.key).into());
    if let Some(child) = &self.value {
      children.push(child.into());
    }
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    for child in self.decorators.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_private_prop(ref_node: &'static swc_ecma_ast::PrivateProp) -> PrivateProp {
  let value = &ref_node.key;
  let field_key = get_view_for_private_name(value);
  let value = &ref_node.value;
  let field_value = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let value = &ref_node.decorators;
  let field_decorators = value.iter().map(|value| get_view_for_decorator(value)).collect();
  let mut node = PrivateProp {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    value: field_value,
    type_ann: field_type_ann,
    decorators: field_decorators,
  };
  let child_parent_ref = unsafe { mem::transmute::<&PrivateProp, &'static PrivateProp>(&node) };
  let parent = Node::PrivateProp(child_parent_ref);
  node.key.parent = parent.clone();
  if let Some(node) = node.value.as_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent.clone();
  }
  for node in node.decorators.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

/// TypeScript's own parser uses ExportAssignment for both `export default` and
/// `export =`. But for @babel/parser, `export default` is an ExportDefaultDecl,
/// so a TsExportAssignment is always `export =`.
pub struct TsExportAssignment {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsExportAssignment,
  pub expr: Box<Expr>,
}

impl Spanned for TsExportAssignment {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsExportAssignment> for Node {
  fn from(node: &TsExportAssignment) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsExportAssignment, &'static TsExportAssignment>(&node) };
    Node::TsExportAssignment(static_ref)
  }
}

impl NodeTrait for TsExportAssignment {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_ts_export_assignment(ref_node: &'static swc_ecma_ast::TsExportAssignment) -> TsExportAssignment {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = TsExportAssignment {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsExportAssignment, &'static TsExportAssignment>(&node) };
  let parent = Node::TsExportAssignment(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsInterfaceBody {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsInterfaceBody,
  pub body: Vec<TsTypeElement>,
}

impl Spanned for TsInterfaceBody {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsInterfaceBody> for Node {
  fn from(node: &TsInterfaceBody) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsInterfaceBody, &'static TsInterfaceBody>(&node) };
    Node::TsInterfaceBody(static_ref)
  }
}

impl NodeTrait for TsInterfaceBody {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.body.len());
    for child in self.body.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_interface_body(ref_node: &'static swc_ecma_ast::TsInterfaceBody) -> TsInterfaceBody {
  let value = &ref_node.body;
  let field_body = value.iter().map(|value| get_view_for_ts_type_element(value)).collect();
  let mut node = TsInterfaceBody {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsInterfaceBody, &'static TsInterfaceBody>(&node) };
  let parent = Node::TsInterfaceBody(child_parent_ref);
  for node in node.body.iter_mut() {
    set_parent_for_ts_type_element(node, parent.clone());
  }
  node
}

pub struct TsTupleElement {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTupleElement,
  /// `Ident` or `RestPat { arg: Ident }`
  pub label: Option<Pat>,
  pub ty: TsType,
}

impl Spanned for TsTupleElement {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTupleElement> for Node {
  fn from(node: &TsTupleElement) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTupleElement, &'static TsTupleElement>(&node) };
    Node::TsTupleElement(static_ref)
  }
}

impl NodeTrait for TsTupleElement {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.label {
      children.push(child.into());
    }
    children.push((&self.ty).into());
    children
  }
}

fn get_view_for_ts_tuple_element(ref_node: &'static swc_ecma_ast::TsTupleElement) -> TsTupleElement {
  let value = &ref_node.label;
  let field_label = match value {
    Some(value) => Some(get_view_for_pat(value)),
    None => None,
  };
  let value = &ref_node.ty;
  let field_ty = get_view_for_ts_type(value);
  let mut node = TsTupleElement {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    label: field_label,
    ty: field_ty,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTupleElement, &'static TsTupleElement>(&node) };
  let parent = Node::TsTupleElement(child_parent_ref);
  if let Some(node) = node.label.as_mut() {
    set_parent_for_pat(node, parent.clone());
  }
  set_parent_for_ts_type(&mut node.ty, parent);
  node
}

pub struct VarDeclarator {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::VarDeclarator,
  pub name: Pat,
  /// Initialization expresion.
  pub init: Option<Box<Expr>>,
}

impl VarDeclarator {
  /// Typescript only
  pub fn definite(&self) -> bool {
    self.inner.definite
  }
}

impl Spanned for VarDeclarator {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&VarDeclarator> for Node {
  fn from(node: &VarDeclarator) -> Node {
    let static_ref = unsafe { mem::transmute::<&VarDeclarator, &'static VarDeclarator>(&node) };
    Node::VarDeclarator(static_ref)
  }
}

impl NodeTrait for VarDeclarator {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    if let Some(child) = &self.init {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_var_declarator(ref_node: &'static swc_ecma_ast::VarDeclarator) -> VarDeclarator {
  let value = &ref_node.name;
  let field_name = get_view_for_pat(value);
  let value = &ref_node.init;
  let field_init = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let mut node = VarDeclarator {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    name: field_name,
    init: field_init,
  };
  let child_parent_ref = unsafe { mem::transmute::<&VarDeclarator, &'static VarDeclarator>(&node) };
  let parent = Node::VarDeclarator(child_parent_ref);
  set_parent_for_pat(&mut node.name, parent.clone());
  if let Some(node) = node.init.as_mut() {
    set_parent_for_expr(node, parent);
  }
  node
}

pub struct JSXMemberExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXMemberExpr,
  pub obj: JSXObject,
  pub prop: Ident,
}

impl Spanned for JSXMemberExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXMemberExpr> for Node {
  fn from(node: &JSXMemberExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXMemberExpr, &'static JSXMemberExpr>(&node) };
    Node::JSXMemberExpr(static_ref)
  }
}

impl NodeTrait for JSXMemberExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push((&self.prop).into());
    children
  }
}

fn get_view_for_jsxmember_expr(ref_node: &'static swc_ecma_ast::JSXMemberExpr) -> JSXMemberExpr {
  let value = &ref_node.obj;
  let field_obj = get_view_for_jsxobject(value);
  let value = &ref_node.prop;
  let field_prop = get_view_for_ident(value);
  let mut node = JSXMemberExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    obj: field_obj,
    prop: field_prop,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXMemberExpr, &'static JSXMemberExpr>(&node) };
  let parent = Node::JSXMemberExpr(child_parent_ref);
  set_parent_for_jsxobject(&mut node.obj, parent.clone());
  node.prop.parent = parent;
  node
}

pub struct TsConstAssertion {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsConstAssertion,
  pub expr: Box<Expr>,
}

impl Spanned for TsConstAssertion {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsConstAssertion> for Node {
  fn from(node: &TsConstAssertion) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsConstAssertion, &'static TsConstAssertion>(&node) };
    Node::TsConstAssertion(static_ref)
  }
}

impl NodeTrait for TsConstAssertion {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_ts_const_assertion(ref_node: &'static swc_ecma_ast::TsConstAssertion) -> TsConstAssertion {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = TsConstAssertion {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsConstAssertion, &'static TsConstAssertion>(&node) };
  let parent = Node::TsConstAssertion(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

/// `export * as foo from 'src';`
pub struct ExportNamespaceSpecifier {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ExportNamespaceSpecifier,
  pub name: Ident,
}

impl Spanned for ExportNamespaceSpecifier {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ExportNamespaceSpecifier> for Node {
  fn from(node: &ExportNamespaceSpecifier) -> Node {
    let static_ref = unsafe { mem::transmute::<&ExportNamespaceSpecifier, &'static ExportNamespaceSpecifier>(&node) };
    Node::ExportNamespaceSpecifier(static_ref)
  }
}

impl NodeTrait for ExportNamespaceSpecifier {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.name).into());
    children
  }
}

fn get_view_for_export_namespace_specifier(ref_node: &'static swc_ecma_ast::ExportNamespaceSpecifier) -> ExportNamespaceSpecifier {
  let value = &ref_node.name;
  let field_name = get_view_for_ident(value);
  let mut node = ExportNamespaceSpecifier {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    name: field_name,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportNamespaceSpecifier, &'static ExportNamespaceSpecifier>(&node) };
  let parent = Node::ExportNamespaceSpecifier(child_parent_ref);
  node.name.parent = parent;
  node
}

/// Object literal.
pub struct ObjectLit {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ObjectLit,
  pub props: Vec<PropOrSpread>,
}

impl Spanned for ObjectLit {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ObjectLit> for Node {
  fn from(node: &ObjectLit) -> Node {
    let static_ref = unsafe { mem::transmute::<&ObjectLit, &'static ObjectLit>(&node) };
    Node::ObjectLit(static_ref)
  }
}

impl NodeTrait for ObjectLit {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.props.len());
    for child in self.props.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_object_lit(ref_node: &'static swc_ecma_ast::ObjectLit) -> ObjectLit {
  let value = &ref_node.props;
  let field_props = value.iter().map(|value| get_view_for_prop_or_spread(value)).collect();
  let mut node = ObjectLit {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    props: field_props,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ObjectLit, &'static ObjectLit>(&node) };
  let parent = Node::ObjectLit(child_parent_ref);
  for node in node.props.iter_mut() {
    set_parent_for_prop_or_spread(node, parent.clone());
  }
  node
}

pub struct Module {
  pub inner: &'static swc_ecma_ast::Module,
  pub body: Vec<ModuleItem>,
}

impl Module {
  pub fn shebang(&self) -> &Option<swc_atoms::JsWord> {
    &self.inner.shebang
  }
}

impl Spanned for Module {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Module> for Node {
  fn from(node: &Module) -> Node {
    let static_ref = unsafe { mem::transmute::<&Module, &'static Module>(&node) };
    Node::Module(static_ref)
  }
}

impl NodeTrait for Module {
  fn parent(&self) -> Option<&Node> {
    None
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.body.len());
    for child in self.body.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_module(ref_node: &'static swc_ecma_ast::Module) -> Module {
  let value = &ref_node.body;
  let field_body = value.iter().map(|value| get_view_for_module_item(value)).collect();
  let mut node = Module {
    inner: ref_node,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&Module, &'static Module>(&node) };
  let parent = Node::Module(child_parent_ref);
  for node in node.body.iter_mut() {
    set_parent_for_module_item(node, parent.clone());
  }
  node
}

pub struct TsIndexSignature {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsIndexSignature,
  pub params: Vec<TsFnParam>,
  pub type_ann: Option<TsTypeAnn>,
}

impl TsIndexSignature {
  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }
}

impl Spanned for TsIndexSignature {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsIndexSignature> for Node {
  fn from(node: &TsIndexSignature) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsIndexSignature, &'static TsIndexSignature>(&node) };
    Node::TsIndexSignature(static_ref)
  }
}

impl NodeTrait for TsIndexSignature {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.params.len() + match &self.type_ann { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_index_signature(ref_node: &'static swc_ecma_ast::TsIndexSignature) -> TsIndexSignature {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_ts_fn_param(value)).collect();
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let mut node = TsIndexSignature {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsIndexSignature, &'static TsIndexSignature>(&node) };
  let parent = Node::TsIndexSignature(child_parent_ref);
  for node in node.params.iter_mut() {
    set_parent_for_ts_fn_param(node, parent.clone());
  }
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsTypeCastExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeCastExpr,
  pub expr: Box<Expr>,
  pub type_ann: TsTypeAnn,
}

impl Spanned for TsTypeCastExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeCastExpr> for Node {
  fn from(node: &TsTypeCastExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeCastExpr, &'static TsTypeCastExpr>(&node) };
    Node::TsTypeCastExpr(static_ref)
  }
}

impl NodeTrait for TsTypeCastExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_type_cast_expr(ref_node: &'static swc_ecma_ast::TsTypeCastExpr) -> TsTypeCastExpr {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let value = &ref_node.type_ann;
  let field_type_ann = get_view_for_ts_type_ann(value);
  let mut node = TsTypeCastExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeCastExpr, &'static TsTypeCastExpr>(&node) };
  let parent = Node::TsTypeCastExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent.clone());
  node.type_ann.parent = parent;
  node
}

pub struct TsTupleType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTupleType,
  pub elem_types: Vec<TsTupleElement>,
}

impl Spanned for TsTupleType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTupleType> for Node {
  fn from(node: &TsTupleType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTupleType, &'static TsTupleType>(&node) };
    Node::TsTupleType(static_ref)
  }
}

impl NodeTrait for TsTupleType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.elem_types.len());
    for child in self.elem_types.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_tuple_type(ref_node: &'static swc_ecma_ast::TsTupleType) -> TsTupleType {
  let value = &ref_node.elem_types;
  let field_elem_types = value.iter().map(|value| get_view_for_ts_tuple_element(value)).collect();
  let mut node = TsTupleType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    elem_types: field_elem_types,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTupleType, &'static TsTupleType>(&node) };
  let parent = Node::TsTupleType(child_parent_ref);
  for node in node.elem_types.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

pub struct Null {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Null,
}

impl Spanned for Null {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Null> for Node {
  fn from(node: &Null) -> Node {
    let static_ref = unsafe { mem::transmute::<&Null, &'static Null>(&node) };
    Node::Null(static_ref)
  }
}

impl NodeTrait for Null {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_null(ref_node: &'static swc_ecma_ast::Null) -> Null {
  let node = Null {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsTypeOperator {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeOperator,
  pub type_ann: Box<TsType>,
}

impl TsTypeOperator {
  pub fn op(&self) -> &TsTypeOperatorOp {
    &self.inner.op
  }
}

impl Spanned for TsTypeOperator {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeOperator> for Node {
  fn from(node: &TsTypeOperator) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeOperator, &'static TsTypeOperator>(&node) };
    Node::TsTypeOperator(static_ref)
  }
}

impl NodeTrait for TsTypeOperator {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_type_operator(ref_node: &'static swc_ecma_ast::TsTypeOperator) -> TsTypeOperator {
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsTypeOperator {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeOperator, &'static TsTypeOperator>(&node) };
  let parent = Node::TsTypeOperator(child_parent_ref);
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct JSXClosingElement {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXClosingElement,
  pub name: JSXElementName,
}

impl Spanned for JSXClosingElement {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXClosingElement> for Node {
  fn from(node: &JSXClosingElement) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXClosingElement, &'static JSXClosingElement>(&node) };
    Node::JSXClosingElement(static_ref)
  }
}

impl NodeTrait for JSXClosingElement {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.name).into());
    children
  }
}

fn get_view_for_jsxclosing_element(ref_node: &'static swc_ecma_ast::JSXClosingElement) -> JSXClosingElement {
  let value = &ref_node.name;
  let field_name = get_view_for_jsxelement_name(value);
  let mut node = JSXClosingElement {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    name: field_name,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXClosingElement, &'static JSXClosingElement>(&node) };
  let parent = Node::JSXClosingElement(child_parent_ref);
  set_parent_for_jsxelement_name(&mut node.name, parent);
  node
}

pub struct BinExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::BinExpr,
  pub left: Box<Expr>,
  pub right: Box<Expr>,
}

impl BinExpr {
  pub fn op(&self) -> &BinaryOp {
    &self.inner.op
  }
}

impl Spanned for BinExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&BinExpr> for Node {
  fn from(node: &BinExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&BinExpr, &'static BinExpr>(&node) };
    Node::BinExpr(static_ref)
  }
}

impl NodeTrait for BinExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children
  }
}

fn get_view_for_bin_expr(ref_node: &'static swc_ecma_ast::BinExpr) -> BinExpr {
  let value = &ref_node.left;
  let field_left = Box::new(get_view_for_expr(value));
  let value = &ref_node.right;
  let field_right = Box::new(get_view_for_expr(value));
  let mut node = BinExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    left: field_left,
    right: field_right,
  };
  let child_parent_ref = unsafe { mem::transmute::<&BinExpr, &'static BinExpr>(&node) };
  let parent = Node::BinExpr(child_parent_ref);
  set_parent_for_expr(&mut node.left, parent.clone());
  set_parent_for_expr(&mut node.right, parent);
  node
}

pub struct UnaryExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::UnaryExpr,
  pub arg: Box<Expr>,
}

impl UnaryExpr {
  pub fn op(&self) -> &UnaryOp {
    &self.inner.op
  }
}

impl Spanned for UnaryExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&UnaryExpr> for Node {
  fn from(node: &UnaryExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&UnaryExpr, &'static UnaryExpr>(&node) };
    Node::UnaryExpr(static_ref)
  }
}

impl NodeTrait for UnaryExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }
}

fn get_view_for_unary_expr(ref_node: &'static swc_ecma_ast::UnaryExpr) -> UnaryExpr {
  let value = &ref_node.arg;
  let field_arg = Box::new(get_view_for_expr(value));
  let mut node = UnaryExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
  };
  let child_parent_ref = unsafe { mem::transmute::<&UnaryExpr, &'static UnaryExpr>(&node) };
  let parent = Node::UnaryExpr(child_parent_ref);
  set_parent_for_expr(&mut node.arg, parent);
  node
}

pub struct TsPropertySignature {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsPropertySignature,
  pub key: Box<Expr>,
  pub init: Option<Box<Expr>>,
  pub params: Vec<TsFnParam>,
  pub type_ann: Option<TsTypeAnn>,
  pub type_params: Option<TsTypeParamDecl>,
}

impl TsPropertySignature {
  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }

  pub fn computed(&self) -> bool {
    self.inner.computed
  }

  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl Spanned for TsPropertySignature {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsPropertySignature> for Node {
  fn from(node: &TsPropertySignature) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsPropertySignature, &'static TsPropertySignature>(&node) };
    Node::TsPropertySignature(static_ref)
  }
}

impl NodeTrait for TsPropertySignature {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, } + self.params.len() + match &self.type_ann { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    if let Some(child) = &self.init {
      children.push(child.into());
    }
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_property_signature(ref_node: &'static swc_ecma_ast::TsPropertySignature) -> TsPropertySignature {
  let value = &ref_node.key;
  let field_key = Box::new(get_view_for_expr(value));
  let value = &ref_node.init;
  let field_init = match value {
    Some(value) => Some(Box::new(get_view_for_expr(value))),
    None => None,
  };
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_ts_fn_param(value)).collect();
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let mut node = TsPropertySignature {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    init: field_init,
    params: field_params,
    type_ann: field_type_ann,
    type_params: field_type_params,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsPropertySignature, &'static TsPropertySignature>(&node) };
  let parent = Node::TsPropertySignature(child_parent_ref);
  set_parent_for_expr(&mut node.key, parent.clone());
  if let Some(node) = node.init.as_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  for node in node.params.iter_mut() {
    set_parent_for_ts_fn_param(node, parent.clone());
  }
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct Constructor {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Constructor,
  pub key: PropName,
  pub params: Vec<ParamOrTsParamProp>,
  pub body: Option<BlockStmt>,
}

impl Constructor {
  pub fn accessibility(&self) -> &Option<Accessibility> {
    &self.inner.accessibility
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }
}

impl Spanned for Constructor {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Constructor> for Node {
  fn from(node: &Constructor) -> Node {
    let static_ref = unsafe { mem::transmute::<&Constructor, &'static Constructor>(&node) };
    Node::Constructor(static_ref)
  }
}

impl NodeTrait for Constructor {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.params.len() + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.body {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_constructor(ref_node: &'static swc_ecma_ast::Constructor) -> Constructor {
  let value = &ref_node.key;
  let field_key = get_view_for_prop_name(value);
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_param_or_ts_param_prop(value)).collect();
  let value = &ref_node.body;
  let field_body = match value {
    Some(value) => Some(get_view_for_block_stmt(value)),
    None => None,
  };
  let mut node = Constructor {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    params: field_params,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&Constructor, &'static Constructor>(&node) };
  let parent = Node::Constructor(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  for node in node.params.iter_mut() {
    set_parent_for_param_or_ts_param_prop(node, parent.clone());
  }
  if let Some(node) = node.body.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct FnDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::FnDecl,
  pub ident: Ident,
  pub function: Function,
}

impl FnDecl {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl Spanned for FnDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&FnDecl> for Node {
  fn from(node: &FnDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&FnDecl, &'static FnDecl>(&node) };
    Node::FnDecl(static_ref)
  }
}

impl NodeTrait for FnDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.ident).into());
    children.push((&self.function).into());
    children
  }
}

fn get_view_for_fn_decl(ref_node: &'static swc_ecma_ast::FnDecl) -> FnDecl {
  let value = &ref_node.ident;
  let field_ident = get_view_for_ident(value);
  let value = &ref_node.function;
  let field_function = get_view_for_function(value);
  let mut node = FnDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    ident: field_ident,
    function: field_function,
  };
  let child_parent_ref = unsafe { mem::transmute::<&FnDecl, &'static FnDecl>(&node) };
  let parent = Node::FnDecl(child_parent_ref);
  node.ident.parent = parent.clone();
  node.function.parent = parent;
  node
}

pub struct TsNonNullExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsNonNullExpr,
  pub expr: Box<Expr>,
}

impl Spanned for TsNonNullExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsNonNullExpr> for Node {
  fn from(node: &TsNonNullExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsNonNullExpr, &'static TsNonNullExpr>(&node) };
    Node::TsNonNullExpr(static_ref)
  }
}

impl NodeTrait for TsNonNullExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_ts_non_null_expr(ref_node: &'static swc_ecma_ast::TsNonNullExpr) -> TsNonNullExpr {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = TsNonNullExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsNonNullExpr, &'static TsNonNullExpr>(&node) };
  let parent = Node::TsNonNullExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

/// Class expression.
pub struct ClassExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ClassExpr,
  pub ident: Option<Ident>,
  pub class: Class,
}

impl Spanned for ClassExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ClassExpr> for Node {
  fn from(node: &ClassExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&ClassExpr, &'static ClassExpr>(&node) };
    Node::ClassExpr(static_ref)
  }
}

impl NodeTrait for ClassExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.ident { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.ident {
      children.push(child.into());
    }
    children.push((&self.class).into());
    children
  }
}

fn get_view_for_class_expr(ref_node: &'static swc_ecma_ast::ClassExpr) -> ClassExpr {
  let value = &ref_node.ident;
  let field_ident = match value {
    Some(value) => Some(get_view_for_ident(value)),
    None => None,
  };
  let value = &ref_node.class;
  let field_class = get_view_for_class(value);
  let mut node = ClassExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    ident: field_ident,
    class: field_class,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ClassExpr, &'static ClassExpr>(&node) };
  let parent = Node::ClassExpr(child_parent_ref);
  if let Some(node) = node.ident.as_mut() {
    node.parent = parent.clone();
  }
  node.class.parent = parent;
  node
}

pub struct ForInStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ForInStmt,
  pub left: VarDeclOrPat,
  pub right: Box<Expr>,
  pub body: Box<Stmt>,
}

impl Spanned for ForInStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ForInStmt> for Node {
  fn from(node: &ForInStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&ForInStmt, &'static ForInStmt>(&node) };
    Node::ForInStmt(static_ref)
  }
}

impl NodeTrait for ForInStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(3);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children.push((&self.body).into());
    children
  }
}

fn get_view_for_for_in_stmt(ref_node: &'static swc_ecma_ast::ForInStmt) -> ForInStmt {
  let value = &ref_node.left;
  let field_left = get_view_for_var_decl_or_pat(value);
  let value = &ref_node.right;
  let field_right = Box::new(get_view_for_expr(value));
  let value = &ref_node.body;
  let field_body = Box::new(get_view_for_stmt(value));
  let mut node = ForInStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    left: field_left,
    right: field_right,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ForInStmt, &'static ForInStmt>(&node) };
  let parent = Node::ForInStmt(child_parent_ref);
  set_parent_for_var_decl_or_pat(&mut node.left, parent.clone());
  set_parent_for_expr(&mut node.right, parent.clone());
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct EmptyStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::EmptyStmt,
}

impl Spanned for EmptyStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&EmptyStmt> for Node {
  fn from(node: &EmptyStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&EmptyStmt, &'static EmptyStmt>(&node) };
    Node::EmptyStmt(static_ref)
  }
}

impl NodeTrait for EmptyStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_empty_stmt(ref_node: &'static swc_ecma_ast::EmptyStmt) -> EmptyStmt {
  let node = EmptyStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct WhileStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::WhileStmt,
  pub test: Box<Expr>,
  pub body: Box<Stmt>,
}

impl Spanned for WhileStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&WhileStmt> for Node {
  fn from(node: &WhileStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&WhileStmt, &'static WhileStmt>(&node) };
    Node::WhileStmt(static_ref)
  }
}

impl NodeTrait for WhileStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.test).into());
    children.push((&self.body).into());
    children
  }
}

fn get_view_for_while_stmt(ref_node: &'static swc_ecma_ast::WhileStmt) -> WhileStmt {
  let value = &ref_node.test;
  let field_test = Box::new(get_view_for_expr(value));
  let value = &ref_node.body;
  let field_body = Box::new(get_view_for_stmt(value));
  let mut node = WhileStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    test: field_test,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&WhileStmt, &'static WhileStmt>(&node) };
  let parent = Node::WhileStmt(child_parent_ref);
  set_parent_for_expr(&mut node.test, parent.clone());
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct Str {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Str,
}

impl Str {
  pub fn value(&self) -> &swc_atoms::JsWord {
    &self.inner.value
  }

  /// This includes line escape.
  pub fn has_escape(&self) -> bool {
    self.inner.has_escape
  }
}

impl Spanned for Str {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Str> for Node {
  fn from(node: &Str) -> Node {
    let static_ref = unsafe { mem::transmute::<&Str, &'static Str>(&node) };
    Node::Str(static_ref)
  }
}

impl NodeTrait for Str {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_str(ref_node: &'static swc_ecma_ast::Str) -> Str {
  let node = Str {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsExprWithTypeArgs {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsExprWithTypeArgs,
  pub expr: TsEntityName,
  pub type_args: Option<TsTypeParamInstantiation>,
}

impl Spanned for TsExprWithTypeArgs {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsExprWithTypeArgs> for Node {
  fn from(node: &TsExprWithTypeArgs) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsExprWithTypeArgs, &'static TsExprWithTypeArgs>(&node) };
    Node::TsExprWithTypeArgs(static_ref)
  }
}

impl NodeTrait for TsExprWithTypeArgs {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.expr).into());
    if let Some(child) = &self.type_args {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_expr_with_type_args(ref_node: &'static swc_ecma_ast::TsExprWithTypeArgs) -> TsExprWithTypeArgs {
  let value = &ref_node.expr;
  let field_expr = get_view_for_ts_entity_name(value);
  let value = &ref_node.type_args;
  let field_type_args = match value {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value)),
    None => None,
  };
  let mut node = TsExprWithTypeArgs {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
    type_args: field_type_args,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsExprWithTypeArgs, &'static TsExprWithTypeArgs>(&node) };
  let parent = Node::TsExprWithTypeArgs(child_parent_ref);
  set_parent_for_ts_entity_name(&mut node.expr, parent.clone());
  if let Some(node) = node.type_args.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct AssignPat {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::AssignPat,
  pub left: Box<Pat>,
  pub right: Box<Expr>,
  pub type_ann: Option<TsTypeAnn>,
}

impl Spanned for AssignPat {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&AssignPat> for Node {
  fn from(node: &AssignPat) -> Node {
    let static_ref = unsafe { mem::transmute::<&AssignPat, &'static AssignPat>(&node) };
    Node::AssignPat(static_ref)
  }
}

impl NodeTrait for AssignPat {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.left).into());
    children.push((&self.right).into());
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_assign_pat(ref_node: &'static swc_ecma_ast::AssignPat) -> AssignPat {
  let value = &ref_node.left;
  let field_left = Box::new(get_view_for_pat(value));
  let value = &ref_node.right;
  let field_right = Box::new(get_view_for_expr(value));
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let mut node = AssignPat {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    left: field_left,
    right: field_right,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&AssignPat, &'static AssignPat>(&node) };
  let parent = Node::AssignPat(child_parent_ref);
  set_parent_for_pat(&mut node.left, parent.clone());
  set_parent_for_expr(&mut node.right, parent.clone());
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct ExportNamedSpecifier {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ExportNamedSpecifier,
  /// `foo` in `export { foo as bar }`
  pub orig: Ident,
  /// `Some(bar)` in `export { foo as bar }`
  pub exported: Option<Ident>,
}

impl Spanned for ExportNamedSpecifier {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ExportNamedSpecifier> for Node {
  fn from(node: &ExportNamedSpecifier) -> Node {
    let static_ref = unsafe { mem::transmute::<&ExportNamedSpecifier, &'static ExportNamedSpecifier>(&node) };
    Node::ExportNamedSpecifier(static_ref)
  }
}

impl NodeTrait for ExportNamedSpecifier {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.exported { Some(_value) => 1, None => 0, });
    children.push((&self.orig).into());
    if let Some(child) = &self.exported {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_export_named_specifier(ref_node: &'static swc_ecma_ast::ExportNamedSpecifier) -> ExportNamedSpecifier {
  let value = &ref_node.orig;
  let field_orig = get_view_for_ident(value);
  let value = &ref_node.exported;
  let field_exported = match value {
    Some(value) => Some(get_view_for_ident(value)),
    None => None,
  };
  let mut node = ExportNamedSpecifier {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    orig: field_orig,
    exported: field_exported,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportNamedSpecifier, &'static ExportNamedSpecifier>(&node) };
  let parent = Node::ExportNamedSpecifier(child_parent_ref);
  node.orig.parent = parent.clone();
  if let Some(node) = node.exported.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsConditionalType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsConditionalType,
  pub check_type: Box<TsType>,
  pub extends_type: Box<TsType>,
  pub true_type: Box<TsType>,
  pub false_type: Box<TsType>,
}

impl Spanned for TsConditionalType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsConditionalType> for Node {
  fn from(node: &TsConditionalType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsConditionalType, &'static TsConditionalType>(&node) };
    Node::TsConditionalType(static_ref)
  }
}

impl NodeTrait for TsConditionalType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(4);
    children.push((&self.check_type).into());
    children.push((&self.extends_type).into());
    children.push((&self.true_type).into());
    children.push((&self.false_type).into());
    children
  }
}

fn get_view_for_ts_conditional_type(ref_node: &'static swc_ecma_ast::TsConditionalType) -> TsConditionalType {
  let value = &ref_node.check_type;
  let field_check_type = Box::new(get_view_for_ts_type(value));
  let value = &ref_node.extends_type;
  let field_extends_type = Box::new(get_view_for_ts_type(value));
  let value = &ref_node.true_type;
  let field_true_type = Box::new(get_view_for_ts_type(value));
  let value = &ref_node.false_type;
  let field_false_type = Box::new(get_view_for_ts_type(value));
  let mut node = TsConditionalType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    check_type: field_check_type,
    extends_type: field_extends_type,
    true_type: field_true_type,
    false_type: field_false_type,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsConditionalType, &'static TsConditionalType>(&node) };
  let parent = Node::TsConditionalType(child_parent_ref);
  set_parent_for_ts_type(&mut node.check_type, parent.clone());
  set_parent_for_ts_type(&mut node.extends_type, parent.clone());
  set_parent_for_ts_type(&mut node.true_type, parent.clone());
  set_parent_for_ts_type(&mut node.false_type, parent);
  node
}

pub struct TsTypeLit {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsTypeLit,
  pub members: Vec<TsTypeElement>,
}

impl Spanned for TsTypeLit {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsTypeLit> for Node {
  fn from(node: &TsTypeLit) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsTypeLit, &'static TsTypeLit>(&node) };
    Node::TsTypeLit(static_ref)
  }
}

impl NodeTrait for TsTypeLit {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.members.len());
    for child in self.members.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_type_lit(ref_node: &'static swc_ecma_ast::TsTypeLit) -> TsTypeLit {
  let value = &ref_node.members;
  let field_members = value.iter().map(|value| get_view_for_ts_type_element(value)).collect();
  let mut node = TsTypeLit {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    members: field_members,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeLit, &'static TsTypeLit>(&node) };
  let parent = Node::TsTypeLit(child_parent_ref);
  for node in node.members.iter_mut() {
    set_parent_for_ts_type_element(node, parent.clone());
  }
  node
}

pub struct BreakStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::BreakStmt,
  pub label: Option<Ident>,
}

impl Spanned for BreakStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&BreakStmt> for Node {
  fn from(node: &BreakStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&BreakStmt, &'static BreakStmt>(&node) };
    Node::BreakStmt(static_ref)
  }
}

impl NodeTrait for BreakStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.label {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_break_stmt(ref_node: &'static swc_ecma_ast::BreakStmt) -> BreakStmt {
  let value = &ref_node.label;
  let field_label = match value {
    Some(value) => Some(get_view_for_ident(value)),
    None => None,
  };
  let mut node = BreakStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    label: field_label,
  };
  let child_parent_ref = unsafe { mem::transmute::<&BreakStmt, &'static BreakStmt>(&node) };
  let parent = Node::BreakStmt(child_parent_ref);
  if let Some(node) = node.label.as_mut() {
    node.parent = parent;
  }
  node
}

/// e.g. `import * as foo from 'mod.js'`.
pub struct ImportStarAsSpecifier {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ImportStarAsSpecifier,
  pub local: Ident,
}

impl Spanned for ImportStarAsSpecifier {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ImportStarAsSpecifier> for Node {
  fn from(node: &ImportStarAsSpecifier) -> Node {
    let static_ref = unsafe { mem::transmute::<&ImportStarAsSpecifier, &'static ImportStarAsSpecifier>(&node) };
    Node::ImportStarAsSpecifier(static_ref)
  }
}

impl NodeTrait for ImportStarAsSpecifier {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.local).into());
    children
  }
}

fn get_view_for_import_star_as_specifier(ref_node: &'static swc_ecma_ast::ImportStarAsSpecifier) -> ImportStarAsSpecifier {
  let value = &ref_node.local;
  let field_local = get_view_for_ident(value);
  let mut node = ImportStarAsSpecifier {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    local: field_local,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ImportStarAsSpecifier, &'static ImportStarAsSpecifier>(&node) };
  let parent = Node::ImportStarAsSpecifier(child_parent_ref);
  node.local.parent = parent;
  node
}

pub struct TsInferType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsInferType,
  pub type_param: TsTypeParam,
}

impl Spanned for TsInferType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsInferType> for Node {
  fn from(node: &TsInferType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsInferType, &'static TsInferType>(&node) };
    Node::TsInferType(static_ref)
  }
}

impl NodeTrait for TsInferType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_param).into());
    children
  }
}

fn get_view_for_ts_infer_type(ref_node: &'static swc_ecma_ast::TsInferType) -> TsInferType {
  let value = &ref_node.type_param;
  let field_type_param = get_view_for_ts_type_param(value);
  let mut node = TsInferType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_param: field_type_param,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsInferType, &'static TsInferType>(&node) };
  let parent = Node::TsInferType(child_parent_ref);
  node.type_param.parent = parent;
  node
}

pub struct PrivateMethod {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::PrivateMethod,
  pub key: PrivateName,
  pub function: Function,
}

impl PrivateMethod {
  pub fn kind(&self) -> &MethodKind {
    &self.inner.kind
  }

  pub fn is_static(&self) -> bool {
    self.inner.is_static
  }

  /// Typescript extension.
  pub fn accessibility(&self) -> &Option<Accessibility> {
    &self.inner.accessibility
  }

  /// Typescript extension.
  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }
}

impl Spanned for PrivateMethod {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&PrivateMethod> for Node {
  fn from(node: &PrivateMethod) -> Node {
    let static_ref = unsafe { mem::transmute::<&PrivateMethod, &'static PrivateMethod>(&node) };
    Node::PrivateMethod(static_ref)
  }
}

impl NodeTrait for PrivateMethod {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.function).into());
    children
  }
}

fn get_view_for_private_method(ref_node: &'static swc_ecma_ast::PrivateMethod) -> PrivateMethod {
  let value = &ref_node.key;
  let field_key = get_view_for_private_name(value);
  let value = &ref_node.function;
  let field_function = get_view_for_function(value);
  let mut node = PrivateMethod {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    function: field_function,
  };
  let child_parent_ref = unsafe { mem::transmute::<&PrivateMethod, &'static PrivateMethod>(&node) };
  let parent = Node::PrivateMethod(child_parent_ref);
  node.key.parent = parent.clone();
  node.function.parent = parent;
  node
}

pub struct ForOfStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ForOfStmt,
  pub left: VarDeclOrPat,
  pub right: Box<Expr>,
  pub body: Box<Stmt>,
}

impl ForOfStmt {
  /// Span of the await token.
  /// 
  /// es2018
  /// 
  /// for-await-of statements, e.g., `for await (const x of xs) {`
  pub fn await_token(&self) -> &Option<swc_common::Span> {
    &self.inner.await_token
  }
}

impl Spanned for ForOfStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ForOfStmt> for Node {
  fn from(node: &ForOfStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&ForOfStmt, &'static ForOfStmt>(&node) };
    Node::ForOfStmt(static_ref)
  }
}

impl NodeTrait for ForOfStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(3);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children.push((&self.body).into());
    children
  }
}

fn get_view_for_for_of_stmt(ref_node: &'static swc_ecma_ast::ForOfStmt) -> ForOfStmt {
  let value = &ref_node.left;
  let field_left = get_view_for_var_decl_or_pat(value);
  let value = &ref_node.right;
  let field_right = Box::new(get_view_for_expr(value));
  let value = &ref_node.body;
  let field_body = Box::new(get_view_for_stmt(value));
  let mut node = ForOfStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    left: field_left,
    right: field_right,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ForOfStmt, &'static ForOfStmt>(&node) };
  let parent = Node::ForOfStmt(child_parent_ref);
  set_parent_for_var_decl_or_pat(&mut node.left, parent.clone());
  set_parent_for_expr(&mut node.right, parent.clone());
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct TsUnionType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsUnionType,
  pub types: Vec<Box<TsType>>,
}

impl Spanned for TsUnionType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsUnionType> for Node {
  fn from(node: &TsUnionType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsUnionType, &'static TsUnionType>(&node) };
    Node::TsUnionType(static_ref)
  }
}

impl NodeTrait for TsUnionType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.types.len());
    for child in self.types.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_union_type(ref_node: &'static swc_ecma_ast::TsUnionType) -> TsUnionType {
  let value = &ref_node.types;
  let field_types = value.iter().map(|value| Box::new(get_view_for_ts_type(value))).collect();
  let mut node = TsUnionType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    types: field_types,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsUnionType, &'static TsUnionType>(&node) };
  let parent = Node::TsUnionType(child_parent_ref);
  for node in node.types.iter_mut() {
    set_parent_for_ts_type(node, parent.clone());
  }
  node
}

pub struct TsModuleDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsModuleDecl,
  pub id: TsModuleName,
  pub body: Option<TsNamespaceBody>,
}

impl TsModuleDecl {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  /// In TypeScript, this is only available through`node.flags`.
  pub fn global(&self) -> bool {
    self.inner.global
  }
}

impl Spanned for TsModuleDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsModuleDecl> for Node {
  fn from(node: &TsModuleDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsModuleDecl, &'static TsModuleDecl>(&node) };
    Node::TsModuleDecl(static_ref)
  }
}

impl NodeTrait for TsModuleDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.id).into());
    if let Some(child) = &self.body {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ts_module_decl(ref_node: &'static swc_ecma_ast::TsModuleDecl) -> TsModuleDecl {
  let value = &ref_node.id;
  let field_id = get_view_for_ts_module_name(value);
  let value = &ref_node.body;
  let field_body = match value {
    Some(value) => Some(get_view_for_ts_namespace_body(value)),
    None => None,
  };
  let mut node = TsModuleDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsModuleDecl, &'static TsModuleDecl>(&node) };
  let parent = Node::TsModuleDecl(child_parent_ref);
  set_parent_for_ts_module_name(&mut node.id, parent.clone());
  if let Some(node) = node.body.as_mut() {
    set_parent_for_ts_namespace_body(node, parent);
  }
  node
}

pub struct GetterProp {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::GetterProp,
  pub key: PropName,
  pub type_ann: Option<TsTypeAnn>,
  pub body: Option<BlockStmt>,
}

impl Spanned for GetterProp {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&GetterProp> for Node {
  fn from(node: &GetterProp) -> Node {
    let static_ref = unsafe { mem::transmute::<&GetterProp, &'static GetterProp>(&node) };
    Node::GetterProp(static_ref)
  }
}

impl NodeTrait for GetterProp {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, } + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    if let Some(child) = &self.body {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_getter_prop(ref_node: &'static swc_ecma_ast::GetterProp) -> GetterProp {
  let value = &ref_node.key;
  let field_key = get_view_for_prop_name(value);
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let value = &ref_node.body;
  let field_body = match value {
    Some(value) => Some(get_view_for_block_stmt(value)),
    None => None,
  };
  let mut node = GetterProp {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    type_ann: field_type_ann,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&GetterProp, &'static GetterProp>(&node) };
  let parent = Node::GetterProp(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.body.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct CondExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::CondExpr,
  pub test: Box<Expr>,
  pub cons: Box<Expr>,
  pub alt: Box<Expr>,
}

impl Spanned for CondExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&CondExpr> for Node {
  fn from(node: &CondExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&CondExpr, &'static CondExpr>(&node) };
    Node::CondExpr(static_ref)
  }
}

impl NodeTrait for CondExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(3);
    children.push((&self.test).into());
    children.push((&self.cons).into());
    children.push((&self.alt).into());
    children
  }
}

fn get_view_for_cond_expr(ref_node: &'static swc_ecma_ast::CondExpr) -> CondExpr {
  let value = &ref_node.test;
  let field_test = Box::new(get_view_for_expr(value));
  let value = &ref_node.cons;
  let field_cons = Box::new(get_view_for_expr(value));
  let value = &ref_node.alt;
  let field_alt = Box::new(get_view_for_expr(value));
  let mut node = CondExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    test: field_test,
    cons: field_cons,
    alt: field_alt,
  };
  let child_parent_ref = unsafe { mem::transmute::<&CondExpr, &'static CondExpr>(&node) };
  let parent = Node::CondExpr(child_parent_ref);
  set_parent_for_expr(&mut node.test, parent.clone());
  set_parent_for_expr(&mut node.cons, parent.clone());
  set_parent_for_expr(&mut node.alt, parent);
  node
}

/// e.g. local = foo, imported = None `import { foo } from 'mod.js'`
/// e.g. local = bar, imported = Some(foo) for `import { foo as bar } from
/// 'mod.js'`
pub struct ImportNamedSpecifier {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ImportNamedSpecifier,
  pub local: Ident,
  pub imported: Option<Ident>,
}

impl Spanned for ImportNamedSpecifier {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ImportNamedSpecifier> for Node {
  fn from(node: &ImportNamedSpecifier) -> Node {
    let static_ref = unsafe { mem::transmute::<&ImportNamedSpecifier, &'static ImportNamedSpecifier>(&node) };
    Node::ImportNamedSpecifier(static_ref)
  }
}

impl NodeTrait for ImportNamedSpecifier {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + match &self.imported { Some(_value) => 1, None => 0, });
    children.push((&self.local).into());
    if let Some(child) = &self.imported {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_import_named_specifier(ref_node: &'static swc_ecma_ast::ImportNamedSpecifier) -> ImportNamedSpecifier {
  let value = &ref_node.local;
  let field_local = get_view_for_ident(value);
  let value = &ref_node.imported;
  let field_imported = match value {
    Some(value) => Some(get_view_for_ident(value)),
    None => None,
  };
  let mut node = ImportNamedSpecifier {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    local: field_local,
    imported: field_imported,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ImportNamedSpecifier, &'static ImportNamedSpecifier>(&node) };
  let parent = Node::ImportNamedSpecifier(child_parent_ref);
  node.local.parent = parent.clone();
  if let Some(node) = node.imported.as_mut() {
    node.parent = parent;
  }
  node
}

/// `export { foo } from 'mod'`
/// `export { foo as bar } from 'mod'`
pub struct NamedExport {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::NamedExport,
  pub specifiers: Vec<ExportSpecifier>,
  pub src: Option<Str>,
}

impl NamedExport {
  pub fn type_only(&self) -> bool {
    self.inner.type_only
  }
}

impl Spanned for NamedExport {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&NamedExport> for Node {
  fn from(node: &NamedExport) -> Node {
    let static_ref = unsafe { mem::transmute::<&NamedExport, &'static NamedExport>(&node) };
    Node::NamedExport(static_ref)
  }
}

impl NodeTrait for NamedExport {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.specifiers.len() + match &self.src { Some(_value) => 1, None => 0, });
    for child in self.specifiers.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.src {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_named_export(ref_node: &'static swc_ecma_ast::NamedExport) -> NamedExport {
  let value = &ref_node.specifiers;
  let field_specifiers = value.iter().map(|value| get_view_for_export_specifier(value)).collect();
  let value = &ref_node.src;
  let field_src = match value {
    Some(value) => Some(get_view_for_str(value)),
    None => None,
  };
  let mut node = NamedExport {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    specifiers: field_specifiers,
    src: field_src,
  };
  let child_parent_ref = unsafe { mem::transmute::<&NamedExport, &'static NamedExport>(&node) };
  let parent = Node::NamedExport(child_parent_ref);
  for node in node.specifiers.iter_mut() {
    set_parent_for_export_specifier(node, parent.clone());
  }
  if let Some(node) = node.src.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct JSXElement {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXElement,
  pub opening: JSXOpeningElement,
  pub children: Vec<JSXElementChild>,
  pub closing: Option<JSXClosingElement>,
}

impl Spanned for JSXElement {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXElement> for Node {
  fn from(node: &JSXElement) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXElement, &'static JSXElement>(&node) };
    Node::JSXElement(static_ref)
  }
}

impl NodeTrait for JSXElement {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.children.len() + match &self.closing { Some(_value) => 1, None => 0, });
    children.push((&self.opening).into());
    for child in self.children.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.closing {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_jsxelement(ref_node: &'static swc_ecma_ast::JSXElement) -> JSXElement {
  let value = &ref_node.opening;
  let field_opening = get_view_for_jsxopening_element(value);
  let value = &ref_node.children;
  let field_children = value.iter().map(|value| get_view_for_jsxelement_child(value)).collect();
  let value = &ref_node.closing;
  let field_closing = match value {
    Some(value) => Some(get_view_for_jsxclosing_element(value)),
    None => None,
  };
  let mut node = JSXElement {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    opening: field_opening,
    children: field_children,
    closing: field_closing,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXElement, &'static JSXElement>(&node) };
  let parent = Node::JSXElement(child_parent_ref);
  node.opening.parent = parent.clone();
  for node in node.children.iter_mut() {
    set_parent_for_jsxelement_child(node, parent.clone());
  }
  if let Some(node) = node.closing.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct ClassDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ClassDecl,
  pub ident: Ident,
  pub class: Class,
}

impl ClassDecl {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl Spanned for ClassDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ClassDecl> for Node {
  fn from(node: &ClassDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&ClassDecl, &'static ClassDecl>(&node) };
    Node::ClassDecl(static_ref)
  }
}

impl NodeTrait for ClassDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.ident).into());
    children.push((&self.class).into());
    children
  }
}

fn get_view_for_class_decl(ref_node: &'static swc_ecma_ast::ClassDecl) -> ClassDecl {
  let value = &ref_node.ident;
  let field_ident = get_view_for_ident(value);
  let value = &ref_node.class;
  let field_class = get_view_for_class(value);
  let mut node = ClassDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    ident: field_ident,
    class: field_class,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ClassDecl, &'static ClassDecl>(&node) };
  let parent = Node::ClassDecl(child_parent_ref);
  node.ident.parent = parent.clone();
  node.class.parent = parent;
  node
}

pub struct ArrayPat {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ArrayPat,
  pub elems: Vec<Option<Pat>>,
  pub type_ann: Option<TsTypeAnn>,
}

impl ArrayPat {
  /// Only in an ambient context
  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl Spanned for ArrayPat {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ArrayPat> for Node {
  fn from(node: &ArrayPat) -> Node {
    let static_ref = unsafe { mem::transmute::<&ArrayPat, &'static ArrayPat>(&node) };
    Node::ArrayPat(static_ref)
  }
}

impl NodeTrait for ArrayPat {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.elems.len() + match &self.type_ann { Some(_value) => 1, None => 0, });
    for child in self.elems.iter() {
      if let Some(child) = &child {
        children.push(child.into());
      }
    }
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_array_pat(ref_node: &'static swc_ecma_ast::ArrayPat) -> ArrayPat {
  let value = &ref_node.elems;
  let field_elems = value.iter().map(|value| match value {
    Some(value) => Some(get_view_for_pat(value)),
    None => None,
  }).collect();
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let mut node = ArrayPat {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    elems: field_elems,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ArrayPat, &'static ArrayPat>(&node) };
  let parent = Node::ArrayPat(child_parent_ref);
  for node in node.elems.iter_mut() {
    if let Some(node) = node.as_mut() {
      set_parent_for_pat(node, parent.clone());
    }
  }
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct DoWhileStmt {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::DoWhileStmt,
  pub test: Box<Expr>,
  pub body: Box<Stmt>,
}

impl Spanned for DoWhileStmt {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&DoWhileStmt> for Node {
  fn from(node: &DoWhileStmt) -> Node {
    let static_ref = unsafe { mem::transmute::<&DoWhileStmt, &'static DoWhileStmt>(&node) };
    Node::DoWhileStmt(static_ref)
  }
}

impl NodeTrait for DoWhileStmt {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.test).into());
    children.push((&self.body).into());
    children
  }
}

fn get_view_for_do_while_stmt(ref_node: &'static swc_ecma_ast::DoWhileStmt) -> DoWhileStmt {
  let value = &ref_node.test;
  let field_test = Box::new(get_view_for_expr(value));
  let value = &ref_node.body;
  let field_body = Box::new(get_view_for_stmt(value));
  let mut node = DoWhileStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    test: field_test,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&DoWhileStmt, &'static DoWhileStmt>(&node) };
  let parent = Node::DoWhileStmt(child_parent_ref);
  set_parent_for_expr(&mut node.test, parent.clone());
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct JSXText {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXText,
}

impl JSXText {
  pub fn value(&self) -> &swc_atoms::JsWord {
    &self.inner.value
  }

  pub fn raw(&self) -> &swc_atoms::JsWord {
    &self.inner.raw
  }
}

impl Spanned for JSXText {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXText> for Node {
  fn from(node: &JSXText) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXText, &'static JSXText>(&node) };
    Node::JSXText(static_ref)
  }
}

impl NodeTrait for JSXText {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    Vec::with_capacity(0)
  }
}

fn get_view_for_jsxtext(ref_node: &'static swc_ecma_ast::JSXText) -> JSXText {
  let node = JSXText {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct VarDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::VarDecl,
  pub decls: Vec<VarDeclarator>,
}

impl VarDecl {
  pub fn kind(&self) -> &VarDeclKind {
    &self.inner.kind
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl Spanned for VarDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&VarDecl> for Node {
  fn from(node: &VarDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&VarDecl, &'static VarDecl>(&node) };
    Node::VarDecl(static_ref)
  }
}

impl NodeTrait for VarDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.decls.len());
    for child in self.decls.iter() {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_var_decl(ref_node: &'static swc_ecma_ast::VarDecl) -> VarDecl {
  let value = &ref_node.decls;
  let field_decls = value.iter().map(|value| get_view_for_var_declarator(value)).collect();
  let mut node = VarDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    decls: field_decls,
  };
  let child_parent_ref = unsafe { mem::transmute::<&VarDecl, &'static VarDecl>(&node) };
  let parent = Node::VarDecl(child_parent_ref);
  for node in node.decls.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

pub struct PrivateName {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::PrivateName,
  pub id: Ident,
}

impl Spanned for PrivateName {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&PrivateName> for Node {
  fn from(node: &PrivateName) -> Node {
    let static_ref = unsafe { mem::transmute::<&PrivateName, &'static PrivateName>(&node) };
    Node::PrivateName(static_ref)
  }
}

impl NodeTrait for PrivateName {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.id).into());
    children
  }
}

fn get_view_for_private_name(ref_node: &'static swc_ecma_ast::PrivateName) -> PrivateName {
  let value = &ref_node.id;
  let field_id = get_view_for_ident(value);
  let mut node = PrivateName {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
  };
  let child_parent_ref = unsafe { mem::transmute::<&PrivateName, &'static PrivateName>(&node) };
  let parent = Node::PrivateName(child_parent_ref);
  node.id.parent = parent;
  node
}

/// XML-based namespace syntax:
pub struct JSXNamespacedName {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXNamespacedName,
  pub ns: Ident,
  pub name: Ident,
}

impl Spanned for JSXNamespacedName {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXNamespacedName> for Node {
  fn from(node: &JSXNamespacedName) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXNamespacedName, &'static JSXNamespacedName>(&node) };
    Node::JSXNamespacedName(static_ref)
  }
}

impl NodeTrait for JSXNamespacedName {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.ns).into());
    children.push((&self.name).into());
    children
  }
}

fn get_view_for_jsxnamespaced_name(ref_node: &'static swc_ecma_ast::JSXNamespacedName) -> JSXNamespacedName {
  let value = &ref_node.ns;
  let field_ns = get_view_for_ident(value);
  let value = &ref_node.name;
  let field_name = get_view_for_ident(value);
  let mut node = JSXNamespacedName {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    ns: field_ns,
    name: field_name,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXNamespacedName, &'static JSXNamespacedName>(&node) };
  let parent = Node::JSXNamespacedName(child_parent_ref);
  node.ns.parent = parent.clone();
  node.name.parent = parent;
  node
}

pub struct JSXOpeningElement {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::JSXOpeningElement,
  pub name: JSXElementName,
  pub attrs: Vec<JSXAttrOrSpread>,
  /// Note: This field's name is differrent from one from babel because it is
  /// misleading
  pub type_args: Option<TsTypeParamInstantiation>,
}

impl JSXOpeningElement {
  pub fn self_closing(&self) -> bool {
    self.inner.self_closing
  }
}

impl Spanned for JSXOpeningElement {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&JSXOpeningElement> for Node {
  fn from(node: &JSXOpeningElement) -> Node {
    let static_ref = unsafe { mem::transmute::<&JSXOpeningElement, &'static JSXOpeningElement>(&node) };
    Node::JSXOpeningElement(static_ref)
  }
}

impl NodeTrait for JSXOpeningElement {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.attrs.len() + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    for child in self.attrs.iter() {
      children.push(child.into());
    }
    if let Some(child) = &self.type_args {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_jsxopening_element(ref_node: &'static swc_ecma_ast::JSXOpeningElement) -> JSXOpeningElement {
  let value = &ref_node.name;
  let field_name = get_view_for_jsxelement_name(value);
  let value = &ref_node.attrs;
  let field_attrs = value.iter().map(|value| get_view_for_jsxattr_or_spread(value)).collect();
  let value = &ref_node.type_args;
  let field_type_args = match value {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value)),
    None => None,
  };
  let mut node = JSXOpeningElement {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    name: field_name,
    attrs: field_attrs,
    type_args: field_type_args,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXOpeningElement, &'static JSXOpeningElement>(&node) };
  let parent = Node::JSXOpeningElement(child_parent_ref);
  set_parent_for_jsxelement_name(&mut node.name, parent.clone());
  for node in node.attrs.iter_mut() {
    set_parent_for_jsxattr_or_spread(node, parent.clone());
  }
  if let Some(node) = node.type_args.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct SpreadElement {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::SpreadElement,
  pub expr: Box<Expr>,
}

impl SpreadElement {
  pub fn dot3_token(&self) -> &swc_common::Span {
    &self.inner.dot3_token
  }
}

impl Spanned for SpreadElement {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&SpreadElement> for Node {
  fn from(node: &SpreadElement) -> Node {
    let static_ref = unsafe { mem::transmute::<&SpreadElement, &'static SpreadElement>(&node) };
    Node::SpreadElement(static_ref)
  }
}

impl NodeTrait for SpreadElement {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_spread_element(ref_node: &'static swc_ecma_ast::SpreadElement) -> SpreadElement {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = SpreadElement {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&SpreadElement, &'static SpreadElement>(&node) };
  let parent = Node::SpreadElement(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct ExportDefaultDecl {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ExportDefaultDecl,
  pub decl: DefaultDecl,
}

impl Spanned for ExportDefaultDecl {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ExportDefaultDecl> for Node {
  fn from(node: &ExportDefaultDecl) -> Node {
    let static_ref = unsafe { mem::transmute::<&ExportDefaultDecl, &'static ExportDefaultDecl>(&node) };
    Node::ExportDefaultDecl(static_ref)
  }
}

impl NodeTrait for ExportDefaultDecl {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.decl).into());
    children
  }
}

fn get_view_for_export_default_decl(ref_node: &'static swc_ecma_ast::ExportDefaultDecl) -> ExportDefaultDecl {
  let value = &ref_node.decl;
  let field_decl = get_view_for_default_decl(value);
  let mut node = ExportDefaultDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    decl: field_decl,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportDefaultDecl, &'static ExportDefaultDecl>(&node) };
  let parent = Node::ExportDefaultDecl(child_parent_ref);
  set_parent_for_default_decl(&mut node.decl, parent);
  node
}

pub struct ArrowExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ArrowExpr,
  pub params: Vec<Pat>,
  pub body: BlockStmtOrExpr,
  pub type_params: Option<TsTypeParamDecl>,
  pub return_type: Option<TsTypeAnn>,
}

impl ArrowExpr {
  pub fn is_async(&self) -> bool {
    self.inner.is_async
  }

  pub fn is_generator(&self) -> bool {
    self.inner.is_generator
  }
}

impl Spanned for ArrowExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ArrowExpr> for Node {
  fn from(node: &ArrowExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&ArrowExpr, &'static ArrowExpr>(&node) };
    Node::ArrowExpr(static_ref)
  }
}

impl NodeTrait for ArrowExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1 + self.params.len() + match &self.type_params { Some(_value) => 1, None => 0, } + match &self.return_type { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    children.push((&self.body).into());
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    if let Some(child) = &self.return_type {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_arrow_expr(ref_node: &'static swc_ecma_ast::ArrowExpr) -> ArrowExpr {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_pat(value)).collect();
  let value = &ref_node.body;
  let field_body = get_view_for_block_stmt_or_expr(value);
  let value = &ref_node.type_params;
  let field_type_params = match value {
    Some(value) => Some(get_view_for_ts_type_param_decl(value)),
    None => None,
  };
  let value = &ref_node.return_type;
  let field_return_type = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let mut node = ArrowExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
    body: field_body,
    type_params: field_type_params,
    return_type: field_return_type,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ArrowExpr, &'static ArrowExpr>(&node) };
  let parent = Node::ArrowExpr(child_parent_ref);
  for node in node.params.iter_mut() {
    set_parent_for_pat(node, parent.clone());
  }
  set_parent_for_block_stmt_or_expr(&mut node.body, parent.clone());
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent.clone();
  }
  if let Some(node) = node.return_type.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsAsExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsAsExpr,
  pub expr: Box<Expr>,
  pub type_ann: Box<TsType>,
}

impl Spanned for TsAsExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsAsExpr> for Node {
  fn from(node: &TsAsExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsAsExpr, &'static TsAsExpr>(&node) };
    Node::TsAsExpr(static_ref)
  }
}

impl NodeTrait for TsAsExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push((&self.type_ann).into());
    children
  }
}

fn get_view_for_ts_as_expr(ref_node: &'static swc_ecma_ast::TsAsExpr) -> TsAsExpr {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsAsExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsAsExpr, &'static TsAsExpr>(&node) };
  let parent = Node::TsAsExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent.clone());
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

/// `{key: value}`
pub struct KeyValuePatProp {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::KeyValuePatProp,
  pub key: PropName,
  pub value: Box<Pat>,
}

impl Spanned for KeyValuePatProp {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&KeyValuePatProp> for Node {
  fn from(node: &KeyValuePatProp) -> Node {
    let static_ref = unsafe { mem::transmute::<&KeyValuePatProp, &'static KeyValuePatProp>(&node) };
    Node::KeyValuePatProp(static_ref)
  }
}

impl NodeTrait for KeyValuePatProp {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.value).into());
    children
  }
}

fn get_view_for_key_value_pat_prop(ref_node: &'static swc_ecma_ast::KeyValuePatProp) -> KeyValuePatProp {
  let value = &ref_node.key;
  let field_key = get_view_for_prop_name(value);
  let value = &ref_node.value;
  let field_value = Box::new(get_view_for_pat(value));
  let mut node = KeyValuePatProp {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    value: field_value,
  };
  let child_parent_ref = unsafe { mem::transmute::<&KeyValuePatProp, &'static KeyValuePatProp>(&node) };
  let parent = Node::KeyValuePatProp(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  set_parent_for_pat(&mut node.value, parent);
  node
}

pub struct TsLitType {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::TsLitType,
  pub lit: TsLit,
}

impl Spanned for TsLitType {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&TsLitType> for Node {
  fn from(node: &TsLitType) -> Node {
    let static_ref = unsafe { mem::transmute::<&TsLitType, &'static TsLitType>(&node) };
    Node::TsLitType(static_ref)
  }
}

impl NodeTrait for TsLitType {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.lit).into());
    children
  }
}

fn get_view_for_ts_lit_type(ref_node: &'static swc_ecma_ast::TsLitType) -> TsLitType {
  let value = &ref_node.lit;
  let field_lit = get_view_for_ts_lit(value);
  let mut node = TsLitType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    lit: field_lit,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsLitType, &'static TsLitType>(&node) };
  let parent = Node::TsLitType(child_parent_ref);
  set_parent_for_ts_lit(&mut node.lit, parent);
  node
}

pub struct AssignExpr {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::AssignExpr,
  pub left: PatOrExpr,
  pub right: Box<Expr>,
}

impl AssignExpr {
  pub fn op(&self) -> &AssignOp {
    &self.inner.op
  }
}

impl Spanned for AssignExpr {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&AssignExpr> for Node {
  fn from(node: &AssignExpr) -> Node {
    let static_ref = unsafe { mem::transmute::<&AssignExpr, &'static AssignExpr>(&node) };
    Node::AssignExpr(static_ref)
  }
}

impl NodeTrait for AssignExpr {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children
  }
}

fn get_view_for_assign_expr(ref_node: &'static swc_ecma_ast::AssignExpr) -> AssignExpr {
  let value = &ref_node.left;
  let field_left = get_view_for_pat_or_expr(value);
  let value = &ref_node.right;
  let field_right = Box::new(get_view_for_expr(value));
  let mut node = AssignExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    left: field_left,
    right: field_right,
  };
  let child_parent_ref = unsafe { mem::transmute::<&AssignExpr, &'static AssignExpr>(&node) };
  let parent = Node::AssignExpr(child_parent_ref);
  set_parent_for_pat_or_expr(&mut node.left, parent.clone());
  set_parent_for_expr(&mut node.right, parent);
  node
}

/// Array literal.
pub struct ArrayLit {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::ArrayLit,
  pub elems: Vec<Option<ExprOrSpread>>,
}

impl Spanned for ArrayLit {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&ArrayLit> for Node {
  fn from(node: &ArrayLit) -> Node {
    let static_ref = unsafe { mem::transmute::<&ArrayLit, &'static ArrayLit>(&node) };
    Node::ArrayLit(static_ref)
  }
}

impl NodeTrait for ArrayLit {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(self.elems.len());
    for child in self.elems.iter() {
      if let Some(child) = &child {
        children.push(child.into());
      }
    }
    children
  }
}

fn get_view_for_array_lit(ref_node: &'static swc_ecma_ast::ArrayLit) -> ArrayLit {
  let value = &ref_node.elems;
  let field_elems = value.iter().map(|value| match value {
    Some(value) => Some(get_view_for_expr_or_spread(value)),
    None => None,
  }).collect();
  let mut node = ArrayLit {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    elems: field_elems,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ArrayLit, &'static ArrayLit>(&node) };
  let parent = Node::ArrayLit(child_parent_ref);
  for node in node.elems.iter_mut() {
    if let Some(node) = node.as_mut() {
      node.parent = parent.clone();
    }
  }
  node
}

pub struct Decorator {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Decorator,
  pub expr: Box<Expr>,
}

impl Spanned for Decorator {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Decorator> for Node {
  fn from(node: &Decorator) -> Node {
    let static_ref = unsafe { mem::transmute::<&Decorator, &'static Decorator>(&node) };
    Node::Decorator(static_ref)
  }
}

impl NodeTrait for Decorator {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }
}

fn get_view_for_decorator(ref_node: &'static swc_ecma_ast::Decorator) -> Decorator {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = Decorator {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&Decorator, &'static Decorator>(&node) };
  let parent = Node::Decorator(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

/// Ident with span.
pub struct Ident {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::Ident,
  pub type_ann: Option<TsTypeAnn>,
}

impl Ident {
  pub fn sym(&self) -> &swc_atoms::JsWord {
    &self.inner.sym
  }

  /// TypeScript only. Used in case of an optional parameter.
  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl Spanned for Ident {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&Ident> for Node {
  fn from(node: &Ident) -> Node {
    let static_ref = unsafe { mem::transmute::<&Ident, &'static Ident>(&node) };
    Node::Ident(static_ref)
  }
}

impl NodeTrait for Ident {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(match &self.type_ann { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }
}

fn get_view_for_ident(ref_node: &'static swc_ecma_ast::Ident) -> Ident {
  let value = &ref_node.type_ann;
  let field_type_ann = match value {
    Some(value) => Some(get_view_for_ts_type_ann(value)),
    None => None,
  };
  let mut node = Ident {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&Ident, &'static Ident>(&node) };
  let parent = Node::Ident(child_parent_ref);
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct MethodProp {
  pub parent: Node,
  pub inner: &'static swc_ecma_ast::MethodProp,
  pub key: PropName,
  pub function: Function,
}

impl Spanned for MethodProp {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl From<&MethodProp> for Node {
  fn from(node: &MethodProp) -> Node {
    let static_ref = unsafe { mem::transmute::<&MethodProp, &'static MethodProp>(&node) };
    Node::MethodProp(static_ref)
  }
}

impl NodeTrait for MethodProp {
  fn parent(&self) -> Option<&Node> {
    Some(&self.parent)
  }

  fn children(&self) -> Vec<Node> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.function).into());
    children
  }
}

fn get_view_for_method_prop(ref_node: &'static swc_ecma_ast::MethodProp) -> MethodProp {
  let value = &ref_node.key;
  let field_key = get_view_for_prop_name(value);
  let value = &ref_node.function;
  let field_function = get_view_for_function(value);
  let mut node = MethodProp {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    key: field_key,
    function: field_function,
  };
  let child_parent_ref = unsafe { mem::transmute::<&MethodProp, &'static MethodProp>(&node) };
  let parent = Node::MethodProp(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  node.function.parent = parent;
  node
}
