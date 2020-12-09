// This code is code generated.
// Run `deno run -A generation/main.ts` from the root directory to regenerate it.
use std::mem::{self, MaybeUninit};
use bumpalo::Bump;
use swc_common::{Span, Spanned};
use swc_ecmascript::ast::{self as swc_ast, VarDeclKind, TsTypeOperatorOp, TsKeywordTypeKind, BinaryOp, AssignOp, UpdateOp, Accessibility, MethodKind, UnaryOp, TruePlusMinus};
use crate::comments::*;
use crate::tokens::*;
use crate::types::*;

thread_local! {
  static LOCAL_BUMP_ALLOCATOR: std::cell::RefCell<Bump> = std::cell::RefCell::new(Bump::new());
}

pub fn with_ast_view<'a, T>(source_file_info: SourceFileInfo, with_view: impl FnOnce(&'a Module<'a>) -> T) -> T {
  LOCAL_BUMP_ALLOCATOR.with(|bump_cell| {
    let mut bump_borrow = bump_cell.borrow_mut();
    let bump_ref = unsafe { mem::transmute::<&Bump, &'a Bump>(&bump_borrow) };
    let info_ref = unsafe { mem::transmute::<&SourceFileInfo, &'a SourceFileInfo<'a>>(&source_file_info) };
    let ast_view = get_view_for_module(info_ref, bump_ref);
    let result = with_view(ast_view);
    bump_borrow.reset();
    result
  })
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

  fn into_node(&self) -> Node<'a> {
    match self {
      Node::SwitchCase(node) => node.into_node(),
      Node::ThrowStmt(node) => node.into_node(),
      Node::JSXClosingFragment(node) => node.into_node(),
      Node::BigInt(node) => node.into_node(),
      Node::ExportDefaultSpecifier(node) => node.into_node(),
      Node::TsTypeParam(node) => node.into_node(),
      Node::WithStmt(node) => node.into_node(),
      Node::Regex(node) => node.into_node(),
      Node::TsMethodSignature(node) => node.into_node(),
      Node::UpdateExpr(node) => node.into_node(),
      Node::SetterProp(node) => node.into_node(),
      Node::TaggedTpl(node) => node.into_node(),
      Node::ExportAll(node) => node.into_node(),
      Node::TsModuleBlock(node) => node.into_node(),
      Node::SwitchStmt(node) => node.into_node(),
      Node::TsEnumMember(node) => node.into_node(),
      Node::TsIndexedAccessType(node) => node.into_node(),
      Node::TsRestType(node) => node.into_node(),
      Node::ExprStmt(node) => node.into_node(),
      Node::TsOptionalType(node) => node.into_node(),
      Node::Tpl(node) => node.into_node(),
      Node::Invalid(node) => node.into_node(),
      Node::ComputedPropName(node) => node.into_node(),
      Node::TsFnType(node) => node.into_node(),
      Node::BlockStmt(node) => node.into_node(),
      Node::TsTypeAliasDecl(node) => node.into_node(),
      Node::MemberExpr(node) => node.into_node(),
      Node::Function(node) => node.into_node(),
      Node::ImportDecl(node) => node.into_node(),
      Node::TsTypePredicate(node) => node.into_node(),
      Node::YieldExpr(node) => node.into_node(),
      Node::KeyValueProp(node) => node.into_node(),
      Node::Param(node) => node.into_node(),
      Node::JSXFragment(node) => node.into_node(),
      Node::ImportDefaultSpecifier(node) => node.into_node(),
      Node::Number(node) => node.into_node(),
      Node::JSXAttr(node) => node.into_node(),
      Node::ParenExpr(node) => node.into_node(),
      Node::Super(node) => node.into_node(),
      Node::TsConstructorType(node) => node.into_node(),
      Node::Class(node) => node.into_node(),
      Node::RestPat(node) => node.into_node(),
      Node::TsNamespaceExportDecl(node) => node.into_node(),
      Node::JSXOpeningFragment(node) => node.into_node(),
      Node::NewExpr(node) => node.into_node(),
      Node::FnExpr(node) => node.into_node(),
      Node::IfStmt(node) => node.into_node(),
      Node::TsParenthesizedType(node) => node.into_node(),
      Node::AssignPatProp(node) => node.into_node(),
      Node::TsImportType(node) => node.into_node(),
      Node::Bool(node) => node.into_node(),
      Node::TsImportEqualsDecl(node) => node.into_node(),
      Node::AssignProp(node) => node.into_node(),
      Node::TsInterfaceDecl(node) => node.into_node(),
      Node::JSXEmptyExpr(node) => node.into_node(),
      Node::TsQualifiedName(node) => node.into_node(),
      Node::ExportDecl(node) => node.into_node(),
      Node::CatchClause(node) => node.into_node(),
      Node::LabeledStmt(node) => node.into_node(),
      Node::ContinueStmt(node) => node.into_node(),
      Node::TsConstructSignatureDecl(node) => node.into_node(),
      Node::TsEnumDecl(node) => node.into_node(),
      Node::OptChainExpr(node) => node.into_node(),
      Node::TsNamespaceDecl(node) => node.into_node(),
      Node::SeqExpr(node) => node.into_node(),
      Node::TsExternalModuleRef(node) => node.into_node(),
      Node::TsTypeParamInstantiation(node) => node.into_node(),
      Node::ReturnStmt(node) => node.into_node(),
      Node::TsTplLitType(node) => node.into_node(),
      Node::ExportDefaultExpr(node) => node.into_node(),
      Node::TsCallSignatureDecl(node) => node.into_node(),
      Node::AwaitExpr(node) => node.into_node(),
      Node::ClassMethod(node) => node.into_node(),
      Node::TsParamProp(node) => node.into_node(),
      Node::ClassProp(node) => node.into_node(),
      Node::TsTypeAnn(node) => node.into_node(),
      Node::ForStmt(node) => node.into_node(),
      Node::ObjectPat(node) => node.into_node(),
      Node::TsTypeQuery(node) => node.into_node(),
      Node::ThisExpr(node) => node.into_node(),
      Node::DebuggerStmt(node) => node.into_node(),
      Node::TsTypeParamDecl(node) => node.into_node(),
      Node::TsTypeAssertion(node) => node.into_node(),
      Node::TplElement(node) => node.into_node(),
      Node::TsKeywordType(node) => node.into_node(),
      Node::JSXSpreadChild(node) => node.into_node(),
      Node::TsIntersectionType(node) => node.into_node(),
      Node::MetaPropExpr(node) => node.into_node(),
      Node::ExprOrSpread(node) => node.into_node(),
      Node::TsArrayType(node) => node.into_node(),
      Node::TsTypeRef(node) => node.into_node(),
      Node::TsThisType(node) => node.into_node(),
      Node::TryStmt(node) => node.into_node(),
      Node::CallExpr(node) => node.into_node(),
      Node::TsMappedType(node) => node.into_node(),
      Node::JSXExprContainer(node) => node.into_node(),
      Node::PrivateProp(node) => node.into_node(),
      Node::TsExportAssignment(node) => node.into_node(),
      Node::TsInterfaceBody(node) => node.into_node(),
      Node::TsTupleElement(node) => node.into_node(),
      Node::VarDeclarator(node) => node.into_node(),
      Node::JSXMemberExpr(node) => node.into_node(),
      Node::TsConstAssertion(node) => node.into_node(),
      Node::ExportNamespaceSpecifier(node) => node.into_node(),
      Node::ObjectLit(node) => node.into_node(),
      Node::Module(node) => node.into_node(),
      Node::TsIndexSignature(node) => node.into_node(),
      Node::TsTypeCastExpr(node) => node.into_node(),
      Node::TsTupleType(node) => node.into_node(),
      Node::Null(node) => node.into_node(),
      Node::TsTypeOperator(node) => node.into_node(),
      Node::JSXClosingElement(node) => node.into_node(),
      Node::BinExpr(node) => node.into_node(),
      Node::UnaryExpr(node) => node.into_node(),
      Node::TsPropertySignature(node) => node.into_node(),
      Node::Constructor(node) => node.into_node(),
      Node::FnDecl(node) => node.into_node(),
      Node::TsNonNullExpr(node) => node.into_node(),
      Node::ClassExpr(node) => node.into_node(),
      Node::ForInStmt(node) => node.into_node(),
      Node::EmptyStmt(node) => node.into_node(),
      Node::WhileStmt(node) => node.into_node(),
      Node::Str(node) => node.into_node(),
      Node::TsExprWithTypeArgs(node) => node.into_node(),
      Node::AssignPat(node) => node.into_node(),
      Node::ExportNamedSpecifier(node) => node.into_node(),
      Node::TsConditionalType(node) => node.into_node(),
      Node::TsTypeLit(node) => node.into_node(),
      Node::BreakStmt(node) => node.into_node(),
      Node::ImportStarAsSpecifier(node) => node.into_node(),
      Node::TsInferType(node) => node.into_node(),
      Node::PrivateMethod(node) => node.into_node(),
      Node::ForOfStmt(node) => node.into_node(),
      Node::TsUnionType(node) => node.into_node(),
      Node::TsModuleDecl(node) => node.into_node(),
      Node::GetterProp(node) => node.into_node(),
      Node::CondExpr(node) => node.into_node(),
      Node::ImportNamedSpecifier(node) => node.into_node(),
      Node::NamedExport(node) => node.into_node(),
      Node::JSXElement(node) => node.into_node(),
      Node::ClassDecl(node) => node.into_node(),
      Node::ArrayPat(node) => node.into_node(),
      Node::DoWhileStmt(node) => node.into_node(),
      Node::JSXText(node) => node.into_node(),
      Node::VarDecl(node) => node.into_node(),
      Node::PrivateName(node) => node.into_node(),
      Node::JSXNamespacedName(node) => node.into_node(),
      Node::JSXOpeningElement(node) => node.into_node(),
      Node::SpreadElement(node) => node.into_node(),
      Node::ExportDefaultDecl(node) => node.into_node(),
      Node::ArrowExpr(node) => node.into_node(),
      Node::TsAsExpr(node) => node.into_node(),
      Node::KeyValuePatProp(node) => node.into_node(),
      Node::TsLitType(node) => node.into_node(),
      Node::AssignExpr(node) => node.into_node(),
      Node::ArrayLit(node) => node.into_node(),
      Node::Decorator(node) => node.into_node(),
      Node::Ident(node) => node.into_node(),
      Node::MethodProp(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Node::SwitchCase(node) => node.kind(),
      Node::ThrowStmt(node) => node.kind(),
      Node::JSXClosingFragment(node) => node.kind(),
      Node::BigInt(node) => node.kind(),
      Node::ExportDefaultSpecifier(node) => node.kind(),
      Node::TsTypeParam(node) => node.kind(),
      Node::WithStmt(node) => node.kind(),
      Node::Regex(node) => node.kind(),
      Node::TsMethodSignature(node) => node.kind(),
      Node::UpdateExpr(node) => node.kind(),
      Node::SetterProp(node) => node.kind(),
      Node::TaggedTpl(node) => node.kind(),
      Node::ExportAll(node) => node.kind(),
      Node::TsModuleBlock(node) => node.kind(),
      Node::SwitchStmt(node) => node.kind(),
      Node::TsEnumMember(node) => node.kind(),
      Node::TsIndexedAccessType(node) => node.kind(),
      Node::TsRestType(node) => node.kind(),
      Node::ExprStmt(node) => node.kind(),
      Node::TsOptionalType(node) => node.kind(),
      Node::Tpl(node) => node.kind(),
      Node::Invalid(node) => node.kind(),
      Node::ComputedPropName(node) => node.kind(),
      Node::TsFnType(node) => node.kind(),
      Node::BlockStmt(node) => node.kind(),
      Node::TsTypeAliasDecl(node) => node.kind(),
      Node::MemberExpr(node) => node.kind(),
      Node::Function(node) => node.kind(),
      Node::ImportDecl(node) => node.kind(),
      Node::TsTypePredicate(node) => node.kind(),
      Node::YieldExpr(node) => node.kind(),
      Node::KeyValueProp(node) => node.kind(),
      Node::Param(node) => node.kind(),
      Node::JSXFragment(node) => node.kind(),
      Node::ImportDefaultSpecifier(node) => node.kind(),
      Node::Number(node) => node.kind(),
      Node::JSXAttr(node) => node.kind(),
      Node::ParenExpr(node) => node.kind(),
      Node::Super(node) => node.kind(),
      Node::TsConstructorType(node) => node.kind(),
      Node::Class(node) => node.kind(),
      Node::RestPat(node) => node.kind(),
      Node::TsNamespaceExportDecl(node) => node.kind(),
      Node::JSXOpeningFragment(node) => node.kind(),
      Node::NewExpr(node) => node.kind(),
      Node::FnExpr(node) => node.kind(),
      Node::IfStmt(node) => node.kind(),
      Node::TsParenthesizedType(node) => node.kind(),
      Node::AssignPatProp(node) => node.kind(),
      Node::TsImportType(node) => node.kind(),
      Node::Bool(node) => node.kind(),
      Node::TsImportEqualsDecl(node) => node.kind(),
      Node::AssignProp(node) => node.kind(),
      Node::TsInterfaceDecl(node) => node.kind(),
      Node::JSXEmptyExpr(node) => node.kind(),
      Node::TsQualifiedName(node) => node.kind(),
      Node::ExportDecl(node) => node.kind(),
      Node::CatchClause(node) => node.kind(),
      Node::LabeledStmt(node) => node.kind(),
      Node::ContinueStmt(node) => node.kind(),
      Node::TsConstructSignatureDecl(node) => node.kind(),
      Node::TsEnumDecl(node) => node.kind(),
      Node::OptChainExpr(node) => node.kind(),
      Node::TsNamespaceDecl(node) => node.kind(),
      Node::SeqExpr(node) => node.kind(),
      Node::TsExternalModuleRef(node) => node.kind(),
      Node::TsTypeParamInstantiation(node) => node.kind(),
      Node::ReturnStmt(node) => node.kind(),
      Node::TsTplLitType(node) => node.kind(),
      Node::ExportDefaultExpr(node) => node.kind(),
      Node::TsCallSignatureDecl(node) => node.kind(),
      Node::AwaitExpr(node) => node.kind(),
      Node::ClassMethod(node) => node.kind(),
      Node::TsParamProp(node) => node.kind(),
      Node::ClassProp(node) => node.kind(),
      Node::TsTypeAnn(node) => node.kind(),
      Node::ForStmt(node) => node.kind(),
      Node::ObjectPat(node) => node.kind(),
      Node::TsTypeQuery(node) => node.kind(),
      Node::ThisExpr(node) => node.kind(),
      Node::DebuggerStmt(node) => node.kind(),
      Node::TsTypeParamDecl(node) => node.kind(),
      Node::TsTypeAssertion(node) => node.kind(),
      Node::TplElement(node) => node.kind(),
      Node::TsKeywordType(node) => node.kind(),
      Node::JSXSpreadChild(node) => node.kind(),
      Node::TsIntersectionType(node) => node.kind(),
      Node::MetaPropExpr(node) => node.kind(),
      Node::ExprOrSpread(node) => node.kind(),
      Node::TsArrayType(node) => node.kind(),
      Node::TsTypeRef(node) => node.kind(),
      Node::TsThisType(node) => node.kind(),
      Node::TryStmt(node) => node.kind(),
      Node::CallExpr(node) => node.kind(),
      Node::TsMappedType(node) => node.kind(),
      Node::JSXExprContainer(node) => node.kind(),
      Node::PrivateProp(node) => node.kind(),
      Node::TsExportAssignment(node) => node.kind(),
      Node::TsInterfaceBody(node) => node.kind(),
      Node::TsTupleElement(node) => node.kind(),
      Node::VarDeclarator(node) => node.kind(),
      Node::JSXMemberExpr(node) => node.kind(),
      Node::TsConstAssertion(node) => node.kind(),
      Node::ExportNamespaceSpecifier(node) => node.kind(),
      Node::ObjectLit(node) => node.kind(),
      Node::Module(node) => node.kind(),
      Node::TsIndexSignature(node) => node.kind(),
      Node::TsTypeCastExpr(node) => node.kind(),
      Node::TsTupleType(node) => node.kind(),
      Node::Null(node) => node.kind(),
      Node::TsTypeOperator(node) => node.kind(),
      Node::JSXClosingElement(node) => node.kind(),
      Node::BinExpr(node) => node.kind(),
      Node::UnaryExpr(node) => node.kind(),
      Node::TsPropertySignature(node) => node.kind(),
      Node::Constructor(node) => node.kind(),
      Node::FnDecl(node) => node.kind(),
      Node::TsNonNullExpr(node) => node.kind(),
      Node::ClassExpr(node) => node.kind(),
      Node::ForInStmt(node) => node.kind(),
      Node::EmptyStmt(node) => node.kind(),
      Node::WhileStmt(node) => node.kind(),
      Node::Str(node) => node.kind(),
      Node::TsExprWithTypeArgs(node) => node.kind(),
      Node::AssignPat(node) => node.kind(),
      Node::ExportNamedSpecifier(node) => node.kind(),
      Node::TsConditionalType(node) => node.kind(),
      Node::TsTypeLit(node) => node.kind(),
      Node::BreakStmt(node) => node.kind(),
      Node::ImportStarAsSpecifier(node) => node.kind(),
      Node::TsInferType(node) => node.kind(),
      Node::PrivateMethod(node) => node.kind(),
      Node::ForOfStmt(node) => node.kind(),
      Node::TsUnionType(node) => node.kind(),
      Node::TsModuleDecl(node) => node.kind(),
      Node::GetterProp(node) => node.kind(),
      Node::CondExpr(node) => node.kind(),
      Node::ImportNamedSpecifier(node) => node.kind(),
      Node::NamedExport(node) => node.kind(),
      Node::JSXElement(node) => node.kind(),
      Node::ClassDecl(node) => node.kind(),
      Node::ArrayPat(node) => node.kind(),
      Node::DoWhileStmt(node) => node.kind(),
      Node::JSXText(node) => node.kind(),
      Node::VarDecl(node) => node.kind(),
      Node::PrivateName(node) => node.kind(),
      Node::JSXNamespacedName(node) => node.kind(),
      Node::JSXOpeningElement(node) => node.kind(),
      Node::SpreadElement(node) => node.kind(),
      Node::ExportDefaultDecl(node) => node.kind(),
      Node::ArrowExpr(node) => node.kind(),
      Node::TsAsExpr(node) => node.kind(),
      Node::KeyValuePatProp(node) => node.kind(),
      Node::TsLitType(node) => node.kind(),
      Node::AssignExpr(node) => node.kind(),
      Node::ArrayLit(node) => node.kind(),
      Node::Decorator(node) => node.kind(),
      Node::Ident(node) => node.kind(),
      Node::MethodProp(node) => node.kind(),
    }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub enum NodeKind {
  SwitchCase,
  ThrowStmt,
  JSXClosingFragment,
  BigInt,
  ExportDefaultSpecifier,
  TsTypeParam,
  WithStmt,
  Regex,
  TsMethodSignature,
  UpdateExpr,
  SetterProp,
  TaggedTpl,
  ExportAll,
  TsModuleBlock,
  SwitchStmt,
  TsEnumMember,
  TsIndexedAccessType,
  TsRestType,
  ExprStmt,
  TsOptionalType,
  Tpl,
  Invalid,
  ComputedPropName,
  TsFnType,
  BlockStmt,
  TsTypeAliasDecl,
  MemberExpr,
  Function,
  ImportDecl,
  TsTypePredicate,
  YieldExpr,
  KeyValueProp,
  Param,
  JSXFragment,
  ImportDefaultSpecifier,
  Number,
  JSXAttr,
  ParenExpr,
  Super,
  TsConstructorType,
  Class,
  RestPat,
  TsNamespaceExportDecl,
  JSXOpeningFragment,
  NewExpr,
  FnExpr,
  IfStmt,
  TsParenthesizedType,
  AssignPatProp,
  TsImportType,
  Bool,
  TsImportEqualsDecl,
  AssignProp,
  TsInterfaceDecl,
  JSXEmptyExpr,
  TsQualifiedName,
  ExportDecl,
  CatchClause,
  LabeledStmt,
  ContinueStmt,
  TsConstructSignatureDecl,
  TsEnumDecl,
  OptChainExpr,
  TsNamespaceDecl,
  SeqExpr,
  TsExternalModuleRef,
  TsTypeParamInstantiation,
  ReturnStmt,
  TsTplLitType,
  ExportDefaultExpr,
  TsCallSignatureDecl,
  AwaitExpr,
  ClassMethod,
  TsParamProp,
  ClassProp,
  TsTypeAnn,
  ForStmt,
  ObjectPat,
  TsTypeQuery,
  ThisExpr,
  DebuggerStmt,
  TsTypeParamDecl,
  TsTypeAssertion,
  TplElement,
  TsKeywordType,
  JSXSpreadChild,
  TsIntersectionType,
  MetaPropExpr,
  ExprOrSpread,
  TsArrayType,
  TsTypeRef,
  TsThisType,
  TryStmt,
  CallExpr,
  TsMappedType,
  JSXExprContainer,
  PrivateProp,
  TsExportAssignment,
  TsInterfaceBody,
  TsTupleElement,
  VarDeclarator,
  JSXMemberExpr,
  TsConstAssertion,
  ExportNamespaceSpecifier,
  ObjectLit,
  Module,
  TsIndexSignature,
  TsTypeCastExpr,
  TsTupleType,
  Null,
  TsTypeOperator,
  JSXClosingElement,
  BinExpr,
  UnaryExpr,
  TsPropertySignature,
  Constructor,
  FnDecl,
  TsNonNullExpr,
  ClassExpr,
  ForInStmt,
  EmptyStmt,
  WhileStmt,
  Str,
  TsExprWithTypeArgs,
  AssignPat,
  ExportNamedSpecifier,
  TsConditionalType,
  TsTypeLit,
  BreakStmt,
  ImportStarAsSpecifier,
  TsInferType,
  PrivateMethod,
  ForOfStmt,
  TsUnionType,
  TsModuleDecl,
  GetterProp,
  CondExpr,
  ImportNamedSpecifier,
  NamedExport,
  JSXElement,
  ClassDecl,
  ArrayPat,
  DoWhileStmt,
  JSXText,
  VarDecl,
  PrivateName,
  JSXNamespacedName,
  JSXOpeningElement,
  SpreadElement,
  ExportDefaultDecl,
  ArrowExpr,
  TsAsExpr,
  KeyValuePatProp,
  TsLitType,
  AssignExpr,
  ArrayLit,
  Decorator,
  Ident,
  MethodProp,
}


pub enum JSXAttrValue<'a> {
  Lit(Lit<'a>),
  JSXExprContainer(&'a JSXExprContainer<'a>),
  JSXElement(&'a JSXElement<'a>),
  JSXFragment(&'a JSXFragment<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      JSXAttrValue::Lit(node) => node.into_node(),
      JSXAttrValue::JSXExprContainer(node) => node.into_node(),
      JSXAttrValue::JSXElement(node) => node.into_node(),
      JSXAttrValue::JSXFragment(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXAttrValue::Lit(node) => node.kind(),
      JSXAttrValue::JSXExprContainer(node) => node.kind(),
      JSXAttrValue::JSXElement(node) => node.kind(),
      JSXAttrValue::JSXFragment(node) => node.kind(),
    }
  }
}
impl<'a> From<&JSXAttrValue<'a>> for Node<'a> {
  fn from(node: &JSXAttrValue<'a>) -> Node<'a> {
    match node {
      JSXAttrValue::Lit(node) => node.into(),
      JSXAttrValue::JSXExprContainer(node) => (*node).into(),
      JSXAttrValue::JSXElement(node) => (*node).into(),
      JSXAttrValue::JSXFragment(node) => (*node).into(),
    }
  }
}

fn get_view_for_jsxattr_value<'a>(inner: &'a swc_ast::JSXAttrValue, parent: Node<'a>, bump: &'a Bump) -> JSXAttrValue<'a> {
  match inner {
    swc_ast::JSXAttrValue::Lit(value) => JSXAttrValue::Lit(get_view_for_lit(value, parent, bump)),
    swc_ast::JSXAttrValue::JSXExprContainer(value) => JSXAttrValue::JSXExprContainer(get_view_for_jsxexpr_container(value, parent, bump)),
    swc_ast::JSXAttrValue::JSXElement(value) => JSXAttrValue::JSXElement(get_view_for_jsxelement(value, parent, bump)),
    swc_ast::JSXAttrValue::JSXFragment(value) => JSXAttrValue::JSXFragment(get_view_for_jsxfragment(value, parent, bump)),
  }
}


pub enum PropOrSpread<'a> {
  /// Spread properties, e.g., `{a: 1, ...obj, b: 2}`.
  Spread(&'a SpreadElement<'a>),
  Prop(Prop<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      PropOrSpread::Spread(node) => node.into_node(),
      PropOrSpread::Prop(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      PropOrSpread::Spread(node) => node.kind(),
      PropOrSpread::Prop(node) => node.kind(),
    }
  }
}
impl<'a> From<&PropOrSpread<'a>> for Node<'a> {
  fn from(node: &PropOrSpread<'a>) -> Node<'a> {
    match node {
      PropOrSpread::Spread(node) => (*node).into(),
      PropOrSpread::Prop(node) => node.into(),
    }
  }
}

fn get_view_for_prop_or_spread<'a>(inner: &'a swc_ast::PropOrSpread, parent: Node<'a>, bump: &'a Bump) -> PropOrSpread<'a> {
  match inner {
    swc_ast::PropOrSpread::Spread(value) => PropOrSpread::Spread(get_view_for_spread_element(value, parent, bump)),
    swc_ast::PropOrSpread::Prop(value) => PropOrSpread::Prop(get_view_for_prop(value, parent, bump)),
  }
}


pub enum VarDeclOrExpr<'a> {
  VarDecl(&'a VarDecl<'a>),
  Expr(Expr<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.into_node(),
      VarDeclOrExpr::Expr(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.kind(),
      VarDeclOrExpr::Expr(node) => node.kind(),
    }
  }
}
impl<'a> From<&VarDeclOrExpr<'a>> for Node<'a> {
  fn from(node: &VarDeclOrExpr<'a>) -> Node<'a> {
    match node {
      VarDeclOrExpr::VarDecl(node) => (*node).into(),
      VarDeclOrExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_var_decl_or_expr<'a>(inner: &'a swc_ast::VarDeclOrExpr, parent: Node<'a>, bump: &'a Bump) -> VarDeclOrExpr<'a> {
  match inner {
    swc_ast::VarDeclOrExpr::VarDecl(value) => VarDeclOrExpr::VarDecl(get_view_for_var_decl(value, parent, bump)),
    swc_ast::VarDeclOrExpr::Expr(value) => VarDeclOrExpr::Expr(get_view_for_expr(value, parent, bump)),
  }
}


pub enum TsThisTypeOrIdent<'a> {
  TsThisType(&'a TsThisType<'a>),
  Ident(&'a Ident<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.into_node(),
      TsThisTypeOrIdent::Ident(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.kind(),
      TsThisTypeOrIdent::Ident(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsThisTypeOrIdent<'a>> for Node<'a> {
  fn from(node: &TsThisTypeOrIdent<'a>) -> Node<'a> {
    match node {
      TsThisTypeOrIdent::TsThisType(node) => (*node).into(),
      TsThisTypeOrIdent::Ident(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_this_type_or_ident<'a>(inner: &'a swc_ast::TsThisTypeOrIdent, parent: Node<'a>, bump: &'a Bump) -> TsThisTypeOrIdent<'a> {
  match inner {
    swc_ast::TsThisTypeOrIdent::TsThisType(value) => TsThisTypeOrIdent::TsThisType(get_view_for_ts_this_type(value, parent, bump)),
    swc_ast::TsThisTypeOrIdent::Ident(value) => TsThisTypeOrIdent::Ident(get_view_for_ident(value, parent, bump)),
  }
}


pub enum Prop<'a> {
  /// `a` in `{ a, }`
  Shorthand(&'a Ident<'a>),
  /// `key: value` in `{ key: value, }`
  KeyValue(&'a KeyValueProp<'a>),
  /// This is **invalid** for object literal.
  Assign(&'a AssignProp<'a>),
  Getter(&'a GetterProp<'a>),
  Setter(&'a SetterProp<'a>),
  Method(&'a MethodProp<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      Prop::Shorthand(node) => node.into_node(),
      Prop::KeyValue(node) => node.into_node(),
      Prop::Assign(node) => node.into_node(),
      Prop::Getter(node) => node.into_node(),
      Prop::Setter(node) => node.into_node(),
      Prop::Method(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Prop::Shorthand(node) => node.kind(),
      Prop::KeyValue(node) => node.kind(),
      Prop::Assign(node) => node.kind(),
      Prop::Getter(node) => node.kind(),
      Prop::Setter(node) => node.kind(),
      Prop::Method(node) => node.kind(),
    }
  }
}
impl<'a> From<&Prop<'a>> for Node<'a> {
  fn from(node: &Prop<'a>) -> Node<'a> {
    match node {
      Prop::Shorthand(node) => (*node).into(),
      Prop::KeyValue(node) => (*node).into(),
      Prop::Assign(node) => (*node).into(),
      Prop::Getter(node) => (*node).into(),
      Prop::Setter(node) => (*node).into(),
      Prop::Method(node) => (*node).into(),
    }
  }
}

fn get_view_for_prop<'a>(inner: &'a swc_ast::Prop, parent: Node<'a>, bump: &'a Bump) -> Prop<'a> {
  match inner {
    swc_ast::Prop::Shorthand(value) => Prop::Shorthand(get_view_for_ident(value, parent, bump)),
    swc_ast::Prop::KeyValue(value) => Prop::KeyValue(get_view_for_key_value_prop(value, parent, bump)),
    swc_ast::Prop::Assign(value) => Prop::Assign(get_view_for_assign_prop(value, parent, bump)),
    swc_ast::Prop::Getter(value) => Prop::Getter(get_view_for_getter_prop(value, parent, bump)),
    swc_ast::Prop::Setter(value) => Prop::Setter(get_view_for_setter_prop(value, parent, bump)),
    swc_ast::Prop::Method(value) => Prop::Method(get_view_for_method_prop(value, parent, bump)),
  }
}


pub enum TsTypeQueryExpr<'a> {
  TsEntityName(TsEntityName<'a>),
  Import(&'a TsImportType<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.into_node(),
      TsTypeQueryExpr::Import(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.kind(),
      TsTypeQueryExpr::Import(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsTypeQueryExpr<'a>> for Node<'a> {
  fn from(node: &TsTypeQueryExpr<'a>) -> Node<'a> {
    match node {
      TsTypeQueryExpr::TsEntityName(node) => node.into(),
      TsTypeQueryExpr::Import(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_type_query_expr<'a>(inner: &'a swc_ast::TsTypeQueryExpr, parent: Node<'a>, bump: &'a Bump) -> TsTypeQueryExpr<'a> {
  match inner {
    swc_ast::TsTypeQueryExpr::TsEntityName(value) => TsTypeQueryExpr::TsEntityName(get_view_for_ts_entity_name(value, parent, bump)),
    swc_ast::TsTypeQueryExpr::Import(value) => TsTypeQueryExpr::Import(get_view_for_ts_import_type(value, parent, bump)),
  }
}


/// `namespace A.B { }` is a namespace named `A` with another TsNamespaceDecl as
/// its body.
pub enum TsNamespaceBody<'a> {
  TsModuleBlock(&'a TsModuleBlock<'a>),
  TsNamespaceDecl(&'a TsNamespaceDecl<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.into_node(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.kind(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsNamespaceBody<'a>> for Node<'a> {
  fn from(node: &TsNamespaceBody<'a>) -> Node<'a> {
    match node {
      TsNamespaceBody::TsModuleBlock(node) => (*node).into(),
      TsNamespaceBody::TsNamespaceDecl(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_namespace_body<'a>(inner: &'a swc_ast::TsNamespaceBody, parent: Node<'a>, bump: &'a Bump) -> TsNamespaceBody<'a> {
  match inner {
    swc_ast::TsNamespaceBody::TsModuleBlock(value) => TsNamespaceBody::TsModuleBlock(get_view_for_ts_module_block(value, parent, bump)),
    swc_ast::TsNamespaceBody::TsNamespaceDecl(value) => TsNamespaceBody::TsNamespaceDecl(get_view_for_ts_namespace_decl(value, parent, bump)),
  }
}


pub enum Lit<'a> {
  Str(&'a Str<'a>),
  Bool(&'a Bool<'a>),
  Null(&'a Null<'a>),
  Num(&'a Number<'a>),
  BigInt(&'a BigInt<'a>),
  Regex(&'a Regex<'a>),
  JSXText(&'a JSXText<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      Lit::Str(node) => node.into_node(),
      Lit::Bool(node) => node.into_node(),
      Lit::Null(node) => node.into_node(),
      Lit::Num(node) => node.into_node(),
      Lit::BigInt(node) => node.into_node(),
      Lit::Regex(node) => node.into_node(),
      Lit::JSXText(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Lit::Str(node) => node.kind(),
      Lit::Bool(node) => node.kind(),
      Lit::Null(node) => node.kind(),
      Lit::Num(node) => node.kind(),
      Lit::BigInt(node) => node.kind(),
      Lit::Regex(node) => node.kind(),
      Lit::JSXText(node) => node.kind(),
    }
  }
}
impl<'a> From<&Lit<'a>> for Node<'a> {
  fn from(node: &Lit<'a>) -> Node<'a> {
    match node {
      Lit::Str(node) => (*node).into(),
      Lit::Bool(node) => (*node).into(),
      Lit::Null(node) => (*node).into(),
      Lit::Num(node) => (*node).into(),
      Lit::BigInt(node) => (*node).into(),
      Lit::Regex(node) => (*node).into(),
      Lit::JSXText(node) => (*node).into(),
    }
  }
}

fn get_view_for_lit<'a>(inner: &'a swc_ast::Lit, parent: Node<'a>, bump: &'a Bump) -> Lit<'a> {
  match inner {
    swc_ast::Lit::Str(value) => Lit::Str(get_view_for_str(value, parent, bump)),
    swc_ast::Lit::Bool(value) => Lit::Bool(get_view_for_bool(value, parent, bump)),
    swc_ast::Lit::Null(value) => Lit::Null(get_view_for_null(value, parent, bump)),
    swc_ast::Lit::Num(value) => Lit::Num(get_view_for_number(value, parent, bump)),
    swc_ast::Lit::BigInt(value) => Lit::BigInt(get_view_for_big_int(value, parent, bump)),
    swc_ast::Lit::Regex(value) => Lit::Regex(get_view_for_regex(value, parent, bump)),
    swc_ast::Lit::JSXText(value) => Lit::JSXText(get_view_for_jsxtext(value, parent, bump)),
  }
}


pub enum ImportSpecifier<'a> {
  Named(&'a ImportNamedSpecifier<'a>),
  Default(&'a ImportDefaultSpecifier<'a>),
  Namespace(&'a ImportStarAsSpecifier<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      ImportSpecifier::Named(node) => node.into_node(),
      ImportSpecifier::Default(node) => node.into_node(),
      ImportSpecifier::Namespace(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ImportSpecifier::Named(node) => node.kind(),
      ImportSpecifier::Default(node) => node.kind(),
      ImportSpecifier::Namespace(node) => node.kind(),
    }
  }
}
impl<'a> From<&ImportSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportSpecifier<'a>) -> Node<'a> {
    match node {
      ImportSpecifier::Named(node) => (*node).into(),
      ImportSpecifier::Default(node) => (*node).into(),
      ImportSpecifier::Namespace(node) => (*node).into(),
    }
  }
}

fn get_view_for_import_specifier<'a>(inner: &'a swc_ast::ImportSpecifier, parent: Node<'a>, bump: &'a Bump) -> ImportSpecifier<'a> {
  match inner {
    swc_ast::ImportSpecifier::Named(value) => ImportSpecifier::Named(get_view_for_import_named_specifier(value, parent, bump)),
    swc_ast::ImportSpecifier::Default(value) => ImportSpecifier::Default(get_view_for_import_default_specifier(value, parent, bump)),
    swc_ast::ImportSpecifier::Namespace(value) => ImportSpecifier::Namespace(get_view_for_import_star_as_specifier(value, parent, bump)),
  }
}


pub enum ExportSpecifier<'a> {
  Namespace(&'a ExportNamespaceSpecifier<'a>),
  Default(&'a ExportDefaultSpecifier<'a>),
  Named(&'a ExportNamedSpecifier<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      ExportSpecifier::Namespace(node) => node.into_node(),
      ExportSpecifier::Default(node) => node.into_node(),
      ExportSpecifier::Named(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ExportSpecifier::Namespace(node) => node.kind(),
      ExportSpecifier::Default(node) => node.kind(),
      ExportSpecifier::Named(node) => node.kind(),
    }
  }
}
impl<'a> From<&ExportSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportSpecifier<'a>) -> Node<'a> {
    match node {
      ExportSpecifier::Namespace(node) => (*node).into(),
      ExportSpecifier::Default(node) => (*node).into(),
      ExportSpecifier::Named(node) => (*node).into(),
    }
  }
}

fn get_view_for_export_specifier<'a>(inner: &'a swc_ast::ExportSpecifier, parent: Node<'a>, bump: &'a Bump) -> ExportSpecifier<'a> {
  match inner {
    swc_ast::ExportSpecifier::Namespace(value) => ExportSpecifier::Namespace(get_view_for_export_namespace_specifier(value, parent, bump)),
    swc_ast::ExportSpecifier::Default(value) => ExportSpecifier::Default(get_view_for_export_default_specifier(value, parent, bump)),
    swc_ast::ExportSpecifier::Named(value) => ExportSpecifier::Named(get_view_for_export_named_specifier(value, parent, bump)),
  }
}


pub enum Stmt<'a> {
  Block(&'a BlockStmt<'a>),
  Empty(&'a EmptyStmt<'a>),
  Debugger(&'a DebuggerStmt<'a>),
  With(&'a WithStmt<'a>),
  Return(&'a ReturnStmt<'a>),
  Labeled(&'a LabeledStmt<'a>),
  Break(&'a BreakStmt<'a>),
  Continue(&'a ContinueStmt<'a>),
  If(&'a IfStmt<'a>),
  Switch(&'a SwitchStmt<'a>),
  Throw(&'a ThrowStmt<'a>),
  /// A try statement. If handler is null then finalizer must be a BlockStmt.
  Try(&'a TryStmt<'a>),
  While(&'a WhileStmt<'a>),
  DoWhile(&'a DoWhileStmt<'a>),
  For(&'a ForStmt<'a>),
  ForIn(&'a ForInStmt<'a>),
  ForOf(&'a ForOfStmt<'a>),
  Decl(Decl<'a>),
  Expr(&'a ExprStmt<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      Stmt::Block(node) => node.into_node(),
      Stmt::Empty(node) => node.into_node(),
      Stmt::Debugger(node) => node.into_node(),
      Stmt::With(node) => node.into_node(),
      Stmt::Return(node) => node.into_node(),
      Stmt::Labeled(node) => node.into_node(),
      Stmt::Break(node) => node.into_node(),
      Stmt::Continue(node) => node.into_node(),
      Stmt::If(node) => node.into_node(),
      Stmt::Switch(node) => node.into_node(),
      Stmt::Throw(node) => node.into_node(),
      Stmt::Try(node) => node.into_node(),
      Stmt::While(node) => node.into_node(),
      Stmt::DoWhile(node) => node.into_node(),
      Stmt::For(node) => node.into_node(),
      Stmt::ForIn(node) => node.into_node(),
      Stmt::ForOf(node) => node.into_node(),
      Stmt::Decl(node) => node.into_node(),
      Stmt::Expr(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Stmt::Block(node) => node.kind(),
      Stmt::Empty(node) => node.kind(),
      Stmt::Debugger(node) => node.kind(),
      Stmt::With(node) => node.kind(),
      Stmt::Return(node) => node.kind(),
      Stmt::Labeled(node) => node.kind(),
      Stmt::Break(node) => node.kind(),
      Stmt::Continue(node) => node.kind(),
      Stmt::If(node) => node.kind(),
      Stmt::Switch(node) => node.kind(),
      Stmt::Throw(node) => node.kind(),
      Stmt::Try(node) => node.kind(),
      Stmt::While(node) => node.kind(),
      Stmt::DoWhile(node) => node.kind(),
      Stmt::For(node) => node.kind(),
      Stmt::ForIn(node) => node.kind(),
      Stmt::ForOf(node) => node.kind(),
      Stmt::Decl(node) => node.kind(),
      Stmt::Expr(node) => node.kind(),
    }
  }
}
impl<'a> From<&Stmt<'a>> for Node<'a> {
  fn from(node: &Stmt<'a>) -> Node<'a> {
    match node {
      Stmt::Block(node) => (*node).into(),
      Stmt::Empty(node) => (*node).into(),
      Stmt::Debugger(node) => (*node).into(),
      Stmt::With(node) => (*node).into(),
      Stmt::Return(node) => (*node).into(),
      Stmt::Labeled(node) => (*node).into(),
      Stmt::Break(node) => (*node).into(),
      Stmt::Continue(node) => (*node).into(),
      Stmt::If(node) => (*node).into(),
      Stmt::Switch(node) => (*node).into(),
      Stmt::Throw(node) => (*node).into(),
      Stmt::Try(node) => (*node).into(),
      Stmt::While(node) => (*node).into(),
      Stmt::DoWhile(node) => (*node).into(),
      Stmt::For(node) => (*node).into(),
      Stmt::ForIn(node) => (*node).into(),
      Stmt::ForOf(node) => (*node).into(),
      Stmt::Decl(node) => node.into(),
      Stmt::Expr(node) => (*node).into(),
    }
  }
}

fn get_view_for_stmt<'a>(inner: &'a swc_ast::Stmt, parent: Node<'a>, bump: &'a Bump) -> Stmt<'a> {
  match inner {
    swc_ast::Stmt::Block(value) => Stmt::Block(get_view_for_block_stmt(value, parent, bump)),
    swc_ast::Stmt::Empty(value) => Stmt::Empty(get_view_for_empty_stmt(value, parent, bump)),
    swc_ast::Stmt::Debugger(value) => Stmt::Debugger(get_view_for_debugger_stmt(value, parent, bump)),
    swc_ast::Stmt::With(value) => Stmt::With(get_view_for_with_stmt(value, parent, bump)),
    swc_ast::Stmt::Return(value) => Stmt::Return(get_view_for_return_stmt(value, parent, bump)),
    swc_ast::Stmt::Labeled(value) => Stmt::Labeled(get_view_for_labeled_stmt(value, parent, bump)),
    swc_ast::Stmt::Break(value) => Stmt::Break(get_view_for_break_stmt(value, parent, bump)),
    swc_ast::Stmt::Continue(value) => Stmt::Continue(get_view_for_continue_stmt(value, parent, bump)),
    swc_ast::Stmt::If(value) => Stmt::If(get_view_for_if_stmt(value, parent, bump)),
    swc_ast::Stmt::Switch(value) => Stmt::Switch(get_view_for_switch_stmt(value, parent, bump)),
    swc_ast::Stmt::Throw(value) => Stmt::Throw(get_view_for_throw_stmt(value, parent, bump)),
    swc_ast::Stmt::Try(value) => Stmt::Try(get_view_for_try_stmt(value, parent, bump)),
    swc_ast::Stmt::While(value) => Stmt::While(get_view_for_while_stmt(value, parent, bump)),
    swc_ast::Stmt::DoWhile(value) => Stmt::DoWhile(get_view_for_do_while_stmt(value, parent, bump)),
    swc_ast::Stmt::For(value) => Stmt::For(get_view_for_for_stmt(value, parent, bump)),
    swc_ast::Stmt::ForIn(value) => Stmt::ForIn(get_view_for_for_in_stmt(value, parent, bump)),
    swc_ast::Stmt::ForOf(value) => Stmt::ForOf(get_view_for_for_of_stmt(value, parent, bump)),
    swc_ast::Stmt::Decl(value) => Stmt::Decl(get_view_for_decl(value, parent, bump)),
    swc_ast::Stmt::Expr(value) => Stmt::Expr(get_view_for_expr_stmt(value, parent, bump)),
  }
}


pub enum Pat<'a> {
  Ident(&'a Ident<'a>),
  Array(&'a ArrayPat<'a>),
  Rest(&'a RestPat<'a>),
  Object(&'a ObjectPat<'a>),
  Assign(&'a AssignPat<'a>),
  Invalid(&'a Invalid<'a>),
  /// Only for for-in / for-of loops. This is *syntatically* valid.
  Expr(Expr<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      Pat::Ident(node) => node.into_node(),
      Pat::Array(node) => node.into_node(),
      Pat::Rest(node) => node.into_node(),
      Pat::Object(node) => node.into_node(),
      Pat::Assign(node) => node.into_node(),
      Pat::Invalid(node) => node.into_node(),
      Pat::Expr(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Pat::Ident(node) => node.kind(),
      Pat::Array(node) => node.kind(),
      Pat::Rest(node) => node.kind(),
      Pat::Object(node) => node.kind(),
      Pat::Assign(node) => node.kind(),
      Pat::Invalid(node) => node.kind(),
      Pat::Expr(node) => node.kind(),
    }
  }
}
impl<'a> From<&Pat<'a>> for Node<'a> {
  fn from(node: &Pat<'a>) -> Node<'a> {
    match node {
      Pat::Ident(node) => (*node).into(),
      Pat::Array(node) => (*node).into(),
      Pat::Rest(node) => (*node).into(),
      Pat::Object(node) => (*node).into(),
      Pat::Assign(node) => (*node).into(),
      Pat::Invalid(node) => (*node).into(),
      Pat::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_pat<'a>(inner: &'a swc_ast::Pat, parent: Node<'a>, bump: &'a Bump) -> Pat<'a> {
  match inner {
    swc_ast::Pat::Ident(value) => Pat::Ident(get_view_for_ident(value, parent, bump)),
    swc_ast::Pat::Array(value) => Pat::Array(get_view_for_array_pat(value, parent, bump)),
    swc_ast::Pat::Rest(value) => Pat::Rest(get_view_for_rest_pat(value, parent, bump)),
    swc_ast::Pat::Object(value) => Pat::Object(get_view_for_object_pat(value, parent, bump)),
    swc_ast::Pat::Assign(value) => Pat::Assign(get_view_for_assign_pat(value, parent, bump)),
    swc_ast::Pat::Invalid(value) => Pat::Invalid(get_view_for_invalid(value, parent, bump)),
    swc_ast::Pat::Expr(value) => Pat::Expr(get_view_for_expr(value, parent, bump)),
  }
}


pub enum TsModuleName<'a> {
  Ident(&'a Ident<'a>),
  Str(&'a Str<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsModuleName::Ident(node) => node.into_node(),
      TsModuleName::Str(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsModuleName::Ident(node) => node.kind(),
      TsModuleName::Str(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsModuleName<'a>> for Node<'a> {
  fn from(node: &TsModuleName<'a>) -> Node<'a> {
    match node {
      TsModuleName::Ident(node) => (*node).into(),
      TsModuleName::Str(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_module_name<'a>(inner: &'a swc_ast::TsModuleName, parent: Node<'a>, bump: &'a Bump) -> TsModuleName<'a> {
  match inner {
    swc_ast::TsModuleName::Ident(value) => TsModuleName::Ident(get_view_for_ident(value, parent, bump)),
    swc_ast::TsModuleName::Str(value) => TsModuleName::Str(get_view_for_str(value, parent, bump)),
  }
}


pub enum TsFnParam<'a> {
  Ident(&'a Ident<'a>),
  Array(&'a ArrayPat<'a>),
  Rest(&'a RestPat<'a>),
  Object(&'a ObjectPat<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsFnParam::Ident(node) => node.into_node(),
      TsFnParam::Array(node) => node.into_node(),
      TsFnParam::Rest(node) => node.into_node(),
      TsFnParam::Object(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsFnParam::Ident(node) => node.kind(),
      TsFnParam::Array(node) => node.kind(),
      TsFnParam::Rest(node) => node.kind(),
      TsFnParam::Object(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsFnParam<'a>> for Node<'a> {
  fn from(node: &TsFnParam<'a>) -> Node<'a> {
    match node {
      TsFnParam::Ident(node) => (*node).into(),
      TsFnParam::Array(node) => (*node).into(),
      TsFnParam::Rest(node) => (*node).into(),
      TsFnParam::Object(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_fn_param<'a>(inner: &'a swc_ast::TsFnParam, parent: Node<'a>, bump: &'a Bump) -> TsFnParam<'a> {
  match inner {
    swc_ast::TsFnParam::Ident(value) => TsFnParam::Ident(get_view_for_ident(value, parent, bump)),
    swc_ast::TsFnParam::Array(value) => TsFnParam::Array(get_view_for_array_pat(value, parent, bump)),
    swc_ast::TsFnParam::Rest(value) => TsFnParam::Rest(get_view_for_rest_pat(value, parent, bump)),
    swc_ast::TsFnParam::Object(value) => TsFnParam::Object(get_view_for_object_pat(value, parent, bump)),
  }
}


pub enum ClassMember<'a> {
  Constructor(&'a Constructor<'a>),
  /// `es2015`
  Method(&'a ClassMethod<'a>),
  PrivateMethod(&'a PrivateMethod<'a>),
  /// stage 0 / Typescript
  ClassProp(&'a ClassProp<'a>),
  PrivateProp(&'a PrivateProp<'a>),
  TsIndexSignature(&'a TsIndexSignature<'a>),
  Empty(&'a EmptyStmt<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      ClassMember::Constructor(node) => node.into_node(),
      ClassMember::Method(node) => node.into_node(),
      ClassMember::PrivateMethod(node) => node.into_node(),
      ClassMember::ClassProp(node) => node.into_node(),
      ClassMember::PrivateProp(node) => node.into_node(),
      ClassMember::TsIndexSignature(node) => node.into_node(),
      ClassMember::Empty(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ClassMember::Constructor(node) => node.kind(),
      ClassMember::Method(node) => node.kind(),
      ClassMember::PrivateMethod(node) => node.kind(),
      ClassMember::ClassProp(node) => node.kind(),
      ClassMember::PrivateProp(node) => node.kind(),
      ClassMember::TsIndexSignature(node) => node.kind(),
      ClassMember::Empty(node) => node.kind(),
    }
  }
}
impl<'a> From<&ClassMember<'a>> for Node<'a> {
  fn from(node: &ClassMember<'a>) -> Node<'a> {
    match node {
      ClassMember::Constructor(node) => (*node).into(),
      ClassMember::Method(node) => (*node).into(),
      ClassMember::PrivateMethod(node) => (*node).into(),
      ClassMember::ClassProp(node) => (*node).into(),
      ClassMember::PrivateProp(node) => (*node).into(),
      ClassMember::TsIndexSignature(node) => (*node).into(),
      ClassMember::Empty(node) => (*node).into(),
    }
  }
}

fn get_view_for_class_member<'a>(inner: &'a swc_ast::ClassMember, parent: Node<'a>, bump: &'a Bump) -> ClassMember<'a> {
  match inner {
    swc_ast::ClassMember::Constructor(value) => ClassMember::Constructor(get_view_for_constructor(value, parent, bump)),
    swc_ast::ClassMember::Method(value) => ClassMember::Method(get_view_for_class_method(value, parent, bump)),
    swc_ast::ClassMember::PrivateMethod(value) => ClassMember::PrivateMethod(get_view_for_private_method(value, parent, bump)),
    swc_ast::ClassMember::ClassProp(value) => ClassMember::ClassProp(get_view_for_class_prop(value, parent, bump)),
    swc_ast::ClassMember::PrivateProp(value) => ClassMember::PrivateProp(get_view_for_private_prop(value, parent, bump)),
    swc_ast::ClassMember::TsIndexSignature(value) => ClassMember::TsIndexSignature(get_view_for_ts_index_signature(value, parent, bump)),
    swc_ast::ClassMember::Empty(value) => ClassMember::Empty(get_view_for_empty_stmt(value, parent, bump)),
  }
}


pub enum VarDeclOrPat<'a> {
  VarDecl(&'a VarDecl<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      VarDeclOrPat::VarDecl(node) => node.into_node(),
      VarDeclOrPat::Pat(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      VarDeclOrPat::VarDecl(node) => node.kind(),
      VarDeclOrPat::Pat(node) => node.kind(),
    }
  }
}
impl<'a> From<&VarDeclOrPat<'a>> for Node<'a> {
  fn from(node: &VarDeclOrPat<'a>) -> Node<'a> {
    match node {
      VarDeclOrPat::VarDecl(node) => (*node).into(),
      VarDeclOrPat::Pat(node) => node.into(),
    }
  }
}

fn get_view_for_var_decl_or_pat<'a>(inner: &'a swc_ast::VarDeclOrPat, parent: Node<'a>, bump: &'a Bump) -> VarDeclOrPat<'a> {
  match inner {
    swc_ast::VarDeclOrPat::VarDecl(value) => VarDeclOrPat::VarDecl(get_view_for_var_decl(value, parent, bump)),
    swc_ast::VarDeclOrPat::Pat(value) => VarDeclOrPat::Pat(get_view_for_pat(value, parent, bump)),
  }
}


pub enum TsModuleRef<'a> {
  TsEntityName(TsEntityName<'a>),
  TsExternalModuleRef(&'a TsExternalModuleRef<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsModuleRef::TsEntityName(node) => node.into_node(),
      TsModuleRef::TsExternalModuleRef(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsModuleRef::TsEntityName(node) => node.kind(),
      TsModuleRef::TsExternalModuleRef(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsModuleRef<'a>> for Node<'a> {
  fn from(node: &TsModuleRef<'a>) -> Node<'a> {
    match node {
      TsModuleRef::TsEntityName(node) => node.into(),
      TsModuleRef::TsExternalModuleRef(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_module_ref<'a>(inner: &'a swc_ast::TsModuleRef, parent: Node<'a>, bump: &'a Bump) -> TsModuleRef<'a> {
  match inner {
    swc_ast::TsModuleRef::TsEntityName(value) => TsModuleRef::TsEntityName(get_view_for_ts_entity_name(value, parent, bump)),
    swc_ast::TsModuleRef::TsExternalModuleRef(value) => TsModuleRef::TsExternalModuleRef(get_view_for_ts_external_module_ref(value, parent, bump)),
  }
}


pub enum JSXAttrOrSpread<'a> {
  JSXAttr(&'a JSXAttr<'a>),
  SpreadElement(&'a SpreadElement<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.into_node(),
      JSXAttrOrSpread::SpreadElement(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.kind(),
      JSXAttrOrSpread::SpreadElement(node) => node.kind(),
    }
  }
}
impl<'a> From<&JSXAttrOrSpread<'a>> for Node<'a> {
  fn from(node: &JSXAttrOrSpread<'a>) -> Node<'a> {
    match node {
      JSXAttrOrSpread::JSXAttr(node) => (*node).into(),
      JSXAttrOrSpread::SpreadElement(node) => (*node).into(),
    }
  }
}

fn get_view_for_jsxattr_or_spread<'a>(inner: &'a swc_ast::JSXAttrOrSpread, parent: Node<'a>, bump: &'a Bump) -> JSXAttrOrSpread<'a> {
  match inner {
    swc_ast::JSXAttrOrSpread::JSXAttr(value) => JSXAttrOrSpread::JSXAttr(get_view_for_jsxattr(value, parent, bump)),
    swc_ast::JSXAttrOrSpread::SpreadElement(value) => JSXAttrOrSpread::SpreadElement(get_view_for_spread_element(value, parent, bump)),
  }
}


pub enum ParamOrTsParamProp<'a> {
  TsParamProp(&'a TsParamProp<'a>),
  Param(&'a Param<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.into_node(),
      ParamOrTsParamProp::Param(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.kind(),
      ParamOrTsParamProp::Param(node) => node.kind(),
    }
  }
}
impl<'a> From<&ParamOrTsParamProp<'a>> for Node<'a> {
  fn from(node: &ParamOrTsParamProp<'a>) -> Node<'a> {
    match node {
      ParamOrTsParamProp::TsParamProp(node) => (*node).into(),
      ParamOrTsParamProp::Param(node) => (*node).into(),
    }
  }
}

fn get_view_for_param_or_ts_param_prop<'a>(inner: &'a swc_ast::ParamOrTsParamProp, parent: Node<'a>, bump: &'a Bump) -> ParamOrTsParamProp<'a> {
  match inner {
    swc_ast::ParamOrTsParamProp::TsParamProp(value) => ParamOrTsParamProp::TsParamProp(get_view_for_ts_param_prop(value, parent, bump)),
    swc_ast::ParamOrTsParamProp::Param(value) => ParamOrTsParamProp::Param(get_view_for_param(value, parent, bump)),
  }
}


pub enum ExprOrSuper<'a> {
  Super(&'a Super<'a>),
  Expr(Expr<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      ExprOrSuper::Super(node) => node.into_node(),
      ExprOrSuper::Expr(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ExprOrSuper::Super(node) => node.kind(),
      ExprOrSuper::Expr(node) => node.kind(),
    }
  }
}
impl<'a> From<&ExprOrSuper<'a>> for Node<'a> {
  fn from(node: &ExprOrSuper<'a>) -> Node<'a> {
    match node {
      ExprOrSuper::Super(node) => (*node).into(),
      ExprOrSuper::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_expr_or_super<'a>(inner: &'a swc_ast::ExprOrSuper, parent: Node<'a>, bump: &'a Bump) -> ExprOrSuper<'a> {
  match inner {
    swc_ast::ExprOrSuper::Super(value) => ExprOrSuper::Super(get_view_for_super(value, parent, bump)),
    swc_ast::ExprOrSuper::Expr(value) => ExprOrSuper::Expr(get_view_for_expr(value, parent, bump)),
  }
}


pub enum TsTypeElement<'a> {
  TsCallSignatureDecl(&'a TsCallSignatureDecl<'a>),
  TsConstructSignatureDecl(&'a TsConstructSignatureDecl<'a>),
  TsPropertySignature(&'a TsPropertySignature<'a>),
  TsMethodSignature(&'a TsMethodSignature<'a>),
  TsIndexSignature(&'a TsIndexSignature<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.into_node(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.into_node(),
      TsTypeElement::TsPropertySignature(node) => node.into_node(),
      TsTypeElement::TsMethodSignature(node) => node.into_node(),
      TsTypeElement::TsIndexSignature(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.kind(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.kind(),
      TsTypeElement::TsPropertySignature(node) => node.kind(),
      TsTypeElement::TsMethodSignature(node) => node.kind(),
      TsTypeElement::TsIndexSignature(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsTypeElement<'a>> for Node<'a> {
  fn from(node: &TsTypeElement<'a>) -> Node<'a> {
    match node {
      TsTypeElement::TsCallSignatureDecl(node) => (*node).into(),
      TsTypeElement::TsConstructSignatureDecl(node) => (*node).into(),
      TsTypeElement::TsPropertySignature(node) => (*node).into(),
      TsTypeElement::TsMethodSignature(node) => (*node).into(),
      TsTypeElement::TsIndexSignature(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_type_element<'a>(inner: &'a swc_ast::TsTypeElement, parent: Node<'a>, bump: &'a Bump) -> TsTypeElement<'a> {
  match inner {
    swc_ast::TsTypeElement::TsCallSignatureDecl(value) => TsTypeElement::TsCallSignatureDecl(get_view_for_ts_call_signature_decl(value, parent, bump)),
    swc_ast::TsTypeElement::TsConstructSignatureDecl(value) => TsTypeElement::TsConstructSignatureDecl(get_view_for_ts_construct_signature_decl(value, parent, bump)),
    swc_ast::TsTypeElement::TsPropertySignature(value) => TsTypeElement::TsPropertySignature(get_view_for_ts_property_signature(value, parent, bump)),
    swc_ast::TsTypeElement::TsMethodSignature(value) => TsTypeElement::TsMethodSignature(get_view_for_ts_method_signature(value, parent, bump)),
    swc_ast::TsTypeElement::TsIndexSignature(value) => TsTypeElement::TsIndexSignature(get_view_for_ts_index_signature(value, parent, bump)),
  }
}


pub enum BlockStmtOrExpr<'a> {
  BlockStmt(&'a BlockStmt<'a>),
  Expr(Expr<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.into_node(),
      BlockStmtOrExpr::Expr(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.kind(),
      BlockStmtOrExpr::Expr(node) => node.kind(),
    }
  }
}
impl<'a> From<&BlockStmtOrExpr<'a>> for Node<'a> {
  fn from(node: &BlockStmtOrExpr<'a>) -> Node<'a> {
    match node {
      BlockStmtOrExpr::BlockStmt(node) => (*node).into(),
      BlockStmtOrExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_block_stmt_or_expr<'a>(inner: &'a swc_ast::BlockStmtOrExpr, parent: Node<'a>, bump: &'a Bump) -> BlockStmtOrExpr<'a> {
  match inner {
    swc_ast::BlockStmtOrExpr::BlockStmt(value) => BlockStmtOrExpr::BlockStmt(get_view_for_block_stmt(value, parent, bump)),
    swc_ast::BlockStmtOrExpr::Expr(value) => BlockStmtOrExpr::Expr(get_view_for_expr(value, parent, bump)),
  }
}


pub enum TsUnionOrIntersectionType<'a> {
  TsUnionType(&'a TsUnionType<'a>),
  TsIntersectionType(&'a TsIntersectionType<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.into_node(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.kind(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsUnionOrIntersectionType<'a>> for Node<'a> {
  fn from(node: &TsUnionOrIntersectionType<'a>) -> Node<'a> {
    match node {
      TsUnionOrIntersectionType::TsUnionType(node) => (*node).into(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_union_or_intersection_type<'a>(inner: &'a swc_ast::TsUnionOrIntersectionType, parent: Node<'a>, bump: &'a Bump) -> TsUnionOrIntersectionType<'a> {
  match inner {
    swc_ast::TsUnionOrIntersectionType::TsUnionType(value) => TsUnionOrIntersectionType::TsUnionType(get_view_for_ts_union_type(value, parent, bump)),
    swc_ast::TsUnionOrIntersectionType::TsIntersectionType(value) => TsUnionOrIntersectionType::TsIntersectionType(get_view_for_ts_intersection_type(value, parent, bump)),
  }
}


pub enum DefaultDecl<'a> {
  Class(&'a ClassExpr<'a>),
  Fn(&'a FnExpr<'a>),
  TsInterfaceDecl(&'a TsInterfaceDecl<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      DefaultDecl::Class(node) => node.into_node(),
      DefaultDecl::Fn(node) => node.into_node(),
      DefaultDecl::TsInterfaceDecl(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      DefaultDecl::Class(node) => node.kind(),
      DefaultDecl::Fn(node) => node.kind(),
      DefaultDecl::TsInterfaceDecl(node) => node.kind(),
    }
  }
}
impl<'a> From<&DefaultDecl<'a>> for Node<'a> {
  fn from(node: &DefaultDecl<'a>) -> Node<'a> {
    match node {
      DefaultDecl::Class(node) => (*node).into(),
      DefaultDecl::Fn(node) => (*node).into(),
      DefaultDecl::TsInterfaceDecl(node) => (*node).into(),
    }
  }
}

fn get_view_for_default_decl<'a>(inner: &'a swc_ast::DefaultDecl, parent: Node<'a>, bump: &'a Bump) -> DefaultDecl<'a> {
  match inner {
    swc_ast::DefaultDecl::Class(value) => DefaultDecl::Class(get_view_for_class_expr(value, parent, bump)),
    swc_ast::DefaultDecl::Fn(value) => DefaultDecl::Fn(get_view_for_fn_expr(value, parent, bump)),
    swc_ast::DefaultDecl::TsInterfaceDecl(value) => DefaultDecl::TsInterfaceDecl(get_view_for_ts_interface_decl(value, parent, bump)),
  }
}


/// 
/// - Invalid: [Ident] with empty symbol.
pub enum TsEnumMemberId<'a> {
  Ident(&'a Ident<'a>),
  Str(&'a Str<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsEnumMemberId::Ident(node) => node.into_node(),
      TsEnumMemberId::Str(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsEnumMemberId::Ident(node) => node.kind(),
      TsEnumMemberId::Str(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsEnumMemberId<'a>> for Node<'a> {
  fn from(node: &TsEnumMemberId<'a>) -> Node<'a> {
    match node {
      TsEnumMemberId::Ident(node) => (*node).into(),
      TsEnumMemberId::Str(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_enum_member_id<'a>(inner: &'a swc_ast::TsEnumMemberId, parent: Node<'a>, bump: &'a Bump) -> TsEnumMemberId<'a> {
  match inner {
    swc_ast::TsEnumMemberId::Ident(value) => TsEnumMemberId::Ident(get_view_for_ident(value, parent, bump)),
    swc_ast::TsEnumMemberId::Str(value) => TsEnumMemberId::Str(get_view_for_str(value, parent, bump)),
  }
}


pub enum TsParamPropParam<'a> {
  Ident(&'a Ident<'a>),
  Assign(&'a AssignPat<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsParamPropParam::Ident(node) => node.into_node(),
      TsParamPropParam::Assign(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsParamPropParam::Ident(node) => node.kind(),
      TsParamPropParam::Assign(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsParamPropParam<'a>> for Node<'a> {
  fn from(node: &TsParamPropParam<'a>) -> Node<'a> {
    match node {
      TsParamPropParam::Ident(node) => (*node).into(),
      TsParamPropParam::Assign(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_param_prop_param<'a>(inner: &'a swc_ast::TsParamPropParam, parent: Node<'a>, bump: &'a Bump) -> TsParamPropParam<'a> {
  match inner {
    swc_ast::TsParamPropParam::Ident(value) => TsParamPropParam::Ident(get_view_for_ident(value, parent, bump)),
    swc_ast::TsParamPropParam::Assign(value) => TsParamPropParam::Assign(get_view_for_assign_pat(value, parent, bump)),
  }
}


pub enum JSXElementChild<'a> {
  JSXText(&'a JSXText<'a>),
  JSXExprContainer(&'a JSXExprContainer<'a>),
  JSXSpreadChild(&'a JSXSpreadChild<'a>),
  JSXElement(&'a JSXElement<'a>),
  JSXFragment(&'a JSXFragment<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      JSXElementChild::JSXText(node) => node.into_node(),
      JSXElementChild::JSXExprContainer(node) => node.into_node(),
      JSXElementChild::JSXSpreadChild(node) => node.into_node(),
      JSXElementChild::JSXElement(node) => node.into_node(),
      JSXElementChild::JSXFragment(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXElementChild::JSXText(node) => node.kind(),
      JSXElementChild::JSXExprContainer(node) => node.kind(),
      JSXElementChild::JSXSpreadChild(node) => node.kind(),
      JSXElementChild::JSXElement(node) => node.kind(),
      JSXElementChild::JSXFragment(node) => node.kind(),
    }
  }
}
impl<'a> From<&JSXElementChild<'a>> for Node<'a> {
  fn from(node: &JSXElementChild<'a>) -> Node<'a> {
    match node {
      JSXElementChild::JSXText(node) => (*node).into(),
      JSXElementChild::JSXExprContainer(node) => (*node).into(),
      JSXElementChild::JSXSpreadChild(node) => (*node).into(),
      JSXElementChild::JSXElement(node) => (*node).into(),
      JSXElementChild::JSXFragment(node) => (*node).into(),
    }
  }
}

fn get_view_for_jsxelement_child<'a>(inner: &'a swc_ast::JSXElementChild, parent: Node<'a>, bump: &'a Bump) -> JSXElementChild<'a> {
  match inner {
    swc_ast::JSXElementChild::JSXText(value) => JSXElementChild::JSXText(get_view_for_jsxtext(value, parent, bump)),
    swc_ast::JSXElementChild::JSXExprContainer(value) => JSXElementChild::JSXExprContainer(get_view_for_jsxexpr_container(value, parent, bump)),
    swc_ast::JSXElementChild::JSXSpreadChild(value) => JSXElementChild::JSXSpreadChild(get_view_for_jsxspread_child(value, parent, bump)),
    swc_ast::JSXElementChild::JSXElement(value) => JSXElementChild::JSXElement(get_view_for_jsxelement(value, parent, bump)),
    swc_ast::JSXElementChild::JSXFragment(value) => JSXElementChild::JSXFragment(get_view_for_jsxfragment(value, parent, bump)),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      ModuleItem::ModuleDecl(node) => node.into_node(),
      ModuleItem::Stmt(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ModuleItem::ModuleDecl(node) => node.kind(),
      ModuleItem::Stmt(node) => node.kind(),
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

fn get_view_for_module_item<'a>(inner: &'a swc_ast::ModuleItem, parent: Node<'a>, bump: &'a Bump) -> ModuleItem<'a> {
  match inner {
    swc_ast::ModuleItem::ModuleDecl(value) => ModuleItem::ModuleDecl(get_view_for_module_decl(value, parent, bump)),
    swc_ast::ModuleItem::Stmt(value) => ModuleItem::Stmt(get_view_for_stmt(value, parent, bump)),
  }
}


pub enum PropName<'a> {
  Ident(&'a Ident<'a>),
  /// String literal.
  Str(&'a Str<'a>),
  /// Numeric literal.
  Num(&'a Number<'a>),
  Computed(&'a ComputedPropName<'a>),
  BigInt(&'a BigInt<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      PropName::Ident(node) => node.into_node(),
      PropName::Str(node) => node.into_node(),
      PropName::Num(node) => node.into_node(),
      PropName::Computed(node) => node.into_node(),
      PropName::BigInt(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      PropName::Ident(node) => node.kind(),
      PropName::Str(node) => node.kind(),
      PropName::Num(node) => node.kind(),
      PropName::Computed(node) => node.kind(),
      PropName::BigInt(node) => node.kind(),
    }
  }
}
impl<'a> From<&PropName<'a>> for Node<'a> {
  fn from(node: &PropName<'a>) -> Node<'a> {
    match node {
      PropName::Ident(node) => (*node).into(),
      PropName::Str(node) => (*node).into(),
      PropName::Num(node) => (*node).into(),
      PropName::Computed(node) => (*node).into(),
      PropName::BigInt(node) => (*node).into(),
    }
  }
}

fn get_view_for_prop_name<'a>(inner: &'a swc_ast::PropName, parent: Node<'a>, bump: &'a Bump) -> PropName<'a> {
  match inner {
    swc_ast::PropName::Ident(value) => PropName::Ident(get_view_for_ident(value, parent, bump)),
    swc_ast::PropName::Str(value) => PropName::Str(get_view_for_str(value, parent, bump)),
    swc_ast::PropName::Num(value) => PropName::Num(get_view_for_number(value, parent, bump)),
    swc_ast::PropName::Computed(value) => PropName::Computed(get_view_for_computed_prop_name(value, parent, bump)),
    swc_ast::PropName::BigInt(value) => PropName::BigInt(get_view_for_big_int(value, parent, bump)),
  }
}


pub enum JSXAttrName<'a> {
  Ident(&'a Ident<'a>),
  JSXNamespacedName(&'a JSXNamespacedName<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      JSXAttrName::Ident(node) => node.into_node(),
      JSXAttrName::JSXNamespacedName(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXAttrName::Ident(node) => node.kind(),
      JSXAttrName::JSXNamespacedName(node) => node.kind(),
    }
  }
}
impl<'a> From<&JSXAttrName<'a>> for Node<'a> {
  fn from(node: &JSXAttrName<'a>) -> Node<'a> {
    match node {
      JSXAttrName::Ident(node) => (*node).into(),
      JSXAttrName::JSXNamespacedName(node) => (*node).into(),
    }
  }
}

fn get_view_for_jsxattr_name<'a>(inner: &'a swc_ast::JSXAttrName, parent: Node<'a>, bump: &'a Bump) -> JSXAttrName<'a> {
  match inner {
    swc_ast::JSXAttrName::Ident(value) => JSXAttrName::Ident(get_view_for_ident(value, parent, bump)),
    swc_ast::JSXAttrName::JSXNamespacedName(value) => JSXAttrName::JSXNamespacedName(get_view_for_jsxnamespaced_name(value, parent, bump)),
  }
}


pub enum Decl<'a> {
  Class(&'a ClassDecl<'a>),
  Fn(&'a FnDecl<'a>),
  Var(&'a VarDecl<'a>),
  TsInterface(&'a TsInterfaceDecl<'a>),
  TsTypeAlias(&'a TsTypeAliasDecl<'a>),
  TsEnum(&'a TsEnumDecl<'a>),
  TsModule(&'a TsModuleDecl<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      Decl::Class(node) => node.into_node(),
      Decl::Fn(node) => node.into_node(),
      Decl::Var(node) => node.into_node(),
      Decl::TsInterface(node) => node.into_node(),
      Decl::TsTypeAlias(node) => node.into_node(),
      Decl::TsEnum(node) => node.into_node(),
      Decl::TsModule(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Decl::Class(node) => node.kind(),
      Decl::Fn(node) => node.kind(),
      Decl::Var(node) => node.kind(),
      Decl::TsInterface(node) => node.kind(),
      Decl::TsTypeAlias(node) => node.kind(),
      Decl::TsEnum(node) => node.kind(),
      Decl::TsModule(node) => node.kind(),
    }
  }
}
impl<'a> From<&Decl<'a>> for Node<'a> {
  fn from(node: &Decl<'a>) -> Node<'a> {
    match node {
      Decl::Class(node) => (*node).into(),
      Decl::Fn(node) => (*node).into(),
      Decl::Var(node) => (*node).into(),
      Decl::TsInterface(node) => (*node).into(),
      Decl::TsTypeAlias(node) => (*node).into(),
      Decl::TsEnum(node) => (*node).into(),
      Decl::TsModule(node) => (*node).into(),
    }
  }
}

fn get_view_for_decl<'a>(inner: &'a swc_ast::Decl, parent: Node<'a>, bump: &'a Bump) -> Decl<'a> {
  match inner {
    swc_ast::Decl::Class(value) => Decl::Class(get_view_for_class_decl(value, parent, bump)),
    swc_ast::Decl::Fn(value) => Decl::Fn(get_view_for_fn_decl(value, parent, bump)),
    swc_ast::Decl::Var(value) => Decl::Var(get_view_for_var_decl(value, parent, bump)),
    swc_ast::Decl::TsInterface(value) => Decl::TsInterface(get_view_for_ts_interface_decl(value, parent, bump)),
    swc_ast::Decl::TsTypeAlias(value) => Decl::TsTypeAlias(get_view_for_ts_type_alias_decl(value, parent, bump)),
    swc_ast::Decl::TsEnum(value) => Decl::TsEnum(get_view_for_ts_enum_decl(value, parent, bump)),
    swc_ast::Decl::TsModule(value) => Decl::TsModule(get_view_for_ts_module_decl(value, parent, bump)),
  }
}


pub enum TsLit<'a> {
  Number(&'a Number<'a>),
  Str(&'a Str<'a>),
  Bool(&'a Bool<'a>),
  BigInt(&'a BigInt<'a>),
  Tpl(&'a TsTplLitType<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsLit::Number(node) => node.into_node(),
      TsLit::Str(node) => node.into_node(),
      TsLit::Bool(node) => node.into_node(),
      TsLit::BigInt(node) => node.into_node(),
      TsLit::Tpl(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsLit::Number(node) => node.kind(),
      TsLit::Str(node) => node.kind(),
      TsLit::Bool(node) => node.kind(),
      TsLit::BigInt(node) => node.kind(),
      TsLit::Tpl(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsLit<'a>> for Node<'a> {
  fn from(node: &TsLit<'a>) -> Node<'a> {
    match node {
      TsLit::Number(node) => (*node).into(),
      TsLit::Str(node) => (*node).into(),
      TsLit::Bool(node) => (*node).into(),
      TsLit::BigInt(node) => (*node).into(),
      TsLit::Tpl(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_lit<'a>(inner: &'a swc_ast::TsLit, parent: Node<'a>, bump: &'a Bump) -> TsLit<'a> {
  match inner {
    swc_ast::TsLit::Number(value) => TsLit::Number(get_view_for_number(value, parent, bump)),
    swc_ast::TsLit::Str(value) => TsLit::Str(get_view_for_str(value, parent, bump)),
    swc_ast::TsLit::Bool(value) => TsLit::Bool(get_view_for_bool(value, parent, bump)),
    swc_ast::TsLit::BigInt(value) => TsLit::BigInt(get_view_for_big_int(value, parent, bump)),
    swc_ast::TsLit::Tpl(value) => TsLit::Tpl(get_view_for_ts_tpl_lit_type(value, parent, bump)),
  }
}


pub enum TsEntityName<'a> {
  TsQualifiedName(&'a TsQualifiedName<'a>),
  Ident(&'a Ident<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsEntityName::TsQualifiedName(node) => node.into_node(),
      TsEntityName::Ident(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsEntityName::TsQualifiedName(node) => node.kind(),
      TsEntityName::Ident(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsEntityName<'a>> for Node<'a> {
  fn from(node: &TsEntityName<'a>) -> Node<'a> {
    match node {
      TsEntityName::TsQualifiedName(node) => (*node).into(),
      TsEntityName::Ident(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_entity_name<'a>(inner: &'a swc_ast::TsEntityName, parent: Node<'a>, bump: &'a Bump) -> TsEntityName<'a> {
  match inner {
    swc_ast::TsEntityName::TsQualifiedName(value) => TsEntityName::TsQualifiedName(get_view_for_ts_qualified_name(value, parent, bump)),
    swc_ast::TsEntityName::Ident(value) => TsEntityName::Ident(get_view_for_ident(value, parent, bump)),
  }
}


pub enum Expr<'a> {
  This(&'a ThisExpr<'a>),
  Array(&'a ArrayLit<'a>),
  Object(&'a ObjectLit<'a>),
  Fn(&'a FnExpr<'a>),
  Unary(&'a UnaryExpr<'a>),
  /// `++v`, `--v`, `v++`, `v--`
  Update(&'a UpdateExpr<'a>),
  Bin(&'a BinExpr<'a>),
  Assign(&'a AssignExpr<'a>),
  /// A member expression. If computed is true, the node corresponds to a
  /// computed (a[b]) member expression and property is an Expression. If
  /// computed is false, the node corresponds to a static (a.b) member
  /// expression and property is an Identifier.
  Member(&'a MemberExpr<'a>),
  /// true ? 'a' : 'b'
  Cond(&'a CondExpr<'a>),
  Call(&'a CallExpr<'a>),
  /// `new Cat()`
  New(&'a NewExpr<'a>),
  Seq(&'a SeqExpr<'a>),
  Ident(&'a Ident<'a>),
  Lit(Lit<'a>),
  Tpl(&'a Tpl<'a>),
  TaggedTpl(&'a TaggedTpl<'a>),
  Arrow(&'a ArrowExpr<'a>),
  Class(&'a ClassExpr<'a>),
  Yield(&'a YieldExpr<'a>),
  MetaProp(&'a MetaPropExpr<'a>),
  Await(&'a AwaitExpr<'a>),
  Paren(&'a ParenExpr<'a>),
  JSXMember(&'a JSXMemberExpr<'a>),
  JSXNamespacedName(&'a JSXNamespacedName<'a>),
  JSXEmpty(&'a JSXEmptyExpr<'a>),
  JSXElement(&'a JSXElement<'a>),
  JSXFragment(&'a JSXFragment<'a>),
  TsTypeAssertion(&'a TsTypeAssertion<'a>),
  TsConstAssertion(&'a TsConstAssertion<'a>),
  TsNonNull(&'a TsNonNullExpr<'a>),
  TsTypeCast(&'a TsTypeCastExpr<'a>),
  TsAs(&'a TsAsExpr<'a>),
  PrivateName(&'a PrivateName<'a>),
  OptChain(&'a OptChainExpr<'a>),
  Invalid(&'a Invalid<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      Expr::This(node) => node.into_node(),
      Expr::Array(node) => node.into_node(),
      Expr::Object(node) => node.into_node(),
      Expr::Fn(node) => node.into_node(),
      Expr::Unary(node) => node.into_node(),
      Expr::Update(node) => node.into_node(),
      Expr::Bin(node) => node.into_node(),
      Expr::Assign(node) => node.into_node(),
      Expr::Member(node) => node.into_node(),
      Expr::Cond(node) => node.into_node(),
      Expr::Call(node) => node.into_node(),
      Expr::New(node) => node.into_node(),
      Expr::Seq(node) => node.into_node(),
      Expr::Ident(node) => node.into_node(),
      Expr::Lit(node) => node.into_node(),
      Expr::Tpl(node) => node.into_node(),
      Expr::TaggedTpl(node) => node.into_node(),
      Expr::Arrow(node) => node.into_node(),
      Expr::Class(node) => node.into_node(),
      Expr::Yield(node) => node.into_node(),
      Expr::MetaProp(node) => node.into_node(),
      Expr::Await(node) => node.into_node(),
      Expr::Paren(node) => node.into_node(),
      Expr::JSXMember(node) => node.into_node(),
      Expr::JSXNamespacedName(node) => node.into_node(),
      Expr::JSXEmpty(node) => node.into_node(),
      Expr::JSXElement(node) => node.into_node(),
      Expr::JSXFragment(node) => node.into_node(),
      Expr::TsTypeAssertion(node) => node.into_node(),
      Expr::TsConstAssertion(node) => node.into_node(),
      Expr::TsNonNull(node) => node.into_node(),
      Expr::TsTypeCast(node) => node.into_node(),
      Expr::TsAs(node) => node.into_node(),
      Expr::PrivateName(node) => node.into_node(),
      Expr::OptChain(node) => node.into_node(),
      Expr::Invalid(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Expr::This(node) => node.kind(),
      Expr::Array(node) => node.kind(),
      Expr::Object(node) => node.kind(),
      Expr::Fn(node) => node.kind(),
      Expr::Unary(node) => node.kind(),
      Expr::Update(node) => node.kind(),
      Expr::Bin(node) => node.kind(),
      Expr::Assign(node) => node.kind(),
      Expr::Member(node) => node.kind(),
      Expr::Cond(node) => node.kind(),
      Expr::Call(node) => node.kind(),
      Expr::New(node) => node.kind(),
      Expr::Seq(node) => node.kind(),
      Expr::Ident(node) => node.kind(),
      Expr::Lit(node) => node.kind(),
      Expr::Tpl(node) => node.kind(),
      Expr::TaggedTpl(node) => node.kind(),
      Expr::Arrow(node) => node.kind(),
      Expr::Class(node) => node.kind(),
      Expr::Yield(node) => node.kind(),
      Expr::MetaProp(node) => node.kind(),
      Expr::Await(node) => node.kind(),
      Expr::Paren(node) => node.kind(),
      Expr::JSXMember(node) => node.kind(),
      Expr::JSXNamespacedName(node) => node.kind(),
      Expr::JSXEmpty(node) => node.kind(),
      Expr::JSXElement(node) => node.kind(),
      Expr::JSXFragment(node) => node.kind(),
      Expr::TsTypeAssertion(node) => node.kind(),
      Expr::TsConstAssertion(node) => node.kind(),
      Expr::TsNonNull(node) => node.kind(),
      Expr::TsTypeCast(node) => node.kind(),
      Expr::TsAs(node) => node.kind(),
      Expr::PrivateName(node) => node.kind(),
      Expr::OptChain(node) => node.kind(),
      Expr::Invalid(node) => node.kind(),
    }
  }
}
impl<'a> From<&Expr<'a>> for Node<'a> {
  fn from(node: &Expr<'a>) -> Node<'a> {
    match node {
      Expr::This(node) => (*node).into(),
      Expr::Array(node) => (*node).into(),
      Expr::Object(node) => (*node).into(),
      Expr::Fn(node) => (*node).into(),
      Expr::Unary(node) => (*node).into(),
      Expr::Update(node) => (*node).into(),
      Expr::Bin(node) => (*node).into(),
      Expr::Assign(node) => (*node).into(),
      Expr::Member(node) => (*node).into(),
      Expr::Cond(node) => (*node).into(),
      Expr::Call(node) => (*node).into(),
      Expr::New(node) => (*node).into(),
      Expr::Seq(node) => (*node).into(),
      Expr::Ident(node) => (*node).into(),
      Expr::Lit(node) => node.into(),
      Expr::Tpl(node) => (*node).into(),
      Expr::TaggedTpl(node) => (*node).into(),
      Expr::Arrow(node) => (*node).into(),
      Expr::Class(node) => (*node).into(),
      Expr::Yield(node) => (*node).into(),
      Expr::MetaProp(node) => (*node).into(),
      Expr::Await(node) => (*node).into(),
      Expr::Paren(node) => (*node).into(),
      Expr::JSXMember(node) => (*node).into(),
      Expr::JSXNamespacedName(node) => (*node).into(),
      Expr::JSXEmpty(node) => (*node).into(),
      Expr::JSXElement(node) => (*node).into(),
      Expr::JSXFragment(node) => (*node).into(),
      Expr::TsTypeAssertion(node) => (*node).into(),
      Expr::TsConstAssertion(node) => (*node).into(),
      Expr::TsNonNull(node) => (*node).into(),
      Expr::TsTypeCast(node) => (*node).into(),
      Expr::TsAs(node) => (*node).into(),
      Expr::PrivateName(node) => (*node).into(),
      Expr::OptChain(node) => (*node).into(),
      Expr::Invalid(node) => (*node).into(),
    }
  }
}

fn get_view_for_expr<'a>(inner: &'a swc_ast::Expr, parent: Node<'a>, bump: &'a Bump) -> Expr<'a> {
  match inner {
    swc_ast::Expr::This(value) => Expr::This(get_view_for_this_expr(value, parent, bump)),
    swc_ast::Expr::Array(value) => Expr::Array(get_view_for_array_lit(value, parent, bump)),
    swc_ast::Expr::Object(value) => Expr::Object(get_view_for_object_lit(value, parent, bump)),
    swc_ast::Expr::Fn(value) => Expr::Fn(get_view_for_fn_expr(value, parent, bump)),
    swc_ast::Expr::Unary(value) => Expr::Unary(get_view_for_unary_expr(value, parent, bump)),
    swc_ast::Expr::Update(value) => Expr::Update(get_view_for_update_expr(value, parent, bump)),
    swc_ast::Expr::Bin(value) => Expr::Bin(get_view_for_bin_expr(value, parent, bump)),
    swc_ast::Expr::Assign(value) => Expr::Assign(get_view_for_assign_expr(value, parent, bump)),
    swc_ast::Expr::Member(value) => Expr::Member(get_view_for_member_expr(value, parent, bump)),
    swc_ast::Expr::Cond(value) => Expr::Cond(get_view_for_cond_expr(value, parent, bump)),
    swc_ast::Expr::Call(value) => Expr::Call(get_view_for_call_expr(value, parent, bump)),
    swc_ast::Expr::New(value) => Expr::New(get_view_for_new_expr(value, parent, bump)),
    swc_ast::Expr::Seq(value) => Expr::Seq(get_view_for_seq_expr(value, parent, bump)),
    swc_ast::Expr::Ident(value) => Expr::Ident(get_view_for_ident(value, parent, bump)),
    swc_ast::Expr::Lit(value) => Expr::Lit(get_view_for_lit(value, parent, bump)),
    swc_ast::Expr::Tpl(value) => Expr::Tpl(get_view_for_tpl(value, parent, bump)),
    swc_ast::Expr::TaggedTpl(value) => Expr::TaggedTpl(get_view_for_tagged_tpl(value, parent, bump)),
    swc_ast::Expr::Arrow(value) => Expr::Arrow(get_view_for_arrow_expr(value, parent, bump)),
    swc_ast::Expr::Class(value) => Expr::Class(get_view_for_class_expr(value, parent, bump)),
    swc_ast::Expr::Yield(value) => Expr::Yield(get_view_for_yield_expr(value, parent, bump)),
    swc_ast::Expr::MetaProp(value) => Expr::MetaProp(get_view_for_meta_prop_expr(value, parent, bump)),
    swc_ast::Expr::Await(value) => Expr::Await(get_view_for_await_expr(value, parent, bump)),
    swc_ast::Expr::Paren(value) => Expr::Paren(get_view_for_paren_expr(value, parent, bump)),
    swc_ast::Expr::JSXMember(value) => Expr::JSXMember(get_view_for_jsxmember_expr(value, parent, bump)),
    swc_ast::Expr::JSXNamespacedName(value) => Expr::JSXNamespacedName(get_view_for_jsxnamespaced_name(value, parent, bump)),
    swc_ast::Expr::JSXEmpty(value) => Expr::JSXEmpty(get_view_for_jsxempty_expr(value, parent, bump)),
    swc_ast::Expr::JSXElement(value) => Expr::JSXElement(get_view_for_jsxelement(value, parent, bump)),
    swc_ast::Expr::JSXFragment(value) => Expr::JSXFragment(get_view_for_jsxfragment(value, parent, bump)),
    swc_ast::Expr::TsTypeAssertion(value) => Expr::TsTypeAssertion(get_view_for_ts_type_assertion(value, parent, bump)),
    swc_ast::Expr::TsConstAssertion(value) => Expr::TsConstAssertion(get_view_for_ts_const_assertion(value, parent, bump)),
    swc_ast::Expr::TsNonNull(value) => Expr::TsNonNull(get_view_for_ts_non_null_expr(value, parent, bump)),
    swc_ast::Expr::TsTypeCast(value) => Expr::TsTypeCast(get_view_for_ts_type_cast_expr(value, parent, bump)),
    swc_ast::Expr::TsAs(value) => Expr::TsAs(get_view_for_ts_as_expr(value, parent, bump)),
    swc_ast::Expr::PrivateName(value) => Expr::PrivateName(get_view_for_private_name(value, parent, bump)),
    swc_ast::Expr::OptChain(value) => Expr::OptChain(get_view_for_opt_chain_expr(value, parent, bump)),
    swc_ast::Expr::Invalid(value) => Expr::Invalid(get_view_for_invalid(value, parent, bump)),
  }
}


/// Used for `obj` property of `JSXMemberExpr`.
pub enum JSXObject<'a> {
  JSXMemberExpr(&'a JSXMemberExpr<'a>),
  Ident(&'a Ident<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      JSXObject::JSXMemberExpr(node) => node.into_node(),
      JSXObject::Ident(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXObject::JSXMemberExpr(node) => node.kind(),
      JSXObject::Ident(node) => node.kind(),
    }
  }
}
impl<'a> From<&JSXObject<'a>> for Node<'a> {
  fn from(node: &JSXObject<'a>) -> Node<'a> {
    match node {
      JSXObject::JSXMemberExpr(node) => (*node).into(),
      JSXObject::Ident(node) => (*node).into(),
    }
  }
}

fn get_view_for_jsxobject<'a>(inner: &'a swc_ast::JSXObject, parent: Node<'a>, bump: &'a Bump) -> JSXObject<'a> {
  match inner {
    swc_ast::JSXObject::JSXMemberExpr(value) => JSXObject::JSXMemberExpr(get_view_for_jsxmember_expr(value, parent, bump)),
    swc_ast::JSXObject::Ident(value) => JSXObject::Ident(get_view_for_ident(value, parent, bump)),
  }
}


pub enum PatOrExpr<'a> {
  Expr(Expr<'a>),
  Pat(Pat<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      PatOrExpr::Expr(node) => node.into_node(),
      PatOrExpr::Pat(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      PatOrExpr::Expr(node) => node.kind(),
      PatOrExpr::Pat(node) => node.kind(),
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

fn get_view_for_pat_or_expr<'a>(inner: &'a swc_ast::PatOrExpr, parent: Node<'a>, bump: &'a Bump) -> PatOrExpr<'a> {
  match inner {
    swc_ast::PatOrExpr::Expr(value) => PatOrExpr::Expr(get_view_for_expr(value, parent, bump)),
    swc_ast::PatOrExpr::Pat(value) => PatOrExpr::Pat(get_view_for_pat(value, parent, bump)),
  }
}


pub enum ModuleDecl<'a> {
  Import(&'a ImportDecl<'a>),
  ExportDecl(&'a ExportDecl<'a>),
  ExportNamed(&'a NamedExport<'a>),
  ExportDefaultDecl(&'a ExportDefaultDecl<'a>),
  ExportDefaultExpr(&'a ExportDefaultExpr<'a>),
  ExportAll(&'a ExportAll<'a>),
  TsImportEquals(&'a TsImportEqualsDecl<'a>),
  TsExportAssignment(&'a TsExportAssignment<'a>),
  TsNamespaceExport(&'a TsNamespaceExportDecl<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      ModuleDecl::Import(node) => node.into_node(),
      ModuleDecl::ExportDecl(node) => node.into_node(),
      ModuleDecl::ExportNamed(node) => node.into_node(),
      ModuleDecl::ExportDefaultDecl(node) => node.into_node(),
      ModuleDecl::ExportDefaultExpr(node) => node.into_node(),
      ModuleDecl::ExportAll(node) => node.into_node(),
      ModuleDecl::TsImportEquals(node) => node.into_node(),
      ModuleDecl::TsExportAssignment(node) => node.into_node(),
      ModuleDecl::TsNamespaceExport(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ModuleDecl::Import(node) => node.kind(),
      ModuleDecl::ExportDecl(node) => node.kind(),
      ModuleDecl::ExportNamed(node) => node.kind(),
      ModuleDecl::ExportDefaultDecl(node) => node.kind(),
      ModuleDecl::ExportDefaultExpr(node) => node.kind(),
      ModuleDecl::ExportAll(node) => node.kind(),
      ModuleDecl::TsImportEquals(node) => node.kind(),
      ModuleDecl::TsExportAssignment(node) => node.kind(),
      ModuleDecl::TsNamespaceExport(node) => node.kind(),
    }
  }
}
impl<'a> From<&ModuleDecl<'a>> for Node<'a> {
  fn from(node: &ModuleDecl<'a>) -> Node<'a> {
    match node {
      ModuleDecl::Import(node) => (*node).into(),
      ModuleDecl::ExportDecl(node) => (*node).into(),
      ModuleDecl::ExportNamed(node) => (*node).into(),
      ModuleDecl::ExportDefaultDecl(node) => (*node).into(),
      ModuleDecl::ExportDefaultExpr(node) => (*node).into(),
      ModuleDecl::ExportAll(node) => (*node).into(),
      ModuleDecl::TsImportEquals(node) => (*node).into(),
      ModuleDecl::TsExportAssignment(node) => (*node).into(),
      ModuleDecl::TsNamespaceExport(node) => (*node).into(),
    }
  }
}

fn get_view_for_module_decl<'a>(inner: &'a swc_ast::ModuleDecl, parent: Node<'a>, bump: &'a Bump) -> ModuleDecl<'a> {
  match inner {
    swc_ast::ModuleDecl::Import(value) => ModuleDecl::Import(get_view_for_import_decl(value, parent, bump)),
    swc_ast::ModuleDecl::ExportDecl(value) => ModuleDecl::ExportDecl(get_view_for_export_decl(value, parent, bump)),
    swc_ast::ModuleDecl::ExportNamed(value) => ModuleDecl::ExportNamed(get_view_for_named_export(value, parent, bump)),
    swc_ast::ModuleDecl::ExportDefaultDecl(value) => ModuleDecl::ExportDefaultDecl(get_view_for_export_default_decl(value, parent, bump)),
    swc_ast::ModuleDecl::ExportDefaultExpr(value) => ModuleDecl::ExportDefaultExpr(get_view_for_export_default_expr(value, parent, bump)),
    swc_ast::ModuleDecl::ExportAll(value) => ModuleDecl::ExportAll(get_view_for_export_all(value, parent, bump)),
    swc_ast::ModuleDecl::TsImportEquals(value) => ModuleDecl::TsImportEquals(get_view_for_ts_import_equals_decl(value, parent, bump)),
    swc_ast::ModuleDecl::TsExportAssignment(value) => ModuleDecl::TsExportAssignment(get_view_for_ts_export_assignment(value, parent, bump)),
    swc_ast::ModuleDecl::TsNamespaceExport(value) => ModuleDecl::TsNamespaceExport(get_view_for_ts_namespace_export_decl(value, parent, bump)),
  }
}


pub enum JSXElementName<'a> {
  Ident(&'a Ident<'a>),
  JSXMemberExpr(&'a JSXMemberExpr<'a>),
  JSXNamespacedName(&'a JSXNamespacedName<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      JSXElementName::Ident(node) => node.into_node(),
      JSXElementName::JSXMemberExpr(node) => node.into_node(),
      JSXElementName::JSXNamespacedName(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXElementName::Ident(node) => node.kind(),
      JSXElementName::JSXMemberExpr(node) => node.kind(),
      JSXElementName::JSXNamespacedName(node) => node.kind(),
    }
  }
}
impl<'a> From<&JSXElementName<'a>> for Node<'a> {
  fn from(node: &JSXElementName<'a>) -> Node<'a> {
    match node {
      JSXElementName::Ident(node) => (*node).into(),
      JSXElementName::JSXMemberExpr(node) => (*node).into(),
      JSXElementName::JSXNamespacedName(node) => (*node).into(),
    }
  }
}

fn get_view_for_jsxelement_name<'a>(inner: &'a swc_ast::JSXElementName, parent: Node<'a>, bump: &'a Bump) -> JSXElementName<'a> {
  match inner {
    swc_ast::JSXElementName::Ident(value) => JSXElementName::Ident(get_view_for_ident(value, parent, bump)),
    swc_ast::JSXElementName::JSXMemberExpr(value) => JSXElementName::JSXMemberExpr(get_view_for_jsxmember_expr(value, parent, bump)),
    swc_ast::JSXElementName::JSXNamespacedName(value) => JSXElementName::JSXNamespacedName(get_view_for_jsxnamespaced_name(value, parent, bump)),
  }
}


pub enum JSXExpr<'a> {
  JSXEmptyExpr(&'a JSXEmptyExpr<'a>),
  Expr(Expr<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.into_node(),
      JSXExpr::Expr(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.kind(),
      JSXExpr::Expr(node) => node.kind(),
    }
  }
}
impl<'a> From<&JSXExpr<'a>> for Node<'a> {
  fn from(node: &JSXExpr<'a>) -> Node<'a> {
    match node {
      JSXExpr::JSXEmptyExpr(node) => (*node).into(),
      JSXExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_jsxexpr<'a>(inner: &'a swc_ast::JSXExpr, parent: Node<'a>, bump: &'a Bump) -> JSXExpr<'a> {
  match inner {
    swc_ast::JSXExpr::JSXEmptyExpr(value) => JSXExpr::JSXEmptyExpr(get_view_for_jsxempty_expr(value, parent, bump)),
    swc_ast::JSXExpr::Expr(value) => JSXExpr::Expr(get_view_for_expr(value, parent, bump)),
  }
}


pub enum TsType<'a> {
  TsKeywordType(&'a TsKeywordType<'a>),
  TsThisType(&'a TsThisType<'a>),
  TsFnOrConstructorType(TsFnOrConstructorType<'a>),
  TsTypeRef(&'a TsTypeRef<'a>),
  TsTypeQuery(&'a TsTypeQuery<'a>),
  TsTypeLit(&'a TsTypeLit<'a>),
  TsArrayType(&'a TsArrayType<'a>),
  TsTupleType(&'a TsTupleType<'a>),
  TsOptionalType(&'a TsOptionalType<'a>),
  TsRestType(&'a TsRestType<'a>),
  TsUnionOrIntersectionType(TsUnionOrIntersectionType<'a>),
  TsConditionalType(&'a TsConditionalType<'a>),
  TsInferType(&'a TsInferType<'a>),
  TsParenthesizedType(&'a TsParenthesizedType<'a>),
  TsTypeOperator(&'a TsTypeOperator<'a>),
  TsIndexedAccessType(&'a TsIndexedAccessType<'a>),
  TsMappedType(&'a TsMappedType<'a>),
  TsLitType(&'a TsLitType<'a>),
  TsTypePredicate(&'a TsTypePredicate<'a>),
  TsImportType(&'a TsImportType<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsType::TsKeywordType(node) => node.into_node(),
      TsType::TsThisType(node) => node.into_node(),
      TsType::TsFnOrConstructorType(node) => node.into_node(),
      TsType::TsTypeRef(node) => node.into_node(),
      TsType::TsTypeQuery(node) => node.into_node(),
      TsType::TsTypeLit(node) => node.into_node(),
      TsType::TsArrayType(node) => node.into_node(),
      TsType::TsTupleType(node) => node.into_node(),
      TsType::TsOptionalType(node) => node.into_node(),
      TsType::TsRestType(node) => node.into_node(),
      TsType::TsUnionOrIntersectionType(node) => node.into_node(),
      TsType::TsConditionalType(node) => node.into_node(),
      TsType::TsInferType(node) => node.into_node(),
      TsType::TsParenthesizedType(node) => node.into_node(),
      TsType::TsTypeOperator(node) => node.into_node(),
      TsType::TsIndexedAccessType(node) => node.into_node(),
      TsType::TsMappedType(node) => node.into_node(),
      TsType::TsLitType(node) => node.into_node(),
      TsType::TsTypePredicate(node) => node.into_node(),
      TsType::TsImportType(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsType::TsKeywordType(node) => node.kind(),
      TsType::TsThisType(node) => node.kind(),
      TsType::TsFnOrConstructorType(node) => node.kind(),
      TsType::TsTypeRef(node) => node.kind(),
      TsType::TsTypeQuery(node) => node.kind(),
      TsType::TsTypeLit(node) => node.kind(),
      TsType::TsArrayType(node) => node.kind(),
      TsType::TsTupleType(node) => node.kind(),
      TsType::TsOptionalType(node) => node.kind(),
      TsType::TsRestType(node) => node.kind(),
      TsType::TsUnionOrIntersectionType(node) => node.kind(),
      TsType::TsConditionalType(node) => node.kind(),
      TsType::TsInferType(node) => node.kind(),
      TsType::TsParenthesizedType(node) => node.kind(),
      TsType::TsTypeOperator(node) => node.kind(),
      TsType::TsIndexedAccessType(node) => node.kind(),
      TsType::TsMappedType(node) => node.kind(),
      TsType::TsLitType(node) => node.kind(),
      TsType::TsTypePredicate(node) => node.kind(),
      TsType::TsImportType(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsType<'a>> for Node<'a> {
  fn from(node: &TsType<'a>) -> Node<'a> {
    match node {
      TsType::TsKeywordType(node) => (*node).into(),
      TsType::TsThisType(node) => (*node).into(),
      TsType::TsFnOrConstructorType(node) => node.into(),
      TsType::TsTypeRef(node) => (*node).into(),
      TsType::TsTypeQuery(node) => (*node).into(),
      TsType::TsTypeLit(node) => (*node).into(),
      TsType::TsArrayType(node) => (*node).into(),
      TsType::TsTupleType(node) => (*node).into(),
      TsType::TsOptionalType(node) => (*node).into(),
      TsType::TsRestType(node) => (*node).into(),
      TsType::TsUnionOrIntersectionType(node) => node.into(),
      TsType::TsConditionalType(node) => (*node).into(),
      TsType::TsInferType(node) => (*node).into(),
      TsType::TsParenthesizedType(node) => (*node).into(),
      TsType::TsTypeOperator(node) => (*node).into(),
      TsType::TsIndexedAccessType(node) => (*node).into(),
      TsType::TsMappedType(node) => (*node).into(),
      TsType::TsLitType(node) => (*node).into(),
      TsType::TsTypePredicate(node) => (*node).into(),
      TsType::TsImportType(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_type<'a>(inner: &'a swc_ast::TsType, parent: Node<'a>, bump: &'a Bump) -> TsType<'a> {
  match inner {
    swc_ast::TsType::TsKeywordType(value) => TsType::TsKeywordType(get_view_for_ts_keyword_type(value, parent, bump)),
    swc_ast::TsType::TsThisType(value) => TsType::TsThisType(get_view_for_ts_this_type(value, parent, bump)),
    swc_ast::TsType::TsFnOrConstructorType(value) => TsType::TsFnOrConstructorType(get_view_for_ts_fn_or_constructor_type(value, parent, bump)),
    swc_ast::TsType::TsTypeRef(value) => TsType::TsTypeRef(get_view_for_ts_type_ref(value, parent, bump)),
    swc_ast::TsType::TsTypeQuery(value) => TsType::TsTypeQuery(get_view_for_ts_type_query(value, parent, bump)),
    swc_ast::TsType::TsTypeLit(value) => TsType::TsTypeLit(get_view_for_ts_type_lit(value, parent, bump)),
    swc_ast::TsType::TsArrayType(value) => TsType::TsArrayType(get_view_for_ts_array_type(value, parent, bump)),
    swc_ast::TsType::TsTupleType(value) => TsType::TsTupleType(get_view_for_ts_tuple_type(value, parent, bump)),
    swc_ast::TsType::TsOptionalType(value) => TsType::TsOptionalType(get_view_for_ts_optional_type(value, parent, bump)),
    swc_ast::TsType::TsRestType(value) => TsType::TsRestType(get_view_for_ts_rest_type(value, parent, bump)),
    swc_ast::TsType::TsUnionOrIntersectionType(value) => TsType::TsUnionOrIntersectionType(get_view_for_ts_union_or_intersection_type(value, parent, bump)),
    swc_ast::TsType::TsConditionalType(value) => TsType::TsConditionalType(get_view_for_ts_conditional_type(value, parent, bump)),
    swc_ast::TsType::TsInferType(value) => TsType::TsInferType(get_view_for_ts_infer_type(value, parent, bump)),
    swc_ast::TsType::TsParenthesizedType(value) => TsType::TsParenthesizedType(get_view_for_ts_parenthesized_type(value, parent, bump)),
    swc_ast::TsType::TsTypeOperator(value) => TsType::TsTypeOperator(get_view_for_ts_type_operator(value, parent, bump)),
    swc_ast::TsType::TsIndexedAccessType(value) => TsType::TsIndexedAccessType(get_view_for_ts_indexed_access_type(value, parent, bump)),
    swc_ast::TsType::TsMappedType(value) => TsType::TsMappedType(get_view_for_ts_mapped_type(value, parent, bump)),
    swc_ast::TsType::TsLitType(value) => TsType::TsLitType(get_view_for_ts_lit_type(value, parent, bump)),
    swc_ast::TsType::TsTypePredicate(value) => TsType::TsTypePredicate(get_view_for_ts_type_predicate(value, parent, bump)),
    swc_ast::TsType::TsImportType(value) => TsType::TsImportType(get_view_for_ts_import_type(value, parent, bump)),
  }
}


pub enum ObjectPatProp<'a> {
  KeyValue(&'a KeyValuePatProp<'a>),
  Assign(&'a AssignPatProp<'a>),
  Rest(&'a RestPat<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      ObjectPatProp::KeyValue(node) => node.into_node(),
      ObjectPatProp::Assign(node) => node.into_node(),
      ObjectPatProp::Rest(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ObjectPatProp::KeyValue(node) => node.kind(),
      ObjectPatProp::Assign(node) => node.kind(),
      ObjectPatProp::Rest(node) => node.kind(),
    }
  }
}
impl<'a> From<&ObjectPatProp<'a>> for Node<'a> {
  fn from(node: &ObjectPatProp<'a>) -> Node<'a> {
    match node {
      ObjectPatProp::KeyValue(node) => (*node).into(),
      ObjectPatProp::Assign(node) => (*node).into(),
      ObjectPatProp::Rest(node) => (*node).into(),
    }
  }
}

fn get_view_for_object_pat_prop<'a>(inner: &'a swc_ast::ObjectPatProp, parent: Node<'a>, bump: &'a Bump) -> ObjectPatProp<'a> {
  match inner {
    swc_ast::ObjectPatProp::KeyValue(value) => ObjectPatProp::KeyValue(get_view_for_key_value_pat_prop(value, parent, bump)),
    swc_ast::ObjectPatProp::Assign(value) => ObjectPatProp::Assign(get_view_for_assign_pat_prop(value, parent, bump)),
    swc_ast::ObjectPatProp::Rest(value) => ObjectPatProp::Rest(get_view_for_rest_pat(value, parent, bump)),
  }
}


pub enum TsFnOrConstructorType<'a> {
  TsFnType(&'a TsFnType<'a>),
  TsConstructorType(&'a TsConstructorType<'a>),
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

  fn into_node(&self) -> Node<'a> {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.into_node(),
      TsFnOrConstructorType::TsConstructorType(node) => node.into_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.kind(),
      TsFnOrConstructorType::TsConstructorType(node) => node.kind(),
    }
  }
}
impl<'a> From<&TsFnOrConstructorType<'a>> for Node<'a> {
  fn from(node: &TsFnOrConstructorType<'a>) -> Node<'a> {
    match node {
      TsFnOrConstructorType::TsFnType(node) => (*node).into(),
      TsFnOrConstructorType::TsConstructorType(node) => (*node).into(),
    }
  }
}

fn get_view_for_ts_fn_or_constructor_type<'a>(inner: &'a swc_ast::TsFnOrConstructorType, parent: Node<'a>, bump: &'a Bump) -> TsFnOrConstructorType<'a> {
  match inner {
    swc_ast::TsFnOrConstructorType::TsFnType(value) => TsFnOrConstructorType::TsFnType(get_view_for_ts_fn_type(value, parent, bump)),
    swc_ast::TsFnOrConstructorType::TsConstructorType(value) => TsFnOrConstructorType::TsConstructorType(get_view_for_ts_constructor_type(value, parent, bump)),
  }
}


pub struct SwitchCase<'a> {
  pub parent: &'a SwitchStmt<'a>,
  pub inner: &'a swc_ast::SwitchCase,
  /// None for `default:`
  pub test: Option<Expr<'a>>,
  pub cons: Vec<Stmt<'a>>,
}

impl<'a> Spanned for SwitchCase<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&SwitchCase<'a>> for Node<'a> {
  fn from(node: &SwitchCase<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SwitchCase<'a>, &'a SwitchCase<'a>>(node) };
    Node::SwitchCase(node)
  }
}

impl<'a> NodeTrait<'a> for &'a SwitchCase<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.test { Some(_value) => 1, None => 0, } + self.cons.len());
    if let Some(child) = self.test.as_ref() {
      children.push(child.into());
    }
    for child in self.cons.iter() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SwitchCase
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

fn get_view_for_switch_case<'a>(inner: &'a swc_ast::SwitchCase, parent: Node<'a>, bump: &'a Bump) -> &'a SwitchCase<'a> {
  let node = bump.alloc(SwitchCase {
    inner,
    parent: parent.to::<SwitchStmt>(),
    test: None,
    cons: Vec::with_capacity(inner.cons.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.test = match &inner.test {
    Some(value) => Some(get_view_for_expr(value, parent.clone(), bump)),
    None => None,
  };
  node.cons.extend(inner.cons.iter().map(|value| get_view_for_stmt(value, parent.clone(), bump)));
  node
}

pub struct ThrowStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ThrowStmt,
  pub arg: Expr<'a>,
}

impl<'a> Spanned for ThrowStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ThrowStmt<'a>> for Node<'a> {
  fn from(node: &ThrowStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ThrowStmt<'a>, &'a ThrowStmt<'a>>(node) };
    Node::ThrowStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ThrowStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ThrowStmt
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

fn get_view_for_throw_stmt<'a>(inner: &'a swc_ast::ThrowStmt, parent: Node<'a>, bump: &'a Bump) -> &'a ThrowStmt<'a> {
  let node = bump.alloc(ThrowStmt {
    inner,
    parent,
    arg: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.arg = get_view_for_expr(&inner.arg, parent, bump);
  node
}

pub struct JSXClosingFragment<'a> {
  pub parent: &'a JSXFragment<'a>,
  pub inner: &'a swc_ast::JSXClosingFragment,
}

impl<'a> Spanned for JSXClosingFragment<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXClosingFragment<'a>> for Node<'a> {
  fn from(node: &JSXClosingFragment<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXClosingFragment<'a>, &'a JSXClosingFragment<'a>>(node) };
    Node::JSXClosingFragment(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXClosingFragment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXClosingFragment
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

fn get_view_for_jsxclosing_fragment<'a>(inner: &'a swc_ast::JSXClosingFragment, parent: Node<'a>, bump: &'a Bump) -> &'a JSXClosingFragment<'a> {
  let node = bump.alloc(JSXClosingFragment {
    inner,
    parent: parent.to::<JSXFragment>(),
  });
  node
}

pub struct BigInt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::BigInt,
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
  fn from(node: &BigInt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&BigInt<'a>, &'a BigInt<'a>>(node) };
    Node::BigInt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a BigInt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::BigInt
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

fn get_view_for_big_int<'a>(inner: &'a swc_ast::BigInt, parent: Node<'a>, bump: &'a Bump) -> &'a BigInt<'a> {
  let node = bump.alloc(BigInt {
    inner,
    parent,
  });
  node
}

pub struct ExportDefaultSpecifier<'a> {
  pub parent: &'a NamedExport<'a>,
  pub inner: &'a swc_ast::ExportDefaultSpecifier,
  pub exported: &'a Ident<'a>,
}

impl<'a> Spanned for ExportDefaultSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportDefaultSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportDefaultSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportDefaultSpecifier<'a>, &'a ExportDefaultSpecifier<'a>>(node) };
    Node::ExportDefaultSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ExportDefaultSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.exported.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportDefaultSpecifier
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

fn get_view_for_export_default_specifier<'a>(inner: &'a swc_ast::ExportDefaultSpecifier, parent: Node<'a>, bump: &'a Bump) -> &'a ExportDefaultSpecifier<'a> {
  let node = bump.alloc(ExportDefaultSpecifier {
    inner,
    parent: parent.to::<NamedExport>(),
    exported: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.exported = get_view_for_ident(&inner.exported, parent, bump);
  node
}

pub struct TsTypeParam<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeParam,
  pub name: &'a Ident<'a>,
  pub constraint: Option<TsType<'a>>,
  pub default: Option<TsType<'a>>,
}

impl<'a> Spanned for TsTypeParam<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeParam<'a>> for Node<'a> {
  fn from(node: &TsTypeParam<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeParam<'a>, &'a TsTypeParam<'a>>(node) };
    Node::TsTypeParam(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeParam<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.constraint { Some(_value) => 1, None => 0, } + match &self.default { Some(_value) => 1, None => 0, });
    children.push(self.name.into());
    if let Some(child) = self.constraint.as_ref() {
      children.push(child.into());
    }
    if let Some(child) = self.default.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeParam
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

fn get_view_for_ts_type_param<'a>(inner: &'a swc_ast::TsTypeParam, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeParam<'a> {
  let node = bump.alloc(TsTypeParam {
    inner,
    parent,
    name: unsafe { MaybeUninit::uninit().assume_init() },
    constraint: None,
    default: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.name = get_view_for_ident(&inner.name, parent.clone(), bump);
  node.constraint = match &inner.constraint {
    Some(value) => Some(get_view_for_ts_type(value, parent.clone(), bump)),
    None => None,
  };
  node.default = match &inner.default {
    Some(value) => Some(get_view_for_ts_type(value, parent, bump)),
    None => None,
  };
  node
}

pub struct WithStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::WithStmt,
  pub obj: Expr<'a>,
  pub body: Stmt<'a>,
}

impl<'a> Spanned for WithStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&WithStmt<'a>> for Node<'a> {
  fn from(node: &WithStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&WithStmt<'a>, &'a WithStmt<'a>>(node) };
    Node::WithStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a WithStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push((&self.body).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::WithStmt
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

fn get_view_for_with_stmt<'a>(inner: &'a swc_ast::WithStmt, parent: Node<'a>, bump: &'a Bump) -> &'a WithStmt<'a> {
  let node = bump.alloc(WithStmt {
    inner,
    parent,
    obj: unsafe { MaybeUninit::uninit().assume_init() },
    body: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.obj = get_view_for_expr(&inner.obj, parent.clone(), bump);
  node.body = get_view_for_stmt(&inner.body, parent, bump);
  node
}

pub struct Regex<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Regex,
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
  fn from(node: &Regex<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Regex<'a>, &'a Regex<'a>>(node) };
    Node::Regex(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Regex<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Regex
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

fn get_view_for_regex<'a>(inner: &'a swc_ast::Regex, parent: Node<'a>, bump: &'a Bump) -> &'a Regex<'a> {
  let node = bump.alloc(Regex {
    inner,
    parent,
  });
  node
}

pub struct TsMethodSignature<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsMethodSignature,
  pub key: Expr<'a>,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
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
  fn from(node: &TsMethodSignature<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsMethodSignature<'a>, &'a TsMethodSignature<'a>>(node) };
    Node::TsMethodSignature(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsMethodSignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.params.len() + match &self.type_ann { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsMethodSignature
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

fn get_view_for_ts_method_signature<'a>(inner: &'a swc_ast::TsMethodSignature, parent: Node<'a>, bump: &'a Bump) -> &'a TsMethodSignature<'a> {
  let node = bump.alloc(TsMethodSignature {
    inner,
    parent,
    key: unsafe { MaybeUninit::uninit().assume_init() },
    params: Vec::with_capacity(inner.params.len()),
    type_ann: None,
    type_params: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_expr(&inner.key, parent.clone(), bump);
  node.params.extend(inner.params.iter().map(|value| get_view_for_ts_fn_param(value, parent.clone(), bump)));
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent.clone(), bump)),
    None => None,
  };
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent, bump)),
    None => None,
  };
  node
}

pub struct UpdateExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::UpdateExpr,
  pub arg: Expr<'a>,
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
  fn from(node: &UpdateExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&UpdateExpr<'a>, &'a UpdateExpr<'a>>(node) };
    Node::UpdateExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a UpdateExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::UpdateExpr
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

fn get_view_for_update_expr<'a>(inner: &'a swc_ast::UpdateExpr, parent: Node<'a>, bump: &'a Bump) -> &'a UpdateExpr<'a> {
  let node = bump.alloc(UpdateExpr {
    inner,
    parent,
    arg: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.arg = get_view_for_expr(&inner.arg, parent, bump);
  node
}

pub struct SetterProp<'a> {
  pub parent: &'a ObjectLit<'a>,
  pub inner: &'a swc_ast::SetterProp,
  pub key: PropName<'a>,
  pub param: Pat<'a>,
  pub body: Option<&'a BlockStmt<'a>>,
}

impl<'a> Spanned for SetterProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&SetterProp<'a>> for Node<'a> {
  fn from(node: &SetterProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SetterProp<'a>, &'a SetterProp<'a>>(node) };
    Node::SetterProp(node)
  }
}

impl<'a> NodeTrait<'a> for &'a SetterProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    children.push((&self.param).into());
    if let Some(child) = self.body {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SetterProp
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

fn get_view_for_setter_prop<'a>(inner: &'a swc_ast::SetterProp, parent: Node<'a>, bump: &'a Bump) -> &'a SetterProp<'a> {
  let node = bump.alloc(SetterProp {
    inner,
    parent: parent.to::<ObjectLit>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    param: unsafe { MaybeUninit::uninit().assume_init() },
    body: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_prop_name(&inner.key, parent.clone(), bump);
  node.param = get_view_for_pat(&inner.param, parent.clone(), bump);
  node.body = match &inner.body {
    Some(value) => Some(get_view_for_block_stmt(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TaggedTpl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TaggedTpl,
  pub tag: Expr<'a>,
  pub exprs: Vec<Expr<'a>>,
  pub quasis: Vec<&'a TplElement<'a>>,
  pub type_params: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for TaggedTpl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TaggedTpl<'a>> for Node<'a> {
  fn from(node: &TaggedTpl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TaggedTpl<'a>, &'a TaggedTpl<'a>>(node) };
    Node::TaggedTpl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TaggedTpl<'a> {
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
      children.push((*child).into());
    }
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TaggedTpl
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

fn get_view_for_tagged_tpl<'a>(inner: &'a swc_ast::TaggedTpl, parent: Node<'a>, bump: &'a Bump) -> &'a TaggedTpl<'a> {
  let node = bump.alloc(TaggedTpl {
    inner,
    parent,
    tag: unsafe { MaybeUninit::uninit().assume_init() },
    exprs: Vec::with_capacity(inner.exprs.len()),
    quasis: Vec::with_capacity(inner.quasis.len()),
    type_params: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.tag = get_view_for_expr(&inner.tag, parent.clone(), bump);
  node.exprs.extend(inner.exprs.iter().map(|value| get_view_for_expr(value, parent.clone(), bump)));
  node.quasis.extend(inner.quasis.iter().map(|value| get_view_for_tpl_element(value, parent.clone(), bump)));
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value, parent, bump)),
    None => None,
  };
  node
}

/// `export * from 'mod'`
pub struct ExportAll<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ExportAll,
  pub src: &'a Str<'a>,
}

impl<'a> Spanned for ExportAll<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportAll<'a>> for Node<'a> {
  fn from(node: &ExportAll<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportAll<'a>, &'a ExportAll<'a>>(node) };
    Node::ExportAll(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ExportAll<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.src.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportAll
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

fn get_view_for_export_all<'a>(inner: &'a swc_ast::ExportAll, parent: Node<'a>, bump: &'a Bump) -> &'a ExportAll<'a> {
  let node = bump.alloc(ExportAll {
    inner,
    parent,
    src: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.src = get_view_for_str(&inner.src, parent, bump);
  node
}

pub struct TsModuleBlock<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsModuleBlock,
  pub body: Vec<ModuleItem<'a>>,
}

impl<'a> Spanned for TsModuleBlock<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsModuleBlock<'a>> for Node<'a> {
  fn from(node: &TsModuleBlock<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsModuleBlock<'a>, &'a TsModuleBlock<'a>>(node) };
    Node::TsModuleBlock(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsModuleBlock<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsModuleBlock
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

fn get_view_for_ts_module_block<'a>(inner: &'a swc_ast::TsModuleBlock, parent: Node<'a>, bump: &'a Bump) -> &'a TsModuleBlock<'a> {
  let node = bump.alloc(TsModuleBlock {
    inner,
    parent,
    body: Vec::with_capacity(inner.body.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.body.extend(inner.body.iter().map(|value| get_view_for_module_item(value, parent.clone(), bump)));
  node
}

pub struct SwitchStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::SwitchStmt,
  pub discriminant: Expr<'a>,
  pub cases: Vec<&'a SwitchCase<'a>>,
}

impl<'a> Spanned for SwitchStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&SwitchStmt<'a>> for Node<'a> {
  fn from(node: &SwitchStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SwitchStmt<'a>, &'a SwitchStmt<'a>>(node) };
    Node::SwitchStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a SwitchStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.cases.len());
    children.push((&self.discriminant).into());
    for child in self.cases.iter() {
      children.push((*child).into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SwitchStmt
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

fn get_view_for_switch_stmt<'a>(inner: &'a swc_ast::SwitchStmt, parent: Node<'a>, bump: &'a Bump) -> &'a SwitchStmt<'a> {
  let node = bump.alloc(SwitchStmt {
    inner,
    parent,
    discriminant: unsafe { MaybeUninit::uninit().assume_init() },
    cases: Vec::with_capacity(inner.cases.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.discriminant = get_view_for_expr(&inner.discriminant, parent.clone(), bump);
  node.cases.extend(inner.cases.iter().map(|value| get_view_for_switch_case(value, parent.clone(), bump)));
  node
}

pub struct TsEnumMember<'a> {
  pub parent: &'a TsEnumDecl<'a>,
  pub inner: &'a swc_ast::TsEnumMember,
  pub id: TsEnumMemberId<'a>,
  pub init: Option<Expr<'a>>,
}

impl<'a> Spanned for TsEnumMember<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsEnumMember<'a>> for Node<'a> {
  fn from(node: &TsEnumMember<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsEnumMember<'a>, &'a TsEnumMember<'a>>(node) };
    Node::TsEnumMember(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsEnumMember<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, });
    children.push((&self.id).into());
    if let Some(child) = self.init.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsEnumMember
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

fn get_view_for_ts_enum_member<'a>(inner: &'a swc_ast::TsEnumMember, parent: Node<'a>, bump: &'a Bump) -> &'a TsEnumMember<'a> {
  let node = bump.alloc(TsEnumMember {
    inner,
    parent: parent.to::<TsEnumDecl>(),
    id: unsafe { MaybeUninit::uninit().assume_init() },
    init: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.id = get_view_for_ts_enum_member_id(&inner.id, parent.clone(), bump);
  node.init = match &inner.init {
    Some(value) => Some(get_view_for_expr(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsIndexedAccessType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsIndexedAccessType,
  pub obj_type: TsType<'a>,
  pub index_type: TsType<'a>,
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
  fn from(node: &TsIndexedAccessType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsIndexedAccessType<'a>, &'a TsIndexedAccessType<'a>>(node) };
    Node::TsIndexedAccessType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsIndexedAccessType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj_type).into());
    children.push((&self.index_type).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsIndexedAccessType
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

fn get_view_for_ts_indexed_access_type<'a>(inner: &'a swc_ast::TsIndexedAccessType, parent: Node<'a>, bump: &'a Bump) -> &'a TsIndexedAccessType<'a> {
  let node = bump.alloc(TsIndexedAccessType {
    inner,
    parent,
    obj_type: unsafe { MaybeUninit::uninit().assume_init() },
    index_type: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.obj_type = get_view_for_ts_type(&inner.obj_type, parent.clone(), bump);
  node.index_type = get_view_for_ts_type(&inner.index_type, parent, bump);
  node
}

pub struct TsRestType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsRestType,
  pub type_ann: TsType<'a>,
}

impl<'a> Spanned for TsRestType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsRestType<'a>> for Node<'a> {
  fn from(node: &TsRestType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsRestType<'a>, &'a TsRestType<'a>>(node) };
    Node::TsRestType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsRestType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsRestType
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

fn get_view_for_ts_rest_type<'a>(inner: &'a swc_ast::TsRestType, parent: Node<'a>, bump: &'a Bump) -> &'a TsRestType<'a> {
  let node = bump.alloc(TsRestType {
    inner,
    parent,
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.type_ann = get_view_for_ts_type(&inner.type_ann, parent, bump);
  node
}

pub struct ExprStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ExprStmt,
  pub expr: Expr<'a>,
}

impl<'a> Spanned for ExprStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExprStmt<'a>> for Node<'a> {
  fn from(node: &ExprStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExprStmt<'a>, &'a ExprStmt<'a>>(node) };
    Node::ExprStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ExprStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExprStmt
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

fn get_view_for_expr_stmt<'a>(inner: &'a swc_ast::ExprStmt, parent: Node<'a>, bump: &'a Bump) -> &'a ExprStmt<'a> {
  let node = bump.alloc(ExprStmt {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

pub struct TsOptionalType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsOptionalType,
  pub type_ann: TsType<'a>,
}

impl<'a> Spanned for TsOptionalType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsOptionalType<'a>> for Node<'a> {
  fn from(node: &TsOptionalType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsOptionalType<'a>, &'a TsOptionalType<'a>>(node) };
    Node::TsOptionalType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsOptionalType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsOptionalType
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

fn get_view_for_ts_optional_type<'a>(inner: &'a swc_ast::TsOptionalType, parent: Node<'a>, bump: &'a Bump) -> &'a TsOptionalType<'a> {
  let node = bump.alloc(TsOptionalType {
    inner,
    parent,
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.type_ann = get_view_for_ts_type(&inner.type_ann, parent, bump);
  node
}

pub struct Tpl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Tpl,
  pub exprs: Vec<Expr<'a>>,
  pub quasis: Vec<&'a TplElement<'a>>,
}

impl<'a> Spanned for Tpl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Tpl<'a>> for Node<'a> {
  fn from(node: &Tpl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Tpl<'a>, &'a Tpl<'a>>(node) };
    Node::Tpl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Tpl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.exprs.len() + self.quasis.len());
    for child in self.exprs.iter() {
      children.push(child.into());
    }
    for child in self.quasis.iter() {
      children.push((*child).into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Tpl
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

fn get_view_for_tpl<'a>(inner: &'a swc_ast::Tpl, parent: Node<'a>, bump: &'a Bump) -> &'a Tpl<'a> {
  let node = bump.alloc(Tpl {
    inner,
    parent,
    exprs: Vec::with_capacity(inner.exprs.len()),
    quasis: Vec::with_capacity(inner.quasis.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.exprs.extend(inner.exprs.iter().map(|value| get_view_for_expr(value, parent.clone(), bump)));
  node.quasis.extend(inner.quasis.iter().map(|value| get_view_for_tpl_element(value, parent.clone(), bump)));
  node
}

/// Represents a invalid node.
pub struct Invalid<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Invalid,
}

impl<'a> Spanned for Invalid<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Invalid<'a>> for Node<'a> {
  fn from(node: &Invalid<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Invalid<'a>, &'a Invalid<'a>>(node) };
    Node::Invalid(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Invalid<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Invalid
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

fn get_view_for_invalid<'a>(inner: &'a swc_ast::Invalid, parent: Node<'a>, bump: &'a Bump) -> &'a Invalid<'a> {
  let node = bump.alloc(Invalid {
    inner,
    parent,
  });
  node
}

pub struct ComputedPropName<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ComputedPropName,
  pub expr: Expr<'a>,
}

impl<'a> Spanned for ComputedPropName<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ComputedPropName<'a>> for Node<'a> {
  fn from(node: &ComputedPropName<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ComputedPropName<'a>, &'a ComputedPropName<'a>>(node) };
    Node::ComputedPropName(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ComputedPropName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ComputedPropName
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

fn get_view_for_computed_prop_name<'a>(inner: &'a swc_ast::ComputedPropName, parent: Node<'a>, bump: &'a Bump) -> &'a ComputedPropName<'a> {
  let node = bump.alloc(ComputedPropName {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

pub struct TsFnType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsFnType,
  pub params: Vec<TsFnParam<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub type_ann: &'a TsTypeAnn<'a>,
}

impl<'a> Spanned for TsFnType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsFnType<'a>> for Node<'a> {
  fn from(node: &TsFnType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsFnType<'a>, &'a TsFnType<'a>>(node) };
    Node::TsFnType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsFnType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.params.len() + match &self.type_params { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children.push(self.type_ann.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsFnType
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

fn get_view_for_ts_fn_type<'a>(inner: &'a swc_ast::TsFnType, parent: Node<'a>, bump: &'a Bump) -> &'a TsFnType<'a> {
  let node = bump.alloc(TsFnType {
    inner,
    parent,
    params: Vec::with_capacity(inner.params.len()),
    type_params: None,
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.params.extend(inner.params.iter().map(|value| get_view_for_ts_fn_param(value, parent.clone(), bump)));
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent.clone(), bump)),
    None => None,
  };
  node.type_ann = get_view_for_ts_type_ann(&inner.type_ann, parent, bump);
  node
}

/// Use when only block statements are allowed.
pub struct BlockStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::BlockStmt,
  pub stmts: Vec<Stmt<'a>>,
}

impl<'a> Spanned for BlockStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&BlockStmt<'a>> for Node<'a> {
  fn from(node: &BlockStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&BlockStmt<'a>, &'a BlockStmt<'a>>(node) };
    Node::BlockStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a BlockStmt<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::BlockStmt
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

fn get_view_for_block_stmt<'a>(inner: &'a swc_ast::BlockStmt, parent: Node<'a>, bump: &'a Bump) -> &'a BlockStmt<'a> {
  let node = bump.alloc(BlockStmt {
    inner,
    parent,
    stmts: Vec::with_capacity(inner.stmts.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.stmts.extend(inner.stmts.iter().map(|value| get_view_for_stmt(value, parent.clone(), bump)));
  node
}

pub struct TsTypeAliasDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeAliasDecl,
  pub id: &'a Ident<'a>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub type_ann: TsType<'a>,
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
  fn from(node: &TsTypeAliasDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeAliasDecl<'a>, &'a TsTypeAliasDecl<'a>>(node) };
    Node::TsTypeAliasDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeAliasDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push(self.id.into());
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children.push((&self.type_ann).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeAliasDecl
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

fn get_view_for_ts_type_alias_decl<'a>(inner: &'a swc_ast::TsTypeAliasDecl, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeAliasDecl<'a> {
  let node = bump.alloc(TsTypeAliasDecl {
    inner,
    parent,
    id: unsafe { MaybeUninit::uninit().assume_init() },
    type_params: None,
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.id = get_view_for_ident(&inner.id, parent.clone(), bump);
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent.clone(), bump)),
    None => None,
  };
  node.type_ann = get_view_for_ts_type(&inner.type_ann, parent, bump);
  node
}

pub struct MemberExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::MemberExpr,
  pub obj: ExprOrSuper<'a>,
  pub prop: Expr<'a>,
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
  fn from(node: &MemberExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&MemberExpr<'a>, &'a MemberExpr<'a>>(node) };
    Node::MemberExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a MemberExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push((&self.prop).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::MemberExpr
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

fn get_view_for_member_expr<'a>(inner: &'a swc_ast::MemberExpr, parent: Node<'a>, bump: &'a Bump) -> &'a MemberExpr<'a> {
  let node = bump.alloc(MemberExpr {
    inner,
    parent,
    obj: unsafe { MaybeUninit::uninit().assume_init() },
    prop: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.obj = get_view_for_expr_or_super(&inner.obj, parent.clone(), bump);
  node.prop = get_view_for_expr(&inner.prop, parent, bump);
  node
}

/// Common parts of function and method.
pub struct Function<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Function,
  pub params: Vec<&'a Param<'a>>,
  pub decorators: Vec<&'a Decorator<'a>>,
  pub body: Option<&'a BlockStmt<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub return_type: Option<&'a TsTypeAnn<'a>>,
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
  fn from(node: &Function<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Function<'a>, &'a Function<'a>>(node) };
    Node::Function(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Function<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.params.len() + self.decorators.len() + match &self.body { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, } + match &self.return_type { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push((*child).into());
    }
    for child in self.decorators.iter() {
      children.push((*child).into());
    }
    if let Some(child) = self.body {
      children.push(child.into());
    }
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    if let Some(child) = self.return_type {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Function
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

fn get_view_for_function<'a>(inner: &'a swc_ast::Function, parent: Node<'a>, bump: &'a Bump) -> &'a Function<'a> {
  let node = bump.alloc(Function {
    inner,
    parent,
    params: Vec::with_capacity(inner.params.len()),
    decorators: Vec::with_capacity(inner.decorators.len()),
    body: None,
    type_params: None,
    return_type: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.params.extend(inner.params.iter().map(|value| get_view_for_param(value, parent.clone(), bump)));
  node.decorators.extend(inner.decorators.iter().map(|value| get_view_for_decorator(value, parent.clone(), bump)));
  node.body = match &inner.body {
    Some(value) => Some(get_view_for_block_stmt(value, parent.clone(), bump)),
    None => None,
  };
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent.clone(), bump)),
    None => None,
  };
  node.return_type = match &inner.return_type {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent, bump)),
    None => None,
  };
  node
}

pub struct ImportDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ImportDecl,
  pub specifiers: Vec<ImportSpecifier<'a>>,
  pub src: &'a Str<'a>,
  pub asserts: Option<&'a ObjectLit<'a>>,
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
  fn from(node: &ImportDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ImportDecl<'a>, &'a ImportDecl<'a>>(node) };
    Node::ImportDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ImportDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.specifiers.len() + match &self.asserts { Some(_value) => 1, None => 0, });
    for child in self.specifiers.iter() {
      children.push(child.into());
    }
    children.push(self.src.into());
    if let Some(child) = self.asserts {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ImportDecl
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

fn get_view_for_import_decl<'a>(inner: &'a swc_ast::ImportDecl, parent: Node<'a>, bump: &'a Bump) -> &'a ImportDecl<'a> {
  let node = bump.alloc(ImportDecl {
    inner,
    parent,
    specifiers: Vec::with_capacity(inner.specifiers.len()),
    src: unsafe { MaybeUninit::uninit().assume_init() },
    asserts: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.specifiers.extend(inner.specifiers.iter().map(|value| get_view_for_import_specifier(value, parent.clone(), bump)));
  node.src = get_view_for_str(&inner.src, parent.clone(), bump);
  node.asserts = match &inner.asserts {
    Some(value) => Some(get_view_for_object_lit(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsTypePredicate<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypePredicate,
  pub param_name: TsThisTypeOrIdent<'a>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
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
  fn from(node: &TsTypePredicate<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypePredicate<'a>, &'a TsTypePredicate<'a>>(node) };
    Node::TsTypePredicate(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypePredicate<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.param_name).into());
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypePredicate
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

fn get_view_for_ts_type_predicate<'a>(inner: &'a swc_ast::TsTypePredicate, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypePredicate<'a> {
  let node = bump.alloc(TsTypePredicate {
    inner,
    parent,
    param_name: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.param_name = get_view_for_ts_this_type_or_ident(&inner.param_name, parent.clone(), bump);
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent, bump)),
    None => None,
  };
  node
}

pub struct YieldExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::YieldExpr,
  pub arg: Option<Expr<'a>>,
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
  fn from(node: &YieldExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&YieldExpr<'a>, &'a YieldExpr<'a>>(node) };
    Node::YieldExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a YieldExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.arg { Some(_value) => 1, None => 0, });
    if let Some(child) = self.arg.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::YieldExpr
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

fn get_view_for_yield_expr<'a>(inner: &'a swc_ast::YieldExpr, parent: Node<'a>, bump: &'a Bump) -> &'a YieldExpr<'a> {
  let node = bump.alloc(YieldExpr {
    inner,
    parent,
    arg: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.arg = match &inner.arg {
    Some(value) => Some(get_view_for_expr(value, parent, bump)),
    None => None,
  };
  node
}

pub struct KeyValueProp<'a> {
  pub parent: &'a ObjectLit<'a>,
  pub inner: &'a swc_ast::KeyValueProp,
  pub key: PropName<'a>,
  pub value: Expr<'a>,
}

impl<'a> Spanned for KeyValueProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&KeyValueProp<'a>> for Node<'a> {
  fn from(node: &KeyValueProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&KeyValueProp<'a>, &'a KeyValueProp<'a>>(node) };
    Node::KeyValueProp(node)
  }
}

impl<'a> NodeTrait<'a> for &'a KeyValueProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.value).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::KeyValueProp
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

fn get_view_for_key_value_prop<'a>(inner: &'a swc_ast::KeyValueProp, parent: Node<'a>, bump: &'a Bump) -> &'a KeyValueProp<'a> {
  let node = bump.alloc(KeyValueProp {
    inner,
    parent: parent.to::<ObjectLit>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    value: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_prop_name(&inner.key, parent.clone(), bump);
  node.value = get_view_for_expr(&inner.value, parent, bump);
  node
}

pub struct Param<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Param,
  pub decorators: Vec<&'a Decorator<'a>>,
  pub pat: Pat<'a>,
}

impl<'a> Spanned for Param<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Param<'a>> for Node<'a> {
  fn from(node: &Param<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Param<'a>, &'a Param<'a>>(node) };
    Node::Param(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Param<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.decorators.len());
    for child in self.decorators.iter() {
      children.push((*child).into());
    }
    children.push((&self.pat).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Param
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

fn get_view_for_param<'a>(inner: &'a swc_ast::Param, parent: Node<'a>, bump: &'a Bump) -> &'a Param<'a> {
  let node = bump.alloc(Param {
    inner,
    parent,
    decorators: Vec::with_capacity(inner.decorators.len()),
    pat: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.decorators.extend(inner.decorators.iter().map(|value| get_view_for_decorator(value, parent.clone(), bump)));
  node.pat = get_view_for_pat(&inner.pat, parent, bump);
  node
}

pub struct JSXFragment<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::JSXFragment,
  pub opening: &'a JSXOpeningFragment<'a>,
  pub children: Vec<JSXElementChild<'a>>,
  pub closing: &'a JSXClosingFragment<'a>,
}

impl<'a> Spanned for JSXFragment<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXFragment<'a>> for Node<'a> {
  fn from(node: &JSXFragment<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXFragment<'a>, &'a JSXFragment<'a>>(node) };
    Node::JSXFragment(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXFragment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + self.children.len());
    children.push(self.opening.into());
    for child in self.children.iter() {
      children.push(child.into());
    }
    children.push(self.closing.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXFragment
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

fn get_view_for_jsxfragment<'a>(inner: &'a swc_ast::JSXFragment, parent: Node<'a>, bump: &'a Bump) -> &'a JSXFragment<'a> {
  let node = bump.alloc(JSXFragment {
    inner,
    parent,
    opening: unsafe { MaybeUninit::uninit().assume_init() },
    children: Vec::with_capacity(inner.children.len()),
    closing: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.opening = get_view_for_jsxopening_fragment(&inner.opening, parent.clone(), bump);
  node.children.extend(inner.children.iter().map(|value| get_view_for_jsxelement_child(value, parent.clone(), bump)));
  node.closing = get_view_for_jsxclosing_fragment(&inner.closing, parent, bump);
  node
}

/// e.g. `import foo from 'mod.js'`
pub struct ImportDefaultSpecifier<'a> {
  pub parent: &'a ImportDecl<'a>,
  pub inner: &'a swc_ast::ImportDefaultSpecifier,
  pub local: &'a Ident<'a>,
}

impl<'a> Spanned for ImportDefaultSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ImportDefaultSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportDefaultSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ImportDefaultSpecifier<'a>, &'a ImportDefaultSpecifier<'a>>(node) };
    Node::ImportDefaultSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ImportDefaultSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.local.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ImportDefaultSpecifier
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

fn get_view_for_import_default_specifier<'a>(inner: &'a swc_ast::ImportDefaultSpecifier, parent: Node<'a>, bump: &'a Bump) -> &'a ImportDefaultSpecifier<'a> {
  let node = bump.alloc(ImportDefaultSpecifier {
    inner,
    parent: parent.to::<ImportDecl>(),
    local: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.local = get_view_for_ident(&inner.local, parent, bump);
  node
}

pub struct Number<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Number,
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
  fn from(node: &Number<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Number<'a>, &'a Number<'a>>(node) };
    Node::Number(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Number<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Number
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

fn get_view_for_number<'a>(inner: &'a swc_ast::Number, parent: Node<'a>, bump: &'a Bump) -> &'a Number<'a> {
  let node = bump.alloc(Number {
    inner,
    parent,
  });
  node
}

pub struct JSXAttr<'a> {
  pub parent: &'a JSXOpeningElement<'a>,
  pub inner: &'a swc_ast::JSXAttr,
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
  fn from(node: &JSXAttr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXAttr<'a>, &'a JSXAttr<'a>>(node) };
    Node::JSXAttr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXAttr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    if let Some(child) = self.value.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXAttr
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

fn get_view_for_jsxattr<'a>(inner: &'a swc_ast::JSXAttr, parent: Node<'a>, bump: &'a Bump) -> &'a JSXAttr<'a> {
  let node = bump.alloc(JSXAttr {
    inner,
    parent: parent.to::<JSXOpeningElement>(),
    name: unsafe { MaybeUninit::uninit().assume_init() },
    value: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.name = get_view_for_jsxattr_name(&inner.name, parent.clone(), bump);
  node.value = match &inner.value {
    Some(value) => Some(get_view_for_jsxattr_value(value, parent, bump)),
    None => None,
  };
  node
}

pub struct ParenExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ParenExpr,
  pub expr: Expr<'a>,
}

impl<'a> Spanned for ParenExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ParenExpr<'a>> for Node<'a> {
  fn from(node: &ParenExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ParenExpr<'a>, &'a ParenExpr<'a>>(node) };
    Node::ParenExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ParenExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ParenExpr
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

fn get_view_for_paren_expr<'a>(inner: &'a swc_ast::ParenExpr, parent: Node<'a>, bump: &'a Bump) -> &'a ParenExpr<'a> {
  let node = bump.alloc(ParenExpr {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

pub struct Super<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Super,
}

impl<'a> Spanned for Super<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Super<'a>> for Node<'a> {
  fn from(node: &Super<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Super<'a>, &'a Super<'a>>(node) };
    Node::Super(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Super<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Super
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

fn get_view_for_super<'a>(inner: &'a swc_ast::Super, parent: Node<'a>, bump: &'a Bump) -> &'a Super<'a> {
  let node = bump.alloc(Super {
    inner,
    parent,
  });
  node
}

pub struct TsConstructorType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsConstructorType,
  pub params: Vec<TsFnParam<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub type_ann: &'a TsTypeAnn<'a>,
}

impl<'a> Spanned for TsConstructorType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsConstructorType<'a>> for Node<'a> {
  fn from(node: &TsConstructorType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsConstructorType<'a>, &'a TsConstructorType<'a>>(node) };
    Node::TsConstructorType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsConstructorType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.params.len() + match &self.type_params { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children.push(self.type_ann.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsConstructorType
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

fn get_view_for_ts_constructor_type<'a>(inner: &'a swc_ast::TsConstructorType, parent: Node<'a>, bump: &'a Bump) -> &'a TsConstructorType<'a> {
  let node = bump.alloc(TsConstructorType {
    inner,
    parent,
    params: Vec::with_capacity(inner.params.len()),
    type_params: None,
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.params.extend(inner.params.iter().map(|value| get_view_for_ts_fn_param(value, parent.clone(), bump)));
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent.clone(), bump)),
    None => None,
  };
  node.type_ann = get_view_for_ts_type_ann(&inner.type_ann, parent, bump);
  node
}

pub struct Class<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Class,
  pub decorators: Vec<&'a Decorator<'a>>,
  pub body: Vec<ClassMember<'a>>,
  pub super_class: Option<Expr<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub super_type_params: Option<&'a TsTypeParamInstantiation<'a>>,
  /// Typescript extension.
  pub implements: Vec<&'a TsExprWithTypeArgs<'a>>,
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
  fn from(node: &Class<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Class<'a>, &'a Class<'a>>(node) };
    Node::Class(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Class<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.decorators.len() + self.body.len() + match &self.super_class { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, } + match &self.super_type_params { Some(_value) => 1, None => 0, } + self.implements.len());
    for child in self.decorators.iter() {
      children.push((*child).into());
    }
    for child in self.body.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.super_class.as_ref() {
      children.push(child.into());
    }
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    if let Some(child) = self.super_type_params {
      children.push(child.into());
    }
    for child in self.implements.iter() {
      children.push((*child).into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Class
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

fn get_view_for_class<'a>(inner: &'a swc_ast::Class, parent: Node<'a>, bump: &'a Bump) -> &'a Class<'a> {
  let node = bump.alloc(Class {
    inner,
    parent,
    decorators: Vec::with_capacity(inner.decorators.len()),
    body: Vec::with_capacity(inner.body.len()),
    super_class: None,
    type_params: None,
    super_type_params: None,
    implements: Vec::with_capacity(inner.implements.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.decorators.extend(inner.decorators.iter().map(|value| get_view_for_decorator(value, parent.clone(), bump)));
  node.body.extend(inner.body.iter().map(|value| get_view_for_class_member(value, parent.clone(), bump)));
  node.super_class = match &inner.super_class {
    Some(value) => Some(get_view_for_expr(value, parent.clone(), bump)),
    None => None,
  };
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent.clone(), bump)),
    None => None,
  };
  node.super_type_params = match &inner.super_type_params {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value, parent.clone(), bump)),
    None => None,
  };
  node.implements.extend(inner.implements.iter().map(|value| get_view_for_ts_expr_with_type_args(value, parent.clone(), bump)));
  node
}

/// EsTree `RestElement`
pub struct RestPat<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::RestPat,
  pub arg: Pat<'a>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
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
  fn from(node: &RestPat<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&RestPat<'a>, &'a RestPat<'a>>(node) };
    Node::RestPat(node)
  }
}

impl<'a> NodeTrait<'a> for &'a RestPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.arg).into());
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::RestPat
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

fn get_view_for_rest_pat<'a>(inner: &'a swc_ast::RestPat, parent: Node<'a>, bump: &'a Bump) -> &'a RestPat<'a> {
  let node = bump.alloc(RestPat {
    inner,
    parent,
    arg: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.arg = get_view_for_pat(&inner.arg, parent.clone(), bump);
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsNamespaceExportDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsNamespaceExportDecl,
  pub id: &'a Ident<'a>,
}

impl<'a> Spanned for TsNamespaceExportDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsNamespaceExportDecl<'a>> for Node<'a> {
  fn from(node: &TsNamespaceExportDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsNamespaceExportDecl<'a>, &'a TsNamespaceExportDecl<'a>>(node) };
    Node::TsNamespaceExportDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsNamespaceExportDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.id.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsNamespaceExportDecl
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

fn get_view_for_ts_namespace_export_decl<'a>(inner: &'a swc_ast::TsNamespaceExportDecl, parent: Node<'a>, bump: &'a Bump) -> &'a TsNamespaceExportDecl<'a> {
  let node = bump.alloc(TsNamespaceExportDecl {
    inner,
    parent,
    id: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.id = get_view_for_ident(&inner.id, parent, bump);
  node
}

pub struct JSXOpeningFragment<'a> {
  pub parent: &'a JSXFragment<'a>,
  pub inner: &'a swc_ast::JSXOpeningFragment,
}

impl<'a> Spanned for JSXOpeningFragment<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXOpeningFragment<'a>> for Node<'a> {
  fn from(node: &JSXOpeningFragment<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXOpeningFragment<'a>, &'a JSXOpeningFragment<'a>>(node) };
    Node::JSXOpeningFragment(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXOpeningFragment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXOpeningFragment
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

fn get_view_for_jsxopening_fragment<'a>(inner: &'a swc_ast::JSXOpeningFragment, parent: Node<'a>, bump: &'a Bump) -> &'a JSXOpeningFragment<'a> {
  let node = bump.alloc(JSXOpeningFragment {
    inner,
    parent: parent.to::<JSXFragment>(),
  });
  node
}

pub struct NewExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::NewExpr,
  pub callee: Expr<'a>,
  pub args: Option<Vec<&'a ExprOrSpread<'a>>>,
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for NewExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&NewExpr<'a>> for Node<'a> {
  fn from(node: &NewExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&NewExpr<'a>, &'a NewExpr<'a>>(node) };
    Node::NewExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a NewExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.args { Some(_value) => _value.len(), None => 0, } + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.callee).into());
    if let Some(child) = self.args.as_ref() {
      for child in child.iter() {
        children.push((*child).into());
      }
    }
    if let Some(child) = self.type_args {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::NewExpr
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

fn get_view_for_new_expr<'a>(inner: &'a swc_ast::NewExpr, parent: Node<'a>, bump: &'a Bump) -> &'a NewExpr<'a> {
  let node = bump.alloc(NewExpr {
    inner,
    parent,
    callee: unsafe { MaybeUninit::uninit().assume_init() },
    args: None,
    type_args: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.callee = get_view_for_expr(&inner.callee, parent.clone(), bump);
  node.args = match &inner.args {
    Some(value) => Some(value.iter().map(|value| get_view_for_expr_or_spread(value, parent.clone(), bump)).collect()),
    None => None,
  };
  node.type_args = match &inner.type_args {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value, parent, bump)),
    None => None,
  };
  node
}

/// Function expression.
pub struct FnExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::FnExpr,
  pub ident: Option<&'a Ident<'a>>,
  pub function: &'a Function<'a>,
}

impl<'a> Spanned for FnExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&FnExpr<'a>> for Node<'a> {
  fn from(node: &FnExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&FnExpr<'a>, &'a FnExpr<'a>>(node) };
    Node::FnExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a FnExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.ident { Some(_value) => 1, None => 0, });
    if let Some(child) = self.ident {
      children.push(child.into());
    }
    children.push(self.function.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::FnExpr
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

fn get_view_for_fn_expr<'a>(inner: &'a swc_ast::FnExpr, parent: Node<'a>, bump: &'a Bump) -> &'a FnExpr<'a> {
  let node = bump.alloc(FnExpr {
    inner,
    parent,
    ident: None,
    function: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.ident = match &inner.ident {
    Some(value) => Some(get_view_for_ident(value, parent.clone(), bump)),
    None => None,
  };
  node.function = get_view_for_function(&inner.function, parent, bump);
  node
}

pub struct IfStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::IfStmt,
  pub test: Expr<'a>,
  pub cons: Stmt<'a>,
  pub alt: Option<Stmt<'a>>,
}

impl<'a> Spanned for IfStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&IfStmt<'a>> for Node<'a> {
  fn from(node: &IfStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&IfStmt<'a>, &'a IfStmt<'a>>(node) };
    Node::IfStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a IfStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + match &self.alt { Some(_value) => 1, None => 0, });
    children.push((&self.test).into());
    children.push((&self.cons).into());
    if let Some(child) = self.alt.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::IfStmt
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

fn get_view_for_if_stmt<'a>(inner: &'a swc_ast::IfStmt, parent: Node<'a>, bump: &'a Bump) -> &'a IfStmt<'a> {
  let node = bump.alloc(IfStmt {
    inner,
    parent,
    test: unsafe { MaybeUninit::uninit().assume_init() },
    cons: unsafe { MaybeUninit::uninit().assume_init() },
    alt: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.test = get_view_for_expr(&inner.test, parent.clone(), bump);
  node.cons = get_view_for_stmt(&inner.cons, parent.clone(), bump);
  node.alt = match &inner.alt {
    Some(value) => Some(get_view_for_stmt(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsParenthesizedType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsParenthesizedType,
  pub type_ann: TsType<'a>,
}

impl<'a> Spanned for TsParenthesizedType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsParenthesizedType<'a>> for Node<'a> {
  fn from(node: &TsParenthesizedType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsParenthesizedType<'a>, &'a TsParenthesizedType<'a>>(node) };
    Node::TsParenthesizedType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsParenthesizedType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsParenthesizedType
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

fn get_view_for_ts_parenthesized_type<'a>(inner: &'a swc_ast::TsParenthesizedType, parent: Node<'a>, bump: &'a Bump) -> &'a TsParenthesizedType<'a> {
  let node = bump.alloc(TsParenthesizedType {
    inner,
    parent,
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.type_ann = get_view_for_ts_type(&inner.type_ann, parent, bump);
  node
}

/// `{key}` or `{key = value}`
pub struct AssignPatProp<'a> {
  pub parent: &'a ObjectPat<'a>,
  pub inner: &'a swc_ast::AssignPatProp,
  pub key: &'a Ident<'a>,
  pub value: Option<Expr<'a>>,
}

impl<'a> Spanned for AssignPatProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&AssignPatProp<'a>> for Node<'a> {
  fn from(node: &AssignPatProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&AssignPatProp<'a>, &'a AssignPatProp<'a>>(node) };
    Node::AssignPatProp(node)
  }
}

impl<'a> NodeTrait<'a> for &'a AssignPatProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, });
    children.push(self.key.into());
    if let Some(child) = self.value.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::AssignPatProp
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

fn get_view_for_assign_pat_prop<'a>(inner: &'a swc_ast::AssignPatProp, parent: Node<'a>, bump: &'a Bump) -> &'a AssignPatProp<'a> {
  let node = bump.alloc(AssignPatProp {
    inner,
    parent: parent.to::<ObjectPat>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    value: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_ident(&inner.key, parent.clone(), bump);
  node.value = match &inner.value {
    Some(value) => Some(get_view_for_expr(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsImportType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsImportType,
  pub arg: &'a Str<'a>,
  pub qualifier: Option<TsEntityName<'a>>,
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for TsImportType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsImportType<'a>> for Node<'a> {
  fn from(node: &TsImportType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsImportType<'a>, &'a TsImportType<'a>>(node) };
    Node::TsImportType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsImportType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.qualifier { Some(_value) => 1, None => 0, } + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push(self.arg.into());
    if let Some(child) = self.qualifier.as_ref() {
      children.push(child.into());
    }
    if let Some(child) = self.type_args {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsImportType
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

fn get_view_for_ts_import_type<'a>(inner: &'a swc_ast::TsImportType, parent: Node<'a>, bump: &'a Bump) -> &'a TsImportType<'a> {
  let node = bump.alloc(TsImportType {
    inner,
    parent,
    arg: unsafe { MaybeUninit::uninit().assume_init() },
    qualifier: None,
    type_args: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.arg = get_view_for_str(&inner.arg, parent.clone(), bump);
  node.qualifier = match &inner.qualifier {
    Some(value) => Some(get_view_for_ts_entity_name(value, parent.clone(), bump)),
    None => None,
  };
  node.type_args = match &inner.type_args {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value, parent, bump)),
    None => None,
  };
  node
}

pub struct Bool<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Bool,
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
  fn from(node: &Bool<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Bool<'a>, &'a Bool<'a>>(node) };
    Node::Bool(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Bool<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Bool
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

fn get_view_for_bool<'a>(inner: &'a swc_ast::Bool, parent: Node<'a>, bump: &'a Bump) -> &'a Bool<'a> {
  let node = bump.alloc(Bool {
    inner,
    parent,
  });
  node
}

pub struct TsImportEqualsDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsImportEqualsDecl,
  pub id: &'a Ident<'a>,
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
  fn from(node: &TsImportEqualsDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsImportEqualsDecl<'a>, &'a TsImportEqualsDecl<'a>>(node) };
    Node::TsImportEqualsDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsImportEqualsDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.id.into());
    children.push((&self.module_ref).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsImportEqualsDecl
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

fn get_view_for_ts_import_equals_decl<'a>(inner: &'a swc_ast::TsImportEqualsDecl, parent: Node<'a>, bump: &'a Bump) -> &'a TsImportEqualsDecl<'a> {
  let node = bump.alloc(TsImportEqualsDecl {
    inner,
    parent,
    id: unsafe { MaybeUninit::uninit().assume_init() },
    module_ref: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.id = get_view_for_ident(&inner.id, parent.clone(), bump);
  node.module_ref = get_view_for_ts_module_ref(&inner.module_ref, parent, bump);
  node
}

pub struct AssignProp<'a> {
  pub parent: &'a ObjectLit<'a>,
  pub inner: &'a swc_ast::AssignProp,
  pub key: &'a Ident<'a>,
  pub value: Expr<'a>,
}

impl<'a> Spanned for AssignProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&AssignProp<'a>> for Node<'a> {
  fn from(node: &AssignProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&AssignProp<'a>, &'a AssignProp<'a>>(node) };
    Node::AssignProp(node)
  }
}

impl<'a> NodeTrait<'a> for &'a AssignProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.key.into());
    children.push((&self.value).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::AssignProp
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

fn get_view_for_assign_prop<'a>(inner: &'a swc_ast::AssignProp, parent: Node<'a>, bump: &'a Bump) -> &'a AssignProp<'a> {
  let node = bump.alloc(AssignProp {
    inner,
    parent: parent.to::<ObjectLit>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    value: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_ident(&inner.key, parent.clone(), bump);
  node.value = get_view_for_expr(&inner.value, parent, bump);
  node
}

pub struct TsInterfaceDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsInterfaceDecl,
  pub id: &'a Ident<'a>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub extends: Vec<&'a TsExprWithTypeArgs<'a>>,
  pub body: &'a TsInterfaceBody<'a>,
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
  fn from(node: &TsInterfaceDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsInterfaceDecl<'a>, &'a TsInterfaceDecl<'a>>(node) };
    Node::TsInterfaceDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsInterfaceDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + match &self.type_params { Some(_value) => 1, None => 0, } + self.extends.len());
    children.push(self.id.into());
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    for child in self.extends.iter() {
      children.push((*child).into());
    }
    children.push(self.body.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsInterfaceDecl
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

fn get_view_for_ts_interface_decl<'a>(inner: &'a swc_ast::TsInterfaceDecl, parent: Node<'a>, bump: &'a Bump) -> &'a TsInterfaceDecl<'a> {
  let node = bump.alloc(TsInterfaceDecl {
    inner,
    parent,
    id: unsafe { MaybeUninit::uninit().assume_init() },
    type_params: None,
    extends: Vec::with_capacity(inner.extends.len()),
    body: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.id = get_view_for_ident(&inner.id, parent.clone(), bump);
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent.clone(), bump)),
    None => None,
  };
  node.extends.extend(inner.extends.iter().map(|value| get_view_for_ts_expr_with_type_args(value, parent.clone(), bump)));
  node.body = get_view_for_ts_interface_body(&inner.body, parent, bump);
  node
}

pub struct JSXEmptyExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::JSXEmptyExpr,
}

impl<'a> Spanned for JSXEmptyExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXEmptyExpr<'a>> for Node<'a> {
  fn from(node: &JSXEmptyExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXEmptyExpr<'a>, &'a JSXEmptyExpr<'a>>(node) };
    Node::JSXEmptyExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXEmptyExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXEmptyExpr
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

fn get_view_for_jsxempty_expr<'a>(inner: &'a swc_ast::JSXEmptyExpr, parent: Node<'a>, bump: &'a Bump) -> &'a JSXEmptyExpr<'a> {
  let node = bump.alloc(JSXEmptyExpr {
    inner,
    parent,
  });
  node
}

pub struct TsQualifiedName<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsQualifiedName,
  pub left: TsEntityName<'a>,
  pub right: &'a Ident<'a>,
}

impl<'a> Spanned for TsQualifiedName<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsQualifiedName<'a>> for Node<'a> {
  fn from(node: &TsQualifiedName<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsQualifiedName<'a>, &'a TsQualifiedName<'a>>(node) };
    Node::TsQualifiedName(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsQualifiedName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push(self.right.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsQualifiedName
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

fn get_view_for_ts_qualified_name<'a>(inner: &'a swc_ast::TsQualifiedName, parent: Node<'a>, bump: &'a Bump) -> &'a TsQualifiedName<'a> {
  let node = bump.alloc(TsQualifiedName {
    inner,
    parent,
    left: unsafe { MaybeUninit::uninit().assume_init() },
    right: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.left = get_view_for_ts_entity_name(&inner.left, parent.clone(), bump);
  node.right = get_view_for_ident(&inner.right, parent, bump);
  node
}

pub struct ExportDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ExportDecl,
  pub decl: Decl<'a>,
}

impl<'a> Spanned for ExportDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportDecl<'a>> for Node<'a> {
  fn from(node: &ExportDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportDecl<'a>, &'a ExportDecl<'a>>(node) };
    Node::ExportDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ExportDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.decl).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportDecl
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

fn get_view_for_export_decl<'a>(inner: &'a swc_ast::ExportDecl, parent: Node<'a>, bump: &'a Bump) -> &'a ExportDecl<'a> {
  let node = bump.alloc(ExportDecl {
    inner,
    parent,
    decl: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.decl = get_view_for_decl(&inner.decl, parent, bump);
  node
}

pub struct CatchClause<'a> {
  pub parent: &'a TryStmt<'a>,
  pub inner: &'a swc_ast::CatchClause,
  /// es2019
  /// 
  /// The param is null if the catch binding is omitted. E.g., try { foo() }
  /// catch { bar() }
  pub param: Option<Pat<'a>>,
  pub body: &'a BlockStmt<'a>,
}

impl<'a> Spanned for CatchClause<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&CatchClause<'a>> for Node<'a> {
  fn from(node: &CatchClause<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&CatchClause<'a>, &'a CatchClause<'a>>(node) };
    Node::CatchClause(node)
  }
}

impl<'a> NodeTrait<'a> for &'a CatchClause<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.param { Some(_value) => 1, None => 0, });
    if let Some(child) = self.param.as_ref() {
      children.push(child.into());
    }
    children.push(self.body.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::CatchClause
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

fn get_view_for_catch_clause<'a>(inner: &'a swc_ast::CatchClause, parent: Node<'a>, bump: &'a Bump) -> &'a CatchClause<'a> {
  let node = bump.alloc(CatchClause {
    inner,
    parent: parent.to::<TryStmt>(),
    param: None,
    body: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.param = match &inner.param {
    Some(value) => Some(get_view_for_pat(value, parent.clone(), bump)),
    None => None,
  };
  node.body = get_view_for_block_stmt(&inner.body, parent, bump);
  node
}

pub struct LabeledStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::LabeledStmt,
  pub label: &'a Ident<'a>,
  pub body: Stmt<'a>,
}

impl<'a> Spanned for LabeledStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&LabeledStmt<'a>> for Node<'a> {
  fn from(node: &LabeledStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&LabeledStmt<'a>, &'a LabeledStmt<'a>>(node) };
    Node::LabeledStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a LabeledStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.label.into());
    children.push((&self.body).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::LabeledStmt
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

fn get_view_for_labeled_stmt<'a>(inner: &'a swc_ast::LabeledStmt, parent: Node<'a>, bump: &'a Bump) -> &'a LabeledStmt<'a> {
  let node = bump.alloc(LabeledStmt {
    inner,
    parent,
    label: unsafe { MaybeUninit::uninit().assume_init() },
    body: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.label = get_view_for_ident(&inner.label, parent.clone(), bump);
  node.body = get_view_for_stmt(&inner.body, parent, bump);
  node
}

pub struct ContinueStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ContinueStmt,
  pub label: Option<&'a Ident<'a>>,
}

impl<'a> Spanned for ContinueStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ContinueStmt<'a>> for Node<'a> {
  fn from(node: &ContinueStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ContinueStmt<'a>, &'a ContinueStmt<'a>>(node) };
    Node::ContinueStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ContinueStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = self.label {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ContinueStmt
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

fn get_view_for_continue_stmt<'a>(inner: &'a swc_ast::ContinueStmt, parent: Node<'a>, bump: &'a Bump) -> &'a ContinueStmt<'a> {
  let node = bump.alloc(ContinueStmt {
    inner,
    parent,
    label: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.label = match &inner.label {
    Some(value) => Some(get_view_for_ident(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsConstructSignatureDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsConstructSignatureDecl,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
}

impl<'a> Spanned for TsConstructSignatureDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsConstructSignatureDecl<'a>> for Node<'a> {
  fn from(node: &TsConstructSignatureDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsConstructSignatureDecl<'a>, &'a TsConstructSignatureDecl<'a>>(node) };
    Node::TsConstructSignatureDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsConstructSignatureDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.params.len() + match &self.type_ann { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsConstructSignatureDecl
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

fn get_view_for_ts_construct_signature_decl<'a>(inner: &'a swc_ast::TsConstructSignatureDecl, parent: Node<'a>, bump: &'a Bump) -> &'a TsConstructSignatureDecl<'a> {
  let node = bump.alloc(TsConstructSignatureDecl {
    inner,
    parent,
    params: Vec::with_capacity(inner.params.len()),
    type_ann: None,
    type_params: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.params.extend(inner.params.iter().map(|value| get_view_for_ts_fn_param(value, parent.clone(), bump)));
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent.clone(), bump)),
    None => None,
  };
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsEnumDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsEnumDecl,
  pub id: &'a Ident<'a>,
  pub members: Vec<&'a TsEnumMember<'a>>,
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
  fn from(node: &TsEnumDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsEnumDecl<'a>, &'a TsEnumDecl<'a>>(node) };
    Node::TsEnumDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsEnumDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.members.len());
    children.push(self.id.into());
    for child in self.members.iter() {
      children.push((*child).into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsEnumDecl
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

fn get_view_for_ts_enum_decl<'a>(inner: &'a swc_ast::TsEnumDecl, parent: Node<'a>, bump: &'a Bump) -> &'a TsEnumDecl<'a> {
  let node = bump.alloc(TsEnumDecl {
    inner,
    parent,
    id: unsafe { MaybeUninit::uninit().assume_init() },
    members: Vec::with_capacity(inner.members.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.id = get_view_for_ident(&inner.id, parent.clone(), bump);
  node.members.extend(inner.members.iter().map(|value| get_view_for_ts_enum_member(value, parent.clone(), bump)));
  node
}

pub struct OptChainExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::OptChainExpr,
  pub expr: Expr<'a>,
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
  fn from(node: &OptChainExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&OptChainExpr<'a>, &'a OptChainExpr<'a>>(node) };
    Node::OptChainExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a OptChainExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::OptChainExpr
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

fn get_view_for_opt_chain_expr<'a>(inner: &'a swc_ast::OptChainExpr, parent: Node<'a>, bump: &'a Bump) -> &'a OptChainExpr<'a> {
  let node = bump.alloc(OptChainExpr {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

pub struct TsNamespaceDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsNamespaceDecl,
  pub id: &'a Ident<'a>,
  pub body: TsNamespaceBody<'a>,
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
  fn from(node: &TsNamespaceDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsNamespaceDecl<'a>, &'a TsNamespaceDecl<'a>>(node) };
    Node::TsNamespaceDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsNamespaceDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.id.into());
    children.push((&self.body).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsNamespaceDecl
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

fn get_view_for_ts_namespace_decl<'a>(inner: &'a swc_ast::TsNamespaceDecl, parent: Node<'a>, bump: &'a Bump) -> &'a TsNamespaceDecl<'a> {
  let node = bump.alloc(TsNamespaceDecl {
    inner,
    parent,
    id: unsafe { MaybeUninit::uninit().assume_init() },
    body: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.id = get_view_for_ident(&inner.id, parent.clone(), bump);
  node.body = get_view_for_ts_namespace_body(&inner.body, parent, bump);
  node
}

pub struct SeqExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::SeqExpr,
  pub exprs: Vec<Expr<'a>>,
}

impl<'a> Spanned for SeqExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&SeqExpr<'a>> for Node<'a> {
  fn from(node: &SeqExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SeqExpr<'a>, &'a SeqExpr<'a>>(node) };
    Node::SeqExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a SeqExpr<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SeqExpr
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

fn get_view_for_seq_expr<'a>(inner: &'a swc_ast::SeqExpr, parent: Node<'a>, bump: &'a Bump) -> &'a SeqExpr<'a> {
  let node = bump.alloc(SeqExpr {
    inner,
    parent,
    exprs: Vec::with_capacity(inner.exprs.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.exprs.extend(inner.exprs.iter().map(|value| get_view_for_expr(value, parent.clone(), bump)));
  node
}

pub struct TsExternalModuleRef<'a> {
  pub parent: &'a TsImportEqualsDecl<'a>,
  pub inner: &'a swc_ast::TsExternalModuleRef,
  pub expr: &'a Str<'a>,
}

impl<'a> Spanned for TsExternalModuleRef<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsExternalModuleRef<'a>> for Node<'a> {
  fn from(node: &TsExternalModuleRef<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsExternalModuleRef<'a>, &'a TsExternalModuleRef<'a>>(node) };
    Node::TsExternalModuleRef(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsExternalModuleRef<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.expr.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsExternalModuleRef
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

fn get_view_for_ts_external_module_ref<'a>(inner: &'a swc_ast::TsExternalModuleRef, parent: Node<'a>, bump: &'a Bump) -> &'a TsExternalModuleRef<'a> {
  let node = bump.alloc(TsExternalModuleRef {
    inner,
    parent: parent.to::<TsImportEqualsDecl>(),
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_str(&inner.expr, parent, bump);
  node
}

pub struct TsTypeParamInstantiation<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeParamInstantiation,
  pub params: Vec<TsType<'a>>,
}

impl<'a> Spanned for TsTypeParamInstantiation<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeParamInstantiation<'a>> for Node<'a> {
  fn from(node: &TsTypeParamInstantiation<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeParamInstantiation<'a>, &'a TsTypeParamInstantiation<'a>>(node) };
    Node::TsTypeParamInstantiation(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeParamInstantiation<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeParamInstantiation
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

fn get_view_for_ts_type_param_instantiation<'a>(inner: &'a swc_ast::TsTypeParamInstantiation, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeParamInstantiation<'a> {
  let node = bump.alloc(TsTypeParamInstantiation {
    inner,
    parent,
    params: Vec::with_capacity(inner.params.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.params.extend(inner.params.iter().map(|value| get_view_for_ts_type(value, parent.clone(), bump)));
  node
}

pub struct ReturnStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ReturnStmt,
  pub arg: Option<Expr<'a>>,
}

impl<'a> Spanned for ReturnStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ReturnStmt<'a>> for Node<'a> {
  fn from(node: &ReturnStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ReturnStmt<'a>, &'a ReturnStmt<'a>>(node) };
    Node::ReturnStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ReturnStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.arg { Some(_value) => 1, None => 0, });
    if let Some(child) = self.arg.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ReturnStmt
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

fn get_view_for_return_stmt<'a>(inner: &'a swc_ast::ReturnStmt, parent: Node<'a>, bump: &'a Bump) -> &'a ReturnStmt<'a> {
  let node = bump.alloc(ReturnStmt {
    inner,
    parent,
    arg: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.arg = match &inner.arg {
    Some(value) => Some(get_view_for_expr(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsTplLitType<'a> {
  pub parent: &'a TsLitType<'a>,
  pub inner: &'a swc_ast::TsTplLitType,
  pub types: Vec<TsType<'a>>,
  pub quasis: Vec<&'a TplElement<'a>>,
}

impl<'a> Spanned for TsTplLitType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTplLitType<'a>> for Node<'a> {
  fn from(node: &TsTplLitType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTplLitType<'a>, &'a TsTplLitType<'a>>(node) };
    Node::TsTplLitType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTplLitType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.types.len() + self.quasis.len());
    for child in self.types.iter() {
      children.push(child.into());
    }
    for child in self.quasis.iter() {
      children.push((*child).into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTplLitType
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

fn get_view_for_ts_tpl_lit_type<'a>(inner: &'a swc_ast::TsTplLitType, parent: Node<'a>, bump: &'a Bump) -> &'a TsTplLitType<'a> {
  let node = bump.alloc(TsTplLitType {
    inner,
    parent: parent.to::<TsLitType>(),
    types: Vec::with_capacity(inner.types.len()),
    quasis: Vec::with_capacity(inner.quasis.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.types.extend(inner.types.iter().map(|value| get_view_for_ts_type(value, parent.clone(), bump)));
  node.quasis.extend(inner.quasis.iter().map(|value| get_view_for_tpl_element(value, parent.clone(), bump)));
  node
}

pub struct ExportDefaultExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ExportDefaultExpr,
  pub expr: Expr<'a>,
}

impl<'a> Spanned for ExportDefaultExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportDefaultExpr<'a>> for Node<'a> {
  fn from(node: &ExportDefaultExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportDefaultExpr<'a>, &'a ExportDefaultExpr<'a>>(node) };
    Node::ExportDefaultExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ExportDefaultExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportDefaultExpr
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

fn get_view_for_export_default_expr<'a>(inner: &'a swc_ast::ExportDefaultExpr, parent: Node<'a>, bump: &'a Bump) -> &'a ExportDefaultExpr<'a> {
  let node = bump.alloc(ExportDefaultExpr {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

pub struct TsCallSignatureDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsCallSignatureDecl,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
}

impl<'a> Spanned for TsCallSignatureDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsCallSignatureDecl<'a>> for Node<'a> {
  fn from(node: &TsCallSignatureDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsCallSignatureDecl<'a>, &'a TsCallSignatureDecl<'a>>(node) };
    Node::TsCallSignatureDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsCallSignatureDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.params.len() + match &self.type_ann { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsCallSignatureDecl
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

fn get_view_for_ts_call_signature_decl<'a>(inner: &'a swc_ast::TsCallSignatureDecl, parent: Node<'a>, bump: &'a Bump) -> &'a TsCallSignatureDecl<'a> {
  let node = bump.alloc(TsCallSignatureDecl {
    inner,
    parent,
    params: Vec::with_capacity(inner.params.len()),
    type_ann: None,
    type_params: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.params.extend(inner.params.iter().map(|value| get_view_for_ts_fn_param(value, parent.clone(), bump)));
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent.clone(), bump)),
    None => None,
  };
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent, bump)),
    None => None,
  };
  node
}

pub struct AwaitExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::AwaitExpr,
  pub arg: Expr<'a>,
}

impl<'a> Spanned for AwaitExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&AwaitExpr<'a>> for Node<'a> {
  fn from(node: &AwaitExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&AwaitExpr<'a>, &'a AwaitExpr<'a>>(node) };
    Node::AwaitExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a AwaitExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::AwaitExpr
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

fn get_view_for_await_expr<'a>(inner: &'a swc_ast::AwaitExpr, parent: Node<'a>, bump: &'a Bump) -> &'a AwaitExpr<'a> {
  let node = bump.alloc(AwaitExpr {
    inner,
    parent,
    arg: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.arg = get_view_for_expr(&inner.arg, parent, bump);
  node
}

pub struct ClassMethod<'a> {
  pub parent: &'a Class<'a>,
  pub inner: &'a swc_ast::ClassMethod,
  pub key: PropName<'a>,
  pub function: &'a Function<'a>,
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
  fn from(node: &ClassMethod<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ClassMethod<'a>, &'a ClassMethod<'a>>(node) };
    Node::ClassMethod(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ClassMethod<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push(self.function.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ClassMethod
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

fn get_view_for_class_method<'a>(inner: &'a swc_ast::ClassMethod, parent: Node<'a>, bump: &'a Bump) -> &'a ClassMethod<'a> {
  let node = bump.alloc(ClassMethod {
    inner,
    parent: parent.to::<Class>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    function: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_prop_name(&inner.key, parent.clone(), bump);
  node.function = get_view_for_function(&inner.function, parent, bump);
  node
}

pub struct TsParamProp<'a> {
  pub parent: &'a Constructor<'a>,
  pub inner: &'a swc_ast::TsParamProp,
  pub decorators: Vec<&'a Decorator<'a>>,
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
  fn from(node: &TsParamProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsParamProp<'a>, &'a TsParamProp<'a>>(node) };
    Node::TsParamProp(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsParamProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.decorators.len());
    for child in self.decorators.iter() {
      children.push((*child).into());
    }
    children.push((&self.param).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsParamProp
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

fn get_view_for_ts_param_prop<'a>(inner: &'a swc_ast::TsParamProp, parent: Node<'a>, bump: &'a Bump) -> &'a TsParamProp<'a> {
  let node = bump.alloc(TsParamProp {
    inner,
    parent: parent.to::<Constructor>(),
    decorators: Vec::with_capacity(inner.decorators.len()),
    param: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.decorators.extend(inner.decorators.iter().map(|value| get_view_for_decorator(value, parent.clone(), bump)));
  node.param = get_view_for_ts_param_prop_param(&inner.param, parent, bump);
  node
}

pub struct ClassProp<'a> {
  pub parent: &'a Class<'a>,
  pub inner: &'a swc_ast::ClassProp,
  pub key: Expr<'a>,
  pub value: Option<Expr<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub decorators: Vec<&'a Decorator<'a>>,
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
  fn from(node: &ClassProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ClassProp<'a>, &'a ClassProp<'a>>(node) };
    Node::ClassProp(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ClassProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, } + match &self.type_ann { Some(_value) => 1, None => 0, } + self.decorators.len());
    children.push((&self.key).into());
    if let Some(child) = self.value.as_ref() {
      children.push(child.into());
    }
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    for child in self.decorators.iter() {
      children.push((*child).into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ClassProp
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

fn get_view_for_class_prop<'a>(inner: &'a swc_ast::ClassProp, parent: Node<'a>, bump: &'a Bump) -> &'a ClassProp<'a> {
  let node = bump.alloc(ClassProp {
    inner,
    parent: parent.to::<Class>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    value: None,
    type_ann: None,
    decorators: Vec::with_capacity(inner.decorators.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_expr(&inner.key, parent.clone(), bump);
  node.value = match &inner.value {
    Some(value) => Some(get_view_for_expr(value, parent.clone(), bump)),
    None => None,
  };
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent.clone(), bump)),
    None => None,
  };
  node.decorators.extend(inner.decorators.iter().map(|value| get_view_for_decorator(value, parent.clone(), bump)));
  node
}

pub struct TsTypeAnn<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeAnn,
  pub type_ann: TsType<'a>,
}

impl<'a> Spanned for TsTypeAnn<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeAnn<'a>> for Node<'a> {
  fn from(node: &TsTypeAnn<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeAnn<'a>, &'a TsTypeAnn<'a>>(node) };
    Node::TsTypeAnn(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeAnn<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeAnn
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

fn get_view_for_ts_type_ann<'a>(inner: &'a swc_ast::TsTypeAnn, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeAnn<'a> {
  let node = bump.alloc(TsTypeAnn {
    inner,
    parent,
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.type_ann = get_view_for_ts_type(&inner.type_ann, parent, bump);
  node
}

pub struct ForStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ForStmt,
  pub init: Option<VarDeclOrExpr<'a>>,
  pub test: Option<Expr<'a>>,
  pub update: Option<Expr<'a>>,
  pub body: Stmt<'a>,
}

impl<'a> Spanned for ForStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ForStmt<'a>> for Node<'a> {
  fn from(node: &ForStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ForStmt<'a>, &'a ForStmt<'a>>(node) };
    Node::ForStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ForStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, } + match &self.test { Some(_value) => 1, None => 0, } + match &self.update { Some(_value) => 1, None => 0, });
    if let Some(child) = self.init.as_ref() {
      children.push(child.into());
    }
    if let Some(child) = self.test.as_ref() {
      children.push(child.into());
    }
    if let Some(child) = self.update.as_ref() {
      children.push(child.into());
    }
    children.push((&self.body).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ForStmt
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

fn get_view_for_for_stmt<'a>(inner: &'a swc_ast::ForStmt, parent: Node<'a>, bump: &'a Bump) -> &'a ForStmt<'a> {
  let node = bump.alloc(ForStmt {
    inner,
    parent,
    init: None,
    test: None,
    update: None,
    body: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.init = match &inner.init {
    Some(value) => Some(get_view_for_var_decl_or_expr(value, parent.clone(), bump)),
    None => None,
  };
  node.test = match &inner.test {
    Some(value) => Some(get_view_for_expr(value, parent.clone(), bump)),
    None => None,
  };
  node.update = match &inner.update {
    Some(value) => Some(get_view_for_expr(value, parent.clone(), bump)),
    None => None,
  };
  node.body = get_view_for_stmt(&inner.body, parent, bump);
  node
}

pub struct ObjectPat<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ObjectPat,
  pub props: Vec<ObjectPatProp<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
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
  fn from(node: &ObjectPat<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ObjectPat<'a>, &'a ObjectPat<'a>>(node) };
    Node::ObjectPat(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ObjectPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.props.len() + match &self.type_ann { Some(_value) => 1, None => 0, });
    for child in self.props.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ObjectPat
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

fn get_view_for_object_pat<'a>(inner: &'a swc_ast::ObjectPat, parent: Node<'a>, bump: &'a Bump) -> &'a ObjectPat<'a> {
  let node = bump.alloc(ObjectPat {
    inner,
    parent,
    props: Vec::with_capacity(inner.props.len()),
    type_ann: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.props.extend(inner.props.iter().map(|value| get_view_for_object_pat_prop(value, parent.clone(), bump)));
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent, bump)),
    None => None,
  };
  node
}

/// `typeof` operator
pub struct TsTypeQuery<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeQuery,
  pub expr_name: TsTypeQueryExpr<'a>,
}

impl<'a> Spanned for TsTypeQuery<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeQuery<'a>> for Node<'a> {
  fn from(node: &TsTypeQuery<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeQuery<'a>, &'a TsTypeQuery<'a>>(node) };
    Node::TsTypeQuery(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeQuery<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr_name).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeQuery
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

fn get_view_for_ts_type_query<'a>(inner: &'a swc_ast::TsTypeQuery, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeQuery<'a> {
  let node = bump.alloc(TsTypeQuery {
    inner,
    parent,
    expr_name: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr_name = get_view_for_ts_type_query_expr(&inner.expr_name, parent, bump);
  node
}

pub struct ThisExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ThisExpr,
}

impl<'a> Spanned for ThisExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ThisExpr<'a>> for Node<'a> {
  fn from(node: &ThisExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ThisExpr<'a>, &'a ThisExpr<'a>>(node) };
    Node::ThisExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ThisExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ThisExpr
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

fn get_view_for_this_expr<'a>(inner: &'a swc_ast::ThisExpr, parent: Node<'a>, bump: &'a Bump) -> &'a ThisExpr<'a> {
  let node = bump.alloc(ThisExpr {
    inner,
    parent,
  });
  node
}

pub struct DebuggerStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::DebuggerStmt,
}

impl<'a> Spanned for DebuggerStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&DebuggerStmt<'a>> for Node<'a> {
  fn from(node: &DebuggerStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&DebuggerStmt<'a>, &'a DebuggerStmt<'a>>(node) };
    Node::DebuggerStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a DebuggerStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::DebuggerStmt
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

fn get_view_for_debugger_stmt<'a>(inner: &'a swc_ast::DebuggerStmt, parent: Node<'a>, bump: &'a Bump) -> &'a DebuggerStmt<'a> {
  let node = bump.alloc(DebuggerStmt {
    inner,
    parent,
  });
  node
}

pub struct TsTypeParamDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeParamDecl,
  pub params: Vec<&'a TsTypeParam<'a>>,
}

impl<'a> Spanned for TsTypeParamDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeParamDecl<'a>> for Node<'a> {
  fn from(node: &TsTypeParamDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeParamDecl<'a>, &'a TsTypeParamDecl<'a>>(node) };
    Node::TsTypeParamDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeParamDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.params.len());
    for child in self.params.iter() {
      children.push((*child).into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeParamDecl
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

fn get_view_for_ts_type_param_decl<'a>(inner: &'a swc_ast::TsTypeParamDecl, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeParamDecl<'a> {
  let node = bump.alloc(TsTypeParamDecl {
    inner,
    parent,
    params: Vec::with_capacity(inner.params.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.params.extend(inner.params.iter().map(|value| get_view_for_ts_type_param(value, parent.clone(), bump)));
  node
}

pub struct TsTypeAssertion<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeAssertion,
  pub expr: Expr<'a>,
  pub type_ann: TsType<'a>,
}

impl<'a> Spanned for TsTypeAssertion<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeAssertion<'a>> for Node<'a> {
  fn from(node: &TsTypeAssertion<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeAssertion<'a>, &'a TsTypeAssertion<'a>>(node) };
    Node::TsTypeAssertion(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeAssertion<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push((&self.type_ann).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeAssertion
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

fn get_view_for_ts_type_assertion<'a>(inner: &'a swc_ast::TsTypeAssertion, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeAssertion<'a> {
  let node = bump.alloc(TsTypeAssertion {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent.clone(), bump);
  node.type_ann = get_view_for_ts_type(&inner.type_ann, parent, bump);
  node
}

pub struct TplElement<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TplElement,
  pub cooked: Option<&'a Str<'a>>,
  pub raw: &'a Str<'a>,
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
  fn from(node: &TplElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TplElement<'a>, &'a TplElement<'a>>(node) };
    Node::TplElement(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TplElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.cooked { Some(_value) => 1, None => 0, });
    if let Some(child) = self.cooked {
      children.push(child.into());
    }
    children.push(self.raw.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TplElement
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

fn get_view_for_tpl_element<'a>(inner: &'a swc_ast::TplElement, parent: Node<'a>, bump: &'a Bump) -> &'a TplElement<'a> {
  let node = bump.alloc(TplElement {
    inner,
    parent,
    cooked: None,
    raw: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.cooked = match &inner.cooked {
    Some(value) => Some(get_view_for_str(value, parent.clone(), bump)),
    None => None,
  };
  node.raw = get_view_for_str(&inner.raw, parent, bump);
  node
}

pub struct TsKeywordType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsKeywordType,
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
  fn from(node: &TsKeywordType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsKeywordType<'a>, &'a TsKeywordType<'a>>(node) };
    Node::TsKeywordType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsKeywordType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsKeywordType
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

fn get_view_for_ts_keyword_type<'a>(inner: &'a swc_ast::TsKeywordType, parent: Node<'a>, bump: &'a Bump) -> &'a TsKeywordType<'a> {
  let node = bump.alloc(TsKeywordType {
    inner,
    parent,
  });
  node
}

pub struct JSXSpreadChild<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::JSXSpreadChild,
  pub expr: Expr<'a>,
}

impl<'a> Spanned for JSXSpreadChild<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXSpreadChild<'a>> for Node<'a> {
  fn from(node: &JSXSpreadChild<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXSpreadChild<'a>, &'a JSXSpreadChild<'a>>(node) };
    Node::JSXSpreadChild(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXSpreadChild<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXSpreadChild
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

fn get_view_for_jsxspread_child<'a>(inner: &'a swc_ast::JSXSpreadChild, parent: Node<'a>, bump: &'a Bump) -> &'a JSXSpreadChild<'a> {
  let node = bump.alloc(JSXSpreadChild {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

pub struct TsIntersectionType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsIntersectionType,
  pub types: Vec<TsType<'a>>,
}

impl<'a> Spanned for TsIntersectionType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsIntersectionType<'a>> for Node<'a> {
  fn from(node: &TsIntersectionType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsIntersectionType<'a>, &'a TsIntersectionType<'a>>(node) };
    Node::TsIntersectionType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsIntersectionType<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsIntersectionType
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

fn get_view_for_ts_intersection_type<'a>(inner: &'a swc_ast::TsIntersectionType, parent: Node<'a>, bump: &'a Bump) -> &'a TsIntersectionType<'a> {
  let node = bump.alloc(TsIntersectionType {
    inner,
    parent,
    types: Vec::with_capacity(inner.types.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.types.extend(inner.types.iter().map(|value| get_view_for_ts_type(value, parent.clone(), bump)));
  node
}

pub struct MetaPropExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::MetaPropExpr,
  pub meta: &'a Ident<'a>,
  pub prop: &'a Ident<'a>,
}

impl<'a> Spanned for MetaPropExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&MetaPropExpr<'a>> for Node<'a> {
  fn from(node: &MetaPropExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&MetaPropExpr<'a>, &'a MetaPropExpr<'a>>(node) };
    Node::MetaPropExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a MetaPropExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.meta.into());
    children.push(self.prop.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::MetaPropExpr
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

fn get_view_for_meta_prop_expr<'a>(inner: &'a swc_ast::MetaPropExpr, parent: Node<'a>, bump: &'a Bump) -> &'a MetaPropExpr<'a> {
  let node = bump.alloc(MetaPropExpr {
    inner,
    parent,
    meta: unsafe { MaybeUninit::uninit().assume_init() },
    prop: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.meta = get_view_for_ident(&inner.meta, parent.clone(), bump);
  node.prop = get_view_for_ident(&inner.prop, parent, bump);
  node
}

pub struct ExprOrSpread<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ExprOrSpread,
  pub expr: Expr<'a>,
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
  fn from(node: &ExprOrSpread<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExprOrSpread<'a>, &'a ExprOrSpread<'a>>(node) };
    Node::ExprOrSpread(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ExprOrSpread<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExprOrSpread
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

fn get_view_for_expr_or_spread<'a>(inner: &'a swc_ast::ExprOrSpread, parent: Node<'a>, bump: &'a Bump) -> &'a ExprOrSpread<'a> {
  let node = bump.alloc(ExprOrSpread {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

pub struct TsArrayType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsArrayType,
  pub elem_type: TsType<'a>,
}

impl<'a> Spanned for TsArrayType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsArrayType<'a>> for Node<'a> {
  fn from(node: &TsArrayType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsArrayType<'a>, &'a TsArrayType<'a>>(node) };
    Node::TsArrayType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsArrayType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.elem_type).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsArrayType
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

fn get_view_for_ts_array_type<'a>(inner: &'a swc_ast::TsArrayType, parent: Node<'a>, bump: &'a Bump) -> &'a TsArrayType<'a> {
  let node = bump.alloc(TsArrayType {
    inner,
    parent,
    elem_type: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.elem_type = get_view_for_ts_type(&inner.elem_type, parent, bump);
  node
}

pub struct TsTypeRef<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeRef,
  pub type_name: TsEntityName<'a>,
  pub type_params: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for TsTypeRef<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeRef<'a>> for Node<'a> {
  fn from(node: &TsTypeRef<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeRef<'a>, &'a TsTypeRef<'a>>(node) };
    Node::TsTypeRef(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeRef<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.type_name).into());
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeRef
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

fn get_view_for_ts_type_ref<'a>(inner: &'a swc_ast::TsTypeRef, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeRef<'a> {
  let node = bump.alloc(TsTypeRef {
    inner,
    parent,
    type_name: unsafe { MaybeUninit::uninit().assume_init() },
    type_params: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.type_name = get_view_for_ts_entity_name(&inner.type_name, parent.clone(), bump);
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsThisType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsThisType,
}

impl<'a> Spanned for TsThisType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsThisType<'a>> for Node<'a> {
  fn from(node: &TsThisType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsThisType<'a>, &'a TsThisType<'a>>(node) };
    Node::TsThisType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsThisType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsThisType
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

fn get_view_for_ts_this_type<'a>(inner: &'a swc_ast::TsThisType, parent: Node<'a>, bump: &'a Bump) -> &'a TsThisType<'a> {
  let node = bump.alloc(TsThisType {
    inner,
    parent,
  });
  node
}

pub struct TryStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TryStmt,
  pub block: &'a BlockStmt<'a>,
  pub handler: Option<&'a CatchClause<'a>>,
  pub finalizer: Option<&'a BlockStmt<'a>>,
}

impl<'a> Spanned for TryStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TryStmt<'a>> for Node<'a> {
  fn from(node: &TryStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TryStmt<'a>, &'a TryStmt<'a>>(node) };
    Node::TryStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TryStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.handler { Some(_value) => 1, None => 0, } + match &self.finalizer { Some(_value) => 1, None => 0, });
    children.push(self.block.into());
    if let Some(child) = self.handler {
      children.push(child.into());
    }
    if let Some(child) = self.finalizer {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TryStmt
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

fn get_view_for_try_stmt<'a>(inner: &'a swc_ast::TryStmt, parent: Node<'a>, bump: &'a Bump) -> &'a TryStmt<'a> {
  let node = bump.alloc(TryStmt {
    inner,
    parent,
    block: unsafe { MaybeUninit::uninit().assume_init() },
    handler: None,
    finalizer: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.block = get_view_for_block_stmt(&inner.block, parent.clone(), bump);
  node.handler = match &inner.handler {
    Some(value) => Some(get_view_for_catch_clause(value, parent.clone(), bump)),
    None => None,
  };
  node.finalizer = match &inner.finalizer {
    Some(value) => Some(get_view_for_block_stmt(value, parent, bump)),
    None => None,
  };
  node
}

pub struct CallExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::CallExpr,
  pub callee: ExprOrSuper<'a>,
  pub args: Vec<&'a ExprOrSpread<'a>>,
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for CallExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&CallExpr<'a>> for Node<'a> {
  fn from(node: &CallExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&CallExpr<'a>, &'a CallExpr<'a>>(node) };
    Node::CallExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a CallExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.args.len() + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.callee).into());
    for child in self.args.iter() {
      children.push((*child).into());
    }
    if let Some(child) = self.type_args {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::CallExpr
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

fn get_view_for_call_expr<'a>(inner: &'a swc_ast::CallExpr, parent: Node<'a>, bump: &'a Bump) -> &'a CallExpr<'a> {
  let node = bump.alloc(CallExpr {
    inner,
    parent,
    callee: unsafe { MaybeUninit::uninit().assume_init() },
    args: Vec::with_capacity(inner.args.len()),
    type_args: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.callee = get_view_for_expr_or_super(&inner.callee, parent.clone(), bump);
  node.args.extend(inner.args.iter().map(|value| get_view_for_expr_or_spread(value, parent.clone(), bump)));
  node.type_args = match &inner.type_args {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsMappedType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsMappedType,
  pub type_param: &'a TsTypeParam<'a>,
  pub name_type: Option<TsType<'a>>,
  pub type_ann: Option<TsType<'a>>,
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
  fn from(node: &TsMappedType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsMappedType<'a>, &'a TsMappedType<'a>>(node) };
    Node::TsMappedType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsMappedType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.name_type { Some(_value) => 1, None => 0, } + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push(self.type_param.into());
    if let Some(child) = self.name_type.as_ref() {
      children.push(child.into());
    }
    if let Some(child) = self.type_ann.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsMappedType
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

fn get_view_for_ts_mapped_type<'a>(inner: &'a swc_ast::TsMappedType, parent: Node<'a>, bump: &'a Bump) -> &'a TsMappedType<'a> {
  let node = bump.alloc(TsMappedType {
    inner,
    parent,
    type_param: unsafe { MaybeUninit::uninit().assume_init() },
    name_type: None,
    type_ann: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.type_param = get_view_for_ts_type_param(&inner.type_param, parent.clone(), bump);
  node.name_type = match &inner.name_type {
    Some(value) => Some(get_view_for_ts_type(value, parent.clone(), bump)),
    None => None,
  };
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type(value, parent, bump)),
    None => None,
  };
  node
}

pub struct JSXExprContainer<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::JSXExprContainer,
  pub expr: JSXExpr<'a>,
}

impl<'a> Spanned for JSXExprContainer<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXExprContainer<'a>> for Node<'a> {
  fn from(node: &JSXExprContainer<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXExprContainer<'a>, &'a JSXExprContainer<'a>>(node) };
    Node::JSXExprContainer(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXExprContainer<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXExprContainer
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

fn get_view_for_jsxexpr_container<'a>(inner: &'a swc_ast::JSXExprContainer, parent: Node<'a>, bump: &'a Bump) -> &'a JSXExprContainer<'a> {
  let node = bump.alloc(JSXExprContainer {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_jsxexpr(&inner.expr, parent, bump);
  node
}

pub struct PrivateProp<'a> {
  pub parent: &'a Class<'a>,
  pub inner: &'a swc_ast::PrivateProp,
  pub key: &'a PrivateName<'a>,
  pub value: Option<Expr<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub decorators: Vec<&'a Decorator<'a>>,
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
  fn from(node: &PrivateProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&PrivateProp<'a>, &'a PrivateProp<'a>>(node) };
    Node::PrivateProp(node)
  }
}

impl<'a> NodeTrait<'a> for &'a PrivateProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, } + match &self.type_ann { Some(_value) => 1, None => 0, } + self.decorators.len());
    children.push(self.key.into());
    if let Some(child) = self.value.as_ref() {
      children.push(child.into());
    }
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    for child in self.decorators.iter() {
      children.push((*child).into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::PrivateProp
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

fn get_view_for_private_prop<'a>(inner: &'a swc_ast::PrivateProp, parent: Node<'a>, bump: &'a Bump) -> &'a PrivateProp<'a> {
  let node = bump.alloc(PrivateProp {
    inner,
    parent: parent.to::<Class>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    value: None,
    type_ann: None,
    decorators: Vec::with_capacity(inner.decorators.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_private_name(&inner.key, parent.clone(), bump);
  node.value = match &inner.value {
    Some(value) => Some(get_view_for_expr(value, parent.clone(), bump)),
    None => None,
  };
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent.clone(), bump)),
    None => None,
  };
  node.decorators.extend(inner.decorators.iter().map(|value| get_view_for_decorator(value, parent.clone(), bump)));
  node
}

/// TypeScript's own parser uses ExportAssignment for both `export default` and
/// `export =`. But for @babel/parser, `export default` is an ExportDefaultDecl,
/// so a TsExportAssignment is always `export =`.
pub struct TsExportAssignment<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsExportAssignment,
  pub expr: Expr<'a>,
}

impl<'a> Spanned for TsExportAssignment<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsExportAssignment<'a>> for Node<'a> {
  fn from(node: &TsExportAssignment<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsExportAssignment<'a>, &'a TsExportAssignment<'a>>(node) };
    Node::TsExportAssignment(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsExportAssignment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsExportAssignment
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

fn get_view_for_ts_export_assignment<'a>(inner: &'a swc_ast::TsExportAssignment, parent: Node<'a>, bump: &'a Bump) -> &'a TsExportAssignment<'a> {
  let node = bump.alloc(TsExportAssignment {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

pub struct TsInterfaceBody<'a> {
  pub parent: &'a TsInterfaceDecl<'a>,
  pub inner: &'a swc_ast::TsInterfaceBody,
  pub body: Vec<TsTypeElement<'a>>,
}

impl<'a> Spanned for TsInterfaceBody<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsInterfaceBody<'a>> for Node<'a> {
  fn from(node: &TsInterfaceBody<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsInterfaceBody<'a>, &'a TsInterfaceBody<'a>>(node) };
    Node::TsInterfaceBody(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsInterfaceBody<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsInterfaceBody
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

fn get_view_for_ts_interface_body<'a>(inner: &'a swc_ast::TsInterfaceBody, parent: Node<'a>, bump: &'a Bump) -> &'a TsInterfaceBody<'a> {
  let node = bump.alloc(TsInterfaceBody {
    inner,
    parent: parent.to::<TsInterfaceDecl>(),
    body: Vec::with_capacity(inner.body.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.body.extend(inner.body.iter().map(|value| get_view_for_ts_type_element(value, parent.clone(), bump)));
  node
}

pub struct TsTupleElement<'a> {
  pub parent: &'a TsTupleType<'a>,
  pub inner: &'a swc_ast::TsTupleElement,
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
  fn from(node: &TsTupleElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTupleElement<'a>, &'a TsTupleElement<'a>>(node) };
    Node::TsTupleElement(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTupleElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = self.label.as_ref() {
      children.push(child.into());
    }
    children.push((&self.ty).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTupleElement
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

fn get_view_for_ts_tuple_element<'a>(inner: &'a swc_ast::TsTupleElement, parent: Node<'a>, bump: &'a Bump) -> &'a TsTupleElement<'a> {
  let node = bump.alloc(TsTupleElement {
    inner,
    parent: parent.to::<TsTupleType>(),
    label: None,
    ty: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.label = match &inner.label {
    Some(value) => Some(get_view_for_pat(value, parent.clone(), bump)),
    None => None,
  };
  node.ty = get_view_for_ts_type(&inner.ty, parent, bump);
  node
}

pub struct VarDeclarator<'a> {
  pub parent: &'a VarDecl<'a>,
  pub inner: &'a swc_ast::VarDeclarator,
  pub name: Pat<'a>,
  /// Initialization expresion.
  pub init: Option<Expr<'a>>,
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
  fn from(node: &VarDeclarator<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&VarDeclarator<'a>, &'a VarDeclarator<'a>>(node) };
    Node::VarDeclarator(node)
  }
}

impl<'a> NodeTrait<'a> for &'a VarDeclarator<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    if let Some(child) = self.init.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::VarDeclarator
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

fn get_view_for_var_declarator<'a>(inner: &'a swc_ast::VarDeclarator, parent: Node<'a>, bump: &'a Bump) -> &'a VarDeclarator<'a> {
  let node = bump.alloc(VarDeclarator {
    inner,
    parent: parent.to::<VarDecl>(),
    name: unsafe { MaybeUninit::uninit().assume_init() },
    init: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.name = get_view_for_pat(&inner.name, parent.clone(), bump);
  node.init = match &inner.init {
    Some(value) => Some(get_view_for_expr(value, parent, bump)),
    None => None,
  };
  node
}

pub struct JSXMemberExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::JSXMemberExpr,
  pub obj: JSXObject<'a>,
  pub prop: &'a Ident<'a>,
}

impl<'a> Spanned for JSXMemberExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXMemberExpr<'a>> for Node<'a> {
  fn from(node: &JSXMemberExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXMemberExpr<'a>, &'a JSXMemberExpr<'a>>(node) };
    Node::JSXMemberExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXMemberExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push(self.prop.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXMemberExpr
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

fn get_view_for_jsxmember_expr<'a>(inner: &'a swc_ast::JSXMemberExpr, parent: Node<'a>, bump: &'a Bump) -> &'a JSXMemberExpr<'a> {
  let node = bump.alloc(JSXMemberExpr {
    inner,
    parent,
    obj: unsafe { MaybeUninit::uninit().assume_init() },
    prop: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.obj = get_view_for_jsxobject(&inner.obj, parent.clone(), bump);
  node.prop = get_view_for_ident(&inner.prop, parent, bump);
  node
}

pub struct TsConstAssertion<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsConstAssertion,
  pub expr: Expr<'a>,
}

impl<'a> Spanned for TsConstAssertion<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsConstAssertion<'a>> for Node<'a> {
  fn from(node: &TsConstAssertion<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsConstAssertion<'a>, &'a TsConstAssertion<'a>>(node) };
    Node::TsConstAssertion(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsConstAssertion<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsConstAssertion
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

fn get_view_for_ts_const_assertion<'a>(inner: &'a swc_ast::TsConstAssertion, parent: Node<'a>, bump: &'a Bump) -> &'a TsConstAssertion<'a> {
  let node = bump.alloc(TsConstAssertion {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

/// `export * as foo from 'src';`
pub struct ExportNamespaceSpecifier<'a> {
  pub parent: &'a NamedExport<'a>,
  pub inner: &'a swc_ast::ExportNamespaceSpecifier,
  pub name: &'a Ident<'a>,
}

impl<'a> Spanned for ExportNamespaceSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportNamespaceSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportNamespaceSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportNamespaceSpecifier<'a>, &'a ExportNamespaceSpecifier<'a>>(node) };
    Node::ExportNamespaceSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ExportNamespaceSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.name.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportNamespaceSpecifier
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

fn get_view_for_export_namespace_specifier<'a>(inner: &'a swc_ast::ExportNamespaceSpecifier, parent: Node<'a>, bump: &'a Bump) -> &'a ExportNamespaceSpecifier<'a> {
  let node = bump.alloc(ExportNamespaceSpecifier {
    inner,
    parent: parent.to::<NamedExport>(),
    name: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.name = get_view_for_ident(&inner.name, parent, bump);
  node
}

/// Object literal.
pub struct ObjectLit<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ObjectLit,
  pub props: Vec<PropOrSpread<'a>>,
}

impl<'a> Spanned for ObjectLit<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ObjectLit<'a>> for Node<'a> {
  fn from(node: &ObjectLit<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ObjectLit<'a>, &'a ObjectLit<'a>>(node) };
    Node::ObjectLit(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ObjectLit<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ObjectLit
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

fn get_view_for_object_lit<'a>(inner: &'a swc_ast::ObjectLit, parent: Node<'a>, bump: &'a Bump) -> &'a ObjectLit<'a> {
  let node = bump.alloc(ObjectLit {
    inner,
    parent,
    props: Vec::with_capacity(inner.props.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.props.extend(inner.props.iter().map(|value| get_view_for_prop_or_spread(value, parent.clone(), bump)));
  node
}

pub struct Module<'a> {
  pub source_file: Option<&'a swc_common::SourceFile>,
  pub tokens: Option<&'a TokenContainer<'a>>,
  pub comments: Option<&'a CommentContainer<'a>>,
  pub inner: &'a swc_ast::Module,
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
  fn from(node: &Module<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Module<'a>, &'a Module<'a>>(node) };
    Node::Module(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Module<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Module
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

fn get_view_for_module<'a>(source_file_info: &'a SourceFileInfo<'a>, bump: &'a Bump) -> &'a Module<'a> {
  let inner = source_file_info.module;
  let node = bump.alloc(Module {
    inner,
    source_file: source_file_info.source_file,
    tokens: source_file_info.tokens,
    comments: source_file_info.comments,
    body: Vec::with_capacity(inner.body.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.body.extend(inner.body.iter().map(|value| get_view_for_module_item(value, parent.clone(), bump)));
  node
}

pub struct TsIndexSignature<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsIndexSignature,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
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
  fn from(node: &TsIndexSignature<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsIndexSignature<'a>, &'a TsIndexSignature<'a>>(node) };
    Node::TsIndexSignature(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsIndexSignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.params.len() + match &self.type_ann { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsIndexSignature
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

fn get_view_for_ts_index_signature<'a>(inner: &'a swc_ast::TsIndexSignature, parent: Node<'a>, bump: &'a Bump) -> &'a TsIndexSignature<'a> {
  let node = bump.alloc(TsIndexSignature {
    inner,
    parent,
    params: Vec::with_capacity(inner.params.len()),
    type_ann: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.params.extend(inner.params.iter().map(|value| get_view_for_ts_fn_param(value, parent.clone(), bump)));
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsTypeCastExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeCastExpr,
  pub expr: Expr<'a>,
  pub type_ann: &'a TsTypeAnn<'a>,
}

impl<'a> Spanned for TsTypeCastExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeCastExpr<'a>> for Node<'a> {
  fn from(node: &TsTypeCastExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeCastExpr<'a>, &'a TsTypeCastExpr<'a>>(node) };
    Node::TsTypeCastExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeCastExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push(self.type_ann.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeCastExpr
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

fn get_view_for_ts_type_cast_expr<'a>(inner: &'a swc_ast::TsTypeCastExpr, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeCastExpr<'a> {
  let node = bump.alloc(TsTypeCastExpr {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent.clone(), bump);
  node.type_ann = get_view_for_ts_type_ann(&inner.type_ann, parent, bump);
  node
}

pub struct TsTupleType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTupleType,
  pub elem_types: Vec<&'a TsTupleElement<'a>>,
}

impl<'a> Spanned for TsTupleType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTupleType<'a>> for Node<'a> {
  fn from(node: &TsTupleType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTupleType<'a>, &'a TsTupleType<'a>>(node) };
    Node::TsTupleType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTupleType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.elem_types.len());
    for child in self.elem_types.iter() {
      children.push((*child).into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTupleType
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

fn get_view_for_ts_tuple_type<'a>(inner: &'a swc_ast::TsTupleType, parent: Node<'a>, bump: &'a Bump) -> &'a TsTupleType<'a> {
  let node = bump.alloc(TsTupleType {
    inner,
    parent,
    elem_types: Vec::with_capacity(inner.elem_types.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.elem_types.extend(inner.elem_types.iter().map(|value| get_view_for_ts_tuple_element(value, parent.clone(), bump)));
  node
}

pub struct Null<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Null,
}

impl<'a> Spanned for Null<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Null<'a>> for Node<'a> {
  fn from(node: &Null<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Null<'a>, &'a Null<'a>>(node) };
    Node::Null(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Null<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Null
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

fn get_view_for_null<'a>(inner: &'a swc_ast::Null, parent: Node<'a>, bump: &'a Bump) -> &'a Null<'a> {
  let node = bump.alloc(Null {
    inner,
    parent,
  });
  node
}

pub struct TsTypeOperator<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeOperator,
  pub type_ann: TsType<'a>,
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
  fn from(node: &TsTypeOperator<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeOperator<'a>, &'a TsTypeOperator<'a>>(node) };
    Node::TsTypeOperator(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeOperator<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeOperator
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

fn get_view_for_ts_type_operator<'a>(inner: &'a swc_ast::TsTypeOperator, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeOperator<'a> {
  let node = bump.alloc(TsTypeOperator {
    inner,
    parent,
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.type_ann = get_view_for_ts_type(&inner.type_ann, parent, bump);
  node
}

pub struct JSXClosingElement<'a> {
  pub parent: &'a JSXElement<'a>,
  pub inner: &'a swc_ast::JSXClosingElement,
  pub name: JSXElementName<'a>,
}

impl<'a> Spanned for JSXClosingElement<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXClosingElement<'a>> for Node<'a> {
  fn from(node: &JSXClosingElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXClosingElement<'a>, &'a JSXClosingElement<'a>>(node) };
    Node::JSXClosingElement(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXClosingElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.name).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXClosingElement
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

fn get_view_for_jsxclosing_element<'a>(inner: &'a swc_ast::JSXClosingElement, parent: Node<'a>, bump: &'a Bump) -> &'a JSXClosingElement<'a> {
  let node = bump.alloc(JSXClosingElement {
    inner,
    parent: parent.to::<JSXElement>(),
    name: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.name = get_view_for_jsxelement_name(&inner.name, parent, bump);
  node
}

pub struct BinExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::BinExpr,
  pub left: Expr<'a>,
  pub right: Expr<'a>,
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
  fn from(node: &BinExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&BinExpr<'a>, &'a BinExpr<'a>>(node) };
    Node::BinExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a BinExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::BinExpr
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

fn get_view_for_bin_expr<'a>(inner: &'a swc_ast::BinExpr, parent: Node<'a>, bump: &'a Bump) -> &'a BinExpr<'a> {
  let node = bump.alloc(BinExpr {
    inner,
    parent,
    left: unsafe { MaybeUninit::uninit().assume_init() },
    right: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.left = get_view_for_expr(&inner.left, parent.clone(), bump);
  node.right = get_view_for_expr(&inner.right, parent, bump);
  node
}

pub struct UnaryExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::UnaryExpr,
  pub arg: Expr<'a>,
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
  fn from(node: &UnaryExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&UnaryExpr<'a>, &'a UnaryExpr<'a>>(node) };
    Node::UnaryExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a UnaryExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::UnaryExpr
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

fn get_view_for_unary_expr<'a>(inner: &'a swc_ast::UnaryExpr, parent: Node<'a>, bump: &'a Bump) -> &'a UnaryExpr<'a> {
  let node = bump.alloc(UnaryExpr {
    inner,
    parent,
    arg: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.arg = get_view_for_expr(&inner.arg, parent, bump);
  node
}

pub struct TsPropertySignature<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsPropertySignature,
  pub key: Expr<'a>,
  pub init: Option<Expr<'a>>,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
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
  fn from(node: &TsPropertySignature<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsPropertySignature<'a>, &'a TsPropertySignature<'a>>(node) };
    Node::TsPropertySignature(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsPropertySignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, } + self.params.len() + match &self.type_ann { Some(_value) => 1, None => 0, } + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    if let Some(child) = self.init.as_ref() {
      children.push(child.into());
    }
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsPropertySignature
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

fn get_view_for_ts_property_signature<'a>(inner: &'a swc_ast::TsPropertySignature, parent: Node<'a>, bump: &'a Bump) -> &'a TsPropertySignature<'a> {
  let node = bump.alloc(TsPropertySignature {
    inner,
    parent,
    key: unsafe { MaybeUninit::uninit().assume_init() },
    init: None,
    params: Vec::with_capacity(inner.params.len()),
    type_ann: None,
    type_params: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_expr(&inner.key, parent.clone(), bump);
  node.init = match &inner.init {
    Some(value) => Some(get_view_for_expr(value, parent.clone(), bump)),
    None => None,
  };
  node.params.extend(inner.params.iter().map(|value| get_view_for_ts_fn_param(value, parent.clone(), bump)));
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent.clone(), bump)),
    None => None,
  };
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent, bump)),
    None => None,
  };
  node
}

pub struct Constructor<'a> {
  pub parent: &'a Class<'a>,
  pub inner: &'a swc_ast::Constructor,
  pub key: PropName<'a>,
  pub params: Vec<ParamOrTsParamProp<'a>>,
  pub body: Option<&'a BlockStmt<'a>>,
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
  fn from(node: &Constructor<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Constructor<'a>, &'a Constructor<'a>>(node) };
    Node::Constructor(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Constructor<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.params.len() + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    for child in self.params.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.body {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Constructor
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

fn get_view_for_constructor<'a>(inner: &'a swc_ast::Constructor, parent: Node<'a>, bump: &'a Bump) -> &'a Constructor<'a> {
  let node = bump.alloc(Constructor {
    inner,
    parent: parent.to::<Class>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    params: Vec::with_capacity(inner.params.len()),
    body: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_prop_name(&inner.key, parent.clone(), bump);
  node.params.extend(inner.params.iter().map(|value| get_view_for_param_or_ts_param_prop(value, parent.clone(), bump)));
  node.body = match &inner.body {
    Some(value) => Some(get_view_for_block_stmt(value, parent, bump)),
    None => None,
  };
  node
}

pub struct FnDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::FnDecl,
  pub ident: &'a Ident<'a>,
  pub function: &'a Function<'a>,
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
  fn from(node: &FnDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&FnDecl<'a>, &'a FnDecl<'a>>(node) };
    Node::FnDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a FnDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.ident.into());
    children.push(self.function.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::FnDecl
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

fn get_view_for_fn_decl<'a>(inner: &'a swc_ast::FnDecl, parent: Node<'a>, bump: &'a Bump) -> &'a FnDecl<'a> {
  let node = bump.alloc(FnDecl {
    inner,
    parent,
    ident: unsafe { MaybeUninit::uninit().assume_init() },
    function: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.ident = get_view_for_ident(&inner.ident, parent.clone(), bump);
  node.function = get_view_for_function(&inner.function, parent, bump);
  node
}

pub struct TsNonNullExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsNonNullExpr,
  pub expr: Expr<'a>,
}

impl<'a> Spanned for TsNonNullExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsNonNullExpr<'a>> for Node<'a> {
  fn from(node: &TsNonNullExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsNonNullExpr<'a>, &'a TsNonNullExpr<'a>>(node) };
    Node::TsNonNullExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsNonNullExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsNonNullExpr
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

fn get_view_for_ts_non_null_expr<'a>(inner: &'a swc_ast::TsNonNullExpr, parent: Node<'a>, bump: &'a Bump) -> &'a TsNonNullExpr<'a> {
  let node = bump.alloc(TsNonNullExpr {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

/// Class expression.
pub struct ClassExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ClassExpr,
  pub ident: Option<&'a Ident<'a>>,
  pub class: &'a Class<'a>,
}

impl<'a> Spanned for ClassExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ClassExpr<'a>> for Node<'a> {
  fn from(node: &ClassExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ClassExpr<'a>, &'a ClassExpr<'a>>(node) };
    Node::ClassExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ClassExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.ident { Some(_value) => 1, None => 0, });
    if let Some(child) = self.ident {
      children.push(child.into());
    }
    children.push(self.class.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ClassExpr
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

fn get_view_for_class_expr<'a>(inner: &'a swc_ast::ClassExpr, parent: Node<'a>, bump: &'a Bump) -> &'a ClassExpr<'a> {
  let node = bump.alloc(ClassExpr {
    inner,
    parent,
    ident: None,
    class: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.ident = match &inner.ident {
    Some(value) => Some(get_view_for_ident(value, parent.clone(), bump)),
    None => None,
  };
  node.class = get_view_for_class(&inner.class, parent, bump);
  node
}

pub struct ForInStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ForInStmt,
  pub left: VarDeclOrPat<'a>,
  pub right: Expr<'a>,
  pub body: Stmt<'a>,
}

impl<'a> Spanned for ForInStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ForInStmt<'a>> for Node<'a> {
  fn from(node: &ForInStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ForInStmt<'a>, &'a ForInStmt<'a>>(node) };
    Node::ForInStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ForInStmt<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ForInStmt
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

fn get_view_for_for_in_stmt<'a>(inner: &'a swc_ast::ForInStmt, parent: Node<'a>, bump: &'a Bump) -> &'a ForInStmt<'a> {
  let node = bump.alloc(ForInStmt {
    inner,
    parent,
    left: unsafe { MaybeUninit::uninit().assume_init() },
    right: unsafe { MaybeUninit::uninit().assume_init() },
    body: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.left = get_view_for_var_decl_or_pat(&inner.left, parent.clone(), bump);
  node.right = get_view_for_expr(&inner.right, parent.clone(), bump);
  node.body = get_view_for_stmt(&inner.body, parent, bump);
  node
}

pub struct EmptyStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::EmptyStmt,
}

impl<'a> Spanned for EmptyStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&EmptyStmt<'a>> for Node<'a> {
  fn from(node: &EmptyStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&EmptyStmt<'a>, &'a EmptyStmt<'a>>(node) };
    Node::EmptyStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a EmptyStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::EmptyStmt
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

fn get_view_for_empty_stmt<'a>(inner: &'a swc_ast::EmptyStmt, parent: Node<'a>, bump: &'a Bump) -> &'a EmptyStmt<'a> {
  let node = bump.alloc(EmptyStmt {
    inner,
    parent,
  });
  node
}

pub struct WhileStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::WhileStmt,
  pub test: Expr<'a>,
  pub body: Stmt<'a>,
}

impl<'a> Spanned for WhileStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&WhileStmt<'a>> for Node<'a> {
  fn from(node: &WhileStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&WhileStmt<'a>, &'a WhileStmt<'a>>(node) };
    Node::WhileStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a WhileStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.test).into());
    children.push((&self.body).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::WhileStmt
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

fn get_view_for_while_stmt<'a>(inner: &'a swc_ast::WhileStmt, parent: Node<'a>, bump: &'a Bump) -> &'a WhileStmt<'a> {
  let node = bump.alloc(WhileStmt {
    inner,
    parent,
    test: unsafe { MaybeUninit::uninit().assume_init() },
    body: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.test = get_view_for_expr(&inner.test, parent.clone(), bump);
  node.body = get_view_for_stmt(&inner.body, parent, bump);
  node
}

pub struct Str<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Str,
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
  fn from(node: &Str<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Str<'a>, &'a Str<'a>>(node) };
    Node::Str(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Str<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Str
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

fn get_view_for_str<'a>(inner: &'a swc_ast::Str, parent: Node<'a>, bump: &'a Bump) -> &'a Str<'a> {
  let node = bump.alloc(Str {
    inner,
    parent,
  });
  node
}

pub struct TsExprWithTypeArgs<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsExprWithTypeArgs,
  pub expr: TsEntityName<'a>,
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> Spanned for TsExprWithTypeArgs<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsExprWithTypeArgs<'a>> for Node<'a> {
  fn from(node: &TsExprWithTypeArgs<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsExprWithTypeArgs<'a>, &'a TsExprWithTypeArgs<'a>>(node) };
    Node::TsExprWithTypeArgs(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsExprWithTypeArgs<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.expr).into());
    if let Some(child) = self.type_args {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsExprWithTypeArgs
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

fn get_view_for_ts_expr_with_type_args<'a>(inner: &'a swc_ast::TsExprWithTypeArgs, parent: Node<'a>, bump: &'a Bump) -> &'a TsExprWithTypeArgs<'a> {
  let node = bump.alloc(TsExprWithTypeArgs {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
    type_args: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_ts_entity_name(&inner.expr, parent.clone(), bump);
  node.type_args = match &inner.type_args {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value, parent, bump)),
    None => None,
  };
  node
}

pub struct AssignPat<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::AssignPat,
  pub left: Pat<'a>,
  pub right: Expr<'a>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> Spanned for AssignPat<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&AssignPat<'a>> for Node<'a> {
  fn from(node: &AssignPat<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&AssignPat<'a>, &'a AssignPat<'a>>(node) };
    Node::AssignPat(node)
  }
}

impl<'a> NodeTrait<'a> for &'a AssignPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.left).into());
    children.push((&self.right).into());
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::AssignPat
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

fn get_view_for_assign_pat<'a>(inner: &'a swc_ast::AssignPat, parent: Node<'a>, bump: &'a Bump) -> &'a AssignPat<'a> {
  let node = bump.alloc(AssignPat {
    inner,
    parent,
    left: unsafe { MaybeUninit::uninit().assume_init() },
    right: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.left = get_view_for_pat(&inner.left, parent.clone(), bump);
  node.right = get_view_for_expr(&inner.right, parent.clone(), bump);
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent, bump)),
    None => None,
  };
  node
}

pub struct ExportNamedSpecifier<'a> {
  pub parent: &'a NamedExport<'a>,
  pub inner: &'a swc_ast::ExportNamedSpecifier,
  /// `foo` in `export { foo as bar }`
  pub orig: &'a Ident<'a>,
  /// `Some(bar)` in `export { foo as bar }`
  pub exported: Option<&'a Ident<'a>>,
}

impl<'a> Spanned for ExportNamedSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportNamedSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportNamedSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportNamedSpecifier<'a>, &'a ExportNamedSpecifier<'a>>(node) };
    Node::ExportNamedSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ExportNamedSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.exported { Some(_value) => 1, None => 0, });
    children.push(self.orig.into());
    if let Some(child) = self.exported {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportNamedSpecifier
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

fn get_view_for_export_named_specifier<'a>(inner: &'a swc_ast::ExportNamedSpecifier, parent: Node<'a>, bump: &'a Bump) -> &'a ExportNamedSpecifier<'a> {
  let node = bump.alloc(ExportNamedSpecifier {
    inner,
    parent: parent.to::<NamedExport>(),
    orig: unsafe { MaybeUninit::uninit().assume_init() },
    exported: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.orig = get_view_for_ident(&inner.orig, parent.clone(), bump);
  node.exported = match &inner.exported {
    Some(value) => Some(get_view_for_ident(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsConditionalType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsConditionalType,
  pub check_type: TsType<'a>,
  pub extends_type: TsType<'a>,
  pub true_type: TsType<'a>,
  pub false_type: TsType<'a>,
}

impl<'a> Spanned for TsConditionalType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsConditionalType<'a>> for Node<'a> {
  fn from(node: &TsConditionalType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsConditionalType<'a>, &'a TsConditionalType<'a>>(node) };
    Node::TsConditionalType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsConditionalType<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsConditionalType
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

fn get_view_for_ts_conditional_type<'a>(inner: &'a swc_ast::TsConditionalType, parent: Node<'a>, bump: &'a Bump) -> &'a TsConditionalType<'a> {
  let node = bump.alloc(TsConditionalType {
    inner,
    parent,
    check_type: unsafe { MaybeUninit::uninit().assume_init() },
    extends_type: unsafe { MaybeUninit::uninit().assume_init() },
    true_type: unsafe { MaybeUninit::uninit().assume_init() },
    false_type: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.check_type = get_view_for_ts_type(&inner.check_type, parent.clone(), bump);
  node.extends_type = get_view_for_ts_type(&inner.extends_type, parent.clone(), bump);
  node.true_type = get_view_for_ts_type(&inner.true_type, parent.clone(), bump);
  node.false_type = get_view_for_ts_type(&inner.false_type, parent, bump);
  node
}

pub struct TsTypeLit<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsTypeLit,
  pub members: Vec<TsTypeElement<'a>>,
}

impl<'a> Spanned for TsTypeLit<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsTypeLit<'a>> for Node<'a> {
  fn from(node: &TsTypeLit<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeLit<'a>, &'a TsTypeLit<'a>>(node) };
    Node::TsTypeLit(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsTypeLit<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeLit
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

fn get_view_for_ts_type_lit<'a>(inner: &'a swc_ast::TsTypeLit, parent: Node<'a>, bump: &'a Bump) -> &'a TsTypeLit<'a> {
  let node = bump.alloc(TsTypeLit {
    inner,
    parent,
    members: Vec::with_capacity(inner.members.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.members.extend(inner.members.iter().map(|value| get_view_for_ts_type_element(value, parent.clone(), bump)));
  node
}

pub struct BreakStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::BreakStmt,
  pub label: Option<&'a Ident<'a>>,
}

impl<'a> Spanned for BreakStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&BreakStmt<'a>> for Node<'a> {
  fn from(node: &BreakStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&BreakStmt<'a>, &'a BreakStmt<'a>>(node) };
    Node::BreakStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a BreakStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = self.label {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::BreakStmt
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

fn get_view_for_break_stmt<'a>(inner: &'a swc_ast::BreakStmt, parent: Node<'a>, bump: &'a Bump) -> &'a BreakStmt<'a> {
  let node = bump.alloc(BreakStmt {
    inner,
    parent,
    label: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.label = match &inner.label {
    Some(value) => Some(get_view_for_ident(value, parent, bump)),
    None => None,
  };
  node
}

/// e.g. `import * as foo from 'mod.js'`.
pub struct ImportStarAsSpecifier<'a> {
  pub parent: &'a ImportDecl<'a>,
  pub inner: &'a swc_ast::ImportStarAsSpecifier,
  pub local: &'a Ident<'a>,
}

impl<'a> Spanned for ImportStarAsSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ImportStarAsSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportStarAsSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ImportStarAsSpecifier<'a>, &'a ImportStarAsSpecifier<'a>>(node) };
    Node::ImportStarAsSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ImportStarAsSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.local.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ImportStarAsSpecifier
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

fn get_view_for_import_star_as_specifier<'a>(inner: &'a swc_ast::ImportStarAsSpecifier, parent: Node<'a>, bump: &'a Bump) -> &'a ImportStarAsSpecifier<'a> {
  let node = bump.alloc(ImportStarAsSpecifier {
    inner,
    parent: parent.to::<ImportDecl>(),
    local: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.local = get_view_for_ident(&inner.local, parent, bump);
  node
}

pub struct TsInferType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsInferType,
  pub type_param: &'a TsTypeParam<'a>,
}

impl<'a> Spanned for TsInferType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsInferType<'a>> for Node<'a> {
  fn from(node: &TsInferType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsInferType<'a>, &'a TsInferType<'a>>(node) };
    Node::TsInferType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsInferType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.type_param.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsInferType
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

fn get_view_for_ts_infer_type<'a>(inner: &'a swc_ast::TsInferType, parent: Node<'a>, bump: &'a Bump) -> &'a TsInferType<'a> {
  let node = bump.alloc(TsInferType {
    inner,
    parent,
    type_param: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.type_param = get_view_for_ts_type_param(&inner.type_param, parent, bump);
  node
}

pub struct PrivateMethod<'a> {
  pub parent: &'a Class<'a>,
  pub inner: &'a swc_ast::PrivateMethod,
  pub key: &'a PrivateName<'a>,
  pub function: &'a Function<'a>,
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
  fn from(node: &PrivateMethod<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&PrivateMethod<'a>, &'a PrivateMethod<'a>>(node) };
    Node::PrivateMethod(node)
  }
}

impl<'a> NodeTrait<'a> for &'a PrivateMethod<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.key.into());
    children.push(self.function.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::PrivateMethod
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

fn get_view_for_private_method<'a>(inner: &'a swc_ast::PrivateMethod, parent: Node<'a>, bump: &'a Bump) -> &'a PrivateMethod<'a> {
  let node = bump.alloc(PrivateMethod {
    inner,
    parent: parent.to::<Class>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    function: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_private_name(&inner.key, parent.clone(), bump);
  node.function = get_view_for_function(&inner.function, parent, bump);
  node
}

pub struct ForOfStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ForOfStmt,
  pub left: VarDeclOrPat<'a>,
  pub right: Expr<'a>,
  pub body: Stmt<'a>,
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
  fn from(node: &ForOfStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ForOfStmt<'a>, &'a ForOfStmt<'a>>(node) };
    Node::ForOfStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ForOfStmt<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ForOfStmt
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

fn get_view_for_for_of_stmt<'a>(inner: &'a swc_ast::ForOfStmt, parent: Node<'a>, bump: &'a Bump) -> &'a ForOfStmt<'a> {
  let node = bump.alloc(ForOfStmt {
    inner,
    parent,
    left: unsafe { MaybeUninit::uninit().assume_init() },
    right: unsafe { MaybeUninit::uninit().assume_init() },
    body: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.left = get_view_for_var_decl_or_pat(&inner.left, parent.clone(), bump);
  node.right = get_view_for_expr(&inner.right, parent.clone(), bump);
  node.body = get_view_for_stmt(&inner.body, parent, bump);
  node
}

pub struct TsUnionType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsUnionType,
  pub types: Vec<TsType<'a>>,
}

impl<'a> Spanned for TsUnionType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsUnionType<'a>> for Node<'a> {
  fn from(node: &TsUnionType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsUnionType<'a>, &'a TsUnionType<'a>>(node) };
    Node::TsUnionType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsUnionType<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsUnionType
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

fn get_view_for_ts_union_type<'a>(inner: &'a swc_ast::TsUnionType, parent: Node<'a>, bump: &'a Bump) -> &'a TsUnionType<'a> {
  let node = bump.alloc(TsUnionType {
    inner,
    parent,
    types: Vec::with_capacity(inner.types.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.types.extend(inner.types.iter().map(|value| get_view_for_ts_type(value, parent.clone(), bump)));
  node
}

pub struct TsModuleDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsModuleDecl,
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
  fn from(node: &TsModuleDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsModuleDecl<'a>, &'a TsModuleDecl<'a>>(node) };
    Node::TsModuleDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsModuleDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.id).into());
    if let Some(child) = self.body.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsModuleDecl
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

fn get_view_for_ts_module_decl<'a>(inner: &'a swc_ast::TsModuleDecl, parent: Node<'a>, bump: &'a Bump) -> &'a TsModuleDecl<'a> {
  let node = bump.alloc(TsModuleDecl {
    inner,
    parent,
    id: unsafe { MaybeUninit::uninit().assume_init() },
    body: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.id = get_view_for_ts_module_name(&inner.id, parent.clone(), bump);
  node.body = match &inner.body {
    Some(value) => Some(get_view_for_ts_namespace_body(value, parent, bump)),
    None => None,
  };
  node
}

pub struct GetterProp<'a> {
  pub parent: &'a ObjectLit<'a>,
  pub inner: &'a swc_ast::GetterProp,
  pub key: PropName<'a>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub body: Option<&'a BlockStmt<'a>>,
}

impl<'a> Spanned for GetterProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&GetterProp<'a>> for Node<'a> {
  fn from(node: &GetterProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&GetterProp<'a>, &'a GetterProp<'a>>(node) };
    Node::GetterProp(node)
  }
}

impl<'a> NodeTrait<'a> for &'a GetterProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, } + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    if let Some(child) = self.body {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::GetterProp
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

fn get_view_for_getter_prop<'a>(inner: &'a swc_ast::GetterProp, parent: Node<'a>, bump: &'a Bump) -> &'a GetterProp<'a> {
  let node = bump.alloc(GetterProp {
    inner,
    parent: parent.to::<ObjectLit>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: None,
    body: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_prop_name(&inner.key, parent.clone(), bump);
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent.clone(), bump)),
    None => None,
  };
  node.body = match &inner.body {
    Some(value) => Some(get_view_for_block_stmt(value, parent, bump)),
    None => None,
  };
  node
}

pub struct CondExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::CondExpr,
  pub test: Expr<'a>,
  pub cons: Expr<'a>,
  pub alt: Expr<'a>,
}

impl<'a> Spanned for CondExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&CondExpr<'a>> for Node<'a> {
  fn from(node: &CondExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&CondExpr<'a>, &'a CondExpr<'a>>(node) };
    Node::CondExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a CondExpr<'a> {
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

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::CondExpr
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

fn get_view_for_cond_expr<'a>(inner: &'a swc_ast::CondExpr, parent: Node<'a>, bump: &'a Bump) -> &'a CondExpr<'a> {
  let node = bump.alloc(CondExpr {
    inner,
    parent,
    test: unsafe { MaybeUninit::uninit().assume_init() },
    cons: unsafe { MaybeUninit::uninit().assume_init() },
    alt: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.test = get_view_for_expr(&inner.test, parent.clone(), bump);
  node.cons = get_view_for_expr(&inner.cons, parent.clone(), bump);
  node.alt = get_view_for_expr(&inner.alt, parent, bump);
  node
}

/// e.g. local = foo, imported = None `import { foo } from 'mod.js'`
/// e.g. local = bar, imported = Some(foo) for `import { foo as bar } from
/// 'mod.js'`
pub struct ImportNamedSpecifier<'a> {
  pub parent: &'a ImportDecl<'a>,
  pub inner: &'a swc_ast::ImportNamedSpecifier,
  pub local: &'a Ident<'a>,
  pub imported: Option<&'a Ident<'a>>,
}

impl<'a> Spanned for ImportNamedSpecifier<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ImportNamedSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportNamedSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ImportNamedSpecifier<'a>, &'a ImportNamedSpecifier<'a>>(node) };
    Node::ImportNamedSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ImportNamedSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.imported { Some(_value) => 1, None => 0, });
    children.push(self.local.into());
    if let Some(child) = self.imported {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ImportNamedSpecifier
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

fn get_view_for_import_named_specifier<'a>(inner: &'a swc_ast::ImportNamedSpecifier, parent: Node<'a>, bump: &'a Bump) -> &'a ImportNamedSpecifier<'a> {
  let node = bump.alloc(ImportNamedSpecifier {
    inner,
    parent: parent.to::<ImportDecl>(),
    local: unsafe { MaybeUninit::uninit().assume_init() },
    imported: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.local = get_view_for_ident(&inner.local, parent.clone(), bump);
  node.imported = match &inner.imported {
    Some(value) => Some(get_view_for_ident(value, parent, bump)),
    None => None,
  };
  node
}

/// `export { foo } from 'mod'`
/// `export { foo as bar } from 'mod'`
pub struct NamedExport<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::NamedExport,
  pub specifiers: Vec<ExportSpecifier<'a>>,
  pub src: Option<&'a Str<'a>>,
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
  fn from(node: &NamedExport<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&NamedExport<'a>, &'a NamedExport<'a>>(node) };
    Node::NamedExport(node)
  }
}

impl<'a> NodeTrait<'a> for &'a NamedExport<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.specifiers.len() + match &self.src { Some(_value) => 1, None => 0, });
    for child in self.specifiers.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.src {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::NamedExport
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

fn get_view_for_named_export<'a>(inner: &'a swc_ast::NamedExport, parent: Node<'a>, bump: &'a Bump) -> &'a NamedExport<'a> {
  let node = bump.alloc(NamedExport {
    inner,
    parent,
    specifiers: Vec::with_capacity(inner.specifiers.len()),
    src: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.specifiers.extend(inner.specifiers.iter().map(|value| get_view_for_export_specifier(value, parent.clone(), bump)));
  node.src = match &inner.src {
    Some(value) => Some(get_view_for_str(value, parent, bump)),
    None => None,
  };
  node
}

pub struct JSXElement<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::JSXElement,
  pub opening: &'a JSXOpeningElement<'a>,
  pub children: Vec<JSXElementChild<'a>>,
  pub closing: Option<&'a JSXClosingElement<'a>>,
}

impl<'a> Spanned for JSXElement<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXElement<'a>> for Node<'a> {
  fn from(node: &JSXElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXElement<'a>, &'a JSXElement<'a>>(node) };
    Node::JSXElement(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.children.len() + match &self.closing { Some(_value) => 1, None => 0, });
    children.push(self.opening.into());
    for child in self.children.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.closing {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXElement
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

fn get_view_for_jsxelement<'a>(inner: &'a swc_ast::JSXElement, parent: Node<'a>, bump: &'a Bump) -> &'a JSXElement<'a> {
  let node = bump.alloc(JSXElement {
    inner,
    parent,
    opening: unsafe { MaybeUninit::uninit().assume_init() },
    children: Vec::with_capacity(inner.children.len()),
    closing: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.opening = get_view_for_jsxopening_element(&inner.opening, parent.clone(), bump);
  node.children.extend(inner.children.iter().map(|value| get_view_for_jsxelement_child(value, parent.clone(), bump)));
  node.closing = match &inner.closing {
    Some(value) => Some(get_view_for_jsxclosing_element(value, parent, bump)),
    None => None,
  };
  node
}

pub struct ClassDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ClassDecl,
  pub ident: &'a Ident<'a>,
  pub class: &'a Class<'a>,
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
  fn from(node: &ClassDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ClassDecl<'a>, &'a ClassDecl<'a>>(node) };
    Node::ClassDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ClassDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.ident.into());
    children.push(self.class.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ClassDecl
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

fn get_view_for_class_decl<'a>(inner: &'a swc_ast::ClassDecl, parent: Node<'a>, bump: &'a Bump) -> &'a ClassDecl<'a> {
  let node = bump.alloc(ClassDecl {
    inner,
    parent,
    ident: unsafe { MaybeUninit::uninit().assume_init() },
    class: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.ident = get_view_for_ident(&inner.ident, parent.clone(), bump);
  node.class = get_view_for_class(&inner.class, parent, bump);
  node
}

pub struct ArrayPat<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ArrayPat,
  pub elems: Vec<Option<Pat<'a>>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
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
  fn from(node: &ArrayPat<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ArrayPat<'a>, &'a ArrayPat<'a>>(node) };
    Node::ArrayPat(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ArrayPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.elems.len() + match &self.type_ann { Some(_value) => 1, None => 0, });
    for child in self.elems.iter() {
      if let Some(child) = child.as_ref() {
        children.push(child.into());
      }
    }
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ArrayPat
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

fn get_view_for_array_pat<'a>(inner: &'a swc_ast::ArrayPat, parent: Node<'a>, bump: &'a Bump) -> &'a ArrayPat<'a> {
  let node = bump.alloc(ArrayPat {
    inner,
    parent,
    elems: Vec::with_capacity(inner.elems.len()),
    type_ann: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.elems.extend(inner.elems.iter().map(|value| match value {
    Some(value) => Some(get_view_for_pat(value, parent.clone(), bump)),
    None => None,
  }));
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent, bump)),
    None => None,
  };
  node
}

pub struct DoWhileStmt<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::DoWhileStmt,
  pub test: Expr<'a>,
  pub body: Stmt<'a>,
}

impl<'a> Spanned for DoWhileStmt<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&DoWhileStmt<'a>> for Node<'a> {
  fn from(node: &DoWhileStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&DoWhileStmt<'a>, &'a DoWhileStmt<'a>>(node) };
    Node::DoWhileStmt(node)
  }
}

impl<'a> NodeTrait<'a> for &'a DoWhileStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.test).into());
    children.push((&self.body).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::DoWhileStmt
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

fn get_view_for_do_while_stmt<'a>(inner: &'a swc_ast::DoWhileStmt, parent: Node<'a>, bump: &'a Bump) -> &'a DoWhileStmt<'a> {
  let node = bump.alloc(DoWhileStmt {
    inner,
    parent,
    test: unsafe { MaybeUninit::uninit().assume_init() },
    body: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.test = get_view_for_expr(&inner.test, parent.clone(), bump);
  node.body = get_view_for_stmt(&inner.body, parent, bump);
  node
}

pub struct JSXText<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::JSXText,
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
  fn from(node: &JSXText<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXText<'a>, &'a JSXText<'a>>(node) };
    Node::JSXText(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXText<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXText
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

fn get_view_for_jsxtext<'a>(inner: &'a swc_ast::JSXText, parent: Node<'a>, bump: &'a Bump) -> &'a JSXText<'a> {
  let node = bump.alloc(JSXText {
    inner,
    parent,
  });
  node
}

pub struct VarDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::VarDecl,
  pub decls: Vec<&'a VarDeclarator<'a>>,
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
  fn from(node: &VarDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&VarDecl<'a>, &'a VarDecl<'a>>(node) };
    Node::VarDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a VarDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.decls.len());
    for child in self.decls.iter() {
      children.push((*child).into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::VarDecl
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

fn get_view_for_var_decl<'a>(inner: &'a swc_ast::VarDecl, parent: Node<'a>, bump: &'a Bump) -> &'a VarDecl<'a> {
  let node = bump.alloc(VarDecl {
    inner,
    parent,
    decls: Vec::with_capacity(inner.decls.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.decls.extend(inner.decls.iter().map(|value| get_view_for_var_declarator(value, parent.clone(), bump)));
  node
}

pub struct PrivateName<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::PrivateName,
  pub id: &'a Ident<'a>,
}

impl<'a> Spanned for PrivateName<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&PrivateName<'a>> for Node<'a> {
  fn from(node: &PrivateName<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&PrivateName<'a>, &'a PrivateName<'a>>(node) };
    Node::PrivateName(node)
  }
}

impl<'a> NodeTrait<'a> for &'a PrivateName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.id.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::PrivateName
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

fn get_view_for_private_name<'a>(inner: &'a swc_ast::PrivateName, parent: Node<'a>, bump: &'a Bump) -> &'a PrivateName<'a> {
  let node = bump.alloc(PrivateName {
    inner,
    parent,
    id: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.id = get_view_for_ident(&inner.id, parent, bump);
  node
}

/// XML-based namespace syntax:
pub struct JSXNamespacedName<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::JSXNamespacedName,
  pub ns: &'a Ident<'a>,
  pub name: &'a Ident<'a>,
}

impl<'a> Spanned for JSXNamespacedName<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&JSXNamespacedName<'a>> for Node<'a> {
  fn from(node: &JSXNamespacedName<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXNamespacedName<'a>, &'a JSXNamespacedName<'a>>(node) };
    Node::JSXNamespacedName(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXNamespacedName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.ns.into());
    children.push(self.name.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXNamespacedName
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

fn get_view_for_jsxnamespaced_name<'a>(inner: &'a swc_ast::JSXNamespacedName, parent: Node<'a>, bump: &'a Bump) -> &'a JSXNamespacedName<'a> {
  let node = bump.alloc(JSXNamespacedName {
    inner,
    parent,
    ns: unsafe { MaybeUninit::uninit().assume_init() },
    name: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.ns = get_view_for_ident(&inner.ns, parent.clone(), bump);
  node.name = get_view_for_ident(&inner.name, parent, bump);
  node
}

pub struct JSXOpeningElement<'a> {
  pub parent: &'a JSXElement<'a>,
  pub inner: &'a swc_ast::JSXOpeningElement,
  pub name: JSXElementName<'a>,
  pub attrs: Vec<JSXAttrOrSpread<'a>>,
  /// Note: This field's name is differrent from one from babel because it is
  /// misleading
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
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
  fn from(node: &JSXOpeningElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXOpeningElement<'a>, &'a JSXOpeningElement<'a>>(node) };
    Node::JSXOpeningElement(node)
  }
}

impl<'a> NodeTrait<'a> for &'a JSXOpeningElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.attrs.len() + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    for child in self.attrs.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.type_args {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXOpeningElement
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

fn get_view_for_jsxopening_element<'a>(inner: &'a swc_ast::JSXOpeningElement, parent: Node<'a>, bump: &'a Bump) -> &'a JSXOpeningElement<'a> {
  let node = bump.alloc(JSXOpeningElement {
    inner,
    parent: parent.to::<JSXElement>(),
    name: unsafe { MaybeUninit::uninit().assume_init() },
    attrs: Vec::with_capacity(inner.attrs.len()),
    type_args: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.name = get_view_for_jsxelement_name(&inner.name, parent.clone(), bump);
  node.attrs.extend(inner.attrs.iter().map(|value| get_view_for_jsxattr_or_spread(value, parent.clone(), bump)));
  node.type_args = match &inner.type_args {
    Some(value) => Some(get_view_for_ts_type_param_instantiation(value, parent, bump)),
    None => None,
  };
  node
}

pub struct SpreadElement<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::SpreadElement,
  pub expr: Expr<'a>,
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
  fn from(node: &SpreadElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SpreadElement<'a>, &'a SpreadElement<'a>>(node) };
    Node::SpreadElement(node)
  }
}

impl<'a> NodeTrait<'a> for &'a SpreadElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SpreadElement
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

fn get_view_for_spread_element<'a>(inner: &'a swc_ast::SpreadElement, parent: Node<'a>, bump: &'a Bump) -> &'a SpreadElement<'a> {
  let node = bump.alloc(SpreadElement {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

pub struct ExportDefaultDecl<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ExportDefaultDecl,
  pub decl: DefaultDecl<'a>,
}

impl<'a> Spanned for ExportDefaultDecl<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ExportDefaultDecl<'a>> for Node<'a> {
  fn from(node: &ExportDefaultDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportDefaultDecl<'a>, &'a ExportDefaultDecl<'a>>(node) };
    Node::ExportDefaultDecl(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ExportDefaultDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.decl).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportDefaultDecl
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

fn get_view_for_export_default_decl<'a>(inner: &'a swc_ast::ExportDefaultDecl, parent: Node<'a>, bump: &'a Bump) -> &'a ExportDefaultDecl<'a> {
  let node = bump.alloc(ExportDefaultDecl {
    inner,
    parent,
    decl: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.decl = get_view_for_default_decl(&inner.decl, parent, bump);
  node
}

pub struct ArrowExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ArrowExpr,
  pub params: Vec<Pat<'a>>,
  pub body: BlockStmtOrExpr<'a>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub return_type: Option<&'a TsTypeAnn<'a>>,
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
  fn from(node: &ArrowExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ArrowExpr<'a>, &'a ArrowExpr<'a>>(node) };
    Node::ArrowExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ArrowExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.params.len() + match &self.type_params { Some(_value) => 1, None => 0, } + match &self.return_type { Some(_value) => 1, None => 0, });
    for child in self.params.iter() {
      children.push(child.into());
    }
    children.push((&self.body).into());
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    if let Some(child) = self.return_type {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ArrowExpr
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

fn get_view_for_arrow_expr<'a>(inner: &'a swc_ast::ArrowExpr, parent: Node<'a>, bump: &'a Bump) -> &'a ArrowExpr<'a> {
  let node = bump.alloc(ArrowExpr {
    inner,
    parent,
    params: Vec::with_capacity(inner.params.len()),
    body: unsafe { MaybeUninit::uninit().assume_init() },
    type_params: None,
    return_type: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.params.extend(inner.params.iter().map(|value| get_view_for_pat(value, parent.clone(), bump)));
  node.body = get_view_for_block_stmt_or_expr(&inner.body, parent.clone(), bump);
  node.type_params = match &inner.type_params {
    Some(value) => Some(get_view_for_ts_type_param_decl(value, parent.clone(), bump)),
    None => None,
  };
  node.return_type = match &inner.return_type {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent, bump)),
    None => None,
  };
  node
}

pub struct TsAsExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsAsExpr,
  pub expr: Expr<'a>,
  pub type_ann: TsType<'a>,
}

impl<'a> Spanned for TsAsExpr<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsAsExpr<'a>> for Node<'a> {
  fn from(node: &TsAsExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsAsExpr<'a>, &'a TsAsExpr<'a>>(node) };
    Node::TsAsExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsAsExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push((&self.type_ann).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsAsExpr
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

fn get_view_for_ts_as_expr<'a>(inner: &'a swc_ast::TsAsExpr, parent: Node<'a>, bump: &'a Bump) -> &'a TsAsExpr<'a> {
  let node = bump.alloc(TsAsExpr {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
    type_ann: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent.clone(), bump);
  node.type_ann = get_view_for_ts_type(&inner.type_ann, parent, bump);
  node
}

/// `{key: value}`
pub struct KeyValuePatProp<'a> {
  pub parent: &'a ObjectPat<'a>,
  pub inner: &'a swc_ast::KeyValuePatProp,
  pub key: PropName<'a>,
  pub value: Pat<'a>,
}

impl<'a> Spanned for KeyValuePatProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&KeyValuePatProp<'a>> for Node<'a> {
  fn from(node: &KeyValuePatProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&KeyValuePatProp<'a>, &'a KeyValuePatProp<'a>>(node) };
    Node::KeyValuePatProp(node)
  }
}

impl<'a> NodeTrait<'a> for &'a KeyValuePatProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.value).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::KeyValuePatProp
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

fn get_view_for_key_value_pat_prop<'a>(inner: &'a swc_ast::KeyValuePatProp, parent: Node<'a>, bump: &'a Bump) -> &'a KeyValuePatProp<'a> {
  let node = bump.alloc(KeyValuePatProp {
    inner,
    parent: parent.to::<ObjectPat>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    value: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_prop_name(&inner.key, parent.clone(), bump);
  node.value = get_view_for_pat(&inner.value, parent, bump);
  node
}

pub struct TsLitType<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::TsLitType,
  pub lit: TsLit<'a>,
}

impl<'a> Spanned for TsLitType<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&TsLitType<'a>> for Node<'a> {
  fn from(node: &TsLitType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsLitType<'a>, &'a TsLitType<'a>>(node) };
    Node::TsLitType(node)
  }
}

impl<'a> NodeTrait<'a> for &'a TsLitType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.lit).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsLitType
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

fn get_view_for_ts_lit_type<'a>(inner: &'a swc_ast::TsLitType, parent: Node<'a>, bump: &'a Bump) -> &'a TsLitType<'a> {
  let node = bump.alloc(TsLitType {
    inner,
    parent,
    lit: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.lit = get_view_for_ts_lit(&inner.lit, parent, bump);
  node
}

pub struct AssignExpr<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::AssignExpr,
  pub left: PatOrExpr<'a>,
  pub right: Expr<'a>,
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
  fn from(node: &AssignExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&AssignExpr<'a>, &'a AssignExpr<'a>>(node) };
    Node::AssignExpr(node)
  }
}

impl<'a> NodeTrait<'a> for &'a AssignExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::AssignExpr
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

fn get_view_for_assign_expr<'a>(inner: &'a swc_ast::AssignExpr, parent: Node<'a>, bump: &'a Bump) -> &'a AssignExpr<'a> {
  let node = bump.alloc(AssignExpr {
    inner,
    parent,
    left: unsafe { MaybeUninit::uninit().assume_init() },
    right: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.left = get_view_for_pat_or_expr(&inner.left, parent.clone(), bump);
  node.right = get_view_for_expr(&inner.right, parent, bump);
  node
}

/// Array literal.
pub struct ArrayLit<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::ArrayLit,
  pub elems: Vec<Option<&'a ExprOrSpread<'a>>>,
}

impl<'a> Spanned for ArrayLit<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&ArrayLit<'a>> for Node<'a> {
  fn from(node: &ArrayLit<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ArrayLit<'a>, &'a ArrayLit<'a>>(node) };
    Node::ArrayLit(node)
  }
}

impl<'a> NodeTrait<'a> for &'a ArrayLit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.elems.len());
    for child in self.elems.iter() {
      if let Some(child) = child {
        children.push((*child).into());
      }
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ArrayLit
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

fn get_view_for_array_lit<'a>(inner: &'a swc_ast::ArrayLit, parent: Node<'a>, bump: &'a Bump) -> &'a ArrayLit<'a> {
  let node = bump.alloc(ArrayLit {
    inner,
    parent,
    elems: Vec::with_capacity(inner.elems.len()),
  });
  let parent: Node<'a> = (&*node).into();
  node.elems.extend(inner.elems.iter().map(|value| match value {
    Some(value) => Some(get_view_for_expr_or_spread(value, parent.clone(), bump)),
    None => None,
  }));
  node
}

pub struct Decorator<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Decorator,
  pub expr: Expr<'a>,
}

impl<'a> Spanned for Decorator<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&Decorator<'a>> for Node<'a> {
  fn from(node: &Decorator<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Decorator<'a>, &'a Decorator<'a>>(node) };
    Node::Decorator(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Decorator<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Decorator
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

fn get_view_for_decorator<'a>(inner: &'a swc_ast::Decorator, parent: Node<'a>, bump: &'a Bump) -> &'a Decorator<'a> {
  let node = bump.alloc(Decorator {
    inner,
    parent,
    expr: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.expr = get_view_for_expr(&inner.expr, parent, bump);
  node
}

/// Ident with span.
pub struct Ident<'a> {
  pub parent: Node<'a>,
  pub inner: &'a swc_ast::Ident,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
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
  fn from(node: &Ident<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Ident<'a>, &'a Ident<'a>>(node) };
    Node::Ident(node)
  }
}

impl<'a> NodeTrait<'a> for &'a Ident<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.type_ann { Some(_value) => 1, None => 0, });
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Ident
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

fn get_view_for_ident<'a>(inner: &'a swc_ast::Ident, parent: Node<'a>, bump: &'a Bump) -> &'a Ident<'a> {
  let node = bump.alloc(Ident {
    inner,
    parent,
    type_ann: None,
  });
  let parent: Node<'a> = (&*node).into();
  node.type_ann = match &inner.type_ann {
    Some(value) => Some(get_view_for_ts_type_ann(value, parent, bump)),
    None => None,
  };
  node
}

pub struct MethodProp<'a> {
  pub parent: &'a ObjectLit<'a>,
  pub inner: &'a swc_ast::MethodProp,
  pub key: PropName<'a>,
  pub function: &'a Function<'a>,
}

impl<'a> Spanned for MethodProp<'a> {
  fn span(&self) -> Span {
    self.inner.span()
  }
}

impl<'a> From<&MethodProp<'a>> for Node<'a> {
  fn from(node: &MethodProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&MethodProp<'a>, &'a MethodProp<'a>>(node) };
    Node::MethodProp(node)
  }
}

impl<'a> NodeTrait<'a> for &'a MethodProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push(self.function.into());
    children
  }

  fn into_node(&self) -> Node<'a> {
    (*self).into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::MethodProp
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

fn get_view_for_method_prop<'a>(inner: &'a swc_ast::MethodProp, parent: Node<'a>, bump: &'a Bump) -> &'a MethodProp<'a> {
  let node = bump.alloc(MethodProp {
    inner,
    parent: parent.to::<ObjectLit>(),
    key: unsafe { MaybeUninit::uninit().assume_init() },
    function: unsafe { MaybeUninit::uninit().assume_init() },
  });
  let parent: Node<'a> = (&*node).into();
  node.key = get_view_for_prop_name(&inner.key, parent.clone(), bump);
  node.function = get_view_for_function(&inner.function, parent, bump);
  node
}
