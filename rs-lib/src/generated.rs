// This code is code generated.
// Run `deno run -A generation/main.ts` from the root directory to regenerate it.
use std::mem::{self, MaybeUninit};
use swc_common::{Span, Spanned};
use swc_ecma_ast::{VarDeclKind, TsTypeOperatorOp, TsKeywordTypeKind, BinaryOp, AssignOp, UpdateOp, Accessibility, MethodKind, UnaryOp, TruePlusMinus};
use crate::types::*;

pub fn with_ast_view<'a>(swc_module: swc_ecma_ast::Module, with_view: impl Fn(Module<'a>) -> Module<'a>) -> swc_ecma_ast::Module {
  let swc_module_ref = unsafe { mem::transmute::<&swc_ecma_ast::Module, &'a swc_ecma_ast::Module>(&swc_module) };
  let module = get_view_for_module(swc_module_ref);
  let _ = with_view(module);
  swc_module
}

impl<'a> From<&Box<Expr<'a>>> for Node<'a> {
  fn from(boxed_node: &Box<Expr<'a>>) -> Node<'a> {
    (&**boxed_node).into()
  }
}

impl<'a> From<&Box<JSXElement<'a>>> for Node<'a> {
  fn from(boxed_node: &Box<JSXElement<'a>>) -> Node<'a> {
    (&**boxed_node).into()
  }
}

impl<'a> From<&Box<JSXMemberExpr<'a>>> for Node<'a> {
  fn from(boxed_node: &Box<JSXMemberExpr<'a>>) -> Node<'a> {
    (&**boxed_node).into()
  }
}

impl<'a> From<&Box<Pat<'a>>> for Node<'a> {
  fn from(boxed_node: &Box<Pat<'a>>) -> Node<'a> {
    (&**boxed_node).into()
  }
}

impl<'a> From<&Box<Prop<'a>>> for Node<'a> {
  fn from(boxed_node: &Box<Prop<'a>>) -> Node<'a> {
    (&**boxed_node).into()
  }
}

impl<'a> From<&Box<Stmt<'a>>> for Node<'a> {
  fn from(boxed_node: &Box<Stmt<'a>>) -> Node<'a> {
    (&**boxed_node).into()
  }
}

impl<'a> From<&Box<TsNamespaceBody<'a>>> for Node<'a> {
  fn from(boxed_node: &Box<TsNamespaceBody<'a>>) -> Node<'a> {
    (&**boxed_node).into()
  }
}

impl<'a> From<&Box<TsQualifiedName<'a>>> for Node<'a> {
  fn from(boxed_node: &Box<TsQualifiedName<'a>>) -> Node<'a> {
    (&**boxed_node).into()
  }
}

impl<'a> From<&Box<TsType<'a>>> for Node<'a> {
  fn from(boxed_node: &Box<TsType<'a>>) -> Node<'a> {
    (&**boxed_node).into()
  }
}

#[derive(Clone)]
pub enum Node<'a> {
  SwitchCase(&'a SwitchCase<'a>),
  ThrowStmt(&'a ThrowStmt<'a>),
  JSXClosingFragment(&'a JSXClosingFragment<'a>),
  BigInt(&'a BigInt<'a>),
  ExportDefaultSpecifier(&'a ExportDefaultSpecifier<'a>),
  TsTypeParam(&'a TsTypeParam<'a>),
  WithStmt(&'a WithStmt<'a>),
  Regex(&'a Regex<'a>),
  TsMethodSignature(&'a TsMethodSignature<'a>),
  UpdateExpr(&'a UpdateExpr<'a>),
  SetterProp(&'a SetterProp<'a>),
  TaggedTpl(&'a TaggedTpl<'a>),
  ExportAll(&'a ExportAll<'a>),
  TsModuleBlock(&'a TsModuleBlock<'a>),
  SwitchStmt(&'a SwitchStmt<'a>),
  TsEnumMember(&'a TsEnumMember<'a>),
  TsIndexedAccessType(&'a TsIndexedAccessType<'a>),
  TsRestType(&'a TsRestType<'a>),
  ExprStmt(&'a ExprStmt<'a>),
  TsOptionalType(&'a TsOptionalType<'a>),
  Tpl(&'a Tpl<'a>),
  Invalid(&'a Invalid<'a>),
  ComputedPropName(&'a ComputedPropName<'a>),
  TsFnType(&'a TsFnType<'a>),
  BlockStmt(&'a BlockStmt<'a>),
  TsTypeAliasDecl(&'a TsTypeAliasDecl<'a>),
  MemberExpr(&'a MemberExpr<'a>),
  Function(&'a Function<'a>),
  ImportDecl(&'a ImportDecl<'a>),
  TsTypePredicate(&'a TsTypePredicate<'a>),
  YieldExpr(&'a YieldExpr<'a>),
  KeyValueProp(&'a KeyValueProp<'a>),
  Param(&'a Param<'a>),
  JSXFragment(&'a JSXFragment<'a>),
  ImportDefaultSpecifier(&'a ImportDefaultSpecifier<'a>),
  Number(&'a Number<'a>),
  JSXAttr(&'a JSXAttr<'a>),
  ParenExpr(&'a ParenExpr<'a>),
  Super(&'a Super<'a>),
  TsConstructorType(&'a TsConstructorType<'a>),
  Class(&'a Class<'a>),
  RestPat(&'a RestPat<'a>),
  TsNamespaceExportDecl(&'a TsNamespaceExportDecl<'a>),
  JSXOpeningFragment(&'a JSXOpeningFragment<'a>),
  NewExpr(&'a NewExpr<'a>),
  FnExpr(&'a FnExpr<'a>),
  IfStmt(&'a IfStmt<'a>),
  TsParenthesizedType(&'a TsParenthesizedType<'a>),
  AssignPatProp(&'a AssignPatProp<'a>),
  TsImportType(&'a TsImportType<'a>),
  Bool(&'a Bool<'a>),
  TsImportEqualsDecl(&'a TsImportEqualsDecl<'a>),
  AssignProp(&'a AssignProp<'a>),
  TsInterfaceDecl(&'a TsInterfaceDecl<'a>),
  JSXEmptyExpr(&'a JSXEmptyExpr<'a>),
  TsQualifiedName(&'a TsQualifiedName<'a>),
  ExportDecl(&'a ExportDecl<'a>),
  CatchClause(&'a CatchClause<'a>),
  LabeledStmt(&'a LabeledStmt<'a>),
  ContinueStmt(&'a ContinueStmt<'a>),
  TsConstructSignatureDecl(&'a TsConstructSignatureDecl<'a>),
  TsEnumDecl(&'a TsEnumDecl<'a>),
  OptChainExpr(&'a OptChainExpr<'a>),
  TsNamespaceDecl(&'a TsNamespaceDecl<'a>),
  SeqExpr(&'a SeqExpr<'a>),
  TsExternalModuleRef(&'a TsExternalModuleRef<'a>),
  TsTypeParamInstantiation(&'a TsTypeParamInstantiation<'a>),
  ReturnStmt(&'a ReturnStmt<'a>),
  TsTplLitType(&'a TsTplLitType<'a>),
  ExportDefaultExpr(&'a ExportDefaultExpr<'a>),
  TsCallSignatureDecl(&'a TsCallSignatureDecl<'a>),
  AwaitExpr(&'a AwaitExpr<'a>),
  ClassMethod(&'a ClassMethod<'a>),
  TsParamProp(&'a TsParamProp<'a>),
  ClassProp(&'a ClassProp<'a>),
  TsTypeAnn(&'a TsTypeAnn<'a>),
  ForStmt(&'a ForStmt<'a>),
  ObjectPat(&'a ObjectPat<'a>),
  TsTypeQuery(&'a TsTypeQuery<'a>),
  ThisExpr(&'a ThisExpr<'a>),
  DebuggerStmt(&'a DebuggerStmt<'a>),
  TsTypeParamDecl(&'a TsTypeParamDecl<'a>),
  TsTypeAssertion(&'a TsTypeAssertion<'a>),
  TplElement(&'a TplElement<'a>),
  TsKeywordType(&'a TsKeywordType<'a>),
  JSXSpreadChild(&'a JSXSpreadChild<'a>),
  TsIntersectionType(&'a TsIntersectionType<'a>),
  MetaPropExpr(&'a MetaPropExpr<'a>),
  ExprOrSpread(&'a ExprOrSpread<'a>),
  TsArrayType(&'a TsArrayType<'a>),
  TsTypeRef(&'a TsTypeRef<'a>),
  TsThisType(&'a TsThisType<'a>),
  TryStmt(&'a TryStmt<'a>),
  CallExpr(&'a CallExpr<'a>),
  TsMappedType(&'a TsMappedType<'a>),
  JSXExprContainer(&'a JSXExprContainer<'a>),
  PrivateProp(&'a PrivateProp<'a>),
  TsExportAssignment(&'a TsExportAssignment<'a>),
  TsInterfaceBody(&'a TsInterfaceBody<'a>),
  TsTupleElement(&'a TsTupleElement<'a>),
  VarDeclarator(&'a VarDeclarator<'a>),
  JSXMemberExpr(&'a JSXMemberExpr<'a>),
  TsConstAssertion(&'a TsConstAssertion<'a>),
  ExportNamespaceSpecifier(&'a ExportNamespaceSpecifier<'a>),
  ObjectLit(&'a ObjectLit<'a>),
  Module(&'a Module<'a>),
  TsIndexSignature(&'a TsIndexSignature<'a>),
  TsTypeCastExpr(&'a TsTypeCastExpr<'a>),
  TsTupleType(&'a TsTupleType<'a>),
  Null(&'a Null<'a>),
  TsTypeOperator(&'a TsTypeOperator<'a>),
  JSXClosingElement(&'a JSXClosingElement<'a>),
  BinExpr(&'a BinExpr<'a>),
  UnaryExpr(&'a UnaryExpr<'a>),
  TsPropertySignature(&'a TsPropertySignature<'a>),
  Constructor(&'a Constructor<'a>),
  FnDecl(&'a FnDecl<'a>),
  TsNonNullExpr(&'a TsNonNullExpr<'a>),
  ClassExpr(&'a ClassExpr<'a>),
  ForInStmt(&'a ForInStmt<'a>),
  EmptyStmt(&'a EmptyStmt<'a>),
  WhileStmt(&'a WhileStmt<'a>),
  Str(&'a Str<'a>),
  TsExprWithTypeArgs(&'a TsExprWithTypeArgs<'a>),
  AssignPat(&'a AssignPat<'a>),
  ExportNamedSpecifier(&'a ExportNamedSpecifier<'a>),
  TsConditionalType(&'a TsConditionalType<'a>),
  TsTypeLit(&'a TsTypeLit<'a>),
  BreakStmt(&'a BreakStmt<'a>),
  ImportStarAsSpecifier(&'a ImportStarAsSpecifier<'a>),
  TsInferType(&'a TsInferType<'a>),
  PrivateMethod(&'a PrivateMethod<'a>),
  ForOfStmt(&'a ForOfStmt<'a>),
  TsUnionType(&'a TsUnionType<'a>),
  TsModuleDecl(&'a TsModuleDecl<'a>),
  GetterProp(&'a GetterProp<'a>),
  CondExpr(&'a CondExpr<'a>),
  ImportNamedSpecifier(&'a ImportNamedSpecifier<'a>),
  NamedExport(&'a NamedExport<'a>),
  JSXElement(&'a JSXElement<'a>),
  ClassDecl(&'a ClassDecl<'a>),
  ArrayPat(&'a ArrayPat<'a>),
  DoWhileStmt(&'a DoWhileStmt<'a>),
  JSXText(&'a JSXText<'a>),
  VarDecl(&'a VarDecl<'a>),
  PrivateName(&'a PrivateName<'a>),
  JSXNamespacedName(&'a JSXNamespacedName<'a>),
  JSXOpeningElement(&'a JSXOpeningElement<'a>),
  SpreadElement(&'a SpreadElement<'a>),
  ExportDefaultDecl(&'a ExportDefaultDecl<'a>),
  ArrowExpr(&'a ArrowExpr<'a>),
  TsAsExpr(&'a TsAsExpr<'a>),
  KeyValuePatProp(&'a KeyValuePatProp<'a>),
  TsLitType(&'a TsLitType<'a>),
  AssignExpr(&'a AssignExpr<'a>),
  ArrayLit(&'a ArrayLit<'a>),
  Decorator(&'a Decorator<'a>),
  Ident(&'a Ident<'a>),
  MethodProp(&'a MethodProp<'a>),
}

impl<'a> Node<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(self)
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(self).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for Node<'a> {
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
impl<'a> NodeTrait<'a> for Node<'a> {
  fn parent(&self) -> Option<Node<'a>> {
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

  fn children(&self) -> Vec<Node<'a>> {
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

pub enum JSXAttrValue<'a> {
  Lit(Lit<'a>),
  JSXExprContainer(JSXExprContainer<'a>),
  JSXElement(Box<JSXElement<'a>>),
  JSXFragment(JSXFragment<'a>),
}

impl<'a> JSXAttrValue<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for JSXAttrValue<'a> {
  fn span(&self) -> Span {
    match self {
      JSXAttrValue::Lit(node) => node.span(),
      JSXAttrValue::JSXExprContainer(node) => node.span(),
      JSXAttrValue::JSXElement(node) => node.span(),
      JSXAttrValue::JSXFragment(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXAttrValue<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXAttrValue::Lit(node) => node.parent(),
      JSXAttrValue::JSXExprContainer(node) => node.parent(),
      JSXAttrValue::JSXElement(node) => node.parent(),
      JSXAttrValue::JSXFragment(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXAttrValue::Lit(node) => node.children(),
      JSXAttrValue::JSXExprContainer(node) => node.children(),
      JSXAttrValue::JSXElement(node) => node.children(),
      JSXAttrValue::JSXFragment(node) => node.children(),
    }
  }
}
impl<'a> From<&JSXAttrValue<'a>> for Node<'a> {
  fn from(node: &JSXAttrValue<'a>) -> Node<'a> {
    match node {
      JSXAttrValue::Lit(node) => node.into(),
      JSXAttrValue::JSXExprContainer(node) => node.into(),
      JSXAttrValue::JSXElement(node) => node.into(),
      JSXAttrValue::JSXFragment(node) => node.into(),
    }
  }
}

fn get_view_for_jsxattr_value<'a>(ref_node: &'a swc_ecma_ast::JSXAttrValue) -> JSXAttrValue<'a> {
  match ref_node {
    swc_ecma_ast::JSXAttrValue::Lit(value) => JSXAttrValue::Lit(get_view_for_lit(value)),
    swc_ecma_ast::JSXAttrValue::JSXExprContainer(value) => JSXAttrValue::JSXExprContainer(get_view_for_jsxexpr_container(value)),
    swc_ecma_ast::JSXAttrValue::JSXElement(value) => JSXAttrValue::JSXElement(Box::new(get_view_for_jsxelement(value))),
    swc_ecma_ast::JSXAttrValue::JSXFragment(value) => JSXAttrValue::JSXFragment(get_view_for_jsxfragment(value)),
  }
}

fn set_parent_for_jsxattr_value<'a>(node: &mut JSXAttrValue<'a>, parent: Node<'a>) {
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

pub enum PropOrSpread<'a> {
  /// Spread properties, e.g., `{a: 1, ...obj, b: 2}`.
  Spread(SpreadElement<'a>),
  Prop(Box<Prop<'a>>),
}

impl<'a> PropOrSpread<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for PropOrSpread<'a> {
  fn span(&self) -> Span {
    match self {
      PropOrSpread::Spread(node) => node.span(),
      PropOrSpread::Prop(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for PropOrSpread<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      PropOrSpread::Spread(node) => node.parent(),
      PropOrSpread::Prop(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      PropOrSpread::Spread(node) => node.children(),
      PropOrSpread::Prop(node) => node.children(),
    }
  }
}
impl<'a> From<&PropOrSpread<'a>> for Node<'a> {
  fn from(node: &PropOrSpread<'a>) -> Node<'a> {
    match node {
      PropOrSpread::Spread(node) => node.into(),
      PropOrSpread::Prop(node) => node.into(),
    }
  }
}

fn get_view_for_prop_or_spread<'a>(ref_node: &'a swc_ecma_ast::PropOrSpread) -> PropOrSpread<'a> {
  match ref_node {
    swc_ecma_ast::PropOrSpread::Spread(value) => PropOrSpread::Spread(get_view_for_spread_element(value)),
    swc_ecma_ast::PropOrSpread::Prop(value) => PropOrSpread::Prop(Box::new(get_view_for_prop(value))),
  }
}

fn set_parent_for_prop_or_spread<'a>(node: &mut PropOrSpread<'a>, parent: Node<'a>) {
  match node {
    PropOrSpread::Spread(node) => {
      node.parent = parent;
    },
    PropOrSpread::Prop(node) => {
      set_parent_for_prop(node, parent);
    },
  }
}

pub enum VarDeclOrExpr<'a> {
  VarDecl(VarDecl<'a>),
  Expr(Box<Expr<'a>>),
}

impl<'a> VarDeclOrExpr<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for VarDeclOrExpr<'a> {
  fn span(&self) -> Span {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.span(),
      VarDeclOrExpr::Expr(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for VarDeclOrExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.parent(),
      VarDeclOrExpr::Expr(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.children(),
      VarDeclOrExpr::Expr(node) => node.children(),
    }
  }
}
impl<'a> From<&VarDeclOrExpr<'a>> for Node<'a> {
  fn from(node: &VarDeclOrExpr<'a>) -> Node<'a> {
    match node {
      VarDeclOrExpr::VarDecl(node) => node.into(),
      VarDeclOrExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_var_decl_or_expr<'a>(ref_node: &'a swc_ecma_ast::VarDeclOrExpr) -> VarDeclOrExpr<'a> {
  match ref_node {
    swc_ecma_ast::VarDeclOrExpr::VarDecl(value) => VarDeclOrExpr::VarDecl(get_view_for_var_decl(value)),
    swc_ecma_ast::VarDeclOrExpr::Expr(value) => VarDeclOrExpr::Expr(Box::new(get_view_for_expr(value))),
  }
}

fn set_parent_for_var_decl_or_expr<'a>(node: &mut VarDeclOrExpr<'a>, parent: Node<'a>) {
  match node {
    VarDeclOrExpr::VarDecl(node) => {
      node.parent = parent;
    },
    VarDeclOrExpr::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
  }
}

pub enum TsThisTypeOrIdent<'a> {
  TsThisType(TsThisType<'a>),
  Ident(Ident<'a>),
}

impl<'a> TsThisTypeOrIdent<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsThisTypeOrIdent<'a> {
  fn span(&self) -> Span {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.span(),
      TsThisTypeOrIdent::Ident(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsThisTypeOrIdent<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.parent(),
      TsThisTypeOrIdent::Ident(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.children(),
      TsThisTypeOrIdent::Ident(node) => node.children(),
    }
  }
}
impl<'a> From<&TsThisTypeOrIdent<'a>> for Node<'a> {
  fn from(node: &TsThisTypeOrIdent<'a>) -> Node<'a> {
    match node {
      TsThisTypeOrIdent::TsThisType(node) => node.into(),
      TsThisTypeOrIdent::Ident(node) => node.into(),
    }
  }
}

fn get_view_for_ts_this_type_or_ident<'a>(ref_node: &'a swc_ecma_ast::TsThisTypeOrIdent) -> TsThisTypeOrIdent<'a> {
  match ref_node {
    swc_ecma_ast::TsThisTypeOrIdent::TsThisType(value) => TsThisTypeOrIdent::TsThisType(get_view_for_ts_this_type(value)),
    swc_ecma_ast::TsThisTypeOrIdent::Ident(value) => TsThisTypeOrIdent::Ident(get_view_for_ident(value)),
  }
}

fn set_parent_for_ts_this_type_or_ident<'a>(node: &mut TsThisTypeOrIdent<'a>, parent: Node<'a>) {
  match node {
    TsThisTypeOrIdent::TsThisType(node) => {
      node.parent = parent;
    },
    TsThisTypeOrIdent::Ident(node) => {
      node.parent = parent;
    },
  }
}

pub enum Prop<'a> {
  /// `a` in `{ a, }`
  Shorthand(Ident<'a>),
  /// `key: value` in `{ key: value, }`
  KeyValue(KeyValueProp<'a>),
  /// This is **invalid** for object literal.
  Assign(AssignProp<'a>),
  Getter(GetterProp<'a>),
  Setter(SetterProp<'a>),
  Method(MethodProp<'a>),
}

impl<'a> Prop<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for Prop<'a> {
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

impl<'a> NodeTrait<'a> for Prop<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      Prop::Shorthand(node) => node.parent(),
      Prop::KeyValue(node) => node.parent(),
      Prop::Assign(node) => node.parent(),
      Prop::Getter(node) => node.parent(),
      Prop::Setter(node) => node.parent(),
      Prop::Method(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
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
impl<'a> From<&Prop<'a>> for Node<'a> {
  fn from(node: &Prop<'a>) -> Node<'a> {
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

fn get_view_for_prop<'a>(ref_node: &'a swc_ecma_ast::Prop) -> Prop<'a> {
  match ref_node {
    swc_ecma_ast::Prop::Shorthand(value) => Prop::Shorthand(get_view_for_ident(value)),
    swc_ecma_ast::Prop::KeyValue(value) => Prop::KeyValue(get_view_for_key_value_prop(value)),
    swc_ecma_ast::Prop::Assign(value) => Prop::Assign(get_view_for_assign_prop(value)),
    swc_ecma_ast::Prop::Getter(value) => Prop::Getter(get_view_for_getter_prop(value)),
    swc_ecma_ast::Prop::Setter(value) => Prop::Setter(get_view_for_setter_prop(value)),
    swc_ecma_ast::Prop::Method(value) => Prop::Method(get_view_for_method_prop(value)),
  }
}

fn set_parent_for_prop<'a>(node: &mut Prop<'a>, parent: Node<'a>) {
  match node {
    Prop::Shorthand(node) => {
      node.parent = parent;
    },
    Prop::KeyValue(node) => {
      node.parent = parent.to::<ObjectLit>();
    },
    Prop::Assign(node) => {
      node.parent = parent.to::<ObjectLit>();
    },
    Prop::Getter(node) => {
      node.parent = parent.to::<ObjectLit>();
    },
    Prop::Setter(node) => {
      node.parent = parent.to::<ObjectLit>();
    },
    Prop::Method(node) => {
      node.parent = parent.to::<ObjectLit>();
    },
  }
}

pub enum TsTypeQueryExpr<'a> {
  TsEntityName(TsEntityName<'a>),
  Import(TsImportType<'a>),
}

impl<'a> TsTypeQueryExpr<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsTypeQueryExpr<'a> {
  fn span(&self) -> Span {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.span(),
      TsTypeQueryExpr::Import(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsTypeQueryExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.parent(),
      TsTypeQueryExpr::Import(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.children(),
      TsTypeQueryExpr::Import(node) => node.children(),
    }
  }
}
impl<'a> From<&TsTypeQueryExpr<'a>> for Node<'a> {
  fn from(node: &TsTypeQueryExpr<'a>) -> Node<'a> {
    match node {
      TsTypeQueryExpr::TsEntityName(node) => node.into(),
      TsTypeQueryExpr::Import(node) => node.into(),
    }
  }
}

fn get_view_for_ts_type_query_expr<'a>(ref_node: &'a swc_ecma_ast::TsTypeQueryExpr) -> TsTypeQueryExpr<'a> {
  match ref_node {
    swc_ecma_ast::TsTypeQueryExpr::TsEntityName(value) => TsTypeQueryExpr::TsEntityName(get_view_for_ts_entity_name(value)),
    swc_ecma_ast::TsTypeQueryExpr::Import(value) => TsTypeQueryExpr::Import(get_view_for_ts_import_type(value)),
  }
}

fn set_parent_for_ts_type_query_expr<'a>(node: &mut TsTypeQueryExpr<'a>, parent: Node<'a>) {
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
pub enum TsNamespaceBody<'a> {
  TsModuleBlock(TsModuleBlock<'a>),
  TsNamespaceDecl(TsNamespaceDecl<'a>),
}

impl<'a> TsNamespaceBody<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsNamespaceBody<'a> {
  fn span(&self) -> Span {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.span(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsNamespaceBody<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.parent(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.children(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.children(),
    }
  }
}
impl<'a> From<&TsNamespaceBody<'a>> for Node<'a> {
  fn from(node: &TsNamespaceBody<'a>) -> Node<'a> {
    match node {
      TsNamespaceBody::TsModuleBlock(node) => node.into(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.into(),
    }
  }
}

fn get_view_for_ts_namespace_body<'a>(ref_node: &'a swc_ecma_ast::TsNamespaceBody) -> TsNamespaceBody<'a> {
  match ref_node {
    swc_ecma_ast::TsNamespaceBody::TsModuleBlock(value) => TsNamespaceBody::TsModuleBlock(get_view_for_ts_module_block(value)),
    swc_ecma_ast::TsNamespaceBody::TsNamespaceDecl(value) => TsNamespaceBody::TsNamespaceDecl(get_view_for_ts_namespace_decl(value)),
  }
}

fn set_parent_for_ts_namespace_body<'a>(node: &mut TsNamespaceBody<'a>, parent: Node<'a>) {
  match node {
    TsNamespaceBody::TsModuleBlock(node) => {
      node.parent = parent;
    },
    TsNamespaceBody::TsNamespaceDecl(node) => {
      node.parent = parent;
    },
  }
}

pub enum Lit<'a> {
  Str(Str<'a>),
  Bool(Bool<'a>),
  Null(Null<'a>),
  Num(Number<'a>),
  BigInt(BigInt<'a>),
  Regex(Regex<'a>),
  JSXText(JSXText<'a>),
}

impl<'a> Lit<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for Lit<'a> {
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

impl<'a> NodeTrait<'a> for Lit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
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

  fn children(&self) -> Vec<Node<'a>> {
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
impl<'a> From<&Lit<'a>> for Node<'a> {
  fn from(node: &Lit<'a>) -> Node<'a> {
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

fn get_view_for_lit<'a>(ref_node: &'a swc_ecma_ast::Lit) -> Lit<'a> {
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

fn set_parent_for_lit<'a>(node: &mut Lit<'a>, parent: Node<'a>) {
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

pub enum ImportSpecifier<'a> {
  Named(ImportNamedSpecifier<'a>),
  Default(ImportDefaultSpecifier<'a>),
  Namespace(ImportStarAsSpecifier<'a>),
}

impl<'a> ImportSpecifier<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for ImportSpecifier<'a> {
  fn span(&self) -> Span {
    match self {
      ImportSpecifier::Named(node) => node.span(),
      ImportSpecifier::Default(node) => node.span(),
      ImportSpecifier::Namespace(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for ImportSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ImportSpecifier::Named(node) => node.parent(),
      ImportSpecifier::Default(node) => node.parent(),
      ImportSpecifier::Namespace(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ImportSpecifier::Named(node) => node.children(),
      ImportSpecifier::Default(node) => node.children(),
      ImportSpecifier::Namespace(node) => node.children(),
    }
  }
}
impl<'a> From<&ImportSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportSpecifier<'a>) -> Node<'a> {
    match node {
      ImportSpecifier::Named(node) => node.into(),
      ImportSpecifier::Default(node) => node.into(),
      ImportSpecifier::Namespace(node) => node.into(),
    }
  }
}

fn get_view_for_import_specifier<'a>(ref_node: &'a swc_ecma_ast::ImportSpecifier) -> ImportSpecifier<'a> {
  match ref_node {
    swc_ecma_ast::ImportSpecifier::Named(value) => ImportSpecifier::Named(get_view_for_import_named_specifier(value)),
    swc_ecma_ast::ImportSpecifier::Default(value) => ImportSpecifier::Default(get_view_for_import_default_specifier(value)),
    swc_ecma_ast::ImportSpecifier::Namespace(value) => ImportSpecifier::Namespace(get_view_for_import_star_as_specifier(value)),
  }
}

fn set_parent_for_import_specifier<'a>(node: &mut ImportSpecifier<'a>, parent: Node<'a>) {
  match node {
    ImportSpecifier::Named(node) => {
      node.parent = parent.to::<ImportDecl>();
    },
    ImportSpecifier::Default(node) => {
      node.parent = parent.to::<ImportDecl>();
    },
    ImportSpecifier::Namespace(node) => {
      node.parent = parent.to::<ImportDecl>();
    },
  }
}

pub enum ExportSpecifier<'a> {
  Namespace(ExportNamespaceSpecifier<'a>),
  Default(ExportDefaultSpecifier<'a>),
  Named(ExportNamedSpecifier<'a>),
}

impl<'a> ExportSpecifier<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for ExportSpecifier<'a> {
  fn span(&self) -> Span {
    match self {
      ExportSpecifier::Namespace(node) => node.span(),
      ExportSpecifier::Default(node) => node.span(),
      ExportSpecifier::Named(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for ExportSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ExportSpecifier::Namespace(node) => node.parent(),
      ExportSpecifier::Default(node) => node.parent(),
      ExportSpecifier::Named(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ExportSpecifier::Namespace(node) => node.children(),
      ExportSpecifier::Default(node) => node.children(),
      ExportSpecifier::Named(node) => node.children(),
    }
  }
}
impl<'a> From<&ExportSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportSpecifier<'a>) -> Node<'a> {
    match node {
      ExportSpecifier::Namespace(node) => node.into(),
      ExportSpecifier::Default(node) => node.into(),
      ExportSpecifier::Named(node) => node.into(),
    }
  }
}

fn get_view_for_export_specifier<'a>(ref_node: &'a swc_ecma_ast::ExportSpecifier) -> ExportSpecifier<'a> {
  match ref_node {
    swc_ecma_ast::ExportSpecifier::Namespace(value) => ExportSpecifier::Namespace(get_view_for_export_namespace_specifier(value)),
    swc_ecma_ast::ExportSpecifier::Default(value) => ExportSpecifier::Default(get_view_for_export_default_specifier(value)),
    swc_ecma_ast::ExportSpecifier::Named(value) => ExportSpecifier::Named(get_view_for_export_named_specifier(value)),
  }
}

fn set_parent_for_export_specifier<'a>(node: &mut ExportSpecifier<'a>, parent: Node<'a>) {
  match node {
    ExportSpecifier::Namespace(node) => {
      node.parent = parent.to::<NamedExport>();
    },
    ExportSpecifier::Default(node) => {
      node.parent = parent.to::<NamedExport>();
    },
    ExportSpecifier::Named(node) => {
      node.parent = parent.to::<NamedExport>();
    },
  }
}

pub enum Stmt<'a> {
  Block(BlockStmt<'a>),
  Empty(EmptyStmt<'a>),
  Debugger(DebuggerStmt<'a>),
  With(WithStmt<'a>),
  Return(ReturnStmt<'a>),
  Labeled(LabeledStmt<'a>),
  Break(BreakStmt<'a>),
  Continue(ContinueStmt<'a>),
  If(IfStmt<'a>),
  Switch(SwitchStmt<'a>),
  Throw(ThrowStmt<'a>),
  /// A try statement. If handler is null then finalizer must be a BlockStmt.
  Try(TryStmt<'a>),
  While(WhileStmt<'a>),
  DoWhile(DoWhileStmt<'a>),
  For(ForStmt<'a>),
  ForIn(ForInStmt<'a>),
  ForOf(ForOfStmt<'a>),
  Decl(Decl<'a>),
  Expr(ExprStmt<'a>),
}

impl<'a> Stmt<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for Stmt<'a> {
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

impl<'a> NodeTrait<'a> for Stmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
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

  fn children(&self) -> Vec<Node<'a>> {
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
impl<'a> From<&Stmt<'a>> for Node<'a> {
  fn from(node: &Stmt<'a>) -> Node<'a> {
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

fn get_view_for_stmt<'a>(ref_node: &'a swc_ecma_ast::Stmt) -> Stmt<'a> {
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

fn set_parent_for_stmt<'a>(node: &mut Stmt<'a>, parent: Node<'a>) {
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

pub enum Pat<'a> {
  Ident(Ident<'a>),
  Array(ArrayPat<'a>),
  Rest(RestPat<'a>),
  Object(ObjectPat<'a>),
  Assign(AssignPat<'a>),
  Invalid(Invalid<'a>),
  /// Only for for-in / for-of loops. This is *syntatically* valid.
  Expr(Box<Expr<'a>>),
}

impl<'a> Pat<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for Pat<'a> {
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

impl<'a> NodeTrait<'a> for Pat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
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

  fn children(&self) -> Vec<Node<'a>> {
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
impl<'a> From<&Pat<'a>> for Node<'a> {
  fn from(node: &Pat<'a>) -> Node<'a> {
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

fn get_view_for_pat<'a>(ref_node: &'a swc_ecma_ast::Pat) -> Pat<'a> {
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

fn set_parent_for_pat<'a>(node: &mut Pat<'a>, parent: Node<'a>) {
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

pub enum TsModuleName<'a> {
  Ident(Ident<'a>),
  Str(Str<'a>),
}

impl<'a> TsModuleName<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsModuleName<'a> {
  fn span(&self) -> Span {
    match self {
      TsModuleName::Ident(node) => node.span(),
      TsModuleName::Str(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsModuleName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsModuleName::Ident(node) => node.parent(),
      TsModuleName::Str(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsModuleName::Ident(node) => node.children(),
      TsModuleName::Str(node) => node.children(),
    }
  }
}
impl<'a> From<&TsModuleName<'a>> for Node<'a> {
  fn from(node: &TsModuleName<'a>) -> Node<'a> {
    match node {
      TsModuleName::Ident(node) => node.into(),
      TsModuleName::Str(node) => node.into(),
    }
  }
}

fn get_view_for_ts_module_name<'a>(ref_node: &'a swc_ecma_ast::TsModuleName) -> TsModuleName<'a> {
  match ref_node {
    swc_ecma_ast::TsModuleName::Ident(value) => TsModuleName::Ident(get_view_for_ident(value)),
    swc_ecma_ast::TsModuleName::Str(value) => TsModuleName::Str(get_view_for_str(value)),
  }
}

fn set_parent_for_ts_module_name<'a>(node: &mut TsModuleName<'a>, parent: Node<'a>) {
  match node {
    TsModuleName::Ident(node) => {
      node.parent = parent;
    },
    TsModuleName::Str(node) => {
      node.parent = parent;
    },
  }
}

pub enum TsFnParam<'a> {
  Ident(Ident<'a>),
  Array(ArrayPat<'a>),
  Rest(RestPat<'a>),
  Object(ObjectPat<'a>),
}

impl<'a> TsFnParam<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsFnParam<'a> {
  fn span(&self) -> Span {
    match self {
      TsFnParam::Ident(node) => node.span(),
      TsFnParam::Array(node) => node.span(),
      TsFnParam::Rest(node) => node.span(),
      TsFnParam::Object(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsFnParam<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsFnParam::Ident(node) => node.parent(),
      TsFnParam::Array(node) => node.parent(),
      TsFnParam::Rest(node) => node.parent(),
      TsFnParam::Object(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsFnParam::Ident(node) => node.children(),
      TsFnParam::Array(node) => node.children(),
      TsFnParam::Rest(node) => node.children(),
      TsFnParam::Object(node) => node.children(),
    }
  }
}
impl<'a> From<&TsFnParam<'a>> for Node<'a> {
  fn from(node: &TsFnParam<'a>) -> Node<'a> {
    match node {
      TsFnParam::Ident(node) => node.into(),
      TsFnParam::Array(node) => node.into(),
      TsFnParam::Rest(node) => node.into(),
      TsFnParam::Object(node) => node.into(),
    }
  }
}

fn get_view_for_ts_fn_param<'a>(ref_node: &'a swc_ecma_ast::TsFnParam) -> TsFnParam<'a> {
  match ref_node {
    swc_ecma_ast::TsFnParam::Ident(value) => TsFnParam::Ident(get_view_for_ident(value)),
    swc_ecma_ast::TsFnParam::Array(value) => TsFnParam::Array(get_view_for_array_pat(value)),
    swc_ecma_ast::TsFnParam::Rest(value) => TsFnParam::Rest(get_view_for_rest_pat(value)),
    swc_ecma_ast::TsFnParam::Object(value) => TsFnParam::Object(get_view_for_object_pat(value)),
  }
}

fn set_parent_for_ts_fn_param<'a>(node: &mut TsFnParam<'a>, parent: Node<'a>) {
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

pub enum ClassMember<'a> {
  Constructor(Constructor<'a>),
  /// `es2015`
  Method(ClassMethod<'a>),
  PrivateMethod(PrivateMethod<'a>),
  /// stage 0 / Typescript
  ClassProp(ClassProp<'a>),
  PrivateProp(PrivateProp<'a>),
  TsIndexSignature(TsIndexSignature<'a>),
  Empty(EmptyStmt<'a>),
}

impl<'a> ClassMember<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for ClassMember<'a> {
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

impl<'a> NodeTrait<'a> for ClassMember<'a> {
  fn parent(&self) -> Option<Node<'a>> {
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

  fn children(&self) -> Vec<Node<'a>> {
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
impl<'a> From<&ClassMember<'a>> for Node<'a> {
  fn from(node: &ClassMember<'a>) -> Node<'a> {
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

fn get_view_for_class_member<'a>(ref_node: &'a swc_ecma_ast::ClassMember) -> ClassMember<'a> {
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

fn set_parent_for_class_member<'a>(node: &mut ClassMember<'a>, parent: Node<'a>) {
  match node {
    ClassMember::Constructor(node) => {
      node.parent = parent.to::<Class>();
    },
    ClassMember::Method(node) => {
      node.parent = parent.to::<Class>();
    },
    ClassMember::PrivateMethod(node) => {
      node.parent = parent.to::<Class>();
    },
    ClassMember::ClassProp(node) => {
      node.parent = parent.to::<Class>();
    },
    ClassMember::PrivateProp(node) => {
      node.parent = parent.to::<Class>();
    },
    ClassMember::TsIndexSignature(node) => {
      node.parent = parent;
    },
    ClassMember::Empty(node) => {
      node.parent = parent;
    },
  }
}

pub enum VarDeclOrPat<'a> {
  VarDecl(VarDecl<'a>),
  Pat(Pat<'a>),
}

impl<'a> VarDeclOrPat<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for VarDeclOrPat<'a> {
  fn span(&self) -> Span {
    match self {
      VarDeclOrPat::VarDecl(node) => node.span(),
      VarDeclOrPat::Pat(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for VarDeclOrPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      VarDeclOrPat::VarDecl(node) => node.parent(),
      VarDeclOrPat::Pat(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      VarDeclOrPat::VarDecl(node) => node.children(),
      VarDeclOrPat::Pat(node) => node.children(),
    }
  }
}
impl<'a> From<&VarDeclOrPat<'a>> for Node<'a> {
  fn from(node: &VarDeclOrPat<'a>) -> Node<'a> {
    match node {
      VarDeclOrPat::VarDecl(node) => node.into(),
      VarDeclOrPat::Pat(node) => node.into(),
    }
  }
}

fn get_view_for_var_decl_or_pat<'a>(ref_node: &'a swc_ecma_ast::VarDeclOrPat) -> VarDeclOrPat<'a> {
  match ref_node {
    swc_ecma_ast::VarDeclOrPat::VarDecl(value) => VarDeclOrPat::VarDecl(get_view_for_var_decl(value)),
    swc_ecma_ast::VarDeclOrPat::Pat(value) => VarDeclOrPat::Pat(get_view_for_pat(value)),
  }
}

fn set_parent_for_var_decl_or_pat<'a>(node: &mut VarDeclOrPat<'a>, parent: Node<'a>) {
  match node {
    VarDeclOrPat::VarDecl(node) => {
      node.parent = parent;
    },
    VarDeclOrPat::Pat(node) => {
      set_parent_for_pat(node, parent);
    },
  }
}

pub enum TsModuleRef<'a> {
  TsEntityName(TsEntityName<'a>),
  TsExternalModuleRef(TsExternalModuleRef<'a>),
}

impl<'a> TsModuleRef<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsModuleRef<'a> {
  fn span(&self) -> Span {
    match self {
      TsModuleRef::TsEntityName(node) => node.span(),
      TsModuleRef::TsExternalModuleRef(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsModuleRef<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsModuleRef::TsEntityName(node) => node.parent(),
      TsModuleRef::TsExternalModuleRef(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsModuleRef::TsEntityName(node) => node.children(),
      TsModuleRef::TsExternalModuleRef(node) => node.children(),
    }
  }
}
impl<'a> From<&TsModuleRef<'a>> for Node<'a> {
  fn from(node: &TsModuleRef<'a>) -> Node<'a> {
    match node {
      TsModuleRef::TsEntityName(node) => node.into(),
      TsModuleRef::TsExternalModuleRef(node) => node.into(),
    }
  }
}

fn get_view_for_ts_module_ref<'a>(ref_node: &'a swc_ecma_ast::TsModuleRef) -> TsModuleRef<'a> {
  match ref_node {
    swc_ecma_ast::TsModuleRef::TsEntityName(value) => TsModuleRef::TsEntityName(get_view_for_ts_entity_name(value)),
    swc_ecma_ast::TsModuleRef::TsExternalModuleRef(value) => TsModuleRef::TsExternalModuleRef(get_view_for_ts_external_module_ref(value)),
  }
}

fn set_parent_for_ts_module_ref<'a>(node: &mut TsModuleRef<'a>, parent: Node<'a>) {
  match node {
    TsModuleRef::TsEntityName(node) => {
      set_parent_for_ts_entity_name(node, parent);
    },
    TsModuleRef::TsExternalModuleRef(node) => {
      node.parent = parent.to::<TsImportEqualsDecl>();
    },
  }
}

pub enum JSXAttrOrSpread<'a> {
  JSXAttr(JSXAttr<'a>),
  SpreadElement(SpreadElement<'a>),
}

impl<'a> JSXAttrOrSpread<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for JSXAttrOrSpread<'a> {
  fn span(&self) -> Span {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.span(),
      JSXAttrOrSpread::SpreadElement(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXAttrOrSpread<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.parent(),
      JSXAttrOrSpread::SpreadElement(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.children(),
      JSXAttrOrSpread::SpreadElement(node) => node.children(),
    }
  }
}
impl<'a> From<&JSXAttrOrSpread<'a>> for Node<'a> {
  fn from(node: &JSXAttrOrSpread<'a>) -> Node<'a> {
    match node {
      JSXAttrOrSpread::JSXAttr(node) => node.into(),
      JSXAttrOrSpread::SpreadElement(node) => node.into(),
    }
  }
}

fn get_view_for_jsxattr_or_spread<'a>(ref_node: &'a swc_ecma_ast::JSXAttrOrSpread) -> JSXAttrOrSpread<'a> {
  match ref_node {
    swc_ecma_ast::JSXAttrOrSpread::JSXAttr(value) => JSXAttrOrSpread::JSXAttr(get_view_for_jsxattr(value)),
    swc_ecma_ast::JSXAttrOrSpread::SpreadElement(value) => JSXAttrOrSpread::SpreadElement(get_view_for_spread_element(value)),
  }
}

fn set_parent_for_jsxattr_or_spread<'a>(node: &mut JSXAttrOrSpread<'a>, parent: Node<'a>) {
  match node {
    JSXAttrOrSpread::JSXAttr(node) => {
      node.parent = parent.to::<JSXOpeningElement>();
    },
    JSXAttrOrSpread::SpreadElement(node) => {
      node.parent = parent;
    },
  }
}

pub enum ParamOrTsParamProp<'a> {
  TsParamProp(TsParamProp<'a>),
  Param(Param<'a>),
}

impl<'a> ParamOrTsParamProp<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for ParamOrTsParamProp<'a> {
  fn span(&self) -> Span {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.span(),
      ParamOrTsParamProp::Param(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for ParamOrTsParamProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.parent(),
      ParamOrTsParamProp::Param(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.children(),
      ParamOrTsParamProp::Param(node) => node.children(),
    }
  }
}
impl<'a> From<&ParamOrTsParamProp<'a>> for Node<'a> {
  fn from(node: &ParamOrTsParamProp<'a>) -> Node<'a> {
    match node {
      ParamOrTsParamProp::TsParamProp(node) => node.into(),
      ParamOrTsParamProp::Param(node) => node.into(),
    }
  }
}

fn get_view_for_param_or_ts_param_prop<'a>(ref_node: &'a swc_ecma_ast::ParamOrTsParamProp) -> ParamOrTsParamProp<'a> {
  match ref_node {
    swc_ecma_ast::ParamOrTsParamProp::TsParamProp(value) => ParamOrTsParamProp::TsParamProp(get_view_for_ts_param_prop(value)),
    swc_ecma_ast::ParamOrTsParamProp::Param(value) => ParamOrTsParamProp::Param(get_view_for_param(value)),
  }
}

fn set_parent_for_param_or_ts_param_prop<'a>(node: &mut ParamOrTsParamProp<'a>, parent: Node<'a>) {
  match node {
    ParamOrTsParamProp::TsParamProp(node) => {
      node.parent = parent.to::<Constructor>();
    },
    ParamOrTsParamProp::Param(node) => {
      node.parent = parent;
    },
  }
}

pub enum ExprOrSuper<'a> {
  Super(Super<'a>),
  Expr(Box<Expr<'a>>),
}

impl<'a> ExprOrSuper<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for ExprOrSuper<'a> {
  fn span(&self) -> Span {
    match self {
      ExprOrSuper::Super(node) => node.span(),
      ExprOrSuper::Expr(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for ExprOrSuper<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ExprOrSuper::Super(node) => node.parent(),
      ExprOrSuper::Expr(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ExprOrSuper::Super(node) => node.children(),
      ExprOrSuper::Expr(node) => node.children(),
    }
  }
}
impl<'a> From<&ExprOrSuper<'a>> for Node<'a> {
  fn from(node: &ExprOrSuper<'a>) -> Node<'a> {
    match node {
      ExprOrSuper::Super(node) => node.into(),
      ExprOrSuper::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_expr_or_super<'a>(ref_node: &'a swc_ecma_ast::ExprOrSuper) -> ExprOrSuper<'a> {
  match ref_node {
    swc_ecma_ast::ExprOrSuper::Super(value) => ExprOrSuper::Super(get_view_for_super(value)),
    swc_ecma_ast::ExprOrSuper::Expr(value) => ExprOrSuper::Expr(Box::new(get_view_for_expr(value))),
  }
}

fn set_parent_for_expr_or_super<'a>(node: &mut ExprOrSuper<'a>, parent: Node<'a>) {
  match node {
    ExprOrSuper::Super(node) => {
      node.parent = parent;
    },
    ExprOrSuper::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
  }
}

pub enum TsTypeElement<'a> {
  TsCallSignatureDecl(TsCallSignatureDecl<'a>),
  TsConstructSignatureDecl(TsConstructSignatureDecl<'a>),
  TsPropertySignature(TsPropertySignature<'a>),
  TsMethodSignature(TsMethodSignature<'a>),
  TsIndexSignature(TsIndexSignature<'a>),
}

impl<'a> TsTypeElement<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsTypeElement<'a> {
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

impl<'a> NodeTrait<'a> for TsTypeElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.parent(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.parent(),
      TsTypeElement::TsPropertySignature(node) => node.parent(),
      TsTypeElement::TsMethodSignature(node) => node.parent(),
      TsTypeElement::TsIndexSignature(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.children(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.children(),
      TsTypeElement::TsPropertySignature(node) => node.children(),
      TsTypeElement::TsMethodSignature(node) => node.children(),
      TsTypeElement::TsIndexSignature(node) => node.children(),
    }
  }
}
impl<'a> From<&TsTypeElement<'a>> for Node<'a> {
  fn from(node: &TsTypeElement<'a>) -> Node<'a> {
    match node {
      TsTypeElement::TsCallSignatureDecl(node) => node.into(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.into(),
      TsTypeElement::TsPropertySignature(node) => node.into(),
      TsTypeElement::TsMethodSignature(node) => node.into(),
      TsTypeElement::TsIndexSignature(node) => node.into(),
    }
  }
}

fn get_view_for_ts_type_element<'a>(ref_node: &'a swc_ecma_ast::TsTypeElement) -> TsTypeElement<'a> {
  match ref_node {
    swc_ecma_ast::TsTypeElement::TsCallSignatureDecl(value) => TsTypeElement::TsCallSignatureDecl(get_view_for_ts_call_signature_decl(value)),
    swc_ecma_ast::TsTypeElement::TsConstructSignatureDecl(value) => TsTypeElement::TsConstructSignatureDecl(get_view_for_ts_construct_signature_decl(value)),
    swc_ecma_ast::TsTypeElement::TsPropertySignature(value) => TsTypeElement::TsPropertySignature(get_view_for_ts_property_signature(value)),
    swc_ecma_ast::TsTypeElement::TsMethodSignature(value) => TsTypeElement::TsMethodSignature(get_view_for_ts_method_signature(value)),
    swc_ecma_ast::TsTypeElement::TsIndexSignature(value) => TsTypeElement::TsIndexSignature(get_view_for_ts_index_signature(value)),
  }
}

fn set_parent_for_ts_type_element<'a>(node: &mut TsTypeElement<'a>, parent: Node<'a>) {
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

pub enum BlockStmtOrExpr<'a> {
  BlockStmt(BlockStmt<'a>),
  Expr(Box<Expr<'a>>),
}

impl<'a> BlockStmtOrExpr<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for BlockStmtOrExpr<'a> {
  fn span(&self) -> Span {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.span(),
      BlockStmtOrExpr::Expr(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for BlockStmtOrExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.parent(),
      BlockStmtOrExpr::Expr(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.children(),
      BlockStmtOrExpr::Expr(node) => node.children(),
    }
  }
}
impl<'a> From<&BlockStmtOrExpr<'a>> for Node<'a> {
  fn from(node: &BlockStmtOrExpr<'a>) -> Node<'a> {
    match node {
      BlockStmtOrExpr::BlockStmt(node) => node.into(),
      BlockStmtOrExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_block_stmt_or_expr<'a>(ref_node: &'a swc_ecma_ast::BlockStmtOrExpr) -> BlockStmtOrExpr<'a> {
  match ref_node {
    swc_ecma_ast::BlockStmtOrExpr::BlockStmt(value) => BlockStmtOrExpr::BlockStmt(get_view_for_block_stmt(value)),
    swc_ecma_ast::BlockStmtOrExpr::Expr(value) => BlockStmtOrExpr::Expr(Box::new(get_view_for_expr(value))),
  }
}

fn set_parent_for_block_stmt_or_expr<'a>(node: &mut BlockStmtOrExpr<'a>, parent: Node<'a>) {
  match node {
    BlockStmtOrExpr::BlockStmt(node) => {
      node.parent = parent;
    },
    BlockStmtOrExpr::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
  }
}

pub enum TsUnionOrIntersectionType<'a> {
  TsUnionType(TsUnionType<'a>),
  TsIntersectionType(TsIntersectionType<'a>),
}

impl<'a> TsUnionOrIntersectionType<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsUnionOrIntersectionType<'a> {
  fn span(&self) -> Span {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.span(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsUnionOrIntersectionType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.parent(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.children(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.children(),
    }
  }
}
impl<'a> From<&TsUnionOrIntersectionType<'a>> for Node<'a> {
  fn from(node: &TsUnionOrIntersectionType<'a>) -> Node<'a> {
    match node {
      TsUnionOrIntersectionType::TsUnionType(node) => node.into(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.into(),
    }
  }
}

fn get_view_for_ts_union_or_intersection_type<'a>(ref_node: &'a swc_ecma_ast::TsUnionOrIntersectionType) -> TsUnionOrIntersectionType<'a> {
  match ref_node {
    swc_ecma_ast::TsUnionOrIntersectionType::TsUnionType(value) => TsUnionOrIntersectionType::TsUnionType(get_view_for_ts_union_type(value)),
    swc_ecma_ast::TsUnionOrIntersectionType::TsIntersectionType(value) => TsUnionOrIntersectionType::TsIntersectionType(get_view_for_ts_intersection_type(value)),
  }
}

fn set_parent_for_ts_union_or_intersection_type<'a>(node: &mut TsUnionOrIntersectionType<'a>, parent: Node<'a>) {
  match node {
    TsUnionOrIntersectionType::TsUnionType(node) => {
      node.parent = parent;
    },
    TsUnionOrIntersectionType::TsIntersectionType(node) => {
      node.parent = parent;
    },
  }
}

pub enum DefaultDecl<'a> {
  Class(ClassExpr<'a>),
  Fn(FnExpr<'a>),
  TsInterfaceDecl(TsInterfaceDecl<'a>),
}

impl<'a> DefaultDecl<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for DefaultDecl<'a> {
  fn span(&self) -> Span {
    match self {
      DefaultDecl::Class(node) => node.span(),
      DefaultDecl::Fn(node) => node.span(),
      DefaultDecl::TsInterfaceDecl(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for DefaultDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      DefaultDecl::Class(node) => node.parent(),
      DefaultDecl::Fn(node) => node.parent(),
      DefaultDecl::TsInterfaceDecl(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      DefaultDecl::Class(node) => node.children(),
      DefaultDecl::Fn(node) => node.children(),
      DefaultDecl::TsInterfaceDecl(node) => node.children(),
    }
  }
}
impl<'a> From<&DefaultDecl<'a>> for Node<'a> {
  fn from(node: &DefaultDecl<'a>) -> Node<'a> {
    match node {
      DefaultDecl::Class(node) => node.into(),
      DefaultDecl::Fn(node) => node.into(),
      DefaultDecl::TsInterfaceDecl(node) => node.into(),
    }
  }
}

fn get_view_for_default_decl<'a>(ref_node: &'a swc_ecma_ast::DefaultDecl) -> DefaultDecl<'a> {
  match ref_node {
    swc_ecma_ast::DefaultDecl::Class(value) => DefaultDecl::Class(get_view_for_class_expr(value)),
    swc_ecma_ast::DefaultDecl::Fn(value) => DefaultDecl::Fn(get_view_for_fn_expr(value)),
    swc_ecma_ast::DefaultDecl::TsInterfaceDecl(value) => DefaultDecl::TsInterfaceDecl(get_view_for_ts_interface_decl(value)),
  }
}

fn set_parent_for_default_decl<'a>(node: &mut DefaultDecl<'a>, parent: Node<'a>) {
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
pub enum TsEnumMemberId<'a> {
  Ident(Ident<'a>),
  Str(Str<'a>),
}

impl<'a> TsEnumMemberId<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsEnumMemberId<'a> {
  fn span(&self) -> Span {
    match self {
      TsEnumMemberId::Ident(node) => node.span(),
      TsEnumMemberId::Str(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsEnumMemberId<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsEnumMemberId::Ident(node) => node.parent(),
      TsEnumMemberId::Str(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsEnumMemberId::Ident(node) => node.children(),
      TsEnumMemberId::Str(node) => node.children(),
    }
  }
}
impl<'a> From<&TsEnumMemberId<'a>> for Node<'a> {
  fn from(node: &TsEnumMemberId<'a>) -> Node<'a> {
    match node {
      TsEnumMemberId::Ident(node) => node.into(),
      TsEnumMemberId::Str(node) => node.into(),
    }
  }
}

fn get_view_for_ts_enum_member_id<'a>(ref_node: &'a swc_ecma_ast::TsEnumMemberId) -> TsEnumMemberId<'a> {
  match ref_node {
    swc_ecma_ast::TsEnumMemberId::Ident(value) => TsEnumMemberId::Ident(get_view_for_ident(value)),
    swc_ecma_ast::TsEnumMemberId::Str(value) => TsEnumMemberId::Str(get_view_for_str(value)),
  }
}

fn set_parent_for_ts_enum_member_id<'a>(node: &mut TsEnumMemberId<'a>, parent: Node<'a>) {
  match node {
    TsEnumMemberId::Ident(node) => {
      node.parent = parent;
    },
    TsEnumMemberId::Str(node) => {
      node.parent = parent;
    },
  }
}

pub enum TsParamPropParam<'a> {
  Ident(Ident<'a>),
  Assign(AssignPat<'a>),
}

impl<'a> TsParamPropParam<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsParamPropParam<'a> {
  fn span(&self) -> Span {
    match self {
      TsParamPropParam::Ident(node) => node.span(),
      TsParamPropParam::Assign(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsParamPropParam<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsParamPropParam::Ident(node) => node.parent(),
      TsParamPropParam::Assign(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsParamPropParam::Ident(node) => node.children(),
      TsParamPropParam::Assign(node) => node.children(),
    }
  }
}
impl<'a> From<&TsParamPropParam<'a>> for Node<'a> {
  fn from(node: &TsParamPropParam<'a>) -> Node<'a> {
    match node {
      TsParamPropParam::Ident(node) => node.into(),
      TsParamPropParam::Assign(node) => node.into(),
    }
  }
}

fn get_view_for_ts_param_prop_param<'a>(ref_node: &'a swc_ecma_ast::TsParamPropParam) -> TsParamPropParam<'a> {
  match ref_node {
    swc_ecma_ast::TsParamPropParam::Ident(value) => TsParamPropParam::Ident(get_view_for_ident(value)),
    swc_ecma_ast::TsParamPropParam::Assign(value) => TsParamPropParam::Assign(get_view_for_assign_pat(value)),
  }
}

fn set_parent_for_ts_param_prop_param<'a>(node: &mut TsParamPropParam<'a>, parent: Node<'a>) {
  match node {
    TsParamPropParam::Ident(node) => {
      node.parent = parent;
    },
    TsParamPropParam::Assign(node) => {
      node.parent = parent;
    },
  }
}

pub enum JSXElementChild<'a> {
  JSXText(JSXText<'a>),
  JSXExprContainer(JSXExprContainer<'a>),
  JSXSpreadChild(JSXSpreadChild<'a>),
  JSXElement(Box<JSXElement<'a>>),
  JSXFragment(JSXFragment<'a>),
}

impl<'a> JSXElementChild<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for JSXElementChild<'a> {
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

impl<'a> NodeTrait<'a> for JSXElementChild<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXElementChild::JSXText(node) => node.parent(),
      JSXElementChild::JSXExprContainer(node) => node.parent(),
      JSXElementChild::JSXSpreadChild(node) => node.parent(),
      JSXElementChild::JSXElement(node) => node.parent(),
      JSXElementChild::JSXFragment(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXElementChild::JSXText(node) => node.children(),
      JSXElementChild::JSXExprContainer(node) => node.children(),
      JSXElementChild::JSXSpreadChild(node) => node.children(),
      JSXElementChild::JSXElement(node) => node.children(),
      JSXElementChild::JSXFragment(node) => node.children(),
    }
  }
}
impl<'a> From<&JSXElementChild<'a>> for Node<'a> {
  fn from(node: &JSXElementChild<'a>) -> Node<'a> {
    match node {
      JSXElementChild::JSXText(node) => node.into(),
      JSXElementChild::JSXExprContainer(node) => node.into(),
      JSXElementChild::JSXSpreadChild(node) => node.into(),
      JSXElementChild::JSXElement(node) => node.into(),
      JSXElementChild::JSXFragment(node) => node.into(),
    }
  }
}

fn get_view_for_jsxelement_child<'a>(ref_node: &'a swc_ecma_ast::JSXElementChild) -> JSXElementChild<'a> {
  match ref_node {
    swc_ecma_ast::JSXElementChild::JSXText(value) => JSXElementChild::JSXText(get_view_for_jsxtext(value)),
    swc_ecma_ast::JSXElementChild::JSXExprContainer(value) => JSXElementChild::JSXExprContainer(get_view_for_jsxexpr_container(value)),
    swc_ecma_ast::JSXElementChild::JSXSpreadChild(value) => JSXElementChild::JSXSpreadChild(get_view_for_jsxspread_child(value)),
    swc_ecma_ast::JSXElementChild::JSXElement(value) => JSXElementChild::JSXElement(Box::new(get_view_for_jsxelement(value))),
    swc_ecma_ast::JSXElementChild::JSXFragment(value) => JSXElementChild::JSXFragment(get_view_for_jsxfragment(value)),
  }
}

fn set_parent_for_jsxelement_child<'a>(node: &mut JSXElementChild<'a>, parent: Node<'a>) {
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

pub enum ModuleItem<'a> {
  ModuleDecl(ModuleDecl<'a>),
  Stmt(Stmt<'a>),
}

impl<'a> ModuleItem<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for ModuleItem<'a> {
  fn span(&self) -> Span {
    match self {
      ModuleItem::ModuleDecl(node) => node.span(),
      ModuleItem::Stmt(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for ModuleItem<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ModuleItem::ModuleDecl(node) => node.parent(),
      ModuleItem::Stmt(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ModuleItem::ModuleDecl(node) => node.children(),
      ModuleItem::Stmt(node) => node.children(),
    }
  }
}
impl<'a> From<&ModuleItem<'a>> for Node<'a> {
  fn from(node: &ModuleItem<'a>) -> Node<'a> {
    match node {
      ModuleItem::ModuleDecl(node) => node.into(),
      ModuleItem::Stmt(node) => node.into(),
    }
  }
}

fn get_view_for_module_item<'a>(ref_node: &'a swc_ecma_ast::ModuleItem) -> ModuleItem<'a> {
  match ref_node {
    swc_ecma_ast::ModuleItem::ModuleDecl(value) => ModuleItem::ModuleDecl(get_view_for_module_decl(value)),
    swc_ecma_ast::ModuleItem::Stmt(value) => ModuleItem::Stmt(get_view_for_stmt(value)),
  }
}

fn set_parent_for_module_item<'a>(node: &mut ModuleItem<'a>, parent: Node<'a>) {
  match node {
    ModuleItem::ModuleDecl(node) => {
      set_parent_for_module_decl(node, parent);
    },
    ModuleItem::Stmt(node) => {
      set_parent_for_stmt(node, parent);
    },
  }
}

pub enum PropName<'a> {
  Ident(Ident<'a>),
  /// String literal.
  Str(Str<'a>),
  /// Numeric literal.
  Num(Number<'a>),
  Computed(ComputedPropName<'a>),
  BigInt(BigInt<'a>),
}

impl<'a> PropName<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for PropName<'a> {
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

impl<'a> NodeTrait<'a> for PropName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      PropName::Ident(node) => node.parent(),
      PropName::Str(node) => node.parent(),
      PropName::Num(node) => node.parent(),
      PropName::Computed(node) => node.parent(),
      PropName::BigInt(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      PropName::Ident(node) => node.children(),
      PropName::Str(node) => node.children(),
      PropName::Num(node) => node.children(),
      PropName::Computed(node) => node.children(),
      PropName::BigInt(node) => node.children(),
    }
  }
}
impl<'a> From<&PropName<'a>> for Node<'a> {
  fn from(node: &PropName<'a>) -> Node<'a> {
    match node {
      PropName::Ident(node) => node.into(),
      PropName::Str(node) => node.into(),
      PropName::Num(node) => node.into(),
      PropName::Computed(node) => node.into(),
      PropName::BigInt(node) => node.into(),
    }
  }
}

fn get_view_for_prop_name<'a>(ref_node: &'a swc_ecma_ast::PropName) -> PropName<'a> {
  match ref_node {
    swc_ecma_ast::PropName::Ident(value) => PropName::Ident(get_view_for_ident(value)),
    swc_ecma_ast::PropName::Str(value) => PropName::Str(get_view_for_str(value)),
    swc_ecma_ast::PropName::Num(value) => PropName::Num(get_view_for_number(value)),
    swc_ecma_ast::PropName::Computed(value) => PropName::Computed(get_view_for_computed_prop_name(value)),
    swc_ecma_ast::PropName::BigInt(value) => PropName::BigInt(get_view_for_big_int(value)),
  }
}

fn set_parent_for_prop_name<'a>(node: &mut PropName<'a>, parent: Node<'a>) {
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

pub enum JSXAttrName<'a> {
  Ident(Ident<'a>),
  JSXNamespacedName(JSXNamespacedName<'a>),
}

impl<'a> JSXAttrName<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for JSXAttrName<'a> {
  fn span(&self) -> Span {
    match self {
      JSXAttrName::Ident(node) => node.span(),
      JSXAttrName::JSXNamespacedName(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXAttrName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXAttrName::Ident(node) => node.parent(),
      JSXAttrName::JSXNamespacedName(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXAttrName::Ident(node) => node.children(),
      JSXAttrName::JSXNamespacedName(node) => node.children(),
    }
  }
}
impl<'a> From<&JSXAttrName<'a>> for Node<'a> {
  fn from(node: &JSXAttrName<'a>) -> Node<'a> {
    match node {
      JSXAttrName::Ident(node) => node.into(),
      JSXAttrName::JSXNamespacedName(node) => node.into(),
    }
  }
}

fn get_view_for_jsxattr_name<'a>(ref_node: &'a swc_ecma_ast::JSXAttrName) -> JSXAttrName<'a> {
  match ref_node {
    swc_ecma_ast::JSXAttrName::Ident(value) => JSXAttrName::Ident(get_view_for_ident(value)),
    swc_ecma_ast::JSXAttrName::JSXNamespacedName(value) => JSXAttrName::JSXNamespacedName(get_view_for_jsxnamespaced_name(value)),
  }
}

fn set_parent_for_jsxattr_name<'a>(node: &mut JSXAttrName<'a>, parent: Node<'a>) {
  match node {
    JSXAttrName::Ident(node) => {
      node.parent = parent;
    },
    JSXAttrName::JSXNamespacedName(node) => {
      node.parent = parent;
    },
  }
}

pub enum Decl<'a> {
  Class(ClassDecl<'a>),
  Fn(FnDecl<'a>),
  Var(VarDecl<'a>),
  TsInterface(TsInterfaceDecl<'a>),
  TsTypeAlias(TsTypeAliasDecl<'a>),
  TsEnum(TsEnumDecl<'a>),
  TsModule(TsModuleDecl<'a>),
}

impl<'a> Decl<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for Decl<'a> {
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

impl<'a> NodeTrait<'a> for Decl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
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

  fn children(&self) -> Vec<Node<'a>> {
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
impl<'a> From<&Decl<'a>> for Node<'a> {
  fn from(node: &Decl<'a>) -> Node<'a> {
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

fn get_view_for_decl<'a>(ref_node: &'a swc_ecma_ast::Decl) -> Decl<'a> {
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

fn set_parent_for_decl<'a>(node: &mut Decl<'a>, parent: Node<'a>) {
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

pub enum TsLit<'a> {
  Number(Number<'a>),
  Str(Str<'a>),
  Bool(Bool<'a>),
  BigInt(BigInt<'a>),
  Tpl(TsTplLitType<'a>),
}

impl<'a> TsLit<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsLit<'a> {
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

impl<'a> NodeTrait<'a> for TsLit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsLit::Number(node) => node.parent(),
      TsLit::Str(node) => node.parent(),
      TsLit::Bool(node) => node.parent(),
      TsLit::BigInt(node) => node.parent(),
      TsLit::Tpl(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsLit::Number(node) => node.children(),
      TsLit::Str(node) => node.children(),
      TsLit::Bool(node) => node.children(),
      TsLit::BigInt(node) => node.children(),
      TsLit::Tpl(node) => node.children(),
    }
  }
}
impl<'a> From<&TsLit<'a>> for Node<'a> {
  fn from(node: &TsLit<'a>) -> Node<'a> {
    match node {
      TsLit::Number(node) => node.into(),
      TsLit::Str(node) => node.into(),
      TsLit::Bool(node) => node.into(),
      TsLit::BigInt(node) => node.into(),
      TsLit::Tpl(node) => node.into(),
    }
  }
}

fn get_view_for_ts_lit<'a>(ref_node: &'a swc_ecma_ast::TsLit) -> TsLit<'a> {
  match ref_node {
    swc_ecma_ast::TsLit::Number(value) => TsLit::Number(get_view_for_number(value)),
    swc_ecma_ast::TsLit::Str(value) => TsLit::Str(get_view_for_str(value)),
    swc_ecma_ast::TsLit::Bool(value) => TsLit::Bool(get_view_for_bool(value)),
    swc_ecma_ast::TsLit::BigInt(value) => TsLit::BigInt(get_view_for_big_int(value)),
    swc_ecma_ast::TsLit::Tpl(value) => TsLit::Tpl(get_view_for_ts_tpl_lit_type(value)),
  }
}

fn set_parent_for_ts_lit<'a>(node: &mut TsLit<'a>, parent: Node<'a>) {
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
      node.parent = parent.to::<TsLitType>();
    },
  }
}

pub enum TsEntityName<'a> {
  TsQualifiedName(Box<TsQualifiedName<'a>>),
  Ident(Ident<'a>),
}

impl<'a> TsEntityName<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsEntityName<'a> {
  fn span(&self) -> Span {
    match self {
      TsEntityName::TsQualifiedName(node) => node.span(),
      TsEntityName::Ident(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsEntityName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsEntityName::TsQualifiedName(node) => node.parent(),
      TsEntityName::Ident(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsEntityName::TsQualifiedName(node) => node.children(),
      TsEntityName::Ident(node) => node.children(),
    }
  }
}
impl<'a> From<&TsEntityName<'a>> for Node<'a> {
  fn from(node: &TsEntityName<'a>) -> Node<'a> {
    match node {
      TsEntityName::TsQualifiedName(node) => node.into(),
      TsEntityName::Ident(node) => node.into(),
    }
  }
}

fn get_view_for_ts_entity_name<'a>(ref_node: &'a swc_ecma_ast::TsEntityName) -> TsEntityName<'a> {
  match ref_node {
    swc_ecma_ast::TsEntityName::TsQualifiedName(value) => TsEntityName::TsQualifiedName(Box::new(get_view_for_ts_qualified_name(value))),
    swc_ecma_ast::TsEntityName::Ident(value) => TsEntityName::Ident(get_view_for_ident(value)),
  }
}

fn set_parent_for_ts_entity_name<'a>(node: &mut TsEntityName<'a>, parent: Node<'a>) {
  match node {
    TsEntityName::TsQualifiedName(node) => {
      node.parent = parent;
    },
    TsEntityName::Ident(node) => {
      node.parent = parent;
    },
  }
}

pub enum Expr<'a> {
  This(ThisExpr<'a>),
  Array(ArrayLit<'a>),
  Object(ObjectLit<'a>),
  Fn(FnExpr<'a>),
  Unary(UnaryExpr<'a>),
  /// `++v`, `--v`, `v++`, `v--`
  Update(UpdateExpr<'a>),
  Bin(BinExpr<'a>),
  Assign(AssignExpr<'a>),
  /// A member expression. If computed is true, the node corresponds to a
  /// computed (a[b]) member expression and property is an Expression. If
  /// computed is false, the node corresponds to a static (a.b) member
  /// expression and property is an Identifier.
  Member(MemberExpr<'a>),
  /// true ? 'a' : 'b'
  Cond(CondExpr<'a>),
  Call(CallExpr<'a>),
  /// `new Cat()`
  New(NewExpr<'a>),
  Seq(SeqExpr<'a>),
  Ident(Ident<'a>),
  Lit(Lit<'a>),
  Tpl(Tpl<'a>),
  TaggedTpl(TaggedTpl<'a>),
  Arrow(ArrowExpr<'a>),
  Class(ClassExpr<'a>),
  Yield(YieldExpr<'a>),
  MetaProp(MetaPropExpr<'a>),
  Await(AwaitExpr<'a>),
  Paren(ParenExpr<'a>),
  JSXMember(JSXMemberExpr<'a>),
  JSXNamespacedName(JSXNamespacedName<'a>),
  JSXEmpty(JSXEmptyExpr<'a>),
  JSXElement(Box<JSXElement<'a>>),
  JSXFragment(JSXFragment<'a>),
  TsTypeAssertion(TsTypeAssertion<'a>),
  TsConstAssertion(TsConstAssertion<'a>),
  TsNonNull(TsNonNullExpr<'a>),
  TsTypeCast(TsTypeCastExpr<'a>),
  TsAs(TsAsExpr<'a>),
  PrivateName(PrivateName<'a>),
  OptChain(OptChainExpr<'a>),
  Invalid(Invalid<'a>),
}

impl<'a> Expr<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for Expr<'a> {
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

impl<'a> NodeTrait<'a> for Expr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
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

  fn children(&self) -> Vec<Node<'a>> {
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
impl<'a> From<&Expr<'a>> for Node<'a> {
  fn from(node: &Expr<'a>) -> Node<'a> {
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

fn get_view_for_expr<'a>(ref_node: &'a swc_ecma_ast::Expr) -> Expr<'a> {
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

fn set_parent_for_expr<'a>(node: &mut Expr<'a>, parent: Node<'a>) {
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
pub enum JSXObject<'a> {
  JSXMemberExpr(Box<JSXMemberExpr<'a>>),
  Ident(Ident<'a>),
}

impl<'a> JSXObject<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for JSXObject<'a> {
  fn span(&self) -> Span {
    match self {
      JSXObject::JSXMemberExpr(node) => node.span(),
      JSXObject::Ident(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXObject<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXObject::JSXMemberExpr(node) => node.parent(),
      JSXObject::Ident(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXObject::JSXMemberExpr(node) => node.children(),
      JSXObject::Ident(node) => node.children(),
    }
  }
}
impl<'a> From<&JSXObject<'a>> for Node<'a> {
  fn from(node: &JSXObject<'a>) -> Node<'a> {
    match node {
      JSXObject::JSXMemberExpr(node) => node.into(),
      JSXObject::Ident(node) => node.into(),
    }
  }
}

fn get_view_for_jsxobject<'a>(ref_node: &'a swc_ecma_ast::JSXObject) -> JSXObject<'a> {
  match ref_node {
    swc_ecma_ast::JSXObject::JSXMemberExpr(value) => JSXObject::JSXMemberExpr(Box::new(get_view_for_jsxmember_expr(value))),
    swc_ecma_ast::JSXObject::Ident(value) => JSXObject::Ident(get_view_for_ident(value)),
  }
}

fn set_parent_for_jsxobject<'a>(node: &mut JSXObject<'a>, parent: Node<'a>) {
  match node {
    JSXObject::JSXMemberExpr(node) => {
      node.parent = parent;
    },
    JSXObject::Ident(node) => {
      node.parent = parent;
    },
  }
}

pub enum PatOrExpr<'a> {
  Expr(Box<Expr<'a>>),
  Pat(Box<Pat<'a>>),
}

impl<'a> PatOrExpr<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for PatOrExpr<'a> {
  fn span(&self) -> Span {
    match self {
      PatOrExpr::Expr(node) => node.span(),
      PatOrExpr::Pat(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for PatOrExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      PatOrExpr::Expr(node) => node.parent(),
      PatOrExpr::Pat(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      PatOrExpr::Expr(node) => node.children(),
      PatOrExpr::Pat(node) => node.children(),
    }
  }
}
impl<'a> From<&PatOrExpr<'a>> for Node<'a> {
  fn from(node: &PatOrExpr<'a>) -> Node<'a> {
    match node {
      PatOrExpr::Expr(node) => node.into(),
      PatOrExpr::Pat(node) => node.into(),
    }
  }
}

fn get_view_for_pat_or_expr<'a>(ref_node: &'a swc_ecma_ast::PatOrExpr) -> PatOrExpr<'a> {
  match ref_node {
    swc_ecma_ast::PatOrExpr::Expr(value) => PatOrExpr::Expr(Box::new(get_view_for_expr(value))),
    swc_ecma_ast::PatOrExpr::Pat(value) => PatOrExpr::Pat(Box::new(get_view_for_pat(value))),
  }
}

fn set_parent_for_pat_or_expr<'a>(node: &mut PatOrExpr<'a>, parent: Node<'a>) {
  match node {
    PatOrExpr::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
    PatOrExpr::Pat(node) => {
      set_parent_for_pat(node, parent);
    },
  }
}

pub enum ModuleDecl<'a> {
  Import(ImportDecl<'a>),
  ExportDecl(ExportDecl<'a>),
  ExportNamed(NamedExport<'a>),
  ExportDefaultDecl(ExportDefaultDecl<'a>),
  ExportDefaultExpr(ExportDefaultExpr<'a>),
  ExportAll(ExportAll<'a>),
  TsImportEquals(TsImportEqualsDecl<'a>),
  TsExportAssignment(TsExportAssignment<'a>),
  TsNamespaceExport(TsNamespaceExportDecl<'a>),
}

impl<'a> ModuleDecl<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for ModuleDecl<'a> {
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

impl<'a> NodeTrait<'a> for ModuleDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
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

  fn children(&self) -> Vec<Node<'a>> {
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
impl<'a> From<&ModuleDecl<'a>> for Node<'a> {
  fn from(node: &ModuleDecl<'a>) -> Node<'a> {
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

fn get_view_for_module_decl<'a>(ref_node: &'a swc_ecma_ast::ModuleDecl) -> ModuleDecl<'a> {
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

fn set_parent_for_module_decl<'a>(node: &mut ModuleDecl<'a>, parent: Node<'a>) {
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

pub enum JSXElementName<'a> {
  Ident(Ident<'a>),
  JSXMemberExpr(JSXMemberExpr<'a>),
  JSXNamespacedName(JSXNamespacedName<'a>),
}

impl<'a> JSXElementName<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for JSXElementName<'a> {
  fn span(&self) -> Span {
    match self {
      JSXElementName::Ident(node) => node.span(),
      JSXElementName::JSXMemberExpr(node) => node.span(),
      JSXElementName::JSXNamespacedName(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXElementName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXElementName::Ident(node) => node.parent(),
      JSXElementName::JSXMemberExpr(node) => node.parent(),
      JSXElementName::JSXNamespacedName(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXElementName::Ident(node) => node.children(),
      JSXElementName::JSXMemberExpr(node) => node.children(),
      JSXElementName::JSXNamespacedName(node) => node.children(),
    }
  }
}
impl<'a> From<&JSXElementName<'a>> for Node<'a> {
  fn from(node: &JSXElementName<'a>) -> Node<'a> {
    match node {
      JSXElementName::Ident(node) => node.into(),
      JSXElementName::JSXMemberExpr(node) => node.into(),
      JSXElementName::JSXNamespacedName(node) => node.into(),
    }
  }
}

fn get_view_for_jsxelement_name<'a>(ref_node: &'a swc_ecma_ast::JSXElementName) -> JSXElementName<'a> {
  match ref_node {
    swc_ecma_ast::JSXElementName::Ident(value) => JSXElementName::Ident(get_view_for_ident(value)),
    swc_ecma_ast::JSXElementName::JSXMemberExpr(value) => JSXElementName::JSXMemberExpr(get_view_for_jsxmember_expr(value)),
    swc_ecma_ast::JSXElementName::JSXNamespacedName(value) => JSXElementName::JSXNamespacedName(get_view_for_jsxnamespaced_name(value)),
  }
}

fn set_parent_for_jsxelement_name<'a>(node: &mut JSXElementName<'a>, parent: Node<'a>) {
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

pub enum JSXExpr<'a> {
  JSXEmptyExpr(JSXEmptyExpr<'a>),
  Expr(Box<Expr<'a>>),
}

impl<'a> JSXExpr<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for JSXExpr<'a> {
  fn span(&self) -> Span {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.span(),
      JSXExpr::Expr(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.parent(),
      JSXExpr::Expr(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.children(),
      JSXExpr::Expr(node) => node.children(),
    }
  }
}
impl<'a> From<&JSXExpr<'a>> for Node<'a> {
  fn from(node: &JSXExpr<'a>) -> Node<'a> {
    match node {
      JSXExpr::JSXEmptyExpr(node) => node.into(),
      JSXExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_jsxexpr<'a>(ref_node: &'a swc_ecma_ast::JSXExpr) -> JSXExpr<'a> {
  match ref_node {
    swc_ecma_ast::JSXExpr::JSXEmptyExpr(value) => JSXExpr::JSXEmptyExpr(get_view_for_jsxempty_expr(value)),
    swc_ecma_ast::JSXExpr::Expr(value) => JSXExpr::Expr(Box::new(get_view_for_expr(value))),
  }
}

fn set_parent_for_jsxexpr<'a>(node: &mut JSXExpr<'a>, parent: Node<'a>) {
  match node {
    JSXExpr::JSXEmptyExpr(node) => {
      node.parent = parent;
    },
    JSXExpr::Expr(node) => {
      set_parent_for_expr(node, parent);
    },
  }
}

pub enum TsType<'a> {
  TsKeywordType(TsKeywordType<'a>),
  TsThisType(TsThisType<'a>),
  TsFnOrConstructorType(TsFnOrConstructorType<'a>),
  TsTypeRef(TsTypeRef<'a>),
  TsTypeQuery(TsTypeQuery<'a>),
  TsTypeLit(TsTypeLit<'a>),
  TsArrayType(TsArrayType<'a>),
  TsTupleType(TsTupleType<'a>),
  TsOptionalType(TsOptionalType<'a>),
  TsRestType(TsRestType<'a>),
  TsUnionOrIntersectionType(TsUnionOrIntersectionType<'a>),
  TsConditionalType(TsConditionalType<'a>),
  TsInferType(TsInferType<'a>),
  TsParenthesizedType(TsParenthesizedType<'a>),
  TsTypeOperator(TsTypeOperator<'a>),
  TsIndexedAccessType(TsIndexedAccessType<'a>),
  TsMappedType(TsMappedType<'a>),
  TsLitType(TsLitType<'a>),
  TsTypePredicate(TsTypePredicate<'a>),
  TsImportType(TsImportType<'a>),
}

impl<'a> TsType<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsType<'a> {
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

impl<'a> NodeTrait<'a> for TsType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
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

  fn children(&self) -> Vec<Node<'a>> {
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
impl<'a> From<&TsType<'a>> for Node<'a> {
  fn from(node: &TsType<'a>) -> Node<'a> {
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

fn get_view_for_ts_type<'a>(ref_node: &'a swc_ecma_ast::TsType) -> TsType<'a> {
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

fn set_parent_for_ts_type<'a>(node: &mut TsType<'a>, parent: Node<'a>) {
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

pub enum ObjectPatProp<'a> {
  KeyValue(KeyValuePatProp<'a>),
  Assign(AssignPatProp<'a>),
  Rest(RestPat<'a>),
}

impl<'a> ObjectPatProp<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for ObjectPatProp<'a> {
  fn span(&self) -> Span {
    match self {
      ObjectPatProp::KeyValue(node) => node.span(),
      ObjectPatProp::Assign(node) => node.span(),
      ObjectPatProp::Rest(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for ObjectPatProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ObjectPatProp::KeyValue(node) => node.parent(),
      ObjectPatProp::Assign(node) => node.parent(),
      ObjectPatProp::Rest(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ObjectPatProp::KeyValue(node) => node.children(),
      ObjectPatProp::Assign(node) => node.children(),
      ObjectPatProp::Rest(node) => node.children(),
    }
  }
}
impl<'a> From<&ObjectPatProp<'a>> for Node<'a> {
  fn from(node: &ObjectPatProp<'a>) -> Node<'a> {
    match node {
      ObjectPatProp::KeyValue(node) => node.into(),
      ObjectPatProp::Assign(node) => node.into(),
      ObjectPatProp::Rest(node) => node.into(),
    }
  }
}

fn get_view_for_object_pat_prop<'a>(ref_node: &'a swc_ecma_ast::ObjectPatProp) -> ObjectPatProp<'a> {
  match ref_node {
    swc_ecma_ast::ObjectPatProp::KeyValue(value) => ObjectPatProp::KeyValue(get_view_for_key_value_pat_prop(value)),
    swc_ecma_ast::ObjectPatProp::Assign(value) => ObjectPatProp::Assign(get_view_for_assign_pat_prop(value)),
    swc_ecma_ast::ObjectPatProp::Rest(value) => ObjectPatProp::Rest(get_view_for_rest_pat(value)),
  }
}

fn set_parent_for_object_pat_prop<'a>(node: &mut ObjectPatProp<'a>, parent: Node<'a>) {
  match node {
    ObjectPatProp::KeyValue(node) => {
      node.parent = parent.to::<ObjectPat>();
    },
    ObjectPatProp::Assign(node) => {
      node.parent = parent.to::<ObjectPat>();
    },
    ObjectPatProp::Rest(node) => {
      node.parent = parent;
    },
  }
}

pub enum TsFnOrConstructorType<'a> {
  TsFnType(TsFnType<'a>),
  TsConstructorType(TsConstructorType<'a>),
}

impl<'a> TsFnOrConstructorType<'a> {
  pub fn try_to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::try_cast(&self.into())
  }

  pub fn to<T: CastableNode<'a>>(&self) -> &'a T {
    T::try_cast(&self.into()).expect("Tried to cast node to incorrect type.")
  }
}

impl<'a> Spanned for TsFnOrConstructorType<'a> {
  fn span(&self) -> Span {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.span(),
      TsFnOrConstructorType::TsConstructorType(node) => node.span(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsFnOrConstructorType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.parent(),
      TsFnOrConstructorType::TsConstructorType(node) => node.parent(),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.children(),
      TsFnOrConstructorType::TsConstructorType(node) => node.children(),
    }
  }
}
impl<'a> From<&TsFnOrConstructorType<'a>> for Node<'a> {
  fn from(node: &TsFnOrConstructorType<'a>) -> Node<'a> {
    match node {
      TsFnOrConstructorType::TsFnType(node) => node.into(),
      TsFnOrConstructorType::TsConstructorType(node) => node.into(),
    }
  }
}

fn get_view_for_ts_fn_or_constructor_type<'a>(ref_node: &'a swc_ecma_ast::TsFnOrConstructorType) -> TsFnOrConstructorType<'a> {
  match ref_node {
    swc_ecma_ast::TsFnOrConstructorType::TsFnType(value) => TsFnOrConstructorType::TsFnType(get_view_for_ts_fn_type(value)),
    swc_ecma_ast::TsFnOrConstructorType::TsConstructorType(value) => TsFnOrConstructorType::TsConstructorType(get_view_for_ts_constructor_type(value)),
  }
}

fn set_parent_for_ts_fn_or_constructor_type<'a>(node: &mut TsFnOrConstructorType<'a>, parent: Node<'a>) {
  match node {
    TsFnOrConstructorType::TsFnType(node) => {
      node.parent = parent;
    },
    TsFnOrConstructorType::TsConstructorType(node) => {
      node.parent = parent;
    },
  }
}

pub struct SwitchCase<'a> {
  pub parent: &'a SwitchStmt<'a>,
  pub inner: &'a swc_ecma_ast::SwitchCase,
  /// None for `default:`
  pub test: Option<Box<Expr<'a>>>,
  pub cons: Vec<Stmt<'a>>,
}

impl<'a> Spanned for SwitchCase<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&SwitchCase<'a>> for Node<'a> {
  fn from(node: &SwitchCase) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&SwitchCase, &'a SwitchCase>(&node) };
    Node::SwitchCase(static_ref)
  }
}

impl<'a> NodeTrait<'a> for SwitchCase<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for SwitchCase<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SwitchCase(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_switch_case<'a>(ref_node: &'a swc_ecma_ast::SwitchCase) -> SwitchCase<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&SwitchCase, &'a SwitchCase>(&node) };
  let parent = Node::SwitchCase(child_parent_ref);
  if let Some(node) = node.test.as_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  for node in node.cons.iter_mut() {
    set_parent_for_stmt(node, parent.clone());
  }
  node
}

pub struct ThrowStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ThrowStmt,
  pub arg: Box<Expr<'a>>,
}

impl<'a> Spanned for ThrowStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ThrowStmt<'a>> for Node<'a> {
  fn from(node: &ThrowStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ThrowStmt, &'a ThrowStmt>(&node) };
    Node::ThrowStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ThrowStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

}

impl<'a> CastableNode<'a> for ThrowStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ThrowStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_throw_stmt<'a>(ref_node: &'a swc_ecma_ast::ThrowStmt) -> ThrowStmt<'a> {
  let value = &ref_node.arg;
  let field_arg = Box::new(get_view_for_expr(value));
  let mut node = ThrowStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ThrowStmt, &'a ThrowStmt>(&node) };
  let parent = Node::ThrowStmt(child_parent_ref);
  set_parent_for_expr(&mut node.arg, parent);
  node
}

pub struct JSXClosingFragment<'a> {
  pub parent: &'a JSXFragment<'a>,
  pub inner: &'a swc_ecma_ast::JSXClosingFragment,
}

impl<'a> Spanned for JSXClosingFragment<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXClosingFragment<'a>> for Node<'a> {
  fn from(node: &JSXClosingFragment) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXClosingFragment, &'a JSXClosingFragment>(&node) };
    Node::JSXClosingFragment(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXClosingFragment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for JSXClosingFragment<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXClosingFragment(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxclosing_fragment<'a>(ref_node: &'a swc_ecma_ast::JSXClosingFragment) -> JSXClosingFragment<'a> {
  let node = JSXClosingFragment {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct BigInt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::BigInt,
}

impl<'a> BigInt<'a> {
  pub fn value(&self) -> &num_bigint::BigInt {
    &self.inner.value
  }
}

impl<'a> Spanned for BigInt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&BigInt<'a>> for Node<'a> {
  fn from(node: &BigInt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&BigInt, &'a BigInt>(&node) };
    Node::BigInt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for BigInt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for BigInt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::BigInt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_big_int<'a>(ref_node: &'a swc_ecma_ast::BigInt) -> BigInt<'a> {
  let node = BigInt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct ExportDefaultSpecifier<'a> {
  pub parent: &'a NamedExport<'a>,
  pub inner: &'a swc_ecma_ast::ExportDefaultSpecifier,
  pub exported: Ident<'a>,
}

impl<'a> Spanned for ExportDefaultSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportDefaultSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportDefaultSpecifier) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ExportDefaultSpecifier, &'a ExportDefaultSpecifier>(&node) };
    Node::ExportDefaultSpecifier(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ExportDefaultSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.exported).into());
    children
  }

}

impl<'a> CastableNode<'a> for ExportDefaultSpecifier<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportDefaultSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_export_default_specifier<'a>(ref_node: &'a swc_ecma_ast::ExportDefaultSpecifier) -> ExportDefaultSpecifier<'a> {
  let value = &ref_node.exported;
  let field_exported = get_view_for_ident(value);
  let mut node = ExportDefaultSpecifier {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    exported: field_exported,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportDefaultSpecifier, &'a ExportDefaultSpecifier>(&node) };
  let parent = Node::ExportDefaultSpecifier(child_parent_ref);
  node.exported.parent = parent;
  node
}

pub struct TsTypeParam<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeParam,
  pub name: Ident<'a>,
  pub constraint: Option<Box<TsType<'a>>>,
  pub default: Option<Box<TsType<'a>>>,
}

impl<'a> Spanned for TsTypeParam<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeParam<'a>> for Node<'a> {
  fn from(node: &TsTypeParam) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeParam, &'a TsTypeParam>(&node) };
    Node::TsTypeParam(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeParam<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsTypeParam<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeParam(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_param<'a>(ref_node: &'a swc_ecma_ast::TsTypeParam) -> TsTypeParam<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeParam, &'a TsTypeParam>(&node) };
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

pub struct WithStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::WithStmt,
  pub obj: Box<Expr<'a>>,
  pub body: Box<Stmt<'a>>,
}

impl<'a> Spanned for WithStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&WithStmt<'a>> for Node<'a> {
  fn from(node: &WithStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&WithStmt, &'a WithStmt>(&node) };
    Node::WithStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for WithStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push((&self.body).into());
    children
  }

}

impl<'a> CastableNode<'a> for WithStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::WithStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_with_stmt<'a>(ref_node: &'a swc_ecma_ast::WithStmt) -> WithStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&WithStmt, &'a WithStmt>(&node) };
  let parent = Node::WithStmt(child_parent_ref);
  set_parent_for_expr(&mut node.obj, parent.clone());
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct Regex<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Regex,
}

impl<'a> Regex<'a> {
  pub fn exp(&self) -> &swc_atoms::JsWord {
    &self.inner.exp
  }

  pub fn flags(&self) -> &swc_atoms::JsWord {
    &self.inner.flags
  }
}

impl<'a> Spanned for Regex<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Regex<'a>> for Node<'a> {
  fn from(node: &Regex) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Regex, &'a Regex>(&node) };
    Node::Regex(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Regex<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for Regex<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Regex(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_regex<'a>(ref_node: &'a swc_ecma_ast::Regex) -> Regex<'a> {
  let node = Regex {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsMethodSignature<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsMethodSignature,
  pub key: Box<Expr<'a>>,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
}

impl<'a> TsMethodSignature<'a> {
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

impl<'a> Spanned for TsMethodSignature<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsMethodSignature<'a>> for Node<'a> {
  fn from(node: &TsMethodSignature) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsMethodSignature, &'a TsMethodSignature>(&node) };
    Node::TsMethodSignature(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsMethodSignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsMethodSignature<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsMethodSignature(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_method_signature<'a>(ref_node: &'a swc_ecma_ast::TsMethodSignature) -> TsMethodSignature<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsMethodSignature, &'a TsMethodSignature>(&node) };
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

pub struct UpdateExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::UpdateExpr,
  pub arg: Box<Expr<'a>>,
}

impl<'a> UpdateExpr<'a> {
  pub fn op(&self) -> &UpdateOp {
    &self.inner.op
  }

  pub fn prefix(&self) -> bool {
    self.inner.prefix
  }
}

impl<'a> Spanned for UpdateExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&UpdateExpr<'a>> for Node<'a> {
  fn from(node: &UpdateExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&UpdateExpr, &'a UpdateExpr>(&node) };
    Node::UpdateExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for UpdateExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

}

impl<'a> CastableNode<'a> for UpdateExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::UpdateExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_update_expr<'a>(ref_node: &'a swc_ecma_ast::UpdateExpr) -> UpdateExpr<'a> {
  let value = &ref_node.arg;
  let field_arg = Box::new(get_view_for_expr(value));
  let mut node = UpdateExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
  };
  let child_parent_ref = unsafe { mem::transmute::<&UpdateExpr, &'a UpdateExpr>(&node) };
  let parent = Node::UpdateExpr(child_parent_ref);
  set_parent_for_expr(&mut node.arg, parent);
  node
}

pub struct SetterProp<'a> {
  pub parent: &'a ObjectLit<'a>,
  pub inner: &'a swc_ecma_ast::SetterProp,
  pub key: PropName<'a>,
  pub param: Pat<'a>,
  pub body: Option<BlockStmt<'a>>,
}

impl<'a> Spanned for SetterProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&SetterProp<'a>> for Node<'a> {
  fn from(node: &SetterProp) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&SetterProp, &'a SetterProp>(&node) };
    Node::SetterProp(static_ref)
  }
}

impl<'a> NodeTrait<'a> for SetterProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    children.push((&self.param).into());
    if let Some(child) = &self.body {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for SetterProp<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SetterProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_setter_prop<'a>(ref_node: &'a swc_ecma_ast::SetterProp) -> SetterProp<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&SetterProp, &'a SetterProp>(&node) };
  let parent = Node::SetterProp(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  set_parent_for_pat(&mut node.param, parent.clone());
  if let Some(node) = node.body.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TaggedTpl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TaggedTpl,
  pub tag: Box<Expr<'a>>,
  pub exprs: Vec<Box<Expr<'a>>>,
  pub quasis: Vec<TplElement<'a>>,
  pub type_params: Option<TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for TaggedTpl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TaggedTpl<'a>> for Node<'a> {
  fn from(node: &TaggedTpl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TaggedTpl, &'a TaggedTpl>(&node) };
    Node::TaggedTpl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TaggedTpl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TaggedTpl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TaggedTpl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_tagged_tpl<'a>(ref_node: &'a swc_ecma_ast::TaggedTpl) -> TaggedTpl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TaggedTpl, &'a TaggedTpl>(&node) };
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
pub struct ExportAll<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ExportAll,
  pub src: Str<'a>,
}

impl<'a> Spanned for ExportAll<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportAll<'a>> for Node<'a> {
  fn from(node: &ExportAll) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ExportAll, &'a ExportAll>(&node) };
    Node::ExportAll(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ExportAll<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.src).into());
    children
  }

}

impl<'a> CastableNode<'a> for ExportAll<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportAll(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_export_all<'a>(ref_node: &'a swc_ecma_ast::ExportAll) -> ExportAll<'a> {
  let value = &ref_node.src;
  let field_src = get_view_for_str(value);
  let mut node = ExportAll {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    src: field_src,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportAll, &'a ExportAll>(&node) };
  let parent = Node::ExportAll(child_parent_ref);
  node.src.parent = parent;
  node
}

pub struct TsModuleBlock<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsModuleBlock,
  pub body: Vec<ModuleItem<'a>>,
}

impl<'a> Spanned for TsModuleBlock<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsModuleBlock<'a>> for Node<'a> {
  fn from(node: &TsModuleBlock) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsModuleBlock, &'a TsModuleBlock>(&node) };
    Node::TsModuleBlock(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsModuleBlock<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.body.len());
    for child in self.body.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsModuleBlock<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsModuleBlock(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_module_block<'a>(ref_node: &'a swc_ecma_ast::TsModuleBlock) -> TsModuleBlock<'a> {
  let value = &ref_node.body;
  let field_body = value.iter().map(|value| get_view_for_module_item(value)).collect();
  let mut node = TsModuleBlock {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsModuleBlock, &'a TsModuleBlock>(&node) };
  let parent = Node::TsModuleBlock(child_parent_ref);
  for node in node.body.iter_mut() {
    set_parent_for_module_item(node, parent.clone());
  }
  node
}

pub struct SwitchStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::SwitchStmt,
  pub discriminant: Box<Expr<'a>>,
  pub cases: Vec<SwitchCase<'a>>,
}

impl<'a> Spanned for SwitchStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&SwitchStmt<'a>> for Node<'a> {
  fn from(node: &SwitchStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&SwitchStmt, &'a SwitchStmt>(&node) };
    Node::SwitchStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for SwitchStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.cases.len());
    children.push((&self.discriminant).into());
    for child in self.cases.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for SwitchStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SwitchStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_switch_stmt<'a>(ref_node: &'a swc_ecma_ast::SwitchStmt) -> SwitchStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&SwitchStmt, &'a SwitchStmt>(&node) };
  let parent = Node::SwitchStmt(child_parent_ref);
  set_parent_for_expr(&mut node.discriminant, parent.clone());
  for node in node.cases.iter_mut() {
    node.parent = parent.to::<SwitchStmt>();
  }
  node
}

pub struct TsEnumMember<'a> {
  pub parent: &'a TsEnumDecl<'a>,
  pub inner: &'a swc_ecma_ast::TsEnumMember,
  pub id: TsEnumMemberId<'a>,
  pub init: Option<Box<Expr<'a>>>,
}

impl<'a> Spanned for TsEnumMember<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsEnumMember<'a>> for Node<'a> {
  fn from(node: &TsEnumMember) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsEnumMember, &'a TsEnumMember>(&node) };
    Node::TsEnumMember(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsEnumMember<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, });
    children.push((&self.id).into());
    if let Some(child) = &self.init {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsEnumMember<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsEnumMember(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_enum_member<'a>(ref_node: &'a swc_ecma_ast::TsEnumMember) -> TsEnumMember<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsEnumMember, &'a TsEnumMember>(&node) };
  let parent = Node::TsEnumMember(child_parent_ref);
  set_parent_for_ts_enum_member_id(&mut node.id, parent.clone());
  if let Some(node) = node.init.as_mut() {
    set_parent_for_expr(node, parent);
  }
  node
}

pub struct TsIndexedAccessType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsIndexedAccessType,
  pub obj_type: Box<TsType<'a>>,
  pub index_type: Box<TsType<'a>>,
}

impl<'a> TsIndexedAccessType<'a> {
  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }
}

impl<'a> Spanned for TsIndexedAccessType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsIndexedAccessType<'a>> for Node<'a> {
  fn from(node: &TsIndexedAccessType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsIndexedAccessType, &'a TsIndexedAccessType>(&node) };
    Node::TsIndexedAccessType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsIndexedAccessType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj_type).into());
    children.push((&self.index_type).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsIndexedAccessType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsIndexedAccessType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_indexed_access_type<'a>(ref_node: &'a swc_ecma_ast::TsIndexedAccessType) -> TsIndexedAccessType<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsIndexedAccessType, &'a TsIndexedAccessType>(&node) };
  let parent = Node::TsIndexedAccessType(child_parent_ref);
  set_parent_for_ts_type(&mut node.obj_type, parent.clone());
  set_parent_for_ts_type(&mut node.index_type, parent);
  node
}

pub struct TsRestType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsRestType,
  pub type_ann: Box<TsType<'a>>,
}

impl<'a> Spanned for TsRestType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsRestType<'a>> for Node<'a> {
  fn from(node: &TsRestType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsRestType, &'a TsRestType>(&node) };
    Node::TsRestType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsRestType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsRestType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsRestType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_rest_type<'a>(ref_node: &'a swc_ecma_ast::TsRestType) -> TsRestType<'a> {
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsRestType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsRestType, &'a TsRestType>(&node) };
  let parent = Node::TsRestType(child_parent_ref);
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct ExprStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ExprStmt,
  pub expr: Box<Expr<'a>>,
}

impl<'a> Spanned for ExprStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExprStmt<'a>> for Node<'a> {
  fn from(node: &ExprStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ExprStmt, &'a ExprStmt>(&node) };
    Node::ExprStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ExprStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for ExprStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExprStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_expr_stmt<'a>(ref_node: &'a swc_ecma_ast::ExprStmt) -> ExprStmt<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = ExprStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExprStmt, &'a ExprStmt>(&node) };
  let parent = Node::ExprStmt(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsOptionalType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsOptionalType,
  pub type_ann: Box<TsType<'a>>,
}

impl<'a> Spanned for TsOptionalType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsOptionalType<'a>> for Node<'a> {
  fn from(node: &TsOptionalType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsOptionalType, &'a TsOptionalType>(&node) };
    Node::TsOptionalType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsOptionalType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsOptionalType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsOptionalType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_optional_type<'a>(ref_node: &'a swc_ecma_ast::TsOptionalType) -> TsOptionalType<'a> {
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsOptionalType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsOptionalType, &'a TsOptionalType>(&node) };
  let parent = Node::TsOptionalType(child_parent_ref);
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct Tpl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Tpl,
  pub exprs: Vec<Box<Expr<'a>>>,
  pub quasis: Vec<TplElement<'a>>,
}

impl<'a> Spanned for Tpl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Tpl<'a>> for Node<'a> {
  fn from(node: &Tpl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Tpl, &'a Tpl>(&node) };
    Node::Tpl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Tpl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for Tpl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Tpl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_tpl<'a>(ref_node: &'a swc_ecma_ast::Tpl) -> Tpl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&Tpl, &'a Tpl>(&node) };
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
pub struct Invalid<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Invalid,
}

impl<'a> Spanned for Invalid<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Invalid<'a>> for Node<'a> {
  fn from(node: &Invalid) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Invalid, &'a Invalid>(&node) };
    Node::Invalid(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Invalid<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for Invalid<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Invalid(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_invalid<'a>(ref_node: &'a swc_ecma_ast::Invalid) -> Invalid<'a> {
  let node = Invalid {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct ComputedPropName<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ComputedPropName,
  pub expr: Box<Expr<'a>>,
}

impl<'a> Spanned for ComputedPropName<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ComputedPropName<'a>> for Node<'a> {
  fn from(node: &ComputedPropName) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ComputedPropName, &'a ComputedPropName>(&node) };
    Node::ComputedPropName(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ComputedPropName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for ComputedPropName<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ComputedPropName(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_computed_prop_name<'a>(ref_node: &'a swc_ecma_ast::ComputedPropName) -> ComputedPropName<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = ComputedPropName {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ComputedPropName, &'a ComputedPropName>(&node) };
  let parent = Node::ComputedPropName(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsFnType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsFnType,
  pub params: Vec<TsFnParam<'a>>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
  pub type_ann: TsTypeAnn<'a>,
}

impl<'a> Spanned for TsFnType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsFnType<'a>> for Node<'a> {
  fn from(node: &TsFnType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsFnType, &'a TsFnType>(&node) };
    Node::TsFnType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsFnType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsFnType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsFnType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_fn_type<'a>(ref_node: &'a swc_ecma_ast::TsFnType) -> TsFnType<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsFnType, &'a TsFnType>(&node) };
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
pub struct BlockStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::BlockStmt,
  pub stmts: Vec<Stmt<'a>>,
}

impl<'a> Spanned for BlockStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&BlockStmt<'a>> for Node<'a> {
  fn from(node: &BlockStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&BlockStmt, &'a BlockStmt>(&node) };
    Node::BlockStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for BlockStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.stmts.len());
    for child in self.stmts.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for BlockStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::BlockStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_block_stmt<'a>(ref_node: &'a swc_ecma_ast::BlockStmt) -> BlockStmt<'a> {
  let value = &ref_node.stmts;
  let field_stmts = value.iter().map(|value| get_view_for_stmt(value)).collect();
  let mut node = BlockStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    stmts: field_stmts,
  };
  let child_parent_ref = unsafe { mem::transmute::<&BlockStmt, &'a BlockStmt>(&node) };
  let parent = Node::BlockStmt(child_parent_ref);
  for node in node.stmts.iter_mut() {
    set_parent_for_stmt(node, parent.clone());
  }
  node
}

pub struct TsTypeAliasDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeAliasDecl,
  pub id: Ident<'a>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
  pub type_ann: Box<TsType<'a>>,
}

impl<'a> TsTypeAliasDecl<'a> {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl<'a> Spanned for TsTypeAliasDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeAliasDecl<'a>> for Node<'a> {
  fn from(node: &TsTypeAliasDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeAliasDecl, &'a TsTypeAliasDecl>(&node) };
    Node::TsTypeAliasDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeAliasDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.id).into());
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children.push((&self.type_ann).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsTypeAliasDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeAliasDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_alias_decl<'a>(ref_node: &'a swc_ecma_ast::TsTypeAliasDecl) -> TsTypeAliasDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeAliasDecl, &'a TsTypeAliasDecl>(&node) };
  let parent = Node::TsTypeAliasDecl(child_parent_ref);
  node.id.parent = parent.clone();
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent.clone();
  }
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct MemberExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::MemberExpr,
  pub obj: ExprOrSuper<'a>,
  pub prop: Box<Expr<'a>>,
}

impl<'a> MemberExpr<'a> {
  pub fn computed(&self) -> bool {
    self.inner.computed
  }
}

impl<'a> Spanned for MemberExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&MemberExpr<'a>> for Node<'a> {
  fn from(node: &MemberExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&MemberExpr, &'a MemberExpr>(&node) };
    Node::MemberExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for MemberExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push((&self.prop).into());
    children
  }

}

impl<'a> CastableNode<'a> for MemberExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::MemberExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_member_expr<'a>(ref_node: &'a swc_ecma_ast::MemberExpr) -> MemberExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&MemberExpr, &'a MemberExpr>(&node) };
  let parent = Node::MemberExpr(child_parent_ref);
  set_parent_for_expr_or_super(&mut node.obj, parent.clone());
  set_parent_for_expr(&mut node.prop, parent);
  node
}

/// Common parts of function and method.
pub struct Function<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Function,
  pub params: Vec<Param<'a>>,
  pub decorators: Vec<Decorator<'a>>,
  pub body: Option<BlockStmt<'a>>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
  pub return_type: Option<TsTypeAnn<'a>>,
}

impl<'a> Function<'a> {
  /// if it's a generator.
  pub fn is_generator(&self) -> bool {
    self.inner.is_generator
  }

  /// if it's an async function.
  pub fn is_async(&self) -> bool {
    self.inner.is_async
  }
}

impl<'a> Spanned for Function<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Function<'a>> for Node<'a> {
  fn from(node: &Function) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Function, &'a Function>(&node) };
    Node::Function(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Function<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for Function<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Function(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_function<'a>(ref_node: &'a swc_ecma_ast::Function) -> Function<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&Function, &'a Function>(&node) };
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

pub struct ImportDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ImportDecl,
  pub specifiers: Vec<ImportSpecifier<'a>>,
  pub src: Str<'a>,
  pub asserts: Option<ObjectLit<'a>>,
}

impl<'a> ImportDecl<'a> {
  pub fn type_only(&self) -> bool {
    self.inner.type_only
  }
}

impl<'a> Spanned for ImportDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ImportDecl<'a>> for Node<'a> {
  fn from(node: &ImportDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ImportDecl, &'a ImportDecl>(&node) };
    Node::ImportDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ImportDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for ImportDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ImportDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_import_decl<'a>(ref_node: &'a swc_ecma_ast::ImportDecl) -> ImportDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ImportDecl, &'a ImportDecl>(&node) };
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

pub struct TsTypePredicate<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypePredicate,
  pub param_name: TsThisTypeOrIdent<'a>,
  pub type_ann: Option<TsTypeAnn<'a>>,
}

impl<'a> TsTypePredicate<'a> {
  pub fn asserts(&self) -> bool {
    self.inner.asserts
  }
}

impl<'a> Spanned for TsTypePredicate<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypePredicate<'a>> for Node<'a> {
  fn from(node: &TsTypePredicate) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypePredicate, &'a TsTypePredicate>(&node) };
    Node::TsTypePredicate(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypePredicate<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.param_name).into());
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsTypePredicate<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypePredicate(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_predicate<'a>(ref_node: &'a swc_ecma_ast::TsTypePredicate) -> TsTypePredicate<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsTypePredicate, &'a TsTypePredicate>(&node) };
  let parent = Node::TsTypePredicate(child_parent_ref);
  set_parent_for_ts_this_type_or_ident(&mut node.param_name, parent.clone());
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct YieldExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::YieldExpr,
  pub arg: Option<Box<Expr<'a>>>,
}

impl<'a> YieldExpr<'a> {
  pub fn delegate(&self) -> bool {
    self.inner.delegate
  }
}

impl<'a> Spanned for YieldExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&YieldExpr<'a>> for Node<'a> {
  fn from(node: &YieldExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&YieldExpr, &'a YieldExpr>(&node) };
    Node::YieldExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for YieldExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.arg { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.arg {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for YieldExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::YieldExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_yield_expr<'a>(ref_node: &'a swc_ecma_ast::YieldExpr) -> YieldExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&YieldExpr, &'a YieldExpr>(&node) };
  let parent = Node::YieldExpr(child_parent_ref);
  if let Some(node) = node.arg.as_mut() {
    set_parent_for_expr(node, parent);
  }
  node
}

pub struct KeyValueProp<'a> {
  pub parent: &'a ObjectLit<'a>,
  pub inner: &'a swc_ecma_ast::KeyValueProp,
  pub key: PropName<'a>,
  pub value: Box<Expr<'a>>,
}

impl<'a> Spanned for KeyValueProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&KeyValueProp<'a>> for Node<'a> {
  fn from(node: &KeyValueProp) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&KeyValueProp, &'a KeyValueProp>(&node) };
    Node::KeyValueProp(static_ref)
  }
}

impl<'a> NodeTrait<'a> for KeyValueProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.value).into());
    children
  }

}

impl<'a> CastableNode<'a> for KeyValueProp<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::KeyValueProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_key_value_prop<'a>(ref_node: &'a swc_ecma_ast::KeyValueProp) -> KeyValueProp<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&KeyValueProp, &'a KeyValueProp>(&node) };
  let parent = Node::KeyValueProp(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  set_parent_for_expr(&mut node.value, parent);
  node
}

pub struct Param<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Param,
  pub decorators: Vec<Decorator<'a>>,
  pub pat: Pat<'a>,
}

impl<'a> Spanned for Param<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Param<'a>> for Node<'a> {
  fn from(node: &Param) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Param, &'a Param>(&node) };
    Node::Param(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Param<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.decorators.len());
    for child in self.decorators.iter() {
      children.push(child.into());
    }
    children.push((&self.pat).into());
    children
  }

}

impl<'a> CastableNode<'a> for Param<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Param(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_param<'a>(ref_node: &'a swc_ecma_ast::Param) -> Param<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&Param, &'a Param>(&node) };
  let parent = Node::Param(child_parent_ref);
  for node in node.decorators.iter_mut() {
    node.parent = parent.clone();
  }
  set_parent_for_pat(&mut node.pat, parent);
  node
}

pub struct JSXFragment<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::JSXFragment,
  pub opening: JSXOpeningFragment<'a>,
  pub children: Vec<JSXElementChild<'a>>,
  pub closing: JSXClosingFragment<'a>,
}

impl<'a> Spanned for JSXFragment<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXFragment<'a>> for Node<'a> {
  fn from(node: &JSXFragment) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXFragment, &'a JSXFragment>(&node) };
    Node::JSXFragment(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXFragment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + self.children.len());
    children.push((&self.opening).into());
    for child in self.children.iter() {
      children.push(child.into());
    }
    children.push((&self.closing).into());
    children
  }

}

impl<'a> CastableNode<'a> for JSXFragment<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXFragment(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxfragment<'a>(ref_node: &'a swc_ecma_ast::JSXFragment) -> JSXFragment<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&JSXFragment, &'a JSXFragment>(&node) };
  let parent = Node::JSXFragment(child_parent_ref);
  node.opening.parent = parent.to::<JSXFragment>();
  for node in node.children.iter_mut() {
    set_parent_for_jsxelement_child(node, parent.clone());
  }
  node.closing.parent = parent.to::<JSXFragment>();
  node
}

/// e.g. `import foo from 'mod.js'`
pub struct ImportDefaultSpecifier<'a> {
  pub parent: &'a ImportDecl<'a>,
  pub inner: &'a swc_ecma_ast::ImportDefaultSpecifier,
  pub local: Ident<'a>,
}

impl<'a> Spanned for ImportDefaultSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ImportDefaultSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportDefaultSpecifier) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ImportDefaultSpecifier, &'a ImportDefaultSpecifier>(&node) };
    Node::ImportDefaultSpecifier(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ImportDefaultSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.local).into());
    children
  }

}

impl<'a> CastableNode<'a> for ImportDefaultSpecifier<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ImportDefaultSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_import_default_specifier<'a>(ref_node: &'a swc_ecma_ast::ImportDefaultSpecifier) -> ImportDefaultSpecifier<'a> {
  let value = &ref_node.local;
  let field_local = get_view_for_ident(value);
  let mut node = ImportDefaultSpecifier {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    local: field_local,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ImportDefaultSpecifier, &'a ImportDefaultSpecifier>(&node) };
  let parent = Node::ImportDefaultSpecifier(child_parent_ref);
  node.local.parent = parent;
  node
}

pub struct Number<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Number,
}

impl<'a> Number<'a> {
  /// **Note**: This should not be `NaN`. Use [crate::Ident] to represent NaN.
  /// 
  /// If you store `NaN` in this field, a hash map will behave strangely.
  pub fn value(&self) -> f64 {
    self.inner.value
  }
}

impl<'a> Spanned for Number<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Number<'a>> for Node<'a> {
  fn from(node: &Number) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Number, &'a Number>(&node) };
    Node::Number(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Number<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for Number<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Number(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_number<'a>(ref_node: &'a swc_ecma_ast::Number) -> Number<'a> {
  let node = Number {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct JSXAttr<'a> {
  pub parent: &'a JSXOpeningElement<'a>,
  pub inner: &'a swc_ecma_ast::JSXAttr,
  pub name: JSXAttrName<'a>,
  /// Babel uses Expr instead of JSXAttrValue
  pub value: Option<JSXAttrValue<'a>>,
}

impl<'a> Spanned for JSXAttr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXAttr<'a>> for Node<'a> {
  fn from(node: &JSXAttr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXAttr, &'a JSXAttr>(&node) };
    Node::JSXAttr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXAttr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    if let Some(child) = &self.value {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for JSXAttr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXAttr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxattr<'a>(ref_node: &'a swc_ecma_ast::JSXAttr) -> JSXAttr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&JSXAttr, &'a JSXAttr>(&node) };
  let parent = Node::JSXAttr(child_parent_ref);
  set_parent_for_jsxattr_name(&mut node.name, parent.clone());
  if let Some(node) = node.value.as_mut() {
    set_parent_for_jsxattr_value(node, parent);
  }
  node
}

pub struct ParenExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ParenExpr,
  pub expr: Box<Expr<'a>>,
}

impl<'a> Spanned for ParenExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ParenExpr<'a>> for Node<'a> {
  fn from(node: &ParenExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ParenExpr, &'a ParenExpr>(&node) };
    Node::ParenExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ParenExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for ParenExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ParenExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_paren_expr<'a>(ref_node: &'a swc_ecma_ast::ParenExpr) -> ParenExpr<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = ParenExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ParenExpr, &'a ParenExpr>(&node) };
  let parent = Node::ParenExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct Super<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Super,
}

impl<'a> Spanned for Super<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Super<'a>> for Node<'a> {
  fn from(node: &Super) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Super, &'a Super>(&node) };
    Node::Super(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Super<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for Super<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Super(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_super<'a>(ref_node: &'a swc_ecma_ast::Super) -> Super<'a> {
  let node = Super {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsConstructorType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsConstructorType,
  pub params: Vec<TsFnParam<'a>>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
  pub type_ann: TsTypeAnn<'a>,
}

impl<'a> Spanned for TsConstructorType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsConstructorType<'a>> for Node<'a> {
  fn from(node: &TsConstructorType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsConstructorType, &'a TsConstructorType>(&node) };
    Node::TsConstructorType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsConstructorType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsConstructorType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsConstructorType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_constructor_type<'a>(ref_node: &'a swc_ecma_ast::TsConstructorType) -> TsConstructorType<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsConstructorType, &'a TsConstructorType>(&node) };
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

pub struct Class<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Class,
  pub decorators: Vec<Decorator<'a>>,
  pub body: Vec<ClassMember<'a>>,
  pub super_class: Option<Box<Expr<'a>>>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
  pub super_type_params: Option<TsTypeParamInstantiation<'a>>,
  /// Typescript extension.
  pub implements: Vec<TsExprWithTypeArgs<'a>>,
}

impl<'a> Class<'a> {
  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }
}

impl<'a> Spanned for Class<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Class<'a>> for Node<'a> {
  fn from(node: &Class) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Class, &'a Class>(&node) };
    Node::Class(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Class<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for Class<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Class(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_class<'a>(ref_node: &'a swc_ecma_ast::Class) -> Class<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&Class, &'a Class>(&node) };
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
pub struct RestPat<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::RestPat,
  pub arg: Box<Pat<'a>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
}

impl<'a> RestPat<'a> {
  pub fn dot3_token(&self) -> &swc_common::Span {
    &self.inner.dot3_token
  }
}

impl<'a> Spanned for RestPat<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&RestPat<'a>> for Node<'a> {
  fn from(node: &RestPat) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&RestPat, &'a RestPat>(&node) };
    Node::RestPat(static_ref)
  }
}

impl<'a> NodeTrait<'a> for RestPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.arg).into());
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for RestPat<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::RestPat(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_rest_pat<'a>(ref_node: &'a swc_ecma_ast::RestPat) -> RestPat<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&RestPat, &'a RestPat>(&node) };
  let parent = Node::RestPat(child_parent_ref);
  set_parent_for_pat(&mut node.arg, parent.clone());
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsNamespaceExportDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsNamespaceExportDecl,
  pub id: Ident<'a>,
}

impl<'a> Spanned for TsNamespaceExportDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsNamespaceExportDecl<'a>> for Node<'a> {
  fn from(node: &TsNamespaceExportDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsNamespaceExportDecl, &'a TsNamespaceExportDecl>(&node) };
    Node::TsNamespaceExportDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsNamespaceExportDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.id).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsNamespaceExportDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsNamespaceExportDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_namespace_export_decl<'a>(ref_node: &'a swc_ecma_ast::TsNamespaceExportDecl) -> TsNamespaceExportDecl<'a> {
  let value = &ref_node.id;
  let field_id = get_view_for_ident(value);
  let mut node = TsNamespaceExportDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsNamespaceExportDecl, &'a TsNamespaceExportDecl>(&node) };
  let parent = Node::TsNamespaceExportDecl(child_parent_ref);
  node.id.parent = parent;
  node
}

pub struct JSXOpeningFragment<'a> {
  pub parent: &'a JSXFragment<'a>,
  pub inner: &'a swc_ecma_ast::JSXOpeningFragment,
}

impl<'a> Spanned for JSXOpeningFragment<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXOpeningFragment<'a>> for Node<'a> {
  fn from(node: &JSXOpeningFragment) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXOpeningFragment, &'a JSXOpeningFragment>(&node) };
    Node::JSXOpeningFragment(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXOpeningFragment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for JSXOpeningFragment<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXOpeningFragment(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxopening_fragment<'a>(ref_node: &'a swc_ecma_ast::JSXOpeningFragment) -> JSXOpeningFragment<'a> {
  let node = JSXOpeningFragment {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct NewExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::NewExpr,
  pub callee: Box<Expr<'a>>,
  pub args: Option<Vec<ExprOrSpread<'a>>>,
  pub type_args: Option<TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for NewExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&NewExpr<'a>> for Node<'a> {
  fn from(node: &NewExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&NewExpr, &'a NewExpr>(&node) };
    Node::NewExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for NewExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for NewExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::NewExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_new_expr<'a>(ref_node: &'a swc_ecma_ast::NewExpr) -> NewExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&NewExpr, &'a NewExpr>(&node) };
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
pub struct FnExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::FnExpr,
  pub ident: Option<Ident<'a>>,
  pub function: Function<'a>,
}

impl<'a> Spanned for FnExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&FnExpr<'a>> for Node<'a> {
  fn from(node: &FnExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&FnExpr, &'a FnExpr>(&node) };
    Node::FnExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for FnExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.ident { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.ident {
      children.push(child.into());
    }
    children.push((&self.function).into());
    children
  }

}

impl<'a> CastableNode<'a> for FnExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::FnExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_fn_expr<'a>(ref_node: &'a swc_ecma_ast::FnExpr) -> FnExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&FnExpr, &'a FnExpr>(&node) };
  let parent = Node::FnExpr(child_parent_ref);
  if let Some(node) = node.ident.as_mut() {
    node.parent = parent.clone();
  }
  node.function.parent = parent;
  node
}

pub struct IfStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::IfStmt,
  pub test: Box<Expr<'a>>,
  pub cons: Box<Stmt<'a>>,
  pub alt: Option<Box<Stmt<'a>>>,
}

impl<'a> Spanned for IfStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&IfStmt<'a>> for Node<'a> {
  fn from(node: &IfStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&IfStmt, &'a IfStmt>(&node) };
    Node::IfStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for IfStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + match &self.alt { Some(_value) => 1, None => 0, });
    children.push((&self.test).into());
    children.push((&self.cons).into());
    if let Some(child) = &self.alt {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for IfStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::IfStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_if_stmt<'a>(ref_node: &'a swc_ecma_ast::IfStmt) -> IfStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&IfStmt, &'a IfStmt>(&node) };
  let parent = Node::IfStmt(child_parent_ref);
  set_parent_for_expr(&mut node.test, parent.clone());
  set_parent_for_stmt(&mut node.cons, parent.clone());
  if let Some(node) = node.alt.as_mut() {
    set_parent_for_stmt(node, parent);
  }
  node
}

pub struct TsParenthesizedType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsParenthesizedType,
  pub type_ann: Box<TsType<'a>>,
}

impl<'a> Spanned for TsParenthesizedType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsParenthesizedType<'a>> for Node<'a> {
  fn from(node: &TsParenthesizedType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsParenthesizedType, &'a TsParenthesizedType>(&node) };
    Node::TsParenthesizedType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsParenthesizedType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsParenthesizedType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsParenthesizedType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_parenthesized_type<'a>(ref_node: &'a swc_ecma_ast::TsParenthesizedType) -> TsParenthesizedType<'a> {
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsParenthesizedType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsParenthesizedType, &'a TsParenthesizedType>(&node) };
  let parent = Node::TsParenthesizedType(child_parent_ref);
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

/// `{key}` or `{key = value}`
pub struct AssignPatProp<'a> {
  pub parent: &'a ObjectPat<'a>,
  pub inner: &'a swc_ecma_ast::AssignPatProp,
  pub key: Ident<'a>,
  pub value: Option<Box<Expr<'a>>>,
}

impl<'a> Spanned for AssignPatProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&AssignPatProp<'a>> for Node<'a> {
  fn from(node: &AssignPatProp) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&AssignPatProp, &'a AssignPatProp>(&node) };
    Node::AssignPatProp(static_ref)
  }
}

impl<'a> NodeTrait<'a> for AssignPatProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    if let Some(child) = &self.value {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for AssignPatProp<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::AssignPatProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_assign_pat_prop<'a>(ref_node: &'a swc_ecma_ast::AssignPatProp) -> AssignPatProp<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&AssignPatProp, &'a AssignPatProp>(&node) };
  let parent = Node::AssignPatProp(child_parent_ref);
  node.key.parent = parent.clone();
  if let Some(node) = node.value.as_mut() {
    set_parent_for_expr(node, parent);
  }
  node
}

pub struct TsImportType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsImportType,
  pub arg: Str<'a>,
  pub qualifier: Option<TsEntityName<'a>>,
  pub type_args: Option<TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for TsImportType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsImportType<'a>> for Node<'a> {
  fn from(node: &TsImportType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsImportType, &'a TsImportType>(&node) };
    Node::TsImportType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsImportType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsImportType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsImportType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_import_type<'a>(ref_node: &'a swc_ecma_ast::TsImportType) -> TsImportType<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsImportType, &'a TsImportType>(&node) };
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

pub struct Bool<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Bool,
}

impl<'a> Bool<'a> {
  pub fn value(&self) -> bool {
    self.inner.value
  }
}

impl<'a> Spanned for Bool<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Bool<'a>> for Node<'a> {
  fn from(node: &Bool) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Bool, &'a Bool>(&node) };
    Node::Bool(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Bool<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for Bool<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Bool(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_bool<'a>(ref_node: &'a swc_ecma_ast::Bool) -> Bool<'a> {
  let node = Bool {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsImportEqualsDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsImportEqualsDecl,
  pub id: Ident<'a>,
  pub module_ref: TsModuleRef<'a>,
}

impl<'a> TsImportEqualsDecl<'a> {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  pub fn is_export(&self) -> bool {
    self.inner.is_export
  }
}

impl<'a> Spanned for TsImportEqualsDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsImportEqualsDecl<'a>> for Node<'a> {
  fn from(node: &TsImportEqualsDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsImportEqualsDecl, &'a TsImportEqualsDecl>(&node) };
    Node::TsImportEqualsDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsImportEqualsDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.id).into());
    children.push((&self.module_ref).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsImportEqualsDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsImportEqualsDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_import_equals_decl<'a>(ref_node: &'a swc_ecma_ast::TsImportEqualsDecl) -> TsImportEqualsDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsImportEqualsDecl, &'a TsImportEqualsDecl>(&node) };
  let parent = Node::TsImportEqualsDecl(child_parent_ref);
  node.id.parent = parent.clone();
  set_parent_for_ts_module_ref(&mut node.module_ref, parent);
  node
}

pub struct AssignProp<'a> {
  pub parent: &'a ObjectLit<'a>,
  pub inner: &'a swc_ecma_ast::AssignProp,
  pub key: Ident<'a>,
  pub value: Box<Expr<'a>>,
}

impl<'a> Spanned for AssignProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&AssignProp<'a>> for Node<'a> {
  fn from(node: &AssignProp) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&AssignProp, &'a AssignProp>(&node) };
    Node::AssignProp(static_ref)
  }
}

impl<'a> NodeTrait<'a> for AssignProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.value).into());
    children
  }

}

impl<'a> CastableNode<'a> for AssignProp<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::AssignProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_assign_prop<'a>(ref_node: &'a swc_ecma_ast::AssignProp) -> AssignProp<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&AssignProp, &'a AssignProp>(&node) };
  let parent = Node::AssignProp(child_parent_ref);
  node.key.parent = parent.clone();
  set_parent_for_expr(&mut node.value, parent);
  node
}

pub struct TsInterfaceDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsInterfaceDecl,
  pub id: Ident<'a>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
  pub extends: Vec<TsExprWithTypeArgs<'a>>,
  pub body: TsInterfaceBody<'a>,
}

impl<'a> TsInterfaceDecl<'a> {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl<'a> Spanned for TsInterfaceDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsInterfaceDecl<'a>> for Node<'a> {
  fn from(node: &TsInterfaceDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsInterfaceDecl, &'a TsInterfaceDecl>(&node) };
    Node::TsInterfaceDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsInterfaceDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsInterfaceDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsInterfaceDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_interface_decl<'a>(ref_node: &'a swc_ecma_ast::TsInterfaceDecl) -> TsInterfaceDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsInterfaceDecl, &'a TsInterfaceDecl>(&node) };
  let parent = Node::TsInterfaceDecl(child_parent_ref);
  node.id.parent = parent.clone();
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent.clone();
  }
  for node in node.extends.iter_mut() {
    node.parent = parent.clone();
  }
  node.body.parent = parent.to::<TsInterfaceDecl>();
  node
}

pub struct JSXEmptyExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::JSXEmptyExpr,
}

impl<'a> Spanned for JSXEmptyExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXEmptyExpr<'a>> for Node<'a> {
  fn from(node: &JSXEmptyExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXEmptyExpr, &'a JSXEmptyExpr>(&node) };
    Node::JSXEmptyExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXEmptyExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for JSXEmptyExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXEmptyExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxempty_expr<'a>(ref_node: &'a swc_ecma_ast::JSXEmptyExpr) -> JSXEmptyExpr<'a> {
  let node = JSXEmptyExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsQualifiedName<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsQualifiedName,
  pub left: TsEntityName<'a>,
  pub right: Ident<'a>,
}

impl<'a> Spanned for TsQualifiedName<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsQualifiedName<'a>> for Node<'a> {
  fn from(node: &TsQualifiedName) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsQualifiedName, &'a TsQualifiedName>(&node) };
    Node::TsQualifiedName(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsQualifiedName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsQualifiedName<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsQualifiedName(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_qualified_name<'a>(ref_node: &'a swc_ecma_ast::TsQualifiedName) -> TsQualifiedName<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsQualifiedName, &'a TsQualifiedName>(&node) };
  let parent = Node::TsQualifiedName(child_parent_ref);
  set_parent_for_ts_entity_name(&mut node.left, parent.clone());
  node.right.parent = parent;
  node
}

pub struct ExportDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ExportDecl,
  pub decl: Decl<'a>,
}

impl<'a> Spanned for ExportDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportDecl<'a>> for Node<'a> {
  fn from(node: &ExportDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ExportDecl, &'a ExportDecl>(&node) };
    Node::ExportDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ExportDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.decl).into());
    children
  }

}

impl<'a> CastableNode<'a> for ExportDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_export_decl<'a>(ref_node: &'a swc_ecma_ast::ExportDecl) -> ExportDecl<'a> {
  let value = &ref_node.decl;
  let field_decl = get_view_for_decl(value);
  let mut node = ExportDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    decl: field_decl,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportDecl, &'a ExportDecl>(&node) };
  let parent = Node::ExportDecl(child_parent_ref);
  set_parent_for_decl(&mut node.decl, parent);
  node
}

pub struct CatchClause<'a> {
  pub parent: &'a TryStmt<'a>,
  pub inner: &'a swc_ecma_ast::CatchClause,
  /// es2019
  /// 
  /// The param is null if the catch binding is omitted. E.g., try { foo() }
  /// catch { bar() }
  pub param: Option<Pat<'a>>,
  pub body: BlockStmt<'a>,
}

impl<'a> Spanned for CatchClause<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&CatchClause<'a>> for Node<'a> {
  fn from(node: &CatchClause) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&CatchClause, &'a CatchClause>(&node) };
    Node::CatchClause(static_ref)
  }
}

impl<'a> NodeTrait<'a> for CatchClause<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.param { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.param {
      children.push(child.into());
    }
    children.push((&self.body).into());
    children
  }

}

impl<'a> CastableNode<'a> for CatchClause<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::CatchClause(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_catch_clause<'a>(ref_node: &'a swc_ecma_ast::CatchClause) -> CatchClause<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&CatchClause, &'a CatchClause>(&node) };
  let parent = Node::CatchClause(child_parent_ref);
  if let Some(node) = node.param.as_mut() {
    set_parent_for_pat(node, parent.clone());
  }
  node.body.parent = parent;
  node
}

pub struct LabeledStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::LabeledStmt,
  pub label: Ident<'a>,
  pub body: Box<Stmt<'a>>,
}

impl<'a> Spanned for LabeledStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&LabeledStmt<'a>> for Node<'a> {
  fn from(node: &LabeledStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&LabeledStmt, &'a LabeledStmt>(&node) };
    Node::LabeledStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for LabeledStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.label).into());
    children.push((&self.body).into());
    children
  }

}

impl<'a> CastableNode<'a> for LabeledStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::LabeledStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_labeled_stmt<'a>(ref_node: &'a swc_ecma_ast::LabeledStmt) -> LabeledStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&LabeledStmt, &'a LabeledStmt>(&node) };
  let parent = Node::LabeledStmt(child_parent_ref);
  node.label.parent = parent.clone();
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct ContinueStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ContinueStmt,
  pub label: Option<Ident<'a>>,
}

impl<'a> Spanned for ContinueStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ContinueStmt<'a>> for Node<'a> {
  fn from(node: &ContinueStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ContinueStmt, &'a ContinueStmt>(&node) };
    Node::ContinueStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ContinueStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.label {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for ContinueStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ContinueStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_continue_stmt<'a>(ref_node: &'a swc_ecma_ast::ContinueStmt) -> ContinueStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ContinueStmt, &'a ContinueStmt>(&node) };
  let parent = Node::ContinueStmt(child_parent_ref);
  if let Some(node) = node.label.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsConstructSignatureDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsConstructSignatureDecl,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
}

impl<'a> Spanned for TsConstructSignatureDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsConstructSignatureDecl<'a>> for Node<'a> {
  fn from(node: &TsConstructSignatureDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsConstructSignatureDecl, &'a TsConstructSignatureDecl>(&node) };
    Node::TsConstructSignatureDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsConstructSignatureDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsConstructSignatureDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsConstructSignatureDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_construct_signature_decl<'a>(ref_node: &'a swc_ecma_ast::TsConstructSignatureDecl) -> TsConstructSignatureDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsConstructSignatureDecl, &'a TsConstructSignatureDecl>(&node) };
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

pub struct TsEnumDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsEnumDecl,
  pub id: Ident<'a>,
  pub members: Vec<TsEnumMember<'a>>,
}

impl<'a> TsEnumDecl<'a> {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  pub fn is_const(&self) -> bool {
    self.inner.is_const
  }
}

impl<'a> Spanned for TsEnumDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsEnumDecl<'a>> for Node<'a> {
  fn from(node: &TsEnumDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsEnumDecl, &'a TsEnumDecl>(&node) };
    Node::TsEnumDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsEnumDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.members.len());
    children.push((&self.id).into());
    for child in self.members.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsEnumDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsEnumDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_enum_decl<'a>(ref_node: &'a swc_ecma_ast::TsEnumDecl) -> TsEnumDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsEnumDecl, &'a TsEnumDecl>(&node) };
  let parent = Node::TsEnumDecl(child_parent_ref);
  node.id.parent = parent.clone();
  for node in node.members.iter_mut() {
    node.parent = parent.to::<TsEnumDecl>();
  }
  node
}

pub struct OptChainExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::OptChainExpr,
  pub expr: Box<Expr<'a>>,
}

impl<'a> OptChainExpr<'a> {
  pub fn question_dot_token(&self) -> &swc_common::Span {
    &self.inner.question_dot_token
  }
}

impl<'a> Spanned for OptChainExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&OptChainExpr<'a>> for Node<'a> {
  fn from(node: &OptChainExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&OptChainExpr, &'a OptChainExpr>(&node) };
    Node::OptChainExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for OptChainExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for OptChainExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::OptChainExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_opt_chain_expr<'a>(ref_node: &'a swc_ecma_ast::OptChainExpr) -> OptChainExpr<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = OptChainExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&OptChainExpr, &'a OptChainExpr>(&node) };
  let parent = Node::OptChainExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsNamespaceDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsNamespaceDecl,
  pub id: Ident<'a>,
  pub body: Box<TsNamespaceBody<'a>>,
}

impl<'a> TsNamespaceDecl<'a> {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  /// In TypeScript, this is only available through`node.flags`.
  pub fn global(&self) -> bool {
    self.inner.global
  }
}

impl<'a> Spanned for TsNamespaceDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsNamespaceDecl<'a>> for Node<'a> {
  fn from(node: &TsNamespaceDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsNamespaceDecl, &'a TsNamespaceDecl>(&node) };
    Node::TsNamespaceDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsNamespaceDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.id).into());
    children.push((&self.body).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsNamespaceDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsNamespaceDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_namespace_decl<'a>(ref_node: &'a swc_ecma_ast::TsNamespaceDecl) -> TsNamespaceDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsNamespaceDecl, &'a TsNamespaceDecl>(&node) };
  let parent = Node::TsNamespaceDecl(child_parent_ref);
  node.id.parent = parent.clone();
  set_parent_for_ts_namespace_body(&mut node.body, parent);
  node
}

pub struct SeqExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::SeqExpr,
  pub exprs: Vec<Box<Expr<'a>>>,
}

impl<'a> Spanned for SeqExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&SeqExpr<'a>> for Node<'a> {
  fn from(node: &SeqExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&SeqExpr, &'a SeqExpr>(&node) };
    Node::SeqExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for SeqExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.exprs.len());
    for child in self.exprs.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for SeqExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SeqExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_seq_expr<'a>(ref_node: &'a swc_ecma_ast::SeqExpr) -> SeqExpr<'a> {
  let value = &ref_node.exprs;
  let field_exprs = value.iter().map(|value| Box::new(get_view_for_expr(value))).collect();
  let mut node = SeqExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    exprs: field_exprs,
  };
  let child_parent_ref = unsafe { mem::transmute::<&SeqExpr, &'a SeqExpr>(&node) };
  let parent = Node::SeqExpr(child_parent_ref);
  for node in node.exprs.iter_mut() {
    set_parent_for_expr(node, parent.clone());
  }
  node
}

pub struct TsExternalModuleRef<'a> {
  pub parent: &'a TsImportEqualsDecl<'a>,
  pub inner: &'a swc_ecma_ast::TsExternalModuleRef,
  pub expr: Str<'a>,
}

impl<'a> Spanned for TsExternalModuleRef<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsExternalModuleRef<'a>> for Node<'a> {
  fn from(node: &TsExternalModuleRef) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsExternalModuleRef, &'a TsExternalModuleRef>(&node) };
    Node::TsExternalModuleRef(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsExternalModuleRef<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsExternalModuleRef<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsExternalModuleRef(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_external_module_ref<'a>(ref_node: &'a swc_ecma_ast::TsExternalModuleRef) -> TsExternalModuleRef<'a> {
  let value = &ref_node.expr;
  let field_expr = get_view_for_str(value);
  let mut node = TsExternalModuleRef {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsExternalModuleRef, &'a TsExternalModuleRef>(&node) };
  let parent = Node::TsExternalModuleRef(child_parent_ref);
  node.expr.parent = parent;
  node
}

pub struct TsTypeParamInstantiation<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeParamInstantiation,
  pub params: Vec<Box<TsType<'a>>>,
}

impl<'a> Spanned for TsTypeParamInstantiation<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeParamInstantiation<'a>> for Node<'a> {
  fn from(node: &TsTypeParamInstantiation) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeParamInstantiation, &'a TsTypeParamInstantiation>(&node) };
    Node::TsTypeParamInstantiation(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeParamInstantiation<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.params.len());
    for child in self.params.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsTypeParamInstantiation<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeParamInstantiation(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_param_instantiation<'a>(ref_node: &'a swc_ecma_ast::TsTypeParamInstantiation) -> TsTypeParamInstantiation<'a> {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| Box::new(get_view_for_ts_type(value))).collect();
  let mut node = TsTypeParamInstantiation {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeParamInstantiation, &'a TsTypeParamInstantiation>(&node) };
  let parent = Node::TsTypeParamInstantiation(child_parent_ref);
  for node in node.params.iter_mut() {
    set_parent_for_ts_type(node, parent.clone());
  }
  node
}

pub struct ReturnStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ReturnStmt,
  pub arg: Option<Box<Expr<'a>>>,
}

impl<'a> Spanned for ReturnStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ReturnStmt<'a>> for Node<'a> {
  fn from(node: &ReturnStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ReturnStmt, &'a ReturnStmt>(&node) };
    Node::ReturnStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ReturnStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.arg { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.arg {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for ReturnStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ReturnStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_return_stmt<'a>(ref_node: &'a swc_ecma_ast::ReturnStmt) -> ReturnStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ReturnStmt, &'a ReturnStmt>(&node) };
  let parent = Node::ReturnStmt(child_parent_ref);
  if let Some(node) = node.arg.as_mut() {
    set_parent_for_expr(node, parent);
  }
  node
}

pub struct TsTplLitType<'a> {
  pub parent: &'a TsLitType<'a>,
  pub inner: &'a swc_ecma_ast::TsTplLitType,
  pub types: Vec<Box<TsType<'a>>>,
  pub quasis: Vec<TplElement<'a>>,
}

impl<'a> Spanned for TsTplLitType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTplLitType<'a>> for Node<'a> {
  fn from(node: &TsTplLitType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTplLitType, &'a TsTplLitType>(&node) };
    Node::TsTplLitType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTplLitType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsTplLitType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTplLitType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_tpl_lit_type<'a>(ref_node: &'a swc_ecma_ast::TsTplLitType) -> TsTplLitType<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsTplLitType, &'a TsTplLitType>(&node) };
  let parent = Node::TsTplLitType(child_parent_ref);
  for node in node.types.iter_mut() {
    set_parent_for_ts_type(node, parent.clone());
  }
  for node in node.quasis.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

pub struct ExportDefaultExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ExportDefaultExpr,
  pub expr: Box<Expr<'a>>,
}

impl<'a> Spanned for ExportDefaultExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportDefaultExpr<'a>> for Node<'a> {
  fn from(node: &ExportDefaultExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ExportDefaultExpr, &'a ExportDefaultExpr>(&node) };
    Node::ExportDefaultExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ExportDefaultExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for ExportDefaultExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportDefaultExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_export_default_expr<'a>(ref_node: &'a swc_ecma_ast::ExportDefaultExpr) -> ExportDefaultExpr<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = ExportDefaultExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportDefaultExpr, &'a ExportDefaultExpr>(&node) };
  let parent = Node::ExportDefaultExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsCallSignatureDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsCallSignatureDecl,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
}

impl<'a> Spanned for TsCallSignatureDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsCallSignatureDecl<'a>> for Node<'a> {
  fn from(node: &TsCallSignatureDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsCallSignatureDecl, &'a TsCallSignatureDecl>(&node) };
    Node::TsCallSignatureDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsCallSignatureDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsCallSignatureDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsCallSignatureDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_call_signature_decl<'a>(ref_node: &'a swc_ecma_ast::TsCallSignatureDecl) -> TsCallSignatureDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsCallSignatureDecl, &'a TsCallSignatureDecl>(&node) };
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

pub struct AwaitExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::AwaitExpr,
  pub arg: Box<Expr<'a>>,
}

impl<'a> Spanned for AwaitExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&AwaitExpr<'a>> for Node<'a> {
  fn from(node: &AwaitExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&AwaitExpr, &'a AwaitExpr>(&node) };
    Node::AwaitExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for AwaitExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

}

impl<'a> CastableNode<'a> for AwaitExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::AwaitExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_await_expr<'a>(ref_node: &'a swc_ecma_ast::AwaitExpr) -> AwaitExpr<'a> {
  let value = &ref_node.arg;
  let field_arg = Box::new(get_view_for_expr(value));
  let mut node = AwaitExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
  };
  let child_parent_ref = unsafe { mem::transmute::<&AwaitExpr, &'a AwaitExpr>(&node) };
  let parent = Node::AwaitExpr(child_parent_ref);
  set_parent_for_expr(&mut node.arg, parent);
  node
}

pub struct ClassMethod<'a> {
  pub parent: &'a Class<'a>,
  pub inner: &'a swc_ecma_ast::ClassMethod,
  pub key: PropName<'a>,
  pub function: Function<'a>,
}

impl<'a> ClassMethod<'a> {
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

impl<'a> Spanned for ClassMethod<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ClassMethod<'a>> for Node<'a> {
  fn from(node: &ClassMethod) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ClassMethod, &'a ClassMethod>(&node) };
    Node::ClassMethod(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ClassMethod<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.function).into());
    children
  }

}

impl<'a> CastableNode<'a> for ClassMethod<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ClassMethod(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_class_method<'a>(ref_node: &'a swc_ecma_ast::ClassMethod) -> ClassMethod<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ClassMethod, &'a ClassMethod>(&node) };
  let parent = Node::ClassMethod(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  node.function.parent = parent;
  node
}

pub struct TsParamProp<'a> {
  pub parent: &'a Constructor<'a>,
  pub inner: &'a swc_ecma_ast::TsParamProp,
  pub decorators: Vec<Decorator<'a>>,
  pub param: TsParamPropParam<'a>,
}

impl<'a> TsParamProp<'a> {
  /// At least one of `accessibility` or `readonly` must be set.
  pub fn accessibility(&self) -> &Option<Accessibility> {
    &self.inner.accessibility
  }

  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }
}

impl<'a> Spanned for TsParamProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsParamProp<'a>> for Node<'a> {
  fn from(node: &TsParamProp) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsParamProp, &'a TsParamProp>(&node) };
    Node::TsParamProp(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsParamProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.decorators.len());
    for child in self.decorators.iter() {
      children.push(child.into());
    }
    children.push((&self.param).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsParamProp<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsParamProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_param_prop<'a>(ref_node: &'a swc_ecma_ast::TsParamProp) -> TsParamProp<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsParamProp, &'a TsParamProp>(&node) };
  let parent = Node::TsParamProp(child_parent_ref);
  for node in node.decorators.iter_mut() {
    node.parent = parent.clone();
  }
  set_parent_for_ts_param_prop_param(&mut node.param, parent);
  node
}

pub struct ClassProp<'a> {
  pub parent: &'a Class<'a>,
  pub inner: &'a swc_ecma_ast::ClassProp,
  pub key: Box<Expr<'a>>,
  pub value: Option<Box<Expr<'a>>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
  pub decorators: Vec<Decorator<'a>>,
}

impl<'a> ClassProp<'a> {
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

impl<'a> Spanned for ClassProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ClassProp<'a>> for Node<'a> {
  fn from(node: &ClassProp) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ClassProp, &'a ClassProp>(&node) };
    Node::ClassProp(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ClassProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for ClassProp<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ClassProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_class_prop<'a>(ref_node: &'a swc_ecma_ast::ClassProp) -> ClassProp<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ClassProp, &'a ClassProp>(&node) };
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

pub struct TsTypeAnn<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeAnn,
  pub type_ann: Box<TsType<'a>>,
}

impl<'a> Spanned for TsTypeAnn<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeAnn<'a>> for Node<'a> {
  fn from(node: &TsTypeAnn) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeAnn, &'a TsTypeAnn>(&node) };
    Node::TsTypeAnn(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeAnn<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsTypeAnn<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeAnn(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_ann<'a>(ref_node: &'a swc_ecma_ast::TsTypeAnn) -> TsTypeAnn<'a> {
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsTypeAnn {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeAnn, &'a TsTypeAnn>(&node) };
  let parent = Node::TsTypeAnn(child_parent_ref);
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct ForStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ForStmt,
  pub init: Option<VarDeclOrExpr<'a>>,
  pub test: Option<Box<Expr<'a>>>,
  pub update: Option<Box<Expr<'a>>>,
  pub body: Box<Stmt<'a>>,
}

impl<'a> Spanned for ForStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ForStmt<'a>> for Node<'a> {
  fn from(node: &ForStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ForStmt, &'a ForStmt>(&node) };
    Node::ForStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ForStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for ForStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ForStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_for_stmt<'a>(ref_node: &'a swc_ecma_ast::ForStmt) -> ForStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ForStmt, &'a ForStmt>(&node) };
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

pub struct ObjectPat<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ObjectPat,
  pub props: Vec<ObjectPatProp<'a>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
}

impl<'a> ObjectPat<'a> {
  /// Only in an ambient context
  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl<'a> Spanned for ObjectPat<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ObjectPat<'a>> for Node<'a> {
  fn from(node: &ObjectPat) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ObjectPat, &'a ObjectPat>(&node) };
    Node::ObjectPat(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ObjectPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for ObjectPat<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ObjectPat(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_object_pat<'a>(ref_node: &'a swc_ecma_ast::ObjectPat) -> ObjectPat<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ObjectPat, &'a ObjectPat>(&node) };
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
pub struct TsTypeQuery<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeQuery,
  pub expr_name: TsTypeQueryExpr<'a>,
}

impl<'a> Spanned for TsTypeQuery<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeQuery<'a>> for Node<'a> {
  fn from(node: &TsTypeQuery) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeQuery, &'a TsTypeQuery>(&node) };
    Node::TsTypeQuery(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeQuery<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr_name).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsTypeQuery<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeQuery(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_query<'a>(ref_node: &'a swc_ecma_ast::TsTypeQuery) -> TsTypeQuery<'a> {
  let value = &ref_node.expr_name;
  let field_expr_name = get_view_for_ts_type_query_expr(value);
  let mut node = TsTypeQuery {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr_name: field_expr_name,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeQuery, &'a TsTypeQuery>(&node) };
  let parent = Node::TsTypeQuery(child_parent_ref);
  set_parent_for_ts_type_query_expr(&mut node.expr_name, parent);
  node
}

pub struct ThisExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ThisExpr,
}

impl<'a> Spanned for ThisExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ThisExpr<'a>> for Node<'a> {
  fn from(node: &ThisExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ThisExpr, &'a ThisExpr>(&node) };
    Node::ThisExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ThisExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for ThisExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ThisExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_this_expr<'a>(ref_node: &'a swc_ecma_ast::ThisExpr) -> ThisExpr<'a> {
  let node = ThisExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct DebuggerStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::DebuggerStmt,
}

impl<'a> Spanned for DebuggerStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&DebuggerStmt<'a>> for Node<'a> {
  fn from(node: &DebuggerStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&DebuggerStmt, &'a DebuggerStmt>(&node) };
    Node::DebuggerStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for DebuggerStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for DebuggerStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::DebuggerStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_debugger_stmt<'a>(ref_node: &'a swc_ecma_ast::DebuggerStmt) -> DebuggerStmt<'a> {
  let node = DebuggerStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsTypeParamDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeParamDecl,
  pub params: Vec<TsTypeParam<'a>>,
}

impl<'a> Spanned for TsTypeParamDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeParamDecl<'a>> for Node<'a> {
  fn from(node: &TsTypeParamDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeParamDecl, &'a TsTypeParamDecl>(&node) };
    Node::TsTypeParamDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeParamDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.params.len());
    for child in self.params.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsTypeParamDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeParamDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_param_decl<'a>(ref_node: &'a swc_ecma_ast::TsTypeParamDecl) -> TsTypeParamDecl<'a> {
  let value = &ref_node.params;
  let field_params = value.iter().map(|value| get_view_for_ts_type_param(value)).collect();
  let mut node = TsTypeParamDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    params: field_params,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeParamDecl, &'a TsTypeParamDecl>(&node) };
  let parent = Node::TsTypeParamDecl(child_parent_ref);
  for node in node.params.iter_mut() {
    node.parent = parent.clone();
  }
  node
}

pub struct TsTypeAssertion<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeAssertion,
  pub expr: Box<Expr<'a>>,
  pub type_ann: Box<TsType<'a>>,
}

impl<'a> Spanned for TsTypeAssertion<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeAssertion<'a>> for Node<'a> {
  fn from(node: &TsTypeAssertion) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeAssertion, &'a TsTypeAssertion>(&node) };
    Node::TsTypeAssertion(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeAssertion<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push((&self.type_ann).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsTypeAssertion<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeAssertion(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_assertion<'a>(ref_node: &'a swc_ecma_ast::TsTypeAssertion) -> TsTypeAssertion<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeAssertion, &'a TsTypeAssertion>(&node) };
  let parent = Node::TsTypeAssertion(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent.clone());
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct TplElement<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TplElement,
  pub cooked: Option<Str<'a>>,
  pub raw: Str<'a>,
}

impl<'a> TplElement<'a> {
  pub fn tail(&self) -> bool {
    self.inner.tail
  }
}

impl<'a> Spanned for TplElement<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TplElement<'a>> for Node<'a> {
  fn from(node: &TplElement) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TplElement, &'a TplElement>(&node) };
    Node::TplElement(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TplElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.cooked { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.cooked {
      children.push(child.into());
    }
    children.push((&self.raw).into());
    children
  }

}

impl<'a> CastableNode<'a> for TplElement<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TplElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_tpl_element<'a>(ref_node: &'a swc_ecma_ast::TplElement) -> TplElement<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TplElement, &'a TplElement>(&node) };
  let parent = Node::TplElement(child_parent_ref);
  if let Some(node) = node.cooked.as_mut() {
    node.parent = parent.clone();
  }
  node.raw.parent = parent;
  node
}

pub struct TsKeywordType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsKeywordType,
}

impl<'a> TsKeywordType<'a> {
  pub fn kind(&self) -> &TsKeywordTypeKind {
    &self.inner.kind
  }
}

impl<'a> Spanned for TsKeywordType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsKeywordType<'a>> for Node<'a> {
  fn from(node: &TsKeywordType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsKeywordType, &'a TsKeywordType>(&node) };
    Node::TsKeywordType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsKeywordType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for TsKeywordType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsKeywordType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_keyword_type<'a>(ref_node: &'a swc_ecma_ast::TsKeywordType) -> TsKeywordType<'a> {
  let node = TsKeywordType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct JSXSpreadChild<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::JSXSpreadChild,
  pub expr: Box<Expr<'a>>,
}

impl<'a> Spanned for JSXSpreadChild<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXSpreadChild<'a>> for Node<'a> {
  fn from(node: &JSXSpreadChild) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXSpreadChild, &'a JSXSpreadChild>(&node) };
    Node::JSXSpreadChild(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXSpreadChild<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for JSXSpreadChild<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXSpreadChild(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxspread_child<'a>(ref_node: &'a swc_ecma_ast::JSXSpreadChild) -> JSXSpreadChild<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = JSXSpreadChild {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXSpreadChild, &'a JSXSpreadChild>(&node) };
  let parent = Node::JSXSpreadChild(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsIntersectionType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsIntersectionType,
  pub types: Vec<Box<TsType<'a>>>,
}

impl<'a> Spanned for TsIntersectionType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsIntersectionType<'a>> for Node<'a> {
  fn from(node: &TsIntersectionType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsIntersectionType, &'a TsIntersectionType>(&node) };
    Node::TsIntersectionType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsIntersectionType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.types.len());
    for child in self.types.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsIntersectionType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsIntersectionType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_intersection_type<'a>(ref_node: &'a swc_ecma_ast::TsIntersectionType) -> TsIntersectionType<'a> {
  let value = &ref_node.types;
  let field_types = value.iter().map(|value| Box::new(get_view_for_ts_type(value))).collect();
  let mut node = TsIntersectionType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    types: field_types,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsIntersectionType, &'a TsIntersectionType>(&node) };
  let parent = Node::TsIntersectionType(child_parent_ref);
  for node in node.types.iter_mut() {
    set_parent_for_ts_type(node, parent.clone());
  }
  node
}

pub struct MetaPropExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::MetaPropExpr,
  pub meta: Ident<'a>,
  pub prop: Ident<'a>,
}

impl<'a> Spanned for MetaPropExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&MetaPropExpr<'a>> for Node<'a> {
  fn from(node: &MetaPropExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&MetaPropExpr, &'a MetaPropExpr>(&node) };
    Node::MetaPropExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for MetaPropExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.meta).into());
    children.push((&self.prop).into());
    children
  }

}

impl<'a> CastableNode<'a> for MetaPropExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::MetaPropExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_meta_prop_expr<'a>(ref_node: &'a swc_ecma_ast::MetaPropExpr) -> MetaPropExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&MetaPropExpr, &'a MetaPropExpr>(&node) };
  let parent = Node::MetaPropExpr(child_parent_ref);
  node.meta.parent = parent.clone();
  node.prop.parent = parent;
  node
}

pub struct ExprOrSpread<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ExprOrSpread,
  pub expr: Box<Expr<'a>>,
}

impl<'a> ExprOrSpread<'a> {
  pub fn spread(&self) -> &Option<swc_common::Span> {
    &self.inner.spread
  }
}

impl<'a> Spanned for ExprOrSpread<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExprOrSpread<'a>> for Node<'a> {
  fn from(node: &ExprOrSpread) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ExprOrSpread, &'a ExprOrSpread>(&node) };
    Node::ExprOrSpread(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ExprOrSpread<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for ExprOrSpread<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExprOrSpread(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_expr_or_spread<'a>(ref_node: &'a swc_ecma_ast::ExprOrSpread) -> ExprOrSpread<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = ExprOrSpread {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExprOrSpread, &'a ExprOrSpread>(&node) };
  let parent = Node::ExprOrSpread(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsArrayType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsArrayType,
  pub elem_type: Box<TsType<'a>>,
}

impl<'a> Spanned for TsArrayType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsArrayType<'a>> for Node<'a> {
  fn from(node: &TsArrayType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsArrayType, &'a TsArrayType>(&node) };
    Node::TsArrayType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsArrayType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.elem_type).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsArrayType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsArrayType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_array_type<'a>(ref_node: &'a swc_ecma_ast::TsArrayType) -> TsArrayType<'a> {
  let value = &ref_node.elem_type;
  let field_elem_type = Box::new(get_view_for_ts_type(value));
  let mut node = TsArrayType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    elem_type: field_elem_type,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsArrayType, &'a TsArrayType>(&node) };
  let parent = Node::TsArrayType(child_parent_ref);
  set_parent_for_ts_type(&mut node.elem_type, parent);
  node
}

pub struct TsTypeRef<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeRef,
  pub type_name: TsEntityName<'a>,
  pub type_params: Option<TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for TsTypeRef<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeRef<'a>> for Node<'a> {
  fn from(node: &TsTypeRef) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeRef, &'a TsTypeRef>(&node) };
    Node::TsTypeRef(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeRef<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.type_name).into());
    if let Some(child) = &self.type_params {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsTypeRef<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeRef(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_ref<'a>(ref_node: &'a swc_ecma_ast::TsTypeRef) -> TsTypeRef<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeRef, &'a TsTypeRef>(&node) };
  let parent = Node::TsTypeRef(child_parent_ref);
  set_parent_for_ts_entity_name(&mut node.type_name, parent.clone());
  if let Some(node) = node.type_params.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsThisType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsThisType,
}

impl<'a> Spanned for TsThisType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsThisType<'a>> for Node<'a> {
  fn from(node: &TsThisType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsThisType, &'a TsThisType>(&node) };
    Node::TsThisType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsThisType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for TsThisType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsThisType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_this_type<'a>(ref_node: &'a swc_ecma_ast::TsThisType) -> TsThisType<'a> {
  let node = TsThisType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TryStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TryStmt,
  pub block: BlockStmt<'a>,
  pub handler: Option<CatchClause<'a>>,
  pub finalizer: Option<BlockStmt<'a>>,
}

impl<'a> Spanned for TryStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TryStmt<'a>> for Node<'a> {
  fn from(node: &TryStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TryStmt, &'a TryStmt>(&node) };
    Node::TryStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TryStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TryStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TryStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_try_stmt<'a>(ref_node: &'a swc_ecma_ast::TryStmt) -> TryStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TryStmt, &'a TryStmt>(&node) };
  let parent = Node::TryStmt(child_parent_ref);
  node.block.parent = parent.clone();
  if let Some(node) = node.handler.as_mut() {
    node.parent = parent.to::<TryStmt>();
  }
  if let Some(node) = node.finalizer.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct CallExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::CallExpr,
  pub callee: ExprOrSuper<'a>,
  pub args: Vec<ExprOrSpread<'a>>,
  pub type_args: Option<TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for CallExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&CallExpr<'a>> for Node<'a> {
  fn from(node: &CallExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&CallExpr, &'a CallExpr>(&node) };
    Node::CallExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for CallExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for CallExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::CallExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_call_expr<'a>(ref_node: &'a swc_ecma_ast::CallExpr) -> CallExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&CallExpr, &'a CallExpr>(&node) };
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

pub struct TsMappedType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsMappedType,
  pub type_param: TsTypeParam<'a>,
  pub name_type: Option<Box<TsType<'a>>>,
  pub type_ann: Option<Box<TsType<'a>>>,
}

impl<'a> TsMappedType<'a> {
  pub fn readonly(&self) -> &Option<TruePlusMinus> {
    &self.inner.readonly
  }

  pub fn optional(&self) -> &Option<TruePlusMinus> {
    &self.inner.optional
  }
}

impl<'a> Spanned for TsMappedType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsMappedType<'a>> for Node<'a> {
  fn from(node: &TsMappedType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsMappedType, &'a TsMappedType>(&node) };
    Node::TsMappedType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsMappedType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsMappedType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsMappedType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_mapped_type<'a>(ref_node: &'a swc_ecma_ast::TsMappedType) -> TsMappedType<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsMappedType, &'a TsMappedType>(&node) };
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

pub struct JSXExprContainer<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::JSXExprContainer,
  pub expr: JSXExpr<'a>,
}

impl<'a> Spanned for JSXExprContainer<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXExprContainer<'a>> for Node<'a> {
  fn from(node: &JSXExprContainer) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXExprContainer, &'a JSXExprContainer>(&node) };
    Node::JSXExprContainer(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXExprContainer<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for JSXExprContainer<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXExprContainer(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxexpr_container<'a>(ref_node: &'a swc_ecma_ast::JSXExprContainer) -> JSXExprContainer<'a> {
  let value = &ref_node.expr;
  let field_expr = get_view_for_jsxexpr(value);
  let mut node = JSXExprContainer {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXExprContainer, &'a JSXExprContainer>(&node) };
  let parent = Node::JSXExprContainer(child_parent_ref);
  set_parent_for_jsxexpr(&mut node.expr, parent);
  node
}

pub struct PrivateProp<'a> {
  pub parent: &'a Class<'a>,
  pub inner: &'a swc_ecma_ast::PrivateProp,
  pub key: PrivateName<'a>,
  pub value: Option<Box<Expr<'a>>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
  pub decorators: Vec<Decorator<'a>>,
}

impl<'a> PrivateProp<'a> {
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

impl<'a> Spanned for PrivateProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&PrivateProp<'a>> for Node<'a> {
  fn from(node: &PrivateProp) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&PrivateProp, &'a PrivateProp>(&node) };
    Node::PrivateProp(static_ref)
  }
}

impl<'a> NodeTrait<'a> for PrivateProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for PrivateProp<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::PrivateProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_private_prop<'a>(ref_node: &'a swc_ecma_ast::PrivateProp) -> PrivateProp<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&PrivateProp, &'a PrivateProp>(&node) };
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
pub struct TsExportAssignment<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsExportAssignment,
  pub expr: Box<Expr<'a>>,
}

impl<'a> Spanned for TsExportAssignment<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsExportAssignment<'a>> for Node<'a> {
  fn from(node: &TsExportAssignment) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsExportAssignment, &'a TsExportAssignment>(&node) };
    Node::TsExportAssignment(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsExportAssignment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsExportAssignment<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsExportAssignment(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_export_assignment<'a>(ref_node: &'a swc_ecma_ast::TsExportAssignment) -> TsExportAssignment<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = TsExportAssignment {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsExportAssignment, &'a TsExportAssignment>(&node) };
  let parent = Node::TsExportAssignment(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct TsInterfaceBody<'a> {
  pub parent: &'a TsInterfaceDecl<'a>,
  pub inner: &'a swc_ecma_ast::TsInterfaceBody,
  pub body: Vec<TsTypeElement<'a>>,
}

impl<'a> Spanned for TsInterfaceBody<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsInterfaceBody<'a>> for Node<'a> {
  fn from(node: &TsInterfaceBody) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsInterfaceBody, &'a TsInterfaceBody>(&node) };
    Node::TsInterfaceBody(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsInterfaceBody<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.body.len());
    for child in self.body.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsInterfaceBody<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsInterfaceBody(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_interface_body<'a>(ref_node: &'a swc_ecma_ast::TsInterfaceBody) -> TsInterfaceBody<'a> {
  let value = &ref_node.body;
  let field_body = value.iter().map(|value| get_view_for_ts_type_element(value)).collect();
  let mut node = TsInterfaceBody {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsInterfaceBody, &'a TsInterfaceBody>(&node) };
  let parent = Node::TsInterfaceBody(child_parent_ref);
  for node in node.body.iter_mut() {
    set_parent_for_ts_type_element(node, parent.clone());
  }
  node
}

pub struct TsTupleElement<'a> {
  pub parent: &'a TsTupleType<'a>,
  pub inner: &'a swc_ecma_ast::TsTupleElement,
  /// `Ident` or `RestPat { arg: Ident }`
  pub label: Option<Pat<'a>>,
  pub ty: TsType<'a>,
}

impl<'a> Spanned for TsTupleElement<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTupleElement<'a>> for Node<'a> {
  fn from(node: &TsTupleElement) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTupleElement, &'a TsTupleElement>(&node) };
    Node::TsTupleElement(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTupleElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.label {
      children.push(child.into());
    }
    children.push((&self.ty).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsTupleElement<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTupleElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_tuple_element<'a>(ref_node: &'a swc_ecma_ast::TsTupleElement) -> TsTupleElement<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsTupleElement, &'a TsTupleElement>(&node) };
  let parent = Node::TsTupleElement(child_parent_ref);
  if let Some(node) = node.label.as_mut() {
    set_parent_for_pat(node, parent.clone());
  }
  set_parent_for_ts_type(&mut node.ty, parent);
  node
}

pub struct VarDeclarator<'a> {
  pub parent: &'a VarDecl<'a>,
  pub inner: &'a swc_ecma_ast::VarDeclarator,
  pub name: Pat<'a>,
  /// Initialization expresion.
  pub init: Option<Box<Expr<'a>>>,
}

impl<'a> VarDeclarator<'a> {
  /// Typescript only
  pub fn definite(&self) -> bool {
    self.inner.definite
  }
}

impl<'a> Spanned for VarDeclarator<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&VarDeclarator<'a>> for Node<'a> {
  fn from(node: &VarDeclarator) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&VarDeclarator, &'a VarDeclarator>(&node) };
    Node::VarDeclarator(static_ref)
  }
}

impl<'a> NodeTrait<'a> for VarDeclarator<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    if let Some(child) = &self.init {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for VarDeclarator<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::VarDeclarator(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_var_declarator<'a>(ref_node: &'a swc_ecma_ast::VarDeclarator) -> VarDeclarator<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&VarDeclarator, &'a VarDeclarator>(&node) };
  let parent = Node::VarDeclarator(child_parent_ref);
  set_parent_for_pat(&mut node.name, parent.clone());
  if let Some(node) = node.init.as_mut() {
    set_parent_for_expr(node, parent);
  }
  node
}

pub struct JSXMemberExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::JSXMemberExpr,
  pub obj: JSXObject<'a>,
  pub prop: Ident<'a>,
}

impl<'a> Spanned for JSXMemberExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXMemberExpr<'a>> for Node<'a> {
  fn from(node: &JSXMemberExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXMemberExpr, &'a JSXMemberExpr>(&node) };
    Node::JSXMemberExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXMemberExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push((&self.prop).into());
    children
  }

}

impl<'a> CastableNode<'a> for JSXMemberExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXMemberExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxmember_expr<'a>(ref_node: &'a swc_ecma_ast::JSXMemberExpr) -> JSXMemberExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&JSXMemberExpr, &'a JSXMemberExpr>(&node) };
  let parent = Node::JSXMemberExpr(child_parent_ref);
  set_parent_for_jsxobject(&mut node.obj, parent.clone());
  node.prop.parent = parent;
  node
}

pub struct TsConstAssertion<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsConstAssertion,
  pub expr: Box<Expr<'a>>,
}

impl<'a> Spanned for TsConstAssertion<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsConstAssertion<'a>> for Node<'a> {
  fn from(node: &TsConstAssertion) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsConstAssertion, &'a TsConstAssertion>(&node) };
    Node::TsConstAssertion(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsConstAssertion<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsConstAssertion<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsConstAssertion(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_const_assertion<'a>(ref_node: &'a swc_ecma_ast::TsConstAssertion) -> TsConstAssertion<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = TsConstAssertion {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsConstAssertion, &'a TsConstAssertion>(&node) };
  let parent = Node::TsConstAssertion(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

/// `export * as foo from 'src';`
pub struct ExportNamespaceSpecifier<'a> {
  pub parent: &'a NamedExport<'a>,
  pub inner: &'a swc_ecma_ast::ExportNamespaceSpecifier,
  pub name: Ident<'a>,
}

impl<'a> Spanned for ExportNamespaceSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportNamespaceSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportNamespaceSpecifier) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ExportNamespaceSpecifier, &'a ExportNamespaceSpecifier>(&node) };
    Node::ExportNamespaceSpecifier(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ExportNamespaceSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.name).into());
    children
  }

}

impl<'a> CastableNode<'a> for ExportNamespaceSpecifier<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportNamespaceSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_export_namespace_specifier<'a>(ref_node: &'a swc_ecma_ast::ExportNamespaceSpecifier) -> ExportNamespaceSpecifier<'a> {
  let value = &ref_node.name;
  let field_name = get_view_for_ident(value);
  let mut node = ExportNamespaceSpecifier {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    name: field_name,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportNamespaceSpecifier, &'a ExportNamespaceSpecifier>(&node) };
  let parent = Node::ExportNamespaceSpecifier(child_parent_ref);
  node.name.parent = parent;
  node
}

/// Object literal.
pub struct ObjectLit<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ObjectLit,
  pub props: Vec<PropOrSpread<'a>>,
}

impl<'a> Spanned for ObjectLit<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ObjectLit<'a>> for Node<'a> {
  fn from(node: &ObjectLit) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ObjectLit, &'a ObjectLit>(&node) };
    Node::ObjectLit(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ObjectLit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.props.len());
    for child in self.props.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for ObjectLit<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ObjectLit(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_object_lit<'a>(ref_node: &'a swc_ecma_ast::ObjectLit) -> ObjectLit<'a> {
  let value = &ref_node.props;
  let field_props = value.iter().map(|value| get_view_for_prop_or_spread(value)).collect();
  let mut node = ObjectLit {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    props: field_props,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ObjectLit, &'a ObjectLit>(&node) };
  let parent = Node::ObjectLit(child_parent_ref);
  for node in node.props.iter_mut() {
    set_parent_for_prop_or_spread(node, parent.clone());
  }
  node
}

pub struct Module<'a> {
  pub inner: &'a swc_ecma_ast::Module,
  pub body: Vec<ModuleItem<'a>>,
}

impl<'a> Module<'a> {
  pub fn shebang(&self) -> &Option<swc_atoms::JsWord> {
    &self.inner.shebang
  }
}

impl<'a> Spanned for Module<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Module<'a>> for Node<'a> {
  fn from(node: &Module) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Module, &'a Module>(&node) };
    Node::Module(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Module<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    None
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.body.len());
    for child in self.body.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for Module<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Module(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_module<'a>(ref_node: &'a swc_ecma_ast::Module) -> Module<'a> {
  let value = &ref_node.body;
  let field_body = value.iter().map(|value| get_view_for_module_item(value)).collect();
  let mut node = Module {
    inner: ref_node,
    body: field_body,
  };
  let child_parent_ref = unsafe { mem::transmute::<&Module, &'a Module>(&node) };
  let parent = Node::Module(child_parent_ref);
  for node in node.body.iter_mut() {
    set_parent_for_module_item(node, parent.clone());
  }
  node
}

pub struct TsIndexSignature<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsIndexSignature,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
}

impl<'a> TsIndexSignature<'a> {
  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }
}

impl<'a> Spanned for TsIndexSignature<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsIndexSignature<'a>> for Node<'a> {
  fn from(node: &TsIndexSignature) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsIndexSignature, &'a TsIndexSignature>(&node) };
    Node::TsIndexSignature(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsIndexSignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsIndexSignature<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsIndexSignature(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_index_signature<'a>(ref_node: &'a swc_ecma_ast::TsIndexSignature) -> TsIndexSignature<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsIndexSignature, &'a TsIndexSignature>(&node) };
  let parent = Node::TsIndexSignature(child_parent_ref);
  for node in node.params.iter_mut() {
    set_parent_for_ts_fn_param(node, parent.clone());
  }
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsTypeCastExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeCastExpr,
  pub expr: Box<Expr<'a>>,
  pub type_ann: TsTypeAnn<'a>,
}

impl<'a> Spanned for TsTypeCastExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeCastExpr<'a>> for Node<'a> {
  fn from(node: &TsTypeCastExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeCastExpr, &'a TsTypeCastExpr>(&node) };
    Node::TsTypeCastExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeCastExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push((&self.type_ann).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsTypeCastExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeCastExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_cast_expr<'a>(ref_node: &'a swc_ecma_ast::TsTypeCastExpr) -> TsTypeCastExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeCastExpr, &'a TsTypeCastExpr>(&node) };
  let parent = Node::TsTypeCastExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent.clone());
  node.type_ann.parent = parent;
  node
}

pub struct TsTupleType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTupleType,
  pub elem_types: Vec<TsTupleElement<'a>>,
}

impl<'a> Spanned for TsTupleType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTupleType<'a>> for Node<'a> {
  fn from(node: &TsTupleType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTupleType, &'a TsTupleType>(&node) };
    Node::TsTupleType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTupleType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.elem_types.len());
    for child in self.elem_types.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsTupleType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTupleType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_tuple_type<'a>(ref_node: &'a swc_ecma_ast::TsTupleType) -> TsTupleType<'a> {
  let value = &ref_node.elem_types;
  let field_elem_types = value.iter().map(|value| get_view_for_ts_tuple_element(value)).collect();
  let mut node = TsTupleType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    elem_types: field_elem_types,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTupleType, &'a TsTupleType>(&node) };
  let parent = Node::TsTupleType(child_parent_ref);
  for node in node.elem_types.iter_mut() {
    node.parent = parent.to::<TsTupleType>();
  }
  node
}

pub struct Null<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Null,
}

impl<'a> Spanned for Null<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Null<'a>> for Node<'a> {
  fn from(node: &Null) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Null, &'a Null>(&node) };
    Node::Null(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Null<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for Null<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Null(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_null<'a>(ref_node: &'a swc_ecma_ast::Null) -> Null<'a> {
  let node = Null {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsTypeOperator<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeOperator,
  pub type_ann: Box<TsType<'a>>,
}

impl<'a> TsTypeOperator<'a> {
  pub fn op(&self) -> &TsTypeOperatorOp {
    &self.inner.op
  }
}

impl<'a> Spanned for TsTypeOperator<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeOperator<'a>> for Node<'a> {
  fn from(node: &TsTypeOperator) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeOperator, &'a TsTypeOperator>(&node) };
    Node::TsTypeOperator(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeOperator<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsTypeOperator<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeOperator(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_operator<'a>(ref_node: &'a swc_ecma_ast::TsTypeOperator) -> TsTypeOperator<'a> {
  let value = &ref_node.type_ann;
  let field_type_ann = Box::new(get_view_for_ts_type(value));
  let mut node = TsTypeOperator {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: field_type_ann,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeOperator, &'a TsTypeOperator>(&node) };
  let parent = Node::TsTypeOperator(child_parent_ref);
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

pub struct JSXClosingElement<'a> {
  pub parent: &'a JSXElement<'a>,
  pub inner: &'a swc_ecma_ast::JSXClosingElement,
  pub name: JSXElementName<'a>,
}

impl<'a> Spanned for JSXClosingElement<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXClosingElement<'a>> for Node<'a> {
  fn from(node: &JSXClosingElement) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXClosingElement, &'a JSXClosingElement>(&node) };
    Node::JSXClosingElement(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXClosingElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.name).into());
    children
  }

}

impl<'a> CastableNode<'a> for JSXClosingElement<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXClosingElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxclosing_element<'a>(ref_node: &'a swc_ecma_ast::JSXClosingElement) -> JSXClosingElement<'a> {
  let value = &ref_node.name;
  let field_name = get_view_for_jsxelement_name(value);
  let mut node = JSXClosingElement {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    name: field_name,
  };
  let child_parent_ref = unsafe { mem::transmute::<&JSXClosingElement, &'a JSXClosingElement>(&node) };
  let parent = Node::JSXClosingElement(child_parent_ref);
  set_parent_for_jsxelement_name(&mut node.name, parent);
  node
}

pub struct BinExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::BinExpr,
  pub left: Box<Expr<'a>>,
  pub right: Box<Expr<'a>>,
}

impl<'a> BinExpr<'a> {
  pub fn op(&self) -> &BinaryOp {
    &self.inner.op
  }
}

impl<'a> Spanned for BinExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&BinExpr<'a>> for Node<'a> {
  fn from(node: &BinExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&BinExpr, &'a BinExpr>(&node) };
    Node::BinExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for BinExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children
  }

}

impl<'a> CastableNode<'a> for BinExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::BinExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_bin_expr<'a>(ref_node: &'a swc_ecma_ast::BinExpr) -> BinExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&BinExpr, &'a BinExpr>(&node) };
  let parent = Node::BinExpr(child_parent_ref);
  set_parent_for_expr(&mut node.left, parent.clone());
  set_parent_for_expr(&mut node.right, parent);
  node
}

pub struct UnaryExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::UnaryExpr,
  pub arg: Box<Expr<'a>>,
}

impl<'a> UnaryExpr<'a> {
  pub fn op(&self) -> &UnaryOp {
    &self.inner.op
  }
}

impl<'a> Spanned for UnaryExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&UnaryExpr<'a>> for Node<'a> {
  fn from(node: &UnaryExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&UnaryExpr, &'a UnaryExpr>(&node) };
    Node::UnaryExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for UnaryExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

}

impl<'a> CastableNode<'a> for UnaryExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::UnaryExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_unary_expr<'a>(ref_node: &'a swc_ecma_ast::UnaryExpr) -> UnaryExpr<'a> {
  let value = &ref_node.arg;
  let field_arg = Box::new(get_view_for_expr(value));
  let mut node = UnaryExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    arg: field_arg,
  };
  let child_parent_ref = unsafe { mem::transmute::<&UnaryExpr, &'a UnaryExpr>(&node) };
  let parent = Node::UnaryExpr(child_parent_ref);
  set_parent_for_expr(&mut node.arg, parent);
  node
}

pub struct TsPropertySignature<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsPropertySignature,
  pub key: Box<Expr<'a>>,
  pub init: Option<Box<Expr<'a>>>,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
}

impl<'a> TsPropertySignature<'a> {
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

impl<'a> Spanned for TsPropertySignature<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsPropertySignature<'a>> for Node<'a> {
  fn from(node: &TsPropertySignature) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsPropertySignature, &'a TsPropertySignature>(&node) };
    Node::TsPropertySignature(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsPropertySignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for TsPropertySignature<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsPropertySignature(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_property_signature<'a>(ref_node: &'a swc_ecma_ast::TsPropertySignature) -> TsPropertySignature<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsPropertySignature, &'a TsPropertySignature>(&node) };
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

pub struct Constructor<'a> {
  pub parent: &'a Class<'a>,
  pub inner: &'a swc_ecma_ast::Constructor,
  pub key: PropName<'a>,
  pub params: Vec<ParamOrTsParamProp<'a>>,
  pub body: Option<BlockStmt<'a>>,
}

impl<'a> Constructor<'a> {
  pub fn accessibility(&self) -> &Option<Accessibility> {
    &self.inner.accessibility
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }
}

impl<'a> Spanned for Constructor<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Constructor<'a>> for Node<'a> {
  fn from(node: &Constructor) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Constructor, &'a Constructor>(&node) };
    Node::Constructor(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Constructor<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for Constructor<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Constructor(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_constructor<'a>(ref_node: &'a swc_ecma_ast::Constructor) -> Constructor<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&Constructor, &'a Constructor>(&node) };
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

pub struct FnDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::FnDecl,
  pub ident: Ident<'a>,
  pub function: Function<'a>,
}

impl<'a> FnDecl<'a> {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl<'a> Spanned for FnDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&FnDecl<'a>> for Node<'a> {
  fn from(node: &FnDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&FnDecl, &'a FnDecl>(&node) };
    Node::FnDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for FnDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.ident).into());
    children.push((&self.function).into());
    children
  }

}

impl<'a> CastableNode<'a> for FnDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::FnDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_fn_decl<'a>(ref_node: &'a swc_ecma_ast::FnDecl) -> FnDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&FnDecl, &'a FnDecl>(&node) };
  let parent = Node::FnDecl(child_parent_ref);
  node.ident.parent = parent.clone();
  node.function.parent = parent;
  node
}

pub struct TsNonNullExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsNonNullExpr,
  pub expr: Box<Expr<'a>>,
}

impl<'a> Spanned for TsNonNullExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsNonNullExpr<'a>> for Node<'a> {
  fn from(node: &TsNonNullExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsNonNullExpr, &'a TsNonNullExpr>(&node) };
    Node::TsNonNullExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsNonNullExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsNonNullExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsNonNullExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_non_null_expr<'a>(ref_node: &'a swc_ecma_ast::TsNonNullExpr) -> TsNonNullExpr<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = TsNonNullExpr {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsNonNullExpr, &'a TsNonNullExpr>(&node) };
  let parent = Node::TsNonNullExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

/// Class expression.
pub struct ClassExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ClassExpr,
  pub ident: Option<Ident<'a>>,
  pub class: Class<'a>,
}

impl<'a> Spanned for ClassExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ClassExpr<'a>> for Node<'a> {
  fn from(node: &ClassExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ClassExpr, &'a ClassExpr>(&node) };
    Node::ClassExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ClassExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.ident { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.ident {
      children.push(child.into());
    }
    children.push((&self.class).into());
    children
  }

}

impl<'a> CastableNode<'a> for ClassExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ClassExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_class_expr<'a>(ref_node: &'a swc_ecma_ast::ClassExpr) -> ClassExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ClassExpr, &'a ClassExpr>(&node) };
  let parent = Node::ClassExpr(child_parent_ref);
  if let Some(node) = node.ident.as_mut() {
    node.parent = parent.clone();
  }
  node.class.parent = parent;
  node
}

pub struct ForInStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ForInStmt,
  pub left: VarDeclOrPat<'a>,
  pub right: Box<Expr<'a>>,
  pub body: Box<Stmt<'a>>,
}

impl<'a> Spanned for ForInStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ForInStmt<'a>> for Node<'a> {
  fn from(node: &ForInStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ForInStmt, &'a ForInStmt>(&node) };
    Node::ForInStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ForInStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(3);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children.push((&self.body).into());
    children
  }

}

impl<'a> CastableNode<'a> for ForInStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ForInStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_for_in_stmt<'a>(ref_node: &'a swc_ecma_ast::ForInStmt) -> ForInStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ForInStmt, &'a ForInStmt>(&node) };
  let parent = Node::ForInStmt(child_parent_ref);
  set_parent_for_var_decl_or_pat(&mut node.left, parent.clone());
  set_parent_for_expr(&mut node.right, parent.clone());
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct EmptyStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::EmptyStmt,
}

impl<'a> Spanned for EmptyStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&EmptyStmt<'a>> for Node<'a> {
  fn from(node: &EmptyStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&EmptyStmt, &'a EmptyStmt>(&node) };
    Node::EmptyStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for EmptyStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for EmptyStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::EmptyStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_empty_stmt<'a>(ref_node: &'a swc_ecma_ast::EmptyStmt) -> EmptyStmt<'a> {
  let node = EmptyStmt {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct WhileStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::WhileStmt,
  pub test: Box<Expr<'a>>,
  pub body: Box<Stmt<'a>>,
}

impl<'a> Spanned for WhileStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&WhileStmt<'a>> for Node<'a> {
  fn from(node: &WhileStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&WhileStmt, &'a WhileStmt>(&node) };
    Node::WhileStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for WhileStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.test).into());
    children.push((&self.body).into());
    children
  }

}

impl<'a> CastableNode<'a> for WhileStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::WhileStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_while_stmt<'a>(ref_node: &'a swc_ecma_ast::WhileStmt) -> WhileStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&WhileStmt, &'a WhileStmt>(&node) };
  let parent = Node::WhileStmt(child_parent_ref);
  set_parent_for_expr(&mut node.test, parent.clone());
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct Str<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Str,
}

impl<'a> Str<'a> {
  pub fn value(&self) -> &swc_atoms::JsWord {
    &self.inner.value
  }

  /// This includes line escape.
  pub fn has_escape(&self) -> bool {
    self.inner.has_escape
  }
}

impl<'a> Spanned for Str<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Str<'a>> for Node<'a> {
  fn from(node: &Str) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Str, &'a Str>(&node) };
    Node::Str(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Str<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for Str<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Str(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_str<'a>(ref_node: &'a swc_ecma_ast::Str) -> Str<'a> {
  let node = Str {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct TsExprWithTypeArgs<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsExprWithTypeArgs,
  pub expr: TsEntityName<'a>,
  pub type_args: Option<TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for TsExprWithTypeArgs<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsExprWithTypeArgs<'a>> for Node<'a> {
  fn from(node: &TsExprWithTypeArgs) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsExprWithTypeArgs, &'a TsExprWithTypeArgs>(&node) };
    Node::TsExprWithTypeArgs(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsExprWithTypeArgs<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.expr).into());
    if let Some(child) = &self.type_args {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsExprWithTypeArgs<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsExprWithTypeArgs(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_expr_with_type_args<'a>(ref_node: &'a swc_ecma_ast::TsExprWithTypeArgs) -> TsExprWithTypeArgs<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsExprWithTypeArgs, &'a TsExprWithTypeArgs>(&node) };
  let parent = Node::TsExprWithTypeArgs(child_parent_ref);
  set_parent_for_ts_entity_name(&mut node.expr, parent.clone());
  if let Some(node) = node.type_args.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct AssignPat<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::AssignPat,
  pub left: Box<Pat<'a>>,
  pub right: Box<Expr<'a>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
}

impl<'a> Spanned for AssignPat<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&AssignPat<'a>> for Node<'a> {
  fn from(node: &AssignPat) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&AssignPat, &'a AssignPat>(&node) };
    Node::AssignPat(static_ref)
  }
}

impl<'a> NodeTrait<'a> for AssignPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.left).into());
    children.push((&self.right).into());
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for AssignPat<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::AssignPat(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_assign_pat<'a>(ref_node: &'a swc_ecma_ast::AssignPat) -> AssignPat<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&AssignPat, &'a AssignPat>(&node) };
  let parent = Node::AssignPat(child_parent_ref);
  set_parent_for_pat(&mut node.left, parent.clone());
  set_parent_for_expr(&mut node.right, parent.clone());
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct ExportNamedSpecifier<'a> {
  pub parent: &'a NamedExport<'a>,
  pub inner: &'a swc_ecma_ast::ExportNamedSpecifier,
  /// `foo` in `export { foo as bar }`
  pub orig: Ident<'a>,
  /// `Some(bar)` in `export { foo as bar }`
  pub exported: Option<Ident<'a>>,
}

impl<'a> Spanned for ExportNamedSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportNamedSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportNamedSpecifier) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ExportNamedSpecifier, &'a ExportNamedSpecifier>(&node) };
    Node::ExportNamedSpecifier(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ExportNamedSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.exported { Some(_value) => 1, None => 0, });
    children.push((&self.orig).into());
    if let Some(child) = &self.exported {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for ExportNamedSpecifier<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportNamedSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_export_named_specifier<'a>(ref_node: &'a swc_ecma_ast::ExportNamedSpecifier) -> ExportNamedSpecifier<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ExportNamedSpecifier, &'a ExportNamedSpecifier>(&node) };
  let parent = Node::ExportNamedSpecifier(child_parent_ref);
  node.orig.parent = parent.clone();
  if let Some(node) = node.exported.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct TsConditionalType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsConditionalType,
  pub check_type: Box<TsType<'a>>,
  pub extends_type: Box<TsType<'a>>,
  pub true_type: Box<TsType<'a>>,
  pub false_type: Box<TsType<'a>>,
}

impl<'a> Spanned for TsConditionalType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsConditionalType<'a>> for Node<'a> {
  fn from(node: &TsConditionalType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsConditionalType, &'a TsConditionalType>(&node) };
    Node::TsConditionalType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsConditionalType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(4);
    children.push((&self.check_type).into());
    children.push((&self.extends_type).into());
    children.push((&self.true_type).into());
    children.push((&self.false_type).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsConditionalType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsConditionalType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_conditional_type<'a>(ref_node: &'a swc_ecma_ast::TsConditionalType) -> TsConditionalType<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsConditionalType, &'a TsConditionalType>(&node) };
  let parent = Node::TsConditionalType(child_parent_ref);
  set_parent_for_ts_type(&mut node.check_type, parent.clone());
  set_parent_for_ts_type(&mut node.extends_type, parent.clone());
  set_parent_for_ts_type(&mut node.true_type, parent.clone());
  set_parent_for_ts_type(&mut node.false_type, parent);
  node
}

pub struct TsTypeLit<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsTypeLit,
  pub members: Vec<TsTypeElement<'a>>,
}

impl<'a> Spanned for TsTypeLit<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeLit<'a>> for Node<'a> {
  fn from(node: &TsTypeLit) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsTypeLit, &'a TsTypeLit>(&node) };
    Node::TsTypeLit(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsTypeLit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.members.len());
    for child in self.members.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsTypeLit<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeLit(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_type_lit<'a>(ref_node: &'a swc_ecma_ast::TsTypeLit) -> TsTypeLit<'a> {
  let value = &ref_node.members;
  let field_members = value.iter().map(|value| get_view_for_ts_type_element(value)).collect();
  let mut node = TsTypeLit {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    members: field_members,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsTypeLit, &'a TsTypeLit>(&node) };
  let parent = Node::TsTypeLit(child_parent_ref);
  for node in node.members.iter_mut() {
    set_parent_for_ts_type_element(node, parent.clone());
  }
  node
}

pub struct BreakStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::BreakStmt,
  pub label: Option<Ident<'a>>,
}

impl<'a> Spanned for BreakStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&BreakStmt<'a>> for Node<'a> {
  fn from(node: &BreakStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&BreakStmt, &'a BreakStmt>(&node) };
    Node::BreakStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for BreakStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.label {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for BreakStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::BreakStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_break_stmt<'a>(ref_node: &'a swc_ecma_ast::BreakStmt) -> BreakStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&BreakStmt, &'a BreakStmt>(&node) };
  let parent = Node::BreakStmt(child_parent_ref);
  if let Some(node) = node.label.as_mut() {
    node.parent = parent;
  }
  node
}

/// e.g. `import * as foo from 'mod.js'`.
pub struct ImportStarAsSpecifier<'a> {
  pub parent: &'a ImportDecl<'a>,
  pub inner: &'a swc_ecma_ast::ImportStarAsSpecifier,
  pub local: Ident<'a>,
}

impl<'a> Spanned for ImportStarAsSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ImportStarAsSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportStarAsSpecifier) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ImportStarAsSpecifier, &'a ImportStarAsSpecifier>(&node) };
    Node::ImportStarAsSpecifier(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ImportStarAsSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.local).into());
    children
  }

}

impl<'a> CastableNode<'a> for ImportStarAsSpecifier<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ImportStarAsSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_import_star_as_specifier<'a>(ref_node: &'a swc_ecma_ast::ImportStarAsSpecifier) -> ImportStarAsSpecifier<'a> {
  let value = &ref_node.local;
  let field_local = get_view_for_ident(value);
  let mut node = ImportStarAsSpecifier {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    local: field_local,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ImportStarAsSpecifier, &'a ImportStarAsSpecifier>(&node) };
  let parent = Node::ImportStarAsSpecifier(child_parent_ref);
  node.local.parent = parent;
  node
}

pub struct TsInferType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsInferType,
  pub type_param: TsTypeParam<'a>,
}

impl<'a> Spanned for TsInferType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsInferType<'a>> for Node<'a> {
  fn from(node: &TsInferType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsInferType, &'a TsInferType>(&node) };
    Node::TsInferType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsInferType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_param).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsInferType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsInferType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_infer_type<'a>(ref_node: &'a swc_ecma_ast::TsInferType) -> TsInferType<'a> {
  let value = &ref_node.type_param;
  let field_type_param = get_view_for_ts_type_param(value);
  let mut node = TsInferType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    type_param: field_type_param,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsInferType, &'a TsInferType>(&node) };
  let parent = Node::TsInferType(child_parent_ref);
  node.type_param.parent = parent;
  node
}

pub struct PrivateMethod<'a> {
  pub parent: &'a Class<'a>,
  pub inner: &'a swc_ecma_ast::PrivateMethod,
  pub key: PrivateName<'a>,
  pub function: Function<'a>,
}

impl<'a> PrivateMethod<'a> {
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

impl<'a> Spanned for PrivateMethod<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&PrivateMethod<'a>> for Node<'a> {
  fn from(node: &PrivateMethod) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&PrivateMethod, &'a PrivateMethod>(&node) };
    Node::PrivateMethod(static_ref)
  }
}

impl<'a> NodeTrait<'a> for PrivateMethod<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.function).into());
    children
  }

}

impl<'a> CastableNode<'a> for PrivateMethod<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::PrivateMethod(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_private_method<'a>(ref_node: &'a swc_ecma_ast::PrivateMethod) -> PrivateMethod<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&PrivateMethod, &'a PrivateMethod>(&node) };
  let parent = Node::PrivateMethod(child_parent_ref);
  node.key.parent = parent.clone();
  node.function.parent = parent;
  node
}

pub struct ForOfStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ForOfStmt,
  pub left: VarDeclOrPat<'a>,
  pub right: Box<Expr<'a>>,
  pub body: Box<Stmt<'a>>,
}

impl<'a> ForOfStmt<'a> {
  /// Span of the await token.
  /// 
  /// es2018
  /// 
  /// for-await-of statements, e.g., `for await (const x of xs) {`
  pub fn await_token(&self) -> &Option<swc_common::Span> {
    &self.inner.await_token
  }
}

impl<'a> Spanned for ForOfStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ForOfStmt<'a>> for Node<'a> {
  fn from(node: &ForOfStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ForOfStmt, &'a ForOfStmt>(&node) };
    Node::ForOfStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ForOfStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(3);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children.push((&self.body).into());
    children
  }

}

impl<'a> CastableNode<'a> for ForOfStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ForOfStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_for_of_stmt<'a>(ref_node: &'a swc_ecma_ast::ForOfStmt) -> ForOfStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ForOfStmt, &'a ForOfStmt>(&node) };
  let parent = Node::ForOfStmt(child_parent_ref);
  set_parent_for_var_decl_or_pat(&mut node.left, parent.clone());
  set_parent_for_expr(&mut node.right, parent.clone());
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct TsUnionType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsUnionType,
  pub types: Vec<Box<TsType<'a>>>,
}

impl<'a> Spanned for TsUnionType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsUnionType<'a>> for Node<'a> {
  fn from(node: &TsUnionType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsUnionType, &'a TsUnionType>(&node) };
    Node::TsUnionType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsUnionType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.types.len());
    for child in self.types.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsUnionType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsUnionType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_union_type<'a>(ref_node: &'a swc_ecma_ast::TsUnionType) -> TsUnionType<'a> {
  let value = &ref_node.types;
  let field_types = value.iter().map(|value| Box::new(get_view_for_ts_type(value))).collect();
  let mut node = TsUnionType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    types: field_types,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsUnionType, &'a TsUnionType>(&node) };
  let parent = Node::TsUnionType(child_parent_ref);
  for node in node.types.iter_mut() {
    set_parent_for_ts_type(node, parent.clone());
  }
  node
}

pub struct TsModuleDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsModuleDecl,
  pub id: TsModuleName<'a>,
  pub body: Option<TsNamespaceBody<'a>>,
}

impl<'a> TsModuleDecl<'a> {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  /// In TypeScript, this is only available through`node.flags`.
  pub fn global(&self) -> bool {
    self.inner.global
  }
}

impl<'a> Spanned for TsModuleDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsModuleDecl<'a>> for Node<'a> {
  fn from(node: &TsModuleDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsModuleDecl, &'a TsModuleDecl>(&node) };
    Node::TsModuleDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsModuleDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.id).into());
    if let Some(child) = &self.body {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for TsModuleDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsModuleDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_module_decl<'a>(ref_node: &'a swc_ecma_ast::TsModuleDecl) -> TsModuleDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsModuleDecl, &'a TsModuleDecl>(&node) };
  let parent = Node::TsModuleDecl(child_parent_ref);
  set_parent_for_ts_module_name(&mut node.id, parent.clone());
  if let Some(node) = node.body.as_mut() {
    set_parent_for_ts_namespace_body(node, parent);
  }
  node
}

pub struct GetterProp<'a> {
  pub parent: &'a ObjectLit<'a>,
  pub inner: &'a swc_ecma_ast::GetterProp,
  pub key: PropName<'a>,
  pub type_ann: Option<TsTypeAnn<'a>>,
  pub body: Option<BlockStmt<'a>>,
}

impl<'a> Spanned for GetterProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&GetterProp<'a>> for Node<'a> {
  fn from(node: &GetterProp) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&GetterProp, &'a GetterProp>(&node) };
    Node::GetterProp(static_ref)
  }
}

impl<'a> NodeTrait<'a> for GetterProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for GetterProp<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::GetterProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_getter_prop<'a>(ref_node: &'a swc_ecma_ast::GetterProp) -> GetterProp<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&GetterProp, &'a GetterProp>(&node) };
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

pub struct CondExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::CondExpr,
  pub test: Box<Expr<'a>>,
  pub cons: Box<Expr<'a>>,
  pub alt: Box<Expr<'a>>,
}

impl<'a> Spanned for CondExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&CondExpr<'a>> for Node<'a> {
  fn from(node: &CondExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&CondExpr, &'a CondExpr>(&node) };
    Node::CondExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for CondExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(3);
    children.push((&self.test).into());
    children.push((&self.cons).into());
    children.push((&self.alt).into());
    children
  }

}

impl<'a> CastableNode<'a> for CondExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::CondExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_cond_expr<'a>(ref_node: &'a swc_ecma_ast::CondExpr) -> CondExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&CondExpr, &'a CondExpr>(&node) };
  let parent = Node::CondExpr(child_parent_ref);
  set_parent_for_expr(&mut node.test, parent.clone());
  set_parent_for_expr(&mut node.cons, parent.clone());
  set_parent_for_expr(&mut node.alt, parent);
  node
}

/// e.g. local = foo, imported = None `import { foo } from 'mod.js'`
/// e.g. local = bar, imported = Some(foo) for `import { foo as bar } from
/// 'mod.js'`
pub struct ImportNamedSpecifier<'a> {
  pub parent: &'a ImportDecl<'a>,
  pub inner: &'a swc_ecma_ast::ImportNamedSpecifier,
  pub local: Ident<'a>,
  pub imported: Option<Ident<'a>>,
}

impl<'a> Spanned for ImportNamedSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ImportNamedSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportNamedSpecifier) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ImportNamedSpecifier, &'a ImportNamedSpecifier>(&node) };
    Node::ImportNamedSpecifier(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ImportNamedSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.imported { Some(_value) => 1, None => 0, });
    children.push((&self.local).into());
    if let Some(child) = &self.imported {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for ImportNamedSpecifier<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ImportNamedSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_import_named_specifier<'a>(ref_node: &'a swc_ecma_ast::ImportNamedSpecifier) -> ImportNamedSpecifier<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ImportNamedSpecifier, &'a ImportNamedSpecifier>(&node) };
  let parent = Node::ImportNamedSpecifier(child_parent_ref);
  node.local.parent = parent.clone();
  if let Some(node) = node.imported.as_mut() {
    node.parent = parent;
  }
  node
}

/// `export { foo } from 'mod'`
/// `export { foo as bar } from 'mod'`
pub struct NamedExport<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::NamedExport,
  pub specifiers: Vec<ExportSpecifier<'a>>,
  pub src: Option<Str<'a>>,
}

impl<'a> NamedExport<'a> {
  pub fn type_only(&self) -> bool {
    self.inner.type_only
  }
}

impl<'a> Spanned for NamedExport<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&NamedExport<'a>> for Node<'a> {
  fn from(node: &NamedExport) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&NamedExport, &'a NamedExport>(&node) };
    Node::NamedExport(static_ref)
  }
}

impl<'a> NodeTrait<'a> for NamedExport<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for NamedExport<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::NamedExport(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_named_export<'a>(ref_node: &'a swc_ecma_ast::NamedExport) -> NamedExport<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&NamedExport, &'a NamedExport>(&node) };
  let parent = Node::NamedExport(child_parent_ref);
  for node in node.specifiers.iter_mut() {
    set_parent_for_export_specifier(node, parent.clone());
  }
  if let Some(node) = node.src.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct JSXElement<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::JSXElement,
  pub opening: JSXOpeningElement<'a>,
  pub children: Vec<JSXElementChild<'a>>,
  pub closing: Option<JSXClosingElement<'a>>,
}

impl<'a> Spanned for JSXElement<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXElement<'a>> for Node<'a> {
  fn from(node: &JSXElement) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXElement, &'a JSXElement>(&node) };
    Node::JSXElement(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for JSXElement<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxelement<'a>(ref_node: &'a swc_ecma_ast::JSXElement) -> JSXElement<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&JSXElement, &'a JSXElement>(&node) };
  let parent = Node::JSXElement(child_parent_ref);
  node.opening.parent = parent.to::<JSXElement>();
  for node in node.children.iter_mut() {
    set_parent_for_jsxelement_child(node, parent.clone());
  }
  if let Some(node) = node.closing.as_mut() {
    node.parent = parent.to::<JSXElement>();
  }
  node
}

pub struct ClassDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ClassDecl,
  pub ident: Ident<'a>,
  pub class: Class<'a>,
}

impl<'a> ClassDecl<'a> {
  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl<'a> Spanned for ClassDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ClassDecl<'a>> for Node<'a> {
  fn from(node: &ClassDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ClassDecl, &'a ClassDecl>(&node) };
    Node::ClassDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ClassDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.ident).into());
    children.push((&self.class).into());
    children
  }

}

impl<'a> CastableNode<'a> for ClassDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ClassDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_class_decl<'a>(ref_node: &'a swc_ecma_ast::ClassDecl) -> ClassDecl<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ClassDecl, &'a ClassDecl>(&node) };
  let parent = Node::ClassDecl(child_parent_ref);
  node.ident.parent = parent.clone();
  node.class.parent = parent;
  node
}

pub struct ArrayPat<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ArrayPat,
  pub elems: Vec<Option<Pat<'a>>>,
  pub type_ann: Option<TsTypeAnn<'a>>,
}

impl<'a> ArrayPat<'a> {
  /// Only in an ambient context
  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl<'a> Spanned for ArrayPat<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ArrayPat<'a>> for Node<'a> {
  fn from(node: &ArrayPat) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ArrayPat, &'a ArrayPat>(&node) };
    Node::ArrayPat(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ArrayPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for ArrayPat<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ArrayPat(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_array_pat<'a>(ref_node: &'a swc_ecma_ast::ArrayPat) -> ArrayPat<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ArrayPat, &'a ArrayPat>(&node) };
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

pub struct DoWhileStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::DoWhileStmt,
  pub test: Box<Expr<'a>>,
  pub body: Box<Stmt<'a>>,
}

impl<'a> Spanned for DoWhileStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&DoWhileStmt<'a>> for Node<'a> {
  fn from(node: &DoWhileStmt) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&DoWhileStmt, &'a DoWhileStmt>(&node) };
    Node::DoWhileStmt(static_ref)
  }
}

impl<'a> NodeTrait<'a> for DoWhileStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.test).into());
    children.push((&self.body).into());
    children
  }

}

impl<'a> CastableNode<'a> for DoWhileStmt<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::DoWhileStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_do_while_stmt<'a>(ref_node: &'a swc_ecma_ast::DoWhileStmt) -> DoWhileStmt<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&DoWhileStmt, &'a DoWhileStmt>(&node) };
  let parent = Node::DoWhileStmt(child_parent_ref);
  set_parent_for_expr(&mut node.test, parent.clone());
  set_parent_for_stmt(&mut node.body, parent);
  node
}

pub struct JSXText<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::JSXText,
}

impl<'a> JSXText<'a> {
  pub fn value(&self) -> &swc_atoms::JsWord {
    &self.inner.value
  }

  pub fn raw(&self) -> &swc_atoms::JsWord {
    &self.inner.raw
  }
}

impl<'a> Spanned for JSXText<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXText<'a>> for Node<'a> {
  fn from(node: &JSXText) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXText, &'a JSXText>(&node) };
    Node::JSXText(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXText<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

}

impl<'a> CastableNode<'a> for JSXText<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXText(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxtext<'a>(ref_node: &'a swc_ecma_ast::JSXText) -> JSXText<'a> {
  let node = JSXText {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
  };
  node
}

pub struct VarDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::VarDecl,
  pub decls: Vec<VarDeclarator<'a>>,
}

impl<'a> VarDecl<'a> {
  pub fn kind(&self) -> &VarDeclKind {
    &self.inner.kind
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl<'a> Spanned for VarDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&VarDecl<'a>> for Node<'a> {
  fn from(node: &VarDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&VarDecl, &'a VarDecl>(&node) };
    Node::VarDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for VarDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.decls.len());
    for child in self.decls.iter() {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for VarDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::VarDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_var_decl<'a>(ref_node: &'a swc_ecma_ast::VarDecl) -> VarDecl<'a> {
  let value = &ref_node.decls;
  let field_decls = value.iter().map(|value| get_view_for_var_declarator(value)).collect();
  let mut node = VarDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    decls: field_decls,
  };
  let child_parent_ref = unsafe { mem::transmute::<&VarDecl, &'a VarDecl>(&node) };
  let parent = Node::VarDecl(child_parent_ref);
  for node in node.decls.iter_mut() {
    node.parent = parent.to::<VarDecl>();
  }
  node
}

pub struct PrivateName<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::PrivateName,
  pub id: Ident<'a>,
}

impl<'a> Spanned for PrivateName<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&PrivateName<'a>> for Node<'a> {
  fn from(node: &PrivateName) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&PrivateName, &'a PrivateName>(&node) };
    Node::PrivateName(static_ref)
  }
}

impl<'a> NodeTrait<'a> for PrivateName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.id).into());
    children
  }

}

impl<'a> CastableNode<'a> for PrivateName<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::PrivateName(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_private_name<'a>(ref_node: &'a swc_ecma_ast::PrivateName) -> PrivateName<'a> {
  let value = &ref_node.id;
  let field_id = get_view_for_ident(value);
  let mut node = PrivateName {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    id: field_id,
  };
  let child_parent_ref = unsafe { mem::transmute::<&PrivateName, &'a PrivateName>(&node) };
  let parent = Node::PrivateName(child_parent_ref);
  node.id.parent = parent;
  node
}

/// XML-based namespace syntax:
pub struct JSXNamespacedName<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::JSXNamespacedName,
  pub ns: Ident<'a>,
  pub name: Ident<'a>,
}

impl<'a> Spanned for JSXNamespacedName<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXNamespacedName<'a>> for Node<'a> {
  fn from(node: &JSXNamespacedName) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXNamespacedName, &'a JSXNamespacedName>(&node) };
    Node::JSXNamespacedName(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXNamespacedName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.ns).into());
    children.push((&self.name).into());
    children
  }

}

impl<'a> CastableNode<'a> for JSXNamespacedName<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXNamespacedName(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxnamespaced_name<'a>(ref_node: &'a swc_ecma_ast::JSXNamespacedName) -> JSXNamespacedName<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&JSXNamespacedName, &'a JSXNamespacedName>(&node) };
  let parent = Node::JSXNamespacedName(child_parent_ref);
  node.ns.parent = parent.clone();
  node.name.parent = parent;
  node
}

pub struct JSXOpeningElement<'a> {
  pub parent: &'a JSXElement<'a>,
  pub inner: &'a swc_ecma_ast::JSXOpeningElement,
  pub name: JSXElementName<'a>,
  pub attrs: Vec<JSXAttrOrSpread<'a>>,
  /// Note: This field's name is differrent from one from babel because it is
  /// misleading
  pub type_args: Option<TsTypeParamInstantiation<'a>>,
}

impl<'a> JSXOpeningElement<'a> {
  pub fn self_closing(&self) -> bool {
    self.inner.self_closing
  }
}

impl<'a> Spanned for JSXOpeningElement<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXOpeningElement<'a>> for Node<'a> {
  fn from(node: &JSXOpeningElement) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&JSXOpeningElement, &'a JSXOpeningElement>(&node) };
    Node::JSXOpeningElement(static_ref)
  }
}

impl<'a> NodeTrait<'a> for JSXOpeningElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for JSXOpeningElement<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXOpeningElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_jsxopening_element<'a>(ref_node: &'a swc_ecma_ast::JSXOpeningElement) -> JSXOpeningElement<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&JSXOpeningElement, &'a JSXOpeningElement>(&node) };
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

pub struct SpreadElement<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::SpreadElement,
  pub expr: Box<Expr<'a>>,
}

impl<'a> SpreadElement<'a> {
  pub fn dot3_token(&self) -> &swc_common::Span {
    &self.inner.dot3_token
  }
}

impl<'a> Spanned for SpreadElement<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&SpreadElement<'a>> for Node<'a> {
  fn from(node: &SpreadElement) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&SpreadElement, &'a SpreadElement>(&node) };
    Node::SpreadElement(static_ref)
  }
}

impl<'a> NodeTrait<'a> for SpreadElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for SpreadElement<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SpreadElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_spread_element<'a>(ref_node: &'a swc_ecma_ast::SpreadElement) -> SpreadElement<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = SpreadElement {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&SpreadElement, &'a SpreadElement>(&node) };
  let parent = Node::SpreadElement(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

pub struct ExportDefaultDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ExportDefaultDecl,
  pub decl: DefaultDecl<'a>,
}

impl<'a> Spanned for ExportDefaultDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportDefaultDecl<'a>> for Node<'a> {
  fn from(node: &ExportDefaultDecl) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ExportDefaultDecl, &'a ExportDefaultDecl>(&node) };
    Node::ExportDefaultDecl(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ExportDefaultDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.decl).into());
    children
  }

}

impl<'a> CastableNode<'a> for ExportDefaultDecl<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportDefaultDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_export_default_decl<'a>(ref_node: &'a swc_ecma_ast::ExportDefaultDecl) -> ExportDefaultDecl<'a> {
  let value = &ref_node.decl;
  let field_decl = get_view_for_default_decl(value);
  let mut node = ExportDefaultDecl {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    decl: field_decl,
  };
  let child_parent_ref = unsafe { mem::transmute::<&ExportDefaultDecl, &'a ExportDefaultDecl>(&node) };
  let parent = Node::ExportDefaultDecl(child_parent_ref);
  set_parent_for_default_decl(&mut node.decl, parent);
  node
}

pub struct ArrowExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ArrowExpr,
  pub params: Vec<Pat<'a>>,
  pub body: BlockStmtOrExpr<'a>,
  pub type_params: Option<TsTypeParamDecl<'a>>,
  pub return_type: Option<TsTypeAnn<'a>>,
}

impl<'a> ArrowExpr<'a> {
  pub fn is_async(&self) -> bool {
    self.inner.is_async
  }

  pub fn is_generator(&self) -> bool {
    self.inner.is_generator
  }
}

impl<'a> Spanned for ArrowExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ArrowExpr<'a>> for Node<'a> {
  fn from(node: &ArrowExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ArrowExpr, &'a ArrowExpr>(&node) };
    Node::ArrowExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ArrowExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
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

impl<'a> CastableNode<'a> for ArrowExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ArrowExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_arrow_expr<'a>(ref_node: &'a swc_ecma_ast::ArrowExpr) -> ArrowExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ArrowExpr, &'a ArrowExpr>(&node) };
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

pub struct TsAsExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsAsExpr,
  pub expr: Box<Expr<'a>>,
  pub type_ann: Box<TsType<'a>>,
}

impl<'a> Spanned for TsAsExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsAsExpr<'a>> for Node<'a> {
  fn from(node: &TsAsExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsAsExpr, &'a TsAsExpr>(&node) };
    Node::TsAsExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsAsExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push((&self.type_ann).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsAsExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsAsExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_as_expr<'a>(ref_node: &'a swc_ecma_ast::TsAsExpr) -> TsAsExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&TsAsExpr, &'a TsAsExpr>(&node) };
  let parent = Node::TsAsExpr(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent.clone());
  set_parent_for_ts_type(&mut node.type_ann, parent);
  node
}

/// `{key: value}`
pub struct KeyValuePatProp<'a> {
  pub parent: &'a ObjectPat<'a>,
  pub inner: &'a swc_ecma_ast::KeyValuePatProp,
  pub key: PropName<'a>,
  pub value: Box<Pat<'a>>,
}

impl<'a> Spanned for KeyValuePatProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&KeyValuePatProp<'a>> for Node<'a> {
  fn from(node: &KeyValuePatProp) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&KeyValuePatProp, &'a KeyValuePatProp>(&node) };
    Node::KeyValuePatProp(static_ref)
  }
}

impl<'a> NodeTrait<'a> for KeyValuePatProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.value).into());
    children
  }

}

impl<'a> CastableNode<'a> for KeyValuePatProp<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::KeyValuePatProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_key_value_pat_prop<'a>(ref_node: &'a swc_ecma_ast::KeyValuePatProp) -> KeyValuePatProp<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&KeyValuePatProp, &'a KeyValuePatProp>(&node) };
  let parent = Node::KeyValuePatProp(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  set_parent_for_pat(&mut node.value, parent);
  node
}

pub struct TsLitType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::TsLitType,
  pub lit: TsLit<'a>,
}

impl<'a> Spanned for TsLitType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsLitType<'a>> for Node<'a> {
  fn from(node: &TsLitType) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&TsLitType, &'a TsLitType>(&node) };
    Node::TsLitType(static_ref)
  }
}

impl<'a> NodeTrait<'a> for TsLitType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.lit).into());
    children
  }

}

impl<'a> CastableNode<'a> for TsLitType<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsLitType(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ts_lit_type<'a>(ref_node: &'a swc_ecma_ast::TsLitType) -> TsLitType<'a> {
  let value = &ref_node.lit;
  let field_lit = get_view_for_ts_lit(value);
  let mut node = TsLitType {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    lit: field_lit,
  };
  let child_parent_ref = unsafe { mem::transmute::<&TsLitType, &'a TsLitType>(&node) };
  let parent = Node::TsLitType(child_parent_ref);
  set_parent_for_ts_lit(&mut node.lit, parent);
  node
}

pub struct AssignExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::AssignExpr,
  pub left: PatOrExpr<'a>,
  pub right: Box<Expr<'a>>,
}

impl<'a> AssignExpr<'a> {
  pub fn op(&self) -> &AssignOp {
    &self.inner.op
  }
}

impl<'a> Spanned for AssignExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&AssignExpr<'a>> for Node<'a> {
  fn from(node: &AssignExpr) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&AssignExpr, &'a AssignExpr>(&node) };
    Node::AssignExpr(static_ref)
  }
}

impl<'a> NodeTrait<'a> for AssignExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children
  }

}

impl<'a> CastableNode<'a> for AssignExpr<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::AssignExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_assign_expr<'a>(ref_node: &'a swc_ecma_ast::AssignExpr) -> AssignExpr<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&AssignExpr, &'a AssignExpr>(&node) };
  let parent = Node::AssignExpr(child_parent_ref);
  set_parent_for_pat_or_expr(&mut node.left, parent.clone());
  set_parent_for_expr(&mut node.right, parent);
  node
}

/// Array literal.
pub struct ArrayLit<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::ArrayLit,
  pub elems: Vec<Option<ExprOrSpread<'a>>>,
}

impl<'a> Spanned for ArrayLit<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ArrayLit<'a>> for Node<'a> {
  fn from(node: &ArrayLit) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&ArrayLit, &'a ArrayLit>(&node) };
    Node::ArrayLit(static_ref)
  }
}

impl<'a> NodeTrait<'a> for ArrayLit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.elems.len());
    for child in self.elems.iter() {
      if let Some(child) = &child {
        children.push(child.into());
      }
    }
    children
  }

}

impl<'a> CastableNode<'a> for ArrayLit<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ArrayLit(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_array_lit<'a>(ref_node: &'a swc_ecma_ast::ArrayLit) -> ArrayLit<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&ArrayLit, &'a ArrayLit>(&node) };
  let parent = Node::ArrayLit(child_parent_ref);
  for node in node.elems.iter_mut() {
    if let Some(node) = node.as_mut() {
      node.parent = parent.clone();
    }
  }
  node
}

pub struct Decorator<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Decorator,
  pub expr: Box<Expr<'a>>,
}

impl<'a> Spanned for Decorator<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Decorator<'a>> for Node<'a> {
  fn from(node: &Decorator) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Decorator, &'a Decorator>(&node) };
    Node::Decorator(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Decorator<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

}

impl<'a> CastableNode<'a> for Decorator<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Decorator(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_decorator<'a>(ref_node: &'a swc_ecma_ast::Decorator) -> Decorator<'a> {
  let value = &ref_node.expr;
  let field_expr = Box::new(get_view_for_expr(value));
  let mut node = Decorator {
    inner: ref_node,
    parent: unsafe { MaybeUninit::uninit().assume_init() },
    expr: field_expr,
  };
  let child_parent_ref = unsafe { mem::transmute::<&Decorator, &'a Decorator>(&node) };
  let parent = Node::Decorator(child_parent_ref);
  set_parent_for_expr(&mut node.expr, parent);
  node
}

/// Ident with span.
pub struct Ident<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ecma_ast::Ident,
  pub type_ann: Option<TsTypeAnn<'a>>,
}

impl<'a> Ident<'a> {
  pub fn sym(&self) -> &swc_atoms::JsWord {
    &self.inner.sym
  }

  /// TypeScript only. Used in case of an optional parameter.
  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl<'a> Spanned for Ident<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Ident<'a>> for Node<'a> {
  fn from(node: &Ident) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&Ident, &'a Ident>(&node) };
    Node::Ident(static_ref)
  }
}

impl<'a> NodeTrait<'a> for Ident<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.type_ann { Some(_value) => 1, None => 0, });
    if let Some(child) = &self.type_ann {
      children.push(child.into());
    }
    children
  }

}

impl<'a> CastableNode<'a> for Ident<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Ident(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_ident<'a>(ref_node: &'a swc_ecma_ast::Ident) -> Ident<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&Ident, &'a Ident>(&node) };
  let parent = Node::Ident(child_parent_ref);
  if let Some(node) = node.type_ann.as_mut() {
    node.parent = parent;
  }
  node
}

pub struct MethodProp<'a> {
  pub parent: &'a ObjectLit<'a>,
  pub inner: &'a swc_ecma_ast::MethodProp,
  pub key: PropName<'a>,
  pub function: Function<'a>,
}

impl<'a> Spanned for MethodProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&MethodProp<'a>> for Node<'a> {
  fn from(node: &MethodProp) -> Node<'a> {
    let static_ref = unsafe { mem::transmute::<&MethodProp, &'a MethodProp>(&node) };
    Node::MethodProp(static_ref)
  }
}

impl<'a> NodeTrait<'a> for MethodProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.function).into());
    children
  }

}

impl<'a> CastableNode<'a> for MethodProp<'a> {
  fn try_cast(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::MethodProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

}

fn get_view_for_method_prop<'a>(ref_node: &'a swc_ecma_ast::MethodProp) -> MethodProp<'a> {
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
  let child_parent_ref = unsafe { mem::transmute::<&MethodProp, &'a MethodProp>(&node) };
  let parent = Node::MethodProp(child_parent_ref);
  set_parent_for_prop_name(&mut node.key, parent.clone());
  node.function.parent = parent;
  node
}
