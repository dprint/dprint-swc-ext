// This code is code generated.
// Run `./scripts/generate.sh` from the root directory to regenerate it.
use std::cell::RefCell;
use std::mem;
use bumpalo::Bump;
use crate::swc::common::Spanned;
use crate::swc::ast as swc_ast;
use crate::swc::atoms as swc_atoms;
pub use crate::swc::ast::{Accessibility, AssignOp, BinaryOp, EsVersion, MetaPropKind, MethodKind, TruePlusMinus, TsKeywordTypeKind, TsTypeOperatorOp, UnaryOp, UpdateOp, VarDeclKind};
use crate::swc::common as swc_common;
use crate::common::*;
use crate::view::types::*;

thread_local! {
  static LOCAL_BUMP_ALLOCATOR: RefCell<Bump> = RefCell::new(Bump::new());
}

pub fn with_ast_view<'a, T>(info: ProgramInfo, with_view: impl FnOnce(Program<'a>) -> T) -> T {
  match info.program {
    ProgramRef::Module(module) => {
      with_ast_view_for_module(ModuleInfo {
        module,
        text_info: info.text_info,
        tokens: info.tokens,
        comments: info.comments,
      }, |module| with_view(Program::Module(module)))
    }
    ProgramRef::Script(script) => {
      with_ast_view_for_script(ScriptInfo {
        script,
        text_info: info.text_info,
        tokens: info.tokens,
        comments: info.comments,
      }, |script| with_view(Program::Script(script)))
    }
  }
}

pub fn with_ast_view_for_module<'a, T>(info: ModuleInfo, with_view: impl FnOnce(&'a Module<'a>) -> T) -> T {
  LOCAL_BUMP_ALLOCATOR.with(|bump_cell| {
    let mut bump_borrow = bump_cell.borrow_mut();
    let bump_ref = unsafe { mem::transmute::<&Bump, &'a Bump>(&bump_borrow) };
    let info_ref = unsafe { mem::transmute::<&ModuleInfo, &'a ModuleInfo<'a>>(&info) };
    let ast_view = get_view_for_module(info_ref, bump_ref);
    let result = with_view(ast_view);
    bump_borrow.reset();
    result
  })
}

pub fn with_ast_view_for_script<'a, T>(info: ScriptInfo, with_view: impl FnOnce(&'a Script<'a>) -> T) -> T {
  LOCAL_BUMP_ALLOCATOR.with(|bump_cell| {
    let mut bump_borrow = bump_cell.borrow_mut();
    let bump_ref = unsafe { mem::transmute::<&Bump, &'a Bump>(&bump_borrow) };
    let info_ref = unsafe { mem::transmute::<&ScriptInfo, &'a ScriptInfo<'a>>(&info) };
    let ast_view = get_view_for_script(info_ref, bump_ref);
    let result = with_view(ast_view);
    bump_borrow.reset();
    result
  })
}

#[derive(Clone, Copy)]
pub enum Node<'a> {
  ArrayLit(&'a ArrayLit<'a>),
  ArrayPat(&'a ArrayPat<'a>),
  ArrowExpr(&'a ArrowExpr<'a>),
  AssignExpr(&'a AssignExpr<'a>),
  AssignPat(&'a AssignPat<'a>),
  AssignPatProp(&'a AssignPatProp<'a>),
  AssignProp(&'a AssignProp<'a>),
  AwaitExpr(&'a AwaitExpr<'a>),
  BigInt(&'a BigInt<'a>),
  BinExpr(&'a BinExpr<'a>),
  BindingIdent(&'a BindingIdent<'a>),
  BlockStmt(&'a BlockStmt<'a>),
  Bool(&'a Bool<'a>),
  BreakStmt(&'a BreakStmt<'a>),
  CallExpr(&'a CallExpr<'a>),
  CatchClause(&'a CatchClause<'a>),
  Class(&'a Class<'a>),
  ClassDecl(&'a ClassDecl<'a>),
  ClassExpr(&'a ClassExpr<'a>),
  ClassMethod(&'a ClassMethod<'a>),
  ClassProp(&'a ClassProp<'a>),
  ComputedPropName(&'a ComputedPropName<'a>),
  CondExpr(&'a CondExpr<'a>),
  Constructor(&'a Constructor<'a>),
  ContinueStmt(&'a ContinueStmt<'a>),
  DebuggerStmt(&'a DebuggerStmt<'a>),
  Decorator(&'a Decorator<'a>),
  DoWhileStmt(&'a DoWhileStmt<'a>),
  EmptyStmt(&'a EmptyStmt<'a>),
  ExportAll(&'a ExportAll<'a>),
  ExportDecl(&'a ExportDecl<'a>),
  ExportDefaultDecl(&'a ExportDefaultDecl<'a>),
  ExportDefaultExpr(&'a ExportDefaultExpr<'a>),
  ExportDefaultSpecifier(&'a ExportDefaultSpecifier<'a>),
  ExportNamedSpecifier(&'a ExportNamedSpecifier<'a>),
  ExportNamespaceSpecifier(&'a ExportNamespaceSpecifier<'a>),
  ExprOrSpread(&'a ExprOrSpread<'a>),
  ExprStmt(&'a ExprStmt<'a>),
  FnDecl(&'a FnDecl<'a>),
  FnExpr(&'a FnExpr<'a>),
  ForInStmt(&'a ForInStmt<'a>),
  ForOfStmt(&'a ForOfStmt<'a>),
  ForStmt(&'a ForStmt<'a>),
  Function(&'a Function<'a>),
  GetterProp(&'a GetterProp<'a>),
  Ident(&'a Ident<'a>),
  IfStmt(&'a IfStmt<'a>),
  Import(&'a Import<'a>),
  ImportDecl(&'a ImportDecl<'a>),
  ImportDefaultSpecifier(&'a ImportDefaultSpecifier<'a>),
  ImportNamedSpecifier(&'a ImportNamedSpecifier<'a>),
  ImportStarAsSpecifier(&'a ImportStarAsSpecifier<'a>),
  Invalid(&'a Invalid<'a>),
  JSXAttr(&'a JSXAttr<'a>),
  JSXClosingElement(&'a JSXClosingElement<'a>),
  JSXClosingFragment(&'a JSXClosingFragment<'a>),
  JSXElement(&'a JSXElement<'a>),
  JSXEmptyExpr(&'a JSXEmptyExpr<'a>),
  JSXExprContainer(&'a JSXExprContainer<'a>),
  JSXFragment(&'a JSXFragment<'a>),
  JSXMemberExpr(&'a JSXMemberExpr<'a>),
  JSXNamespacedName(&'a JSXNamespacedName<'a>),
  JSXOpeningElement(&'a JSXOpeningElement<'a>),
  JSXOpeningFragment(&'a JSXOpeningFragment<'a>),
  JSXSpreadChild(&'a JSXSpreadChild<'a>),
  JSXText(&'a JSXText<'a>),
  KeyValuePatProp(&'a KeyValuePatProp<'a>),
  KeyValueProp(&'a KeyValueProp<'a>),
  LabeledStmt(&'a LabeledStmt<'a>),
  MemberExpr(&'a MemberExpr<'a>),
  MetaPropExpr(&'a MetaPropExpr<'a>),
  MethodProp(&'a MethodProp<'a>),
  Module(&'a Module<'a>),
  NamedExport(&'a NamedExport<'a>),
  NewExpr(&'a NewExpr<'a>),
  Null(&'a Null<'a>),
  Number(&'a Number<'a>),
  ObjectLit(&'a ObjectLit<'a>),
  ObjectPat(&'a ObjectPat<'a>),
  OptCall(&'a OptCall<'a>),
  OptChainExpr(&'a OptChainExpr<'a>),
  Param(&'a Param<'a>),
  ParenExpr(&'a ParenExpr<'a>),
  PrivateMethod(&'a PrivateMethod<'a>),
  PrivateName(&'a PrivateName<'a>),
  PrivateProp(&'a PrivateProp<'a>),
  Regex(&'a Regex<'a>),
  RestPat(&'a RestPat<'a>),
  ReturnStmt(&'a ReturnStmt<'a>),
  Script(&'a Script<'a>),
  SeqExpr(&'a SeqExpr<'a>),
  SetterProp(&'a SetterProp<'a>),
  SpreadElement(&'a SpreadElement<'a>),
  StaticBlock(&'a StaticBlock<'a>),
  Str(&'a Str<'a>),
  Super(&'a Super<'a>),
  SuperPropExpr(&'a SuperPropExpr<'a>),
  SwitchCase(&'a SwitchCase<'a>),
  SwitchStmt(&'a SwitchStmt<'a>),
  TaggedTpl(&'a TaggedTpl<'a>),
  ThisExpr(&'a ThisExpr<'a>),
  ThrowStmt(&'a ThrowStmt<'a>),
  Tpl(&'a Tpl<'a>),
  TplElement(&'a TplElement<'a>),
  TryStmt(&'a TryStmt<'a>),
  TsArrayType(&'a TsArrayType<'a>),
  TsAsExpr(&'a TsAsExpr<'a>),
  TsCallSignatureDecl(&'a TsCallSignatureDecl<'a>),
  TsConditionalType(&'a TsConditionalType<'a>),
  TsConstAssertion(&'a TsConstAssertion<'a>),
  TsConstructSignatureDecl(&'a TsConstructSignatureDecl<'a>),
  TsConstructorType(&'a TsConstructorType<'a>),
  TsEnumDecl(&'a TsEnumDecl<'a>),
  TsEnumMember(&'a TsEnumMember<'a>),
  TsExportAssignment(&'a TsExportAssignment<'a>),
  TsExprWithTypeArgs(&'a TsExprWithTypeArgs<'a>),
  TsExternalModuleRef(&'a TsExternalModuleRef<'a>),
  TsFnType(&'a TsFnType<'a>),
  TsGetterSignature(&'a TsGetterSignature<'a>),
  TsImportEqualsDecl(&'a TsImportEqualsDecl<'a>),
  TsImportType(&'a TsImportType<'a>),
  TsIndexSignature(&'a TsIndexSignature<'a>),
  TsIndexedAccessType(&'a TsIndexedAccessType<'a>),
  TsInferType(&'a TsInferType<'a>),
  TsInstantiation(&'a TsInstantiation<'a>),
  TsInterfaceBody(&'a TsInterfaceBody<'a>),
  TsInterfaceDecl(&'a TsInterfaceDecl<'a>),
  TsIntersectionType(&'a TsIntersectionType<'a>),
  TsKeywordType(&'a TsKeywordType<'a>),
  TsLitType(&'a TsLitType<'a>),
  TsMappedType(&'a TsMappedType<'a>),
  TsMethodSignature(&'a TsMethodSignature<'a>),
  TsModuleBlock(&'a TsModuleBlock<'a>),
  TsModuleDecl(&'a TsModuleDecl<'a>),
  TsNamespaceDecl(&'a TsNamespaceDecl<'a>),
  TsNamespaceExportDecl(&'a TsNamespaceExportDecl<'a>),
  TsNonNullExpr(&'a TsNonNullExpr<'a>),
  TsOptionalType(&'a TsOptionalType<'a>),
  TsParamProp(&'a TsParamProp<'a>),
  TsParenthesizedType(&'a TsParenthesizedType<'a>),
  TsPropertySignature(&'a TsPropertySignature<'a>),
  TsQualifiedName(&'a TsQualifiedName<'a>),
  TsRestType(&'a TsRestType<'a>),
  TsSetterSignature(&'a TsSetterSignature<'a>),
  TsThisType(&'a TsThisType<'a>),
  TsTplLitType(&'a TsTplLitType<'a>),
  TsTupleElement(&'a TsTupleElement<'a>),
  TsTupleType(&'a TsTupleType<'a>),
  TsTypeAliasDecl(&'a TsTypeAliasDecl<'a>),
  TsTypeAnn(&'a TsTypeAnn<'a>),
  TsTypeAssertion(&'a TsTypeAssertion<'a>),
  TsTypeLit(&'a TsTypeLit<'a>),
  TsTypeOperator(&'a TsTypeOperator<'a>),
  TsTypeParam(&'a TsTypeParam<'a>),
  TsTypeParamDecl(&'a TsTypeParamDecl<'a>),
  TsTypeParamInstantiation(&'a TsTypeParamInstantiation<'a>),
  TsTypePredicate(&'a TsTypePredicate<'a>),
  TsTypeQuery(&'a TsTypeQuery<'a>),
  TsTypeRef(&'a TsTypeRef<'a>),
  TsUnionType(&'a TsUnionType<'a>),
  UnaryExpr(&'a UnaryExpr<'a>),
  UpdateExpr(&'a UpdateExpr<'a>),
  VarDecl(&'a VarDecl<'a>),
  VarDeclarator(&'a VarDeclarator<'a>),
  WhileStmt(&'a WhileStmt<'a>),
  WithStmt(&'a WithStmt<'a>),
  YieldExpr(&'a YieldExpr<'a>),
}

impl<'a> Node<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(self)
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    if let Some(result) = T::to(self) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", self.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for Node<'a> {
  fn start(&self) -> SourcePos {
    match self {
      Node::ArrayLit(node) => node.start(),
      Node::ArrayPat(node) => node.start(),
      Node::ArrowExpr(node) => node.start(),
      Node::AssignExpr(node) => node.start(),
      Node::AssignPat(node) => node.start(),
      Node::AssignPatProp(node) => node.start(),
      Node::AssignProp(node) => node.start(),
      Node::AwaitExpr(node) => node.start(),
      Node::BigInt(node) => node.start(),
      Node::BinExpr(node) => node.start(),
      Node::BindingIdent(node) => node.start(),
      Node::BlockStmt(node) => node.start(),
      Node::Bool(node) => node.start(),
      Node::BreakStmt(node) => node.start(),
      Node::CallExpr(node) => node.start(),
      Node::CatchClause(node) => node.start(),
      Node::Class(node) => node.start(),
      Node::ClassDecl(node) => node.start(),
      Node::ClassExpr(node) => node.start(),
      Node::ClassMethod(node) => node.start(),
      Node::ClassProp(node) => node.start(),
      Node::ComputedPropName(node) => node.start(),
      Node::CondExpr(node) => node.start(),
      Node::Constructor(node) => node.start(),
      Node::ContinueStmt(node) => node.start(),
      Node::DebuggerStmt(node) => node.start(),
      Node::Decorator(node) => node.start(),
      Node::DoWhileStmt(node) => node.start(),
      Node::EmptyStmt(node) => node.start(),
      Node::ExportAll(node) => node.start(),
      Node::ExportDecl(node) => node.start(),
      Node::ExportDefaultDecl(node) => node.start(),
      Node::ExportDefaultExpr(node) => node.start(),
      Node::ExportDefaultSpecifier(node) => node.start(),
      Node::ExportNamedSpecifier(node) => node.start(),
      Node::ExportNamespaceSpecifier(node) => node.start(),
      Node::ExprOrSpread(node) => node.start(),
      Node::ExprStmt(node) => node.start(),
      Node::FnDecl(node) => node.start(),
      Node::FnExpr(node) => node.start(),
      Node::ForInStmt(node) => node.start(),
      Node::ForOfStmt(node) => node.start(),
      Node::ForStmt(node) => node.start(),
      Node::Function(node) => node.start(),
      Node::GetterProp(node) => node.start(),
      Node::Ident(node) => node.start(),
      Node::IfStmt(node) => node.start(),
      Node::Import(node) => node.start(),
      Node::ImportDecl(node) => node.start(),
      Node::ImportDefaultSpecifier(node) => node.start(),
      Node::ImportNamedSpecifier(node) => node.start(),
      Node::ImportStarAsSpecifier(node) => node.start(),
      Node::Invalid(node) => node.start(),
      Node::JSXAttr(node) => node.start(),
      Node::JSXClosingElement(node) => node.start(),
      Node::JSXClosingFragment(node) => node.start(),
      Node::JSXElement(node) => node.start(),
      Node::JSXEmptyExpr(node) => node.start(),
      Node::JSXExprContainer(node) => node.start(),
      Node::JSXFragment(node) => node.start(),
      Node::JSXMemberExpr(node) => node.start(),
      Node::JSXNamespacedName(node) => node.start(),
      Node::JSXOpeningElement(node) => node.start(),
      Node::JSXOpeningFragment(node) => node.start(),
      Node::JSXSpreadChild(node) => node.start(),
      Node::JSXText(node) => node.start(),
      Node::KeyValuePatProp(node) => node.start(),
      Node::KeyValueProp(node) => node.start(),
      Node::LabeledStmt(node) => node.start(),
      Node::MemberExpr(node) => node.start(),
      Node::MetaPropExpr(node) => node.start(),
      Node::MethodProp(node) => node.start(),
      Node::Module(node) => node.start(),
      Node::NamedExport(node) => node.start(),
      Node::NewExpr(node) => node.start(),
      Node::Null(node) => node.start(),
      Node::Number(node) => node.start(),
      Node::ObjectLit(node) => node.start(),
      Node::ObjectPat(node) => node.start(),
      Node::OptCall(node) => node.start(),
      Node::OptChainExpr(node) => node.start(),
      Node::Param(node) => node.start(),
      Node::ParenExpr(node) => node.start(),
      Node::PrivateMethod(node) => node.start(),
      Node::PrivateName(node) => node.start(),
      Node::PrivateProp(node) => node.start(),
      Node::Regex(node) => node.start(),
      Node::RestPat(node) => node.start(),
      Node::ReturnStmt(node) => node.start(),
      Node::Script(node) => node.start(),
      Node::SeqExpr(node) => node.start(),
      Node::SetterProp(node) => node.start(),
      Node::SpreadElement(node) => node.start(),
      Node::StaticBlock(node) => node.start(),
      Node::Str(node) => node.start(),
      Node::Super(node) => node.start(),
      Node::SuperPropExpr(node) => node.start(),
      Node::SwitchCase(node) => node.start(),
      Node::SwitchStmt(node) => node.start(),
      Node::TaggedTpl(node) => node.start(),
      Node::ThisExpr(node) => node.start(),
      Node::ThrowStmt(node) => node.start(),
      Node::Tpl(node) => node.start(),
      Node::TplElement(node) => node.start(),
      Node::TryStmt(node) => node.start(),
      Node::TsArrayType(node) => node.start(),
      Node::TsAsExpr(node) => node.start(),
      Node::TsCallSignatureDecl(node) => node.start(),
      Node::TsConditionalType(node) => node.start(),
      Node::TsConstAssertion(node) => node.start(),
      Node::TsConstructSignatureDecl(node) => node.start(),
      Node::TsConstructorType(node) => node.start(),
      Node::TsEnumDecl(node) => node.start(),
      Node::TsEnumMember(node) => node.start(),
      Node::TsExportAssignment(node) => node.start(),
      Node::TsExprWithTypeArgs(node) => node.start(),
      Node::TsExternalModuleRef(node) => node.start(),
      Node::TsFnType(node) => node.start(),
      Node::TsGetterSignature(node) => node.start(),
      Node::TsImportEqualsDecl(node) => node.start(),
      Node::TsImportType(node) => node.start(),
      Node::TsIndexSignature(node) => node.start(),
      Node::TsIndexedAccessType(node) => node.start(),
      Node::TsInferType(node) => node.start(),
      Node::TsInstantiation(node) => node.start(),
      Node::TsInterfaceBody(node) => node.start(),
      Node::TsInterfaceDecl(node) => node.start(),
      Node::TsIntersectionType(node) => node.start(),
      Node::TsKeywordType(node) => node.start(),
      Node::TsLitType(node) => node.start(),
      Node::TsMappedType(node) => node.start(),
      Node::TsMethodSignature(node) => node.start(),
      Node::TsModuleBlock(node) => node.start(),
      Node::TsModuleDecl(node) => node.start(),
      Node::TsNamespaceDecl(node) => node.start(),
      Node::TsNamespaceExportDecl(node) => node.start(),
      Node::TsNonNullExpr(node) => node.start(),
      Node::TsOptionalType(node) => node.start(),
      Node::TsParamProp(node) => node.start(),
      Node::TsParenthesizedType(node) => node.start(),
      Node::TsPropertySignature(node) => node.start(),
      Node::TsQualifiedName(node) => node.start(),
      Node::TsRestType(node) => node.start(),
      Node::TsSetterSignature(node) => node.start(),
      Node::TsThisType(node) => node.start(),
      Node::TsTplLitType(node) => node.start(),
      Node::TsTupleElement(node) => node.start(),
      Node::TsTupleType(node) => node.start(),
      Node::TsTypeAliasDecl(node) => node.start(),
      Node::TsTypeAnn(node) => node.start(),
      Node::TsTypeAssertion(node) => node.start(),
      Node::TsTypeLit(node) => node.start(),
      Node::TsTypeOperator(node) => node.start(),
      Node::TsTypeParam(node) => node.start(),
      Node::TsTypeParamDecl(node) => node.start(),
      Node::TsTypeParamInstantiation(node) => node.start(),
      Node::TsTypePredicate(node) => node.start(),
      Node::TsTypeQuery(node) => node.start(),
      Node::TsTypeRef(node) => node.start(),
      Node::TsUnionType(node) => node.start(),
      Node::UnaryExpr(node) => node.start(),
      Node::UpdateExpr(node) => node.start(),
      Node::VarDecl(node) => node.start(),
      Node::VarDeclarator(node) => node.start(),
      Node::WhileStmt(node) => node.start(),
      Node::WithStmt(node) => node.start(),
      Node::YieldExpr(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      Node::ArrayLit(node) => node.end(),
      Node::ArrayPat(node) => node.end(),
      Node::ArrowExpr(node) => node.end(),
      Node::AssignExpr(node) => node.end(),
      Node::AssignPat(node) => node.end(),
      Node::AssignPatProp(node) => node.end(),
      Node::AssignProp(node) => node.end(),
      Node::AwaitExpr(node) => node.end(),
      Node::BigInt(node) => node.end(),
      Node::BinExpr(node) => node.end(),
      Node::BindingIdent(node) => node.end(),
      Node::BlockStmt(node) => node.end(),
      Node::Bool(node) => node.end(),
      Node::BreakStmt(node) => node.end(),
      Node::CallExpr(node) => node.end(),
      Node::CatchClause(node) => node.end(),
      Node::Class(node) => node.end(),
      Node::ClassDecl(node) => node.end(),
      Node::ClassExpr(node) => node.end(),
      Node::ClassMethod(node) => node.end(),
      Node::ClassProp(node) => node.end(),
      Node::ComputedPropName(node) => node.end(),
      Node::CondExpr(node) => node.end(),
      Node::Constructor(node) => node.end(),
      Node::ContinueStmt(node) => node.end(),
      Node::DebuggerStmt(node) => node.end(),
      Node::Decorator(node) => node.end(),
      Node::DoWhileStmt(node) => node.end(),
      Node::EmptyStmt(node) => node.end(),
      Node::ExportAll(node) => node.end(),
      Node::ExportDecl(node) => node.end(),
      Node::ExportDefaultDecl(node) => node.end(),
      Node::ExportDefaultExpr(node) => node.end(),
      Node::ExportDefaultSpecifier(node) => node.end(),
      Node::ExportNamedSpecifier(node) => node.end(),
      Node::ExportNamespaceSpecifier(node) => node.end(),
      Node::ExprOrSpread(node) => node.end(),
      Node::ExprStmt(node) => node.end(),
      Node::FnDecl(node) => node.end(),
      Node::FnExpr(node) => node.end(),
      Node::ForInStmt(node) => node.end(),
      Node::ForOfStmt(node) => node.end(),
      Node::ForStmt(node) => node.end(),
      Node::Function(node) => node.end(),
      Node::GetterProp(node) => node.end(),
      Node::Ident(node) => node.end(),
      Node::IfStmt(node) => node.end(),
      Node::Import(node) => node.end(),
      Node::ImportDecl(node) => node.end(),
      Node::ImportDefaultSpecifier(node) => node.end(),
      Node::ImportNamedSpecifier(node) => node.end(),
      Node::ImportStarAsSpecifier(node) => node.end(),
      Node::Invalid(node) => node.end(),
      Node::JSXAttr(node) => node.end(),
      Node::JSXClosingElement(node) => node.end(),
      Node::JSXClosingFragment(node) => node.end(),
      Node::JSXElement(node) => node.end(),
      Node::JSXEmptyExpr(node) => node.end(),
      Node::JSXExprContainer(node) => node.end(),
      Node::JSXFragment(node) => node.end(),
      Node::JSXMemberExpr(node) => node.end(),
      Node::JSXNamespacedName(node) => node.end(),
      Node::JSXOpeningElement(node) => node.end(),
      Node::JSXOpeningFragment(node) => node.end(),
      Node::JSXSpreadChild(node) => node.end(),
      Node::JSXText(node) => node.end(),
      Node::KeyValuePatProp(node) => node.end(),
      Node::KeyValueProp(node) => node.end(),
      Node::LabeledStmt(node) => node.end(),
      Node::MemberExpr(node) => node.end(),
      Node::MetaPropExpr(node) => node.end(),
      Node::MethodProp(node) => node.end(),
      Node::Module(node) => node.end(),
      Node::NamedExport(node) => node.end(),
      Node::NewExpr(node) => node.end(),
      Node::Null(node) => node.end(),
      Node::Number(node) => node.end(),
      Node::ObjectLit(node) => node.end(),
      Node::ObjectPat(node) => node.end(),
      Node::OptCall(node) => node.end(),
      Node::OptChainExpr(node) => node.end(),
      Node::Param(node) => node.end(),
      Node::ParenExpr(node) => node.end(),
      Node::PrivateMethod(node) => node.end(),
      Node::PrivateName(node) => node.end(),
      Node::PrivateProp(node) => node.end(),
      Node::Regex(node) => node.end(),
      Node::RestPat(node) => node.end(),
      Node::ReturnStmt(node) => node.end(),
      Node::Script(node) => node.end(),
      Node::SeqExpr(node) => node.end(),
      Node::SetterProp(node) => node.end(),
      Node::SpreadElement(node) => node.end(),
      Node::StaticBlock(node) => node.end(),
      Node::Str(node) => node.end(),
      Node::Super(node) => node.end(),
      Node::SuperPropExpr(node) => node.end(),
      Node::SwitchCase(node) => node.end(),
      Node::SwitchStmt(node) => node.end(),
      Node::TaggedTpl(node) => node.end(),
      Node::ThisExpr(node) => node.end(),
      Node::ThrowStmt(node) => node.end(),
      Node::Tpl(node) => node.end(),
      Node::TplElement(node) => node.end(),
      Node::TryStmt(node) => node.end(),
      Node::TsArrayType(node) => node.end(),
      Node::TsAsExpr(node) => node.end(),
      Node::TsCallSignatureDecl(node) => node.end(),
      Node::TsConditionalType(node) => node.end(),
      Node::TsConstAssertion(node) => node.end(),
      Node::TsConstructSignatureDecl(node) => node.end(),
      Node::TsConstructorType(node) => node.end(),
      Node::TsEnumDecl(node) => node.end(),
      Node::TsEnumMember(node) => node.end(),
      Node::TsExportAssignment(node) => node.end(),
      Node::TsExprWithTypeArgs(node) => node.end(),
      Node::TsExternalModuleRef(node) => node.end(),
      Node::TsFnType(node) => node.end(),
      Node::TsGetterSignature(node) => node.end(),
      Node::TsImportEqualsDecl(node) => node.end(),
      Node::TsImportType(node) => node.end(),
      Node::TsIndexSignature(node) => node.end(),
      Node::TsIndexedAccessType(node) => node.end(),
      Node::TsInferType(node) => node.end(),
      Node::TsInstantiation(node) => node.end(),
      Node::TsInterfaceBody(node) => node.end(),
      Node::TsInterfaceDecl(node) => node.end(),
      Node::TsIntersectionType(node) => node.end(),
      Node::TsKeywordType(node) => node.end(),
      Node::TsLitType(node) => node.end(),
      Node::TsMappedType(node) => node.end(),
      Node::TsMethodSignature(node) => node.end(),
      Node::TsModuleBlock(node) => node.end(),
      Node::TsModuleDecl(node) => node.end(),
      Node::TsNamespaceDecl(node) => node.end(),
      Node::TsNamespaceExportDecl(node) => node.end(),
      Node::TsNonNullExpr(node) => node.end(),
      Node::TsOptionalType(node) => node.end(),
      Node::TsParamProp(node) => node.end(),
      Node::TsParenthesizedType(node) => node.end(),
      Node::TsPropertySignature(node) => node.end(),
      Node::TsQualifiedName(node) => node.end(),
      Node::TsRestType(node) => node.end(),
      Node::TsSetterSignature(node) => node.end(),
      Node::TsThisType(node) => node.end(),
      Node::TsTplLitType(node) => node.end(),
      Node::TsTupleElement(node) => node.end(),
      Node::TsTupleType(node) => node.end(),
      Node::TsTypeAliasDecl(node) => node.end(),
      Node::TsTypeAnn(node) => node.end(),
      Node::TsTypeAssertion(node) => node.end(),
      Node::TsTypeLit(node) => node.end(),
      Node::TsTypeOperator(node) => node.end(),
      Node::TsTypeParam(node) => node.end(),
      Node::TsTypeParamDecl(node) => node.end(),
      Node::TsTypeParamInstantiation(node) => node.end(),
      Node::TsTypePredicate(node) => node.end(),
      Node::TsTypeQuery(node) => node.end(),
      Node::TsTypeRef(node) => node.end(),
      Node::TsUnionType(node) => node.end(),
      Node::UnaryExpr(node) => node.end(),
      Node::UpdateExpr(node) => node.end(),
      Node::VarDecl(node) => node.end(),
      Node::VarDeclarator(node) => node.end(),
      Node::WhileStmt(node) => node.end(),
      Node::WithStmt(node) => node.end(),
      Node::YieldExpr(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for Node<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      Node::ArrayLit(node) => NodeTrait::parent(*node),
      Node::ArrayPat(node) => NodeTrait::parent(*node),
      Node::ArrowExpr(node) => NodeTrait::parent(*node),
      Node::AssignExpr(node) => NodeTrait::parent(*node),
      Node::AssignPat(node) => NodeTrait::parent(*node),
      Node::AssignPatProp(node) => NodeTrait::parent(*node),
      Node::AssignProp(node) => NodeTrait::parent(*node),
      Node::AwaitExpr(node) => NodeTrait::parent(*node),
      Node::BigInt(node) => NodeTrait::parent(*node),
      Node::BinExpr(node) => NodeTrait::parent(*node),
      Node::BindingIdent(node) => NodeTrait::parent(*node),
      Node::BlockStmt(node) => NodeTrait::parent(*node),
      Node::Bool(node) => NodeTrait::parent(*node),
      Node::BreakStmt(node) => NodeTrait::parent(*node),
      Node::CallExpr(node) => NodeTrait::parent(*node),
      Node::CatchClause(node) => NodeTrait::parent(*node),
      Node::Class(node) => NodeTrait::parent(*node),
      Node::ClassDecl(node) => NodeTrait::parent(*node),
      Node::ClassExpr(node) => NodeTrait::parent(*node),
      Node::ClassMethod(node) => NodeTrait::parent(*node),
      Node::ClassProp(node) => NodeTrait::parent(*node),
      Node::ComputedPropName(node) => NodeTrait::parent(*node),
      Node::CondExpr(node) => NodeTrait::parent(*node),
      Node::Constructor(node) => NodeTrait::parent(*node),
      Node::ContinueStmt(node) => NodeTrait::parent(*node),
      Node::DebuggerStmt(node) => NodeTrait::parent(*node),
      Node::Decorator(node) => NodeTrait::parent(*node),
      Node::DoWhileStmt(node) => NodeTrait::parent(*node),
      Node::EmptyStmt(node) => NodeTrait::parent(*node),
      Node::ExportAll(node) => NodeTrait::parent(*node),
      Node::ExportDecl(node) => NodeTrait::parent(*node),
      Node::ExportDefaultDecl(node) => NodeTrait::parent(*node),
      Node::ExportDefaultExpr(node) => NodeTrait::parent(*node),
      Node::ExportDefaultSpecifier(node) => NodeTrait::parent(*node),
      Node::ExportNamedSpecifier(node) => NodeTrait::parent(*node),
      Node::ExportNamespaceSpecifier(node) => NodeTrait::parent(*node),
      Node::ExprOrSpread(node) => NodeTrait::parent(*node),
      Node::ExprStmt(node) => NodeTrait::parent(*node),
      Node::FnDecl(node) => NodeTrait::parent(*node),
      Node::FnExpr(node) => NodeTrait::parent(*node),
      Node::ForInStmt(node) => NodeTrait::parent(*node),
      Node::ForOfStmt(node) => NodeTrait::parent(*node),
      Node::ForStmt(node) => NodeTrait::parent(*node),
      Node::Function(node) => NodeTrait::parent(*node),
      Node::GetterProp(node) => NodeTrait::parent(*node),
      Node::Ident(node) => NodeTrait::parent(*node),
      Node::IfStmt(node) => NodeTrait::parent(*node),
      Node::Import(node) => NodeTrait::parent(*node),
      Node::ImportDecl(node) => NodeTrait::parent(*node),
      Node::ImportDefaultSpecifier(node) => NodeTrait::parent(*node),
      Node::ImportNamedSpecifier(node) => NodeTrait::parent(*node),
      Node::ImportStarAsSpecifier(node) => NodeTrait::parent(*node),
      Node::Invalid(node) => NodeTrait::parent(*node),
      Node::JSXAttr(node) => NodeTrait::parent(*node),
      Node::JSXClosingElement(node) => NodeTrait::parent(*node),
      Node::JSXClosingFragment(node) => NodeTrait::parent(*node),
      Node::JSXElement(node) => NodeTrait::parent(*node),
      Node::JSXEmptyExpr(node) => NodeTrait::parent(*node),
      Node::JSXExprContainer(node) => NodeTrait::parent(*node),
      Node::JSXFragment(node) => NodeTrait::parent(*node),
      Node::JSXMemberExpr(node) => NodeTrait::parent(*node),
      Node::JSXNamespacedName(node) => NodeTrait::parent(*node),
      Node::JSXOpeningElement(node) => NodeTrait::parent(*node),
      Node::JSXOpeningFragment(node) => NodeTrait::parent(*node),
      Node::JSXSpreadChild(node) => NodeTrait::parent(*node),
      Node::JSXText(node) => NodeTrait::parent(*node),
      Node::KeyValuePatProp(node) => NodeTrait::parent(*node),
      Node::KeyValueProp(node) => NodeTrait::parent(*node),
      Node::LabeledStmt(node) => NodeTrait::parent(*node),
      Node::MemberExpr(node) => NodeTrait::parent(*node),
      Node::MetaPropExpr(node) => NodeTrait::parent(*node),
      Node::MethodProp(node) => NodeTrait::parent(*node),
      Node::Module(node) => NodeTrait::parent(*node),
      Node::NamedExport(node) => NodeTrait::parent(*node),
      Node::NewExpr(node) => NodeTrait::parent(*node),
      Node::Null(node) => NodeTrait::parent(*node),
      Node::Number(node) => NodeTrait::parent(*node),
      Node::ObjectLit(node) => NodeTrait::parent(*node),
      Node::ObjectPat(node) => NodeTrait::parent(*node),
      Node::OptCall(node) => NodeTrait::parent(*node),
      Node::OptChainExpr(node) => NodeTrait::parent(*node),
      Node::Param(node) => NodeTrait::parent(*node),
      Node::ParenExpr(node) => NodeTrait::parent(*node),
      Node::PrivateMethod(node) => NodeTrait::parent(*node),
      Node::PrivateName(node) => NodeTrait::parent(*node),
      Node::PrivateProp(node) => NodeTrait::parent(*node),
      Node::Regex(node) => NodeTrait::parent(*node),
      Node::RestPat(node) => NodeTrait::parent(*node),
      Node::ReturnStmt(node) => NodeTrait::parent(*node),
      Node::Script(node) => NodeTrait::parent(*node),
      Node::SeqExpr(node) => NodeTrait::parent(*node),
      Node::SetterProp(node) => NodeTrait::parent(*node),
      Node::SpreadElement(node) => NodeTrait::parent(*node),
      Node::StaticBlock(node) => NodeTrait::parent(*node),
      Node::Str(node) => NodeTrait::parent(*node),
      Node::Super(node) => NodeTrait::parent(*node),
      Node::SuperPropExpr(node) => NodeTrait::parent(*node),
      Node::SwitchCase(node) => NodeTrait::parent(*node),
      Node::SwitchStmt(node) => NodeTrait::parent(*node),
      Node::TaggedTpl(node) => NodeTrait::parent(*node),
      Node::ThisExpr(node) => NodeTrait::parent(*node),
      Node::ThrowStmt(node) => NodeTrait::parent(*node),
      Node::Tpl(node) => NodeTrait::parent(*node),
      Node::TplElement(node) => NodeTrait::parent(*node),
      Node::TryStmt(node) => NodeTrait::parent(*node),
      Node::TsArrayType(node) => NodeTrait::parent(*node),
      Node::TsAsExpr(node) => NodeTrait::parent(*node),
      Node::TsCallSignatureDecl(node) => NodeTrait::parent(*node),
      Node::TsConditionalType(node) => NodeTrait::parent(*node),
      Node::TsConstAssertion(node) => NodeTrait::parent(*node),
      Node::TsConstructSignatureDecl(node) => NodeTrait::parent(*node),
      Node::TsConstructorType(node) => NodeTrait::parent(*node),
      Node::TsEnumDecl(node) => NodeTrait::parent(*node),
      Node::TsEnumMember(node) => NodeTrait::parent(*node),
      Node::TsExportAssignment(node) => NodeTrait::parent(*node),
      Node::TsExprWithTypeArgs(node) => NodeTrait::parent(*node),
      Node::TsExternalModuleRef(node) => NodeTrait::parent(*node),
      Node::TsFnType(node) => NodeTrait::parent(*node),
      Node::TsGetterSignature(node) => NodeTrait::parent(*node),
      Node::TsImportEqualsDecl(node) => NodeTrait::parent(*node),
      Node::TsImportType(node) => NodeTrait::parent(*node),
      Node::TsIndexSignature(node) => NodeTrait::parent(*node),
      Node::TsIndexedAccessType(node) => NodeTrait::parent(*node),
      Node::TsInferType(node) => NodeTrait::parent(*node),
      Node::TsInstantiation(node) => NodeTrait::parent(*node),
      Node::TsInterfaceBody(node) => NodeTrait::parent(*node),
      Node::TsInterfaceDecl(node) => NodeTrait::parent(*node),
      Node::TsIntersectionType(node) => NodeTrait::parent(*node),
      Node::TsKeywordType(node) => NodeTrait::parent(*node),
      Node::TsLitType(node) => NodeTrait::parent(*node),
      Node::TsMappedType(node) => NodeTrait::parent(*node),
      Node::TsMethodSignature(node) => NodeTrait::parent(*node),
      Node::TsModuleBlock(node) => NodeTrait::parent(*node),
      Node::TsModuleDecl(node) => NodeTrait::parent(*node),
      Node::TsNamespaceDecl(node) => NodeTrait::parent(*node),
      Node::TsNamespaceExportDecl(node) => NodeTrait::parent(*node),
      Node::TsNonNullExpr(node) => NodeTrait::parent(*node),
      Node::TsOptionalType(node) => NodeTrait::parent(*node),
      Node::TsParamProp(node) => NodeTrait::parent(*node),
      Node::TsParenthesizedType(node) => NodeTrait::parent(*node),
      Node::TsPropertySignature(node) => NodeTrait::parent(*node),
      Node::TsQualifiedName(node) => NodeTrait::parent(*node),
      Node::TsRestType(node) => NodeTrait::parent(*node),
      Node::TsSetterSignature(node) => NodeTrait::parent(*node),
      Node::TsThisType(node) => NodeTrait::parent(*node),
      Node::TsTplLitType(node) => NodeTrait::parent(*node),
      Node::TsTupleElement(node) => NodeTrait::parent(*node),
      Node::TsTupleType(node) => NodeTrait::parent(*node),
      Node::TsTypeAliasDecl(node) => NodeTrait::parent(*node),
      Node::TsTypeAnn(node) => NodeTrait::parent(*node),
      Node::TsTypeAssertion(node) => NodeTrait::parent(*node),
      Node::TsTypeLit(node) => NodeTrait::parent(*node),
      Node::TsTypeOperator(node) => NodeTrait::parent(*node),
      Node::TsTypeParam(node) => NodeTrait::parent(*node),
      Node::TsTypeParamDecl(node) => NodeTrait::parent(*node),
      Node::TsTypeParamInstantiation(node) => NodeTrait::parent(*node),
      Node::TsTypePredicate(node) => NodeTrait::parent(*node),
      Node::TsTypeQuery(node) => NodeTrait::parent(*node),
      Node::TsTypeRef(node) => NodeTrait::parent(*node),
      Node::TsUnionType(node) => NodeTrait::parent(*node),
      Node::UnaryExpr(node) => NodeTrait::parent(*node),
      Node::UpdateExpr(node) => NodeTrait::parent(*node),
      Node::VarDecl(node) => NodeTrait::parent(*node),
      Node::VarDeclarator(node) => NodeTrait::parent(*node),
      Node::WhileStmt(node) => NodeTrait::parent(*node),
      Node::WithStmt(node) => NodeTrait::parent(*node),
      Node::YieldExpr(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      Node::ArrayLit(node) => node.children(),
      Node::ArrayPat(node) => node.children(),
      Node::ArrowExpr(node) => node.children(),
      Node::AssignExpr(node) => node.children(),
      Node::AssignPat(node) => node.children(),
      Node::AssignPatProp(node) => node.children(),
      Node::AssignProp(node) => node.children(),
      Node::AwaitExpr(node) => node.children(),
      Node::BigInt(node) => node.children(),
      Node::BinExpr(node) => node.children(),
      Node::BindingIdent(node) => node.children(),
      Node::BlockStmt(node) => node.children(),
      Node::Bool(node) => node.children(),
      Node::BreakStmt(node) => node.children(),
      Node::CallExpr(node) => node.children(),
      Node::CatchClause(node) => node.children(),
      Node::Class(node) => node.children(),
      Node::ClassDecl(node) => node.children(),
      Node::ClassExpr(node) => node.children(),
      Node::ClassMethod(node) => node.children(),
      Node::ClassProp(node) => node.children(),
      Node::ComputedPropName(node) => node.children(),
      Node::CondExpr(node) => node.children(),
      Node::Constructor(node) => node.children(),
      Node::ContinueStmt(node) => node.children(),
      Node::DebuggerStmt(node) => node.children(),
      Node::Decorator(node) => node.children(),
      Node::DoWhileStmt(node) => node.children(),
      Node::EmptyStmt(node) => node.children(),
      Node::ExportAll(node) => node.children(),
      Node::ExportDecl(node) => node.children(),
      Node::ExportDefaultDecl(node) => node.children(),
      Node::ExportDefaultExpr(node) => node.children(),
      Node::ExportDefaultSpecifier(node) => node.children(),
      Node::ExportNamedSpecifier(node) => node.children(),
      Node::ExportNamespaceSpecifier(node) => node.children(),
      Node::ExprOrSpread(node) => node.children(),
      Node::ExprStmt(node) => node.children(),
      Node::FnDecl(node) => node.children(),
      Node::FnExpr(node) => node.children(),
      Node::ForInStmt(node) => node.children(),
      Node::ForOfStmt(node) => node.children(),
      Node::ForStmt(node) => node.children(),
      Node::Function(node) => node.children(),
      Node::GetterProp(node) => node.children(),
      Node::Ident(node) => node.children(),
      Node::IfStmt(node) => node.children(),
      Node::Import(node) => node.children(),
      Node::ImportDecl(node) => node.children(),
      Node::ImportDefaultSpecifier(node) => node.children(),
      Node::ImportNamedSpecifier(node) => node.children(),
      Node::ImportStarAsSpecifier(node) => node.children(),
      Node::Invalid(node) => node.children(),
      Node::JSXAttr(node) => node.children(),
      Node::JSXClosingElement(node) => node.children(),
      Node::JSXClosingFragment(node) => node.children(),
      Node::JSXElement(node) => node.children(),
      Node::JSXEmptyExpr(node) => node.children(),
      Node::JSXExprContainer(node) => node.children(),
      Node::JSXFragment(node) => node.children(),
      Node::JSXMemberExpr(node) => node.children(),
      Node::JSXNamespacedName(node) => node.children(),
      Node::JSXOpeningElement(node) => node.children(),
      Node::JSXOpeningFragment(node) => node.children(),
      Node::JSXSpreadChild(node) => node.children(),
      Node::JSXText(node) => node.children(),
      Node::KeyValuePatProp(node) => node.children(),
      Node::KeyValueProp(node) => node.children(),
      Node::LabeledStmt(node) => node.children(),
      Node::MemberExpr(node) => node.children(),
      Node::MetaPropExpr(node) => node.children(),
      Node::MethodProp(node) => node.children(),
      Node::Module(node) => node.children(),
      Node::NamedExport(node) => node.children(),
      Node::NewExpr(node) => node.children(),
      Node::Null(node) => node.children(),
      Node::Number(node) => node.children(),
      Node::ObjectLit(node) => node.children(),
      Node::ObjectPat(node) => node.children(),
      Node::OptCall(node) => node.children(),
      Node::OptChainExpr(node) => node.children(),
      Node::Param(node) => node.children(),
      Node::ParenExpr(node) => node.children(),
      Node::PrivateMethod(node) => node.children(),
      Node::PrivateName(node) => node.children(),
      Node::PrivateProp(node) => node.children(),
      Node::Regex(node) => node.children(),
      Node::RestPat(node) => node.children(),
      Node::ReturnStmt(node) => node.children(),
      Node::Script(node) => node.children(),
      Node::SeqExpr(node) => node.children(),
      Node::SetterProp(node) => node.children(),
      Node::SpreadElement(node) => node.children(),
      Node::StaticBlock(node) => node.children(),
      Node::Str(node) => node.children(),
      Node::Super(node) => node.children(),
      Node::SuperPropExpr(node) => node.children(),
      Node::SwitchCase(node) => node.children(),
      Node::SwitchStmt(node) => node.children(),
      Node::TaggedTpl(node) => node.children(),
      Node::ThisExpr(node) => node.children(),
      Node::ThrowStmt(node) => node.children(),
      Node::Tpl(node) => node.children(),
      Node::TplElement(node) => node.children(),
      Node::TryStmt(node) => node.children(),
      Node::TsArrayType(node) => node.children(),
      Node::TsAsExpr(node) => node.children(),
      Node::TsCallSignatureDecl(node) => node.children(),
      Node::TsConditionalType(node) => node.children(),
      Node::TsConstAssertion(node) => node.children(),
      Node::TsConstructSignatureDecl(node) => node.children(),
      Node::TsConstructorType(node) => node.children(),
      Node::TsEnumDecl(node) => node.children(),
      Node::TsEnumMember(node) => node.children(),
      Node::TsExportAssignment(node) => node.children(),
      Node::TsExprWithTypeArgs(node) => node.children(),
      Node::TsExternalModuleRef(node) => node.children(),
      Node::TsFnType(node) => node.children(),
      Node::TsGetterSignature(node) => node.children(),
      Node::TsImportEqualsDecl(node) => node.children(),
      Node::TsImportType(node) => node.children(),
      Node::TsIndexSignature(node) => node.children(),
      Node::TsIndexedAccessType(node) => node.children(),
      Node::TsInferType(node) => node.children(),
      Node::TsInstantiation(node) => node.children(),
      Node::TsInterfaceBody(node) => node.children(),
      Node::TsInterfaceDecl(node) => node.children(),
      Node::TsIntersectionType(node) => node.children(),
      Node::TsKeywordType(node) => node.children(),
      Node::TsLitType(node) => node.children(),
      Node::TsMappedType(node) => node.children(),
      Node::TsMethodSignature(node) => node.children(),
      Node::TsModuleBlock(node) => node.children(),
      Node::TsModuleDecl(node) => node.children(),
      Node::TsNamespaceDecl(node) => node.children(),
      Node::TsNamespaceExportDecl(node) => node.children(),
      Node::TsNonNullExpr(node) => node.children(),
      Node::TsOptionalType(node) => node.children(),
      Node::TsParamProp(node) => node.children(),
      Node::TsParenthesizedType(node) => node.children(),
      Node::TsPropertySignature(node) => node.children(),
      Node::TsQualifiedName(node) => node.children(),
      Node::TsRestType(node) => node.children(),
      Node::TsSetterSignature(node) => node.children(),
      Node::TsThisType(node) => node.children(),
      Node::TsTplLitType(node) => node.children(),
      Node::TsTupleElement(node) => node.children(),
      Node::TsTupleType(node) => node.children(),
      Node::TsTypeAliasDecl(node) => node.children(),
      Node::TsTypeAnn(node) => node.children(),
      Node::TsTypeAssertion(node) => node.children(),
      Node::TsTypeLit(node) => node.children(),
      Node::TsTypeOperator(node) => node.children(),
      Node::TsTypeParam(node) => node.children(),
      Node::TsTypeParamDecl(node) => node.children(),
      Node::TsTypeParamInstantiation(node) => node.children(),
      Node::TsTypePredicate(node) => node.children(),
      Node::TsTypeQuery(node) => node.children(),
      Node::TsTypeRef(node) => node.children(),
      Node::TsUnionType(node) => node.children(),
      Node::UnaryExpr(node) => node.children(),
      Node::UpdateExpr(node) => node.children(),
      Node::VarDecl(node) => node.children(),
      Node::VarDeclarator(node) => node.children(),
      Node::WhileStmt(node) => node.children(),
      Node::WithStmt(node) => node.children(),
      Node::YieldExpr(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      Node::ArrayLit(node) => node.as_node(),
      Node::ArrayPat(node) => node.as_node(),
      Node::ArrowExpr(node) => node.as_node(),
      Node::AssignExpr(node) => node.as_node(),
      Node::AssignPat(node) => node.as_node(),
      Node::AssignPatProp(node) => node.as_node(),
      Node::AssignProp(node) => node.as_node(),
      Node::AwaitExpr(node) => node.as_node(),
      Node::BigInt(node) => node.as_node(),
      Node::BinExpr(node) => node.as_node(),
      Node::BindingIdent(node) => node.as_node(),
      Node::BlockStmt(node) => node.as_node(),
      Node::Bool(node) => node.as_node(),
      Node::BreakStmt(node) => node.as_node(),
      Node::CallExpr(node) => node.as_node(),
      Node::CatchClause(node) => node.as_node(),
      Node::Class(node) => node.as_node(),
      Node::ClassDecl(node) => node.as_node(),
      Node::ClassExpr(node) => node.as_node(),
      Node::ClassMethod(node) => node.as_node(),
      Node::ClassProp(node) => node.as_node(),
      Node::ComputedPropName(node) => node.as_node(),
      Node::CondExpr(node) => node.as_node(),
      Node::Constructor(node) => node.as_node(),
      Node::ContinueStmt(node) => node.as_node(),
      Node::DebuggerStmt(node) => node.as_node(),
      Node::Decorator(node) => node.as_node(),
      Node::DoWhileStmt(node) => node.as_node(),
      Node::EmptyStmt(node) => node.as_node(),
      Node::ExportAll(node) => node.as_node(),
      Node::ExportDecl(node) => node.as_node(),
      Node::ExportDefaultDecl(node) => node.as_node(),
      Node::ExportDefaultExpr(node) => node.as_node(),
      Node::ExportDefaultSpecifier(node) => node.as_node(),
      Node::ExportNamedSpecifier(node) => node.as_node(),
      Node::ExportNamespaceSpecifier(node) => node.as_node(),
      Node::ExprOrSpread(node) => node.as_node(),
      Node::ExprStmt(node) => node.as_node(),
      Node::FnDecl(node) => node.as_node(),
      Node::FnExpr(node) => node.as_node(),
      Node::ForInStmt(node) => node.as_node(),
      Node::ForOfStmt(node) => node.as_node(),
      Node::ForStmt(node) => node.as_node(),
      Node::Function(node) => node.as_node(),
      Node::GetterProp(node) => node.as_node(),
      Node::Ident(node) => node.as_node(),
      Node::IfStmt(node) => node.as_node(),
      Node::Import(node) => node.as_node(),
      Node::ImportDecl(node) => node.as_node(),
      Node::ImportDefaultSpecifier(node) => node.as_node(),
      Node::ImportNamedSpecifier(node) => node.as_node(),
      Node::ImportStarAsSpecifier(node) => node.as_node(),
      Node::Invalid(node) => node.as_node(),
      Node::JSXAttr(node) => node.as_node(),
      Node::JSXClosingElement(node) => node.as_node(),
      Node::JSXClosingFragment(node) => node.as_node(),
      Node::JSXElement(node) => node.as_node(),
      Node::JSXEmptyExpr(node) => node.as_node(),
      Node::JSXExprContainer(node) => node.as_node(),
      Node::JSXFragment(node) => node.as_node(),
      Node::JSXMemberExpr(node) => node.as_node(),
      Node::JSXNamespacedName(node) => node.as_node(),
      Node::JSXOpeningElement(node) => node.as_node(),
      Node::JSXOpeningFragment(node) => node.as_node(),
      Node::JSXSpreadChild(node) => node.as_node(),
      Node::JSXText(node) => node.as_node(),
      Node::KeyValuePatProp(node) => node.as_node(),
      Node::KeyValueProp(node) => node.as_node(),
      Node::LabeledStmt(node) => node.as_node(),
      Node::MemberExpr(node) => node.as_node(),
      Node::MetaPropExpr(node) => node.as_node(),
      Node::MethodProp(node) => node.as_node(),
      Node::Module(node) => node.as_node(),
      Node::NamedExport(node) => node.as_node(),
      Node::NewExpr(node) => node.as_node(),
      Node::Null(node) => node.as_node(),
      Node::Number(node) => node.as_node(),
      Node::ObjectLit(node) => node.as_node(),
      Node::ObjectPat(node) => node.as_node(),
      Node::OptCall(node) => node.as_node(),
      Node::OptChainExpr(node) => node.as_node(),
      Node::Param(node) => node.as_node(),
      Node::ParenExpr(node) => node.as_node(),
      Node::PrivateMethod(node) => node.as_node(),
      Node::PrivateName(node) => node.as_node(),
      Node::PrivateProp(node) => node.as_node(),
      Node::Regex(node) => node.as_node(),
      Node::RestPat(node) => node.as_node(),
      Node::ReturnStmt(node) => node.as_node(),
      Node::Script(node) => node.as_node(),
      Node::SeqExpr(node) => node.as_node(),
      Node::SetterProp(node) => node.as_node(),
      Node::SpreadElement(node) => node.as_node(),
      Node::StaticBlock(node) => node.as_node(),
      Node::Str(node) => node.as_node(),
      Node::Super(node) => node.as_node(),
      Node::SuperPropExpr(node) => node.as_node(),
      Node::SwitchCase(node) => node.as_node(),
      Node::SwitchStmt(node) => node.as_node(),
      Node::TaggedTpl(node) => node.as_node(),
      Node::ThisExpr(node) => node.as_node(),
      Node::ThrowStmt(node) => node.as_node(),
      Node::Tpl(node) => node.as_node(),
      Node::TplElement(node) => node.as_node(),
      Node::TryStmt(node) => node.as_node(),
      Node::TsArrayType(node) => node.as_node(),
      Node::TsAsExpr(node) => node.as_node(),
      Node::TsCallSignatureDecl(node) => node.as_node(),
      Node::TsConditionalType(node) => node.as_node(),
      Node::TsConstAssertion(node) => node.as_node(),
      Node::TsConstructSignatureDecl(node) => node.as_node(),
      Node::TsConstructorType(node) => node.as_node(),
      Node::TsEnumDecl(node) => node.as_node(),
      Node::TsEnumMember(node) => node.as_node(),
      Node::TsExportAssignment(node) => node.as_node(),
      Node::TsExprWithTypeArgs(node) => node.as_node(),
      Node::TsExternalModuleRef(node) => node.as_node(),
      Node::TsFnType(node) => node.as_node(),
      Node::TsGetterSignature(node) => node.as_node(),
      Node::TsImportEqualsDecl(node) => node.as_node(),
      Node::TsImportType(node) => node.as_node(),
      Node::TsIndexSignature(node) => node.as_node(),
      Node::TsIndexedAccessType(node) => node.as_node(),
      Node::TsInferType(node) => node.as_node(),
      Node::TsInstantiation(node) => node.as_node(),
      Node::TsInterfaceBody(node) => node.as_node(),
      Node::TsInterfaceDecl(node) => node.as_node(),
      Node::TsIntersectionType(node) => node.as_node(),
      Node::TsKeywordType(node) => node.as_node(),
      Node::TsLitType(node) => node.as_node(),
      Node::TsMappedType(node) => node.as_node(),
      Node::TsMethodSignature(node) => node.as_node(),
      Node::TsModuleBlock(node) => node.as_node(),
      Node::TsModuleDecl(node) => node.as_node(),
      Node::TsNamespaceDecl(node) => node.as_node(),
      Node::TsNamespaceExportDecl(node) => node.as_node(),
      Node::TsNonNullExpr(node) => node.as_node(),
      Node::TsOptionalType(node) => node.as_node(),
      Node::TsParamProp(node) => node.as_node(),
      Node::TsParenthesizedType(node) => node.as_node(),
      Node::TsPropertySignature(node) => node.as_node(),
      Node::TsQualifiedName(node) => node.as_node(),
      Node::TsRestType(node) => node.as_node(),
      Node::TsSetterSignature(node) => node.as_node(),
      Node::TsThisType(node) => node.as_node(),
      Node::TsTplLitType(node) => node.as_node(),
      Node::TsTupleElement(node) => node.as_node(),
      Node::TsTupleType(node) => node.as_node(),
      Node::TsTypeAliasDecl(node) => node.as_node(),
      Node::TsTypeAnn(node) => node.as_node(),
      Node::TsTypeAssertion(node) => node.as_node(),
      Node::TsTypeLit(node) => node.as_node(),
      Node::TsTypeOperator(node) => node.as_node(),
      Node::TsTypeParam(node) => node.as_node(),
      Node::TsTypeParamDecl(node) => node.as_node(),
      Node::TsTypeParamInstantiation(node) => node.as_node(),
      Node::TsTypePredicate(node) => node.as_node(),
      Node::TsTypeQuery(node) => node.as_node(),
      Node::TsTypeRef(node) => node.as_node(),
      Node::TsUnionType(node) => node.as_node(),
      Node::UnaryExpr(node) => node.as_node(),
      Node::UpdateExpr(node) => node.as_node(),
      Node::VarDecl(node) => node.as_node(),
      Node::VarDeclarator(node) => node.as_node(),
      Node::WhileStmt(node) => node.as_node(),
      Node::WithStmt(node) => node.as_node(),
      Node::YieldExpr(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Node::ArrayLit(_) => NodeKind::ArrayLit,
      Node::ArrayPat(_) => NodeKind::ArrayPat,
      Node::ArrowExpr(_) => NodeKind::ArrowExpr,
      Node::AssignExpr(_) => NodeKind::AssignExpr,
      Node::AssignPat(_) => NodeKind::AssignPat,
      Node::AssignPatProp(_) => NodeKind::AssignPatProp,
      Node::AssignProp(_) => NodeKind::AssignProp,
      Node::AwaitExpr(_) => NodeKind::AwaitExpr,
      Node::BigInt(_) => NodeKind::BigInt,
      Node::BinExpr(_) => NodeKind::BinExpr,
      Node::BindingIdent(_) => NodeKind::BindingIdent,
      Node::BlockStmt(_) => NodeKind::BlockStmt,
      Node::Bool(_) => NodeKind::Bool,
      Node::BreakStmt(_) => NodeKind::BreakStmt,
      Node::CallExpr(_) => NodeKind::CallExpr,
      Node::CatchClause(_) => NodeKind::CatchClause,
      Node::Class(_) => NodeKind::Class,
      Node::ClassDecl(_) => NodeKind::ClassDecl,
      Node::ClassExpr(_) => NodeKind::ClassExpr,
      Node::ClassMethod(_) => NodeKind::ClassMethod,
      Node::ClassProp(_) => NodeKind::ClassProp,
      Node::ComputedPropName(_) => NodeKind::ComputedPropName,
      Node::CondExpr(_) => NodeKind::CondExpr,
      Node::Constructor(_) => NodeKind::Constructor,
      Node::ContinueStmt(_) => NodeKind::ContinueStmt,
      Node::DebuggerStmt(_) => NodeKind::DebuggerStmt,
      Node::Decorator(_) => NodeKind::Decorator,
      Node::DoWhileStmt(_) => NodeKind::DoWhileStmt,
      Node::EmptyStmt(_) => NodeKind::EmptyStmt,
      Node::ExportAll(_) => NodeKind::ExportAll,
      Node::ExportDecl(_) => NodeKind::ExportDecl,
      Node::ExportDefaultDecl(_) => NodeKind::ExportDefaultDecl,
      Node::ExportDefaultExpr(_) => NodeKind::ExportDefaultExpr,
      Node::ExportDefaultSpecifier(_) => NodeKind::ExportDefaultSpecifier,
      Node::ExportNamedSpecifier(_) => NodeKind::ExportNamedSpecifier,
      Node::ExportNamespaceSpecifier(_) => NodeKind::ExportNamespaceSpecifier,
      Node::ExprOrSpread(_) => NodeKind::ExprOrSpread,
      Node::ExprStmt(_) => NodeKind::ExprStmt,
      Node::FnDecl(_) => NodeKind::FnDecl,
      Node::FnExpr(_) => NodeKind::FnExpr,
      Node::ForInStmt(_) => NodeKind::ForInStmt,
      Node::ForOfStmt(_) => NodeKind::ForOfStmt,
      Node::ForStmt(_) => NodeKind::ForStmt,
      Node::Function(_) => NodeKind::Function,
      Node::GetterProp(_) => NodeKind::GetterProp,
      Node::Ident(_) => NodeKind::Ident,
      Node::IfStmt(_) => NodeKind::IfStmt,
      Node::Import(_) => NodeKind::Import,
      Node::ImportDecl(_) => NodeKind::ImportDecl,
      Node::ImportDefaultSpecifier(_) => NodeKind::ImportDefaultSpecifier,
      Node::ImportNamedSpecifier(_) => NodeKind::ImportNamedSpecifier,
      Node::ImportStarAsSpecifier(_) => NodeKind::ImportStarAsSpecifier,
      Node::Invalid(_) => NodeKind::Invalid,
      Node::JSXAttr(_) => NodeKind::JSXAttr,
      Node::JSXClosingElement(_) => NodeKind::JSXClosingElement,
      Node::JSXClosingFragment(_) => NodeKind::JSXClosingFragment,
      Node::JSXElement(_) => NodeKind::JSXElement,
      Node::JSXEmptyExpr(_) => NodeKind::JSXEmptyExpr,
      Node::JSXExprContainer(_) => NodeKind::JSXExprContainer,
      Node::JSXFragment(_) => NodeKind::JSXFragment,
      Node::JSXMemberExpr(_) => NodeKind::JSXMemberExpr,
      Node::JSXNamespacedName(_) => NodeKind::JSXNamespacedName,
      Node::JSXOpeningElement(_) => NodeKind::JSXOpeningElement,
      Node::JSXOpeningFragment(_) => NodeKind::JSXOpeningFragment,
      Node::JSXSpreadChild(_) => NodeKind::JSXSpreadChild,
      Node::JSXText(_) => NodeKind::JSXText,
      Node::KeyValuePatProp(_) => NodeKind::KeyValuePatProp,
      Node::KeyValueProp(_) => NodeKind::KeyValueProp,
      Node::LabeledStmt(_) => NodeKind::LabeledStmt,
      Node::MemberExpr(_) => NodeKind::MemberExpr,
      Node::MetaPropExpr(_) => NodeKind::MetaPropExpr,
      Node::MethodProp(_) => NodeKind::MethodProp,
      Node::Module(_) => NodeKind::Module,
      Node::NamedExport(_) => NodeKind::NamedExport,
      Node::NewExpr(_) => NodeKind::NewExpr,
      Node::Null(_) => NodeKind::Null,
      Node::Number(_) => NodeKind::Number,
      Node::ObjectLit(_) => NodeKind::ObjectLit,
      Node::ObjectPat(_) => NodeKind::ObjectPat,
      Node::OptCall(_) => NodeKind::OptCall,
      Node::OptChainExpr(_) => NodeKind::OptChainExpr,
      Node::Param(_) => NodeKind::Param,
      Node::ParenExpr(_) => NodeKind::ParenExpr,
      Node::PrivateMethod(_) => NodeKind::PrivateMethod,
      Node::PrivateName(_) => NodeKind::PrivateName,
      Node::PrivateProp(_) => NodeKind::PrivateProp,
      Node::Regex(_) => NodeKind::Regex,
      Node::RestPat(_) => NodeKind::RestPat,
      Node::ReturnStmt(_) => NodeKind::ReturnStmt,
      Node::Script(_) => NodeKind::Script,
      Node::SeqExpr(_) => NodeKind::SeqExpr,
      Node::SetterProp(_) => NodeKind::SetterProp,
      Node::SpreadElement(_) => NodeKind::SpreadElement,
      Node::StaticBlock(_) => NodeKind::StaticBlock,
      Node::Str(_) => NodeKind::Str,
      Node::Super(_) => NodeKind::Super,
      Node::SuperPropExpr(_) => NodeKind::SuperPropExpr,
      Node::SwitchCase(_) => NodeKind::SwitchCase,
      Node::SwitchStmt(_) => NodeKind::SwitchStmt,
      Node::TaggedTpl(_) => NodeKind::TaggedTpl,
      Node::ThisExpr(_) => NodeKind::ThisExpr,
      Node::ThrowStmt(_) => NodeKind::ThrowStmt,
      Node::Tpl(_) => NodeKind::Tpl,
      Node::TplElement(_) => NodeKind::TplElement,
      Node::TryStmt(_) => NodeKind::TryStmt,
      Node::TsArrayType(_) => NodeKind::TsArrayType,
      Node::TsAsExpr(_) => NodeKind::TsAsExpr,
      Node::TsCallSignatureDecl(_) => NodeKind::TsCallSignatureDecl,
      Node::TsConditionalType(_) => NodeKind::TsConditionalType,
      Node::TsConstAssertion(_) => NodeKind::TsConstAssertion,
      Node::TsConstructSignatureDecl(_) => NodeKind::TsConstructSignatureDecl,
      Node::TsConstructorType(_) => NodeKind::TsConstructorType,
      Node::TsEnumDecl(_) => NodeKind::TsEnumDecl,
      Node::TsEnumMember(_) => NodeKind::TsEnumMember,
      Node::TsExportAssignment(_) => NodeKind::TsExportAssignment,
      Node::TsExprWithTypeArgs(_) => NodeKind::TsExprWithTypeArgs,
      Node::TsExternalModuleRef(_) => NodeKind::TsExternalModuleRef,
      Node::TsFnType(_) => NodeKind::TsFnType,
      Node::TsGetterSignature(_) => NodeKind::TsGetterSignature,
      Node::TsImportEqualsDecl(_) => NodeKind::TsImportEqualsDecl,
      Node::TsImportType(_) => NodeKind::TsImportType,
      Node::TsIndexSignature(_) => NodeKind::TsIndexSignature,
      Node::TsIndexedAccessType(_) => NodeKind::TsIndexedAccessType,
      Node::TsInferType(_) => NodeKind::TsInferType,
      Node::TsInstantiation(_) => NodeKind::TsInstantiation,
      Node::TsInterfaceBody(_) => NodeKind::TsInterfaceBody,
      Node::TsInterfaceDecl(_) => NodeKind::TsInterfaceDecl,
      Node::TsIntersectionType(_) => NodeKind::TsIntersectionType,
      Node::TsKeywordType(_) => NodeKind::TsKeywordType,
      Node::TsLitType(_) => NodeKind::TsLitType,
      Node::TsMappedType(_) => NodeKind::TsMappedType,
      Node::TsMethodSignature(_) => NodeKind::TsMethodSignature,
      Node::TsModuleBlock(_) => NodeKind::TsModuleBlock,
      Node::TsModuleDecl(_) => NodeKind::TsModuleDecl,
      Node::TsNamespaceDecl(_) => NodeKind::TsNamespaceDecl,
      Node::TsNamespaceExportDecl(_) => NodeKind::TsNamespaceExportDecl,
      Node::TsNonNullExpr(_) => NodeKind::TsNonNullExpr,
      Node::TsOptionalType(_) => NodeKind::TsOptionalType,
      Node::TsParamProp(_) => NodeKind::TsParamProp,
      Node::TsParenthesizedType(_) => NodeKind::TsParenthesizedType,
      Node::TsPropertySignature(_) => NodeKind::TsPropertySignature,
      Node::TsQualifiedName(_) => NodeKind::TsQualifiedName,
      Node::TsRestType(_) => NodeKind::TsRestType,
      Node::TsSetterSignature(_) => NodeKind::TsSetterSignature,
      Node::TsThisType(_) => NodeKind::TsThisType,
      Node::TsTplLitType(_) => NodeKind::TsTplLitType,
      Node::TsTupleElement(_) => NodeKind::TsTupleElement,
      Node::TsTupleType(_) => NodeKind::TsTupleType,
      Node::TsTypeAliasDecl(_) => NodeKind::TsTypeAliasDecl,
      Node::TsTypeAnn(_) => NodeKind::TsTypeAnn,
      Node::TsTypeAssertion(_) => NodeKind::TsTypeAssertion,
      Node::TsTypeLit(_) => NodeKind::TsTypeLit,
      Node::TsTypeOperator(_) => NodeKind::TsTypeOperator,
      Node::TsTypeParam(_) => NodeKind::TsTypeParam,
      Node::TsTypeParamDecl(_) => NodeKind::TsTypeParamDecl,
      Node::TsTypeParamInstantiation(_) => NodeKind::TsTypeParamInstantiation,
      Node::TsTypePredicate(_) => NodeKind::TsTypePredicate,
      Node::TsTypeQuery(_) => NodeKind::TsTypeQuery,
      Node::TsTypeRef(_) => NodeKind::TsTypeRef,
      Node::TsUnionType(_) => NodeKind::TsUnionType,
      Node::UnaryExpr(_) => NodeKind::UnaryExpr,
      Node::UpdateExpr(_) => NodeKind::UpdateExpr,
      Node::VarDecl(_) => NodeKind::VarDecl,
      Node::VarDeclarator(_) => NodeKind::VarDeclarator,
      Node::WhileStmt(_) => NodeKind::WhileStmt,
      Node::WithStmt(_) => NodeKind::WithStmt,
      Node::YieldExpr(_) => NodeKind::YieldExpr,
    }
  }
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum NodeKind {
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
  Import,
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
  OptCall,
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
  StaticBlock,
  Str,
  Super,
  SuperPropExpr,
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
  TsGetterSignature,
  TsImportEqualsDecl,
  TsImportType,
  TsIndexSignature,
  TsIndexedAccessType,
  TsInferType,
  TsInstantiation,
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
  TsSetterSignature,
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

impl std::fmt::Display for NodeKind {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      NodeKind::ArrayLit => "ArrayLit",
      NodeKind::ArrayPat => "ArrayPat",
      NodeKind::ArrowExpr => "ArrowExpr",
      NodeKind::AssignExpr => "AssignExpr",
      NodeKind::AssignPat => "AssignPat",
      NodeKind::AssignPatProp => "AssignPatProp",
      NodeKind::AssignProp => "AssignProp",
      NodeKind::AwaitExpr => "AwaitExpr",
      NodeKind::BigInt => "BigInt",
      NodeKind::BinExpr => "BinExpr",
      NodeKind::BindingIdent => "BindingIdent",
      NodeKind::BlockStmt => "BlockStmt",
      NodeKind::Bool => "Bool",
      NodeKind::BreakStmt => "BreakStmt",
      NodeKind::CallExpr => "CallExpr",
      NodeKind::CatchClause => "CatchClause",
      NodeKind::Class => "Class",
      NodeKind::ClassDecl => "ClassDecl",
      NodeKind::ClassExpr => "ClassExpr",
      NodeKind::ClassMethod => "ClassMethod",
      NodeKind::ClassProp => "ClassProp",
      NodeKind::ComputedPropName => "ComputedPropName",
      NodeKind::CondExpr => "CondExpr",
      NodeKind::Constructor => "Constructor",
      NodeKind::ContinueStmt => "ContinueStmt",
      NodeKind::DebuggerStmt => "DebuggerStmt",
      NodeKind::Decorator => "Decorator",
      NodeKind::DoWhileStmt => "DoWhileStmt",
      NodeKind::EmptyStmt => "EmptyStmt",
      NodeKind::ExportAll => "ExportAll",
      NodeKind::ExportDecl => "ExportDecl",
      NodeKind::ExportDefaultDecl => "ExportDefaultDecl",
      NodeKind::ExportDefaultExpr => "ExportDefaultExpr",
      NodeKind::ExportDefaultSpecifier => "ExportDefaultSpecifier",
      NodeKind::ExportNamedSpecifier => "ExportNamedSpecifier",
      NodeKind::ExportNamespaceSpecifier => "ExportNamespaceSpecifier",
      NodeKind::ExprOrSpread => "ExprOrSpread",
      NodeKind::ExprStmt => "ExprStmt",
      NodeKind::FnDecl => "FnDecl",
      NodeKind::FnExpr => "FnExpr",
      NodeKind::ForInStmt => "ForInStmt",
      NodeKind::ForOfStmt => "ForOfStmt",
      NodeKind::ForStmt => "ForStmt",
      NodeKind::Function => "Function",
      NodeKind::GetterProp => "GetterProp",
      NodeKind::Ident => "Ident",
      NodeKind::IfStmt => "IfStmt",
      NodeKind::Import => "Import",
      NodeKind::ImportDecl => "ImportDecl",
      NodeKind::ImportDefaultSpecifier => "ImportDefaultSpecifier",
      NodeKind::ImportNamedSpecifier => "ImportNamedSpecifier",
      NodeKind::ImportStarAsSpecifier => "ImportStarAsSpecifier",
      NodeKind::Invalid => "Invalid",
      NodeKind::JSXAttr => "JSXAttr",
      NodeKind::JSXClosingElement => "JSXClosingElement",
      NodeKind::JSXClosingFragment => "JSXClosingFragment",
      NodeKind::JSXElement => "JSXElement",
      NodeKind::JSXEmptyExpr => "JSXEmptyExpr",
      NodeKind::JSXExprContainer => "JSXExprContainer",
      NodeKind::JSXFragment => "JSXFragment",
      NodeKind::JSXMemberExpr => "JSXMemberExpr",
      NodeKind::JSXNamespacedName => "JSXNamespacedName",
      NodeKind::JSXOpeningElement => "JSXOpeningElement",
      NodeKind::JSXOpeningFragment => "JSXOpeningFragment",
      NodeKind::JSXSpreadChild => "JSXSpreadChild",
      NodeKind::JSXText => "JSXText",
      NodeKind::KeyValuePatProp => "KeyValuePatProp",
      NodeKind::KeyValueProp => "KeyValueProp",
      NodeKind::LabeledStmt => "LabeledStmt",
      NodeKind::MemberExpr => "MemberExpr",
      NodeKind::MetaPropExpr => "MetaPropExpr",
      NodeKind::MethodProp => "MethodProp",
      NodeKind::Module => "Module",
      NodeKind::NamedExport => "NamedExport",
      NodeKind::NewExpr => "NewExpr",
      NodeKind::Null => "Null",
      NodeKind::Number => "Number",
      NodeKind::ObjectLit => "ObjectLit",
      NodeKind::ObjectPat => "ObjectPat",
      NodeKind::OptCall => "OptCall",
      NodeKind::OptChainExpr => "OptChainExpr",
      NodeKind::Param => "Param",
      NodeKind::ParenExpr => "ParenExpr",
      NodeKind::PrivateMethod => "PrivateMethod",
      NodeKind::PrivateName => "PrivateName",
      NodeKind::PrivateProp => "PrivateProp",
      NodeKind::Regex => "Regex",
      NodeKind::RestPat => "RestPat",
      NodeKind::ReturnStmt => "ReturnStmt",
      NodeKind::Script => "Script",
      NodeKind::SeqExpr => "SeqExpr",
      NodeKind::SetterProp => "SetterProp",
      NodeKind::SpreadElement => "SpreadElement",
      NodeKind::StaticBlock => "StaticBlock",
      NodeKind::Str => "Str",
      NodeKind::Super => "Super",
      NodeKind::SuperPropExpr => "SuperPropExpr",
      NodeKind::SwitchCase => "SwitchCase",
      NodeKind::SwitchStmt => "SwitchStmt",
      NodeKind::TaggedTpl => "TaggedTpl",
      NodeKind::ThisExpr => "ThisExpr",
      NodeKind::ThrowStmt => "ThrowStmt",
      NodeKind::Tpl => "Tpl",
      NodeKind::TplElement => "TplElement",
      NodeKind::TryStmt => "TryStmt",
      NodeKind::TsArrayType => "TsArrayType",
      NodeKind::TsAsExpr => "TsAsExpr",
      NodeKind::TsCallSignatureDecl => "TsCallSignatureDecl",
      NodeKind::TsConditionalType => "TsConditionalType",
      NodeKind::TsConstAssertion => "TsConstAssertion",
      NodeKind::TsConstructSignatureDecl => "TsConstructSignatureDecl",
      NodeKind::TsConstructorType => "TsConstructorType",
      NodeKind::TsEnumDecl => "TsEnumDecl",
      NodeKind::TsEnumMember => "TsEnumMember",
      NodeKind::TsExportAssignment => "TsExportAssignment",
      NodeKind::TsExprWithTypeArgs => "TsExprWithTypeArgs",
      NodeKind::TsExternalModuleRef => "TsExternalModuleRef",
      NodeKind::TsFnType => "TsFnType",
      NodeKind::TsGetterSignature => "TsGetterSignature",
      NodeKind::TsImportEqualsDecl => "TsImportEqualsDecl",
      NodeKind::TsImportType => "TsImportType",
      NodeKind::TsIndexSignature => "TsIndexSignature",
      NodeKind::TsIndexedAccessType => "TsIndexedAccessType",
      NodeKind::TsInferType => "TsInferType",
      NodeKind::TsInstantiation => "TsInstantiation",
      NodeKind::TsInterfaceBody => "TsInterfaceBody",
      NodeKind::TsInterfaceDecl => "TsInterfaceDecl",
      NodeKind::TsIntersectionType => "TsIntersectionType",
      NodeKind::TsKeywordType => "TsKeywordType",
      NodeKind::TsLitType => "TsLitType",
      NodeKind::TsMappedType => "TsMappedType",
      NodeKind::TsMethodSignature => "TsMethodSignature",
      NodeKind::TsModuleBlock => "TsModuleBlock",
      NodeKind::TsModuleDecl => "TsModuleDecl",
      NodeKind::TsNamespaceDecl => "TsNamespaceDecl",
      NodeKind::TsNamespaceExportDecl => "TsNamespaceExportDecl",
      NodeKind::TsNonNullExpr => "TsNonNullExpr",
      NodeKind::TsOptionalType => "TsOptionalType",
      NodeKind::TsParamProp => "TsParamProp",
      NodeKind::TsParenthesizedType => "TsParenthesizedType",
      NodeKind::TsPropertySignature => "TsPropertySignature",
      NodeKind::TsQualifiedName => "TsQualifiedName",
      NodeKind::TsRestType => "TsRestType",
      NodeKind::TsSetterSignature => "TsSetterSignature",
      NodeKind::TsThisType => "TsThisType",
      NodeKind::TsTplLitType => "TsTplLitType",
      NodeKind::TsTupleElement => "TsTupleElement",
      NodeKind::TsTupleType => "TsTupleType",
      NodeKind::TsTypeAliasDecl => "TsTypeAliasDecl",
      NodeKind::TsTypeAnn => "TsTypeAnn",
      NodeKind::TsTypeAssertion => "TsTypeAssertion",
      NodeKind::TsTypeLit => "TsTypeLit",
      NodeKind::TsTypeOperator => "TsTypeOperator",
      NodeKind::TsTypeParam => "TsTypeParam",
      NodeKind::TsTypeParamDecl => "TsTypeParamDecl",
      NodeKind::TsTypeParamInstantiation => "TsTypeParamInstantiation",
      NodeKind::TsTypePredicate => "TsTypePredicate",
      NodeKind::TsTypeQuery => "TsTypeQuery",
      NodeKind::TsTypeRef => "TsTypeRef",
      NodeKind::TsUnionType => "TsUnionType",
      NodeKind::UnaryExpr => "UnaryExpr",
      NodeKind::UpdateExpr => "UpdateExpr",
      NodeKind::VarDecl => "VarDecl",
      NodeKind::VarDeclarator => "VarDeclarator",
      NodeKind::WhileStmt => "WhileStmt",
      NodeKind::WithStmt => "WithStmt",
      NodeKind::YieldExpr => "YieldExpr",
    })
  }
}

#[derive(Copy, Clone)]
pub enum BlockStmtOrExpr<'a> {
  BlockStmt(&'a BlockStmt<'a>),
  Expr(Expr<'a>),
}

impl<'a> BlockStmtOrExpr<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for BlockStmtOrExpr<'a> {
  fn start(&self) -> SourcePos {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.start(),
      BlockStmtOrExpr::Expr(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.end(),
      BlockStmtOrExpr::Expr(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for BlockStmtOrExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => NodeTrait::parent(*node),
      BlockStmtOrExpr::Expr(node) => NodeTrait::parent(node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.children(),
      BlockStmtOrExpr::Expr(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      BlockStmtOrExpr::BlockStmt(node) => node.as_node(),
      BlockStmtOrExpr::Expr(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      BlockStmtOrExpr::BlockStmt(_) => NodeKind::BlockStmt,
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

impl<'a> From<BlockStmtOrExpr<'a>> for Node<'a> {
  fn from(node: BlockStmtOrExpr<'a>) -> Node<'a> {
    match node {
      BlockStmtOrExpr::BlockStmt(node) => node.into(),
      BlockStmtOrExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_block_stmt_or_expr<'a>(inner: &'a swc_ast::BlockStmtOrExpr, bump: &'a Bump) -> BlockStmtOrExpr<'a> {
  match inner {
    swc_ast::BlockStmtOrExpr::BlockStmt(value) => BlockStmtOrExpr::BlockStmt(get_view_for_block_stmt(value, bump)),
    swc_ast::BlockStmtOrExpr::Expr(value) => BlockStmtOrExpr::Expr(get_view_for_expr(value, bump)),
  }
}

fn set_parent_for_block_stmt_or_expr<'a>(node: &BlockStmtOrExpr<'a>, parent: Node<'a>) {
  match node {
    BlockStmtOrExpr::BlockStmt(value) => set_parent_for_block_stmt(value, parent),
    BlockStmtOrExpr::Expr(value) => set_parent_for_expr(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum Callee<'a> {
  Super(&'a Super<'a>),
  Import(&'a Import<'a>),
  Expr(Expr<'a>),
}

impl<'a> Callee<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for Callee<'a> {
  fn start(&self) -> SourcePos {
    match self {
      Callee::Super(node) => node.start(),
      Callee::Import(node) => node.start(),
      Callee::Expr(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      Callee::Super(node) => node.end(),
      Callee::Import(node) => node.end(),
      Callee::Expr(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for Callee<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      Callee::Super(node) => NodeTrait::parent(*node),
      Callee::Import(node) => NodeTrait::parent(*node),
      Callee::Expr(node) => NodeTrait::parent(node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      Callee::Super(node) => node.children(),
      Callee::Import(node) => node.children(),
      Callee::Expr(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      Callee::Super(node) => node.as_node(),
      Callee::Import(node) => node.as_node(),
      Callee::Expr(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Callee::Super(_) => NodeKind::Super,
      Callee::Import(_) => NodeKind::Import,
      Callee::Expr(node) => node.kind(),
    }
  }
}

impl<'a> From<&Callee<'a>> for Node<'a> {
  fn from(node: &Callee<'a>) -> Node<'a> {
    match node {
      Callee::Super(node) => (*node).into(),
      Callee::Import(node) => (*node).into(),
      Callee::Expr(node) => node.into(),
    }
  }
}

impl<'a> From<Callee<'a>> for Node<'a> {
  fn from(node: Callee<'a>) -> Node<'a> {
    match node {
      Callee::Super(node) => node.into(),
      Callee::Import(node) => node.into(),
      Callee::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_callee<'a>(inner: &'a swc_ast::Callee, bump: &'a Bump) -> Callee<'a> {
  match inner {
    swc_ast::Callee::Super(value) => Callee::Super(get_view_for_super(value, bump)),
    swc_ast::Callee::Import(value) => Callee::Import(get_view_for_import(value, bump)),
    swc_ast::Callee::Expr(value) => Callee::Expr(get_view_for_expr(value, bump)),
  }
}

fn set_parent_for_callee<'a>(node: &Callee<'a>, parent: Node<'a>) {
  match node {
    Callee::Super(value) => set_parent_for_super(value, parent),
    Callee::Import(value) => set_parent_for_import(value, parent),
    Callee::Expr(value) => set_parent_for_expr(value, parent),
  }
}

#[derive(Copy, Clone)]
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
  StaticBlock(&'a StaticBlock<'a>),
}

impl<'a> ClassMember<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for ClassMember<'a> {
  fn start(&self) -> SourcePos {
    match self {
      ClassMember::Constructor(node) => node.start(),
      ClassMember::Method(node) => node.start(),
      ClassMember::PrivateMethod(node) => node.start(),
      ClassMember::ClassProp(node) => node.start(),
      ClassMember::PrivateProp(node) => node.start(),
      ClassMember::TsIndexSignature(node) => node.start(),
      ClassMember::Empty(node) => node.start(),
      ClassMember::StaticBlock(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      ClassMember::Constructor(node) => node.end(),
      ClassMember::Method(node) => node.end(),
      ClassMember::PrivateMethod(node) => node.end(),
      ClassMember::ClassProp(node) => node.end(),
      ClassMember::PrivateProp(node) => node.end(),
      ClassMember::TsIndexSignature(node) => node.end(),
      ClassMember::Empty(node) => node.end(),
      ClassMember::StaticBlock(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for ClassMember<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ClassMember::Constructor(node) => NodeTrait::parent(*node),
      ClassMember::Method(node) => NodeTrait::parent(*node),
      ClassMember::PrivateMethod(node) => NodeTrait::parent(*node),
      ClassMember::ClassProp(node) => NodeTrait::parent(*node),
      ClassMember::PrivateProp(node) => NodeTrait::parent(*node),
      ClassMember::TsIndexSignature(node) => NodeTrait::parent(*node),
      ClassMember::Empty(node) => NodeTrait::parent(*node),
      ClassMember::StaticBlock(node) => NodeTrait::parent(*node),
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
      ClassMember::StaticBlock(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      ClassMember::Constructor(node) => node.as_node(),
      ClassMember::Method(node) => node.as_node(),
      ClassMember::PrivateMethod(node) => node.as_node(),
      ClassMember::ClassProp(node) => node.as_node(),
      ClassMember::PrivateProp(node) => node.as_node(),
      ClassMember::TsIndexSignature(node) => node.as_node(),
      ClassMember::Empty(node) => node.as_node(),
      ClassMember::StaticBlock(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ClassMember::Constructor(_) => NodeKind::Constructor,
      ClassMember::Method(_) => NodeKind::ClassMethod,
      ClassMember::PrivateMethod(_) => NodeKind::PrivateMethod,
      ClassMember::ClassProp(_) => NodeKind::ClassProp,
      ClassMember::PrivateProp(_) => NodeKind::PrivateProp,
      ClassMember::TsIndexSignature(_) => NodeKind::TsIndexSignature,
      ClassMember::Empty(_) => NodeKind::EmptyStmt,
      ClassMember::StaticBlock(_) => NodeKind::StaticBlock,
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
      ClassMember::StaticBlock(node) => (*node).into(),
    }
  }
}

impl<'a> From<ClassMember<'a>> for Node<'a> {
  fn from(node: ClassMember<'a>) -> Node<'a> {
    match node {
      ClassMember::Constructor(node) => node.into(),
      ClassMember::Method(node) => node.into(),
      ClassMember::PrivateMethod(node) => node.into(),
      ClassMember::ClassProp(node) => node.into(),
      ClassMember::PrivateProp(node) => node.into(),
      ClassMember::TsIndexSignature(node) => node.into(),
      ClassMember::Empty(node) => node.into(),
      ClassMember::StaticBlock(node) => node.into(),
    }
  }
}

fn get_view_for_class_member<'a>(inner: &'a swc_ast::ClassMember, bump: &'a Bump) -> ClassMember<'a> {
  match inner {
    swc_ast::ClassMember::Constructor(value) => ClassMember::Constructor(get_view_for_constructor(value, bump)),
    swc_ast::ClassMember::Method(value) => ClassMember::Method(get_view_for_class_method(value, bump)),
    swc_ast::ClassMember::PrivateMethod(value) => ClassMember::PrivateMethod(get_view_for_private_method(value, bump)),
    swc_ast::ClassMember::ClassProp(value) => ClassMember::ClassProp(get_view_for_class_prop(value, bump)),
    swc_ast::ClassMember::PrivateProp(value) => ClassMember::PrivateProp(get_view_for_private_prop(value, bump)),
    swc_ast::ClassMember::TsIndexSignature(value) => ClassMember::TsIndexSignature(get_view_for_ts_index_signature(value, bump)),
    swc_ast::ClassMember::Empty(value) => ClassMember::Empty(get_view_for_empty_stmt(value, bump)),
    swc_ast::ClassMember::StaticBlock(value) => ClassMember::StaticBlock(get_view_for_static_block(value, bump)),
  }
}

fn set_parent_for_class_member<'a>(node: &ClassMember<'a>, parent: Node<'a>) {
  match node {
    ClassMember::Constructor(value) => set_parent_for_constructor(value, parent),
    ClassMember::Method(value) => set_parent_for_class_method(value, parent),
    ClassMember::PrivateMethod(value) => set_parent_for_private_method(value, parent),
    ClassMember::ClassProp(value) => set_parent_for_class_prop(value, parent),
    ClassMember::PrivateProp(value) => set_parent_for_private_prop(value, parent),
    ClassMember::TsIndexSignature(value) => set_parent_for_ts_index_signature(value, parent),
    ClassMember::Empty(value) => set_parent_for_empty_stmt(value, parent),
    ClassMember::StaticBlock(value) => set_parent_for_static_block(value, parent),
  }
}

#[derive(Copy, Clone)]
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
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for Decl<'a> {
  fn start(&self) -> SourcePos {
    match self {
      Decl::Class(node) => node.start(),
      Decl::Fn(node) => node.start(),
      Decl::Var(node) => node.start(),
      Decl::TsInterface(node) => node.start(),
      Decl::TsTypeAlias(node) => node.start(),
      Decl::TsEnum(node) => node.start(),
      Decl::TsModule(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      Decl::Class(node) => node.end(),
      Decl::Fn(node) => node.end(),
      Decl::Var(node) => node.end(),
      Decl::TsInterface(node) => node.end(),
      Decl::TsTypeAlias(node) => node.end(),
      Decl::TsEnum(node) => node.end(),
      Decl::TsModule(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for Decl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      Decl::Class(node) => NodeTrait::parent(*node),
      Decl::Fn(node) => NodeTrait::parent(*node),
      Decl::Var(node) => NodeTrait::parent(*node),
      Decl::TsInterface(node) => NodeTrait::parent(*node),
      Decl::TsTypeAlias(node) => NodeTrait::parent(*node),
      Decl::TsEnum(node) => NodeTrait::parent(*node),
      Decl::TsModule(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      Decl::Class(node) => node.as_node(),
      Decl::Fn(node) => node.as_node(),
      Decl::Var(node) => node.as_node(),
      Decl::TsInterface(node) => node.as_node(),
      Decl::TsTypeAlias(node) => node.as_node(),
      Decl::TsEnum(node) => node.as_node(),
      Decl::TsModule(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Decl::Class(_) => NodeKind::ClassDecl,
      Decl::Fn(_) => NodeKind::FnDecl,
      Decl::Var(_) => NodeKind::VarDecl,
      Decl::TsInterface(_) => NodeKind::TsInterfaceDecl,
      Decl::TsTypeAlias(_) => NodeKind::TsTypeAliasDecl,
      Decl::TsEnum(_) => NodeKind::TsEnumDecl,
      Decl::TsModule(_) => NodeKind::TsModuleDecl,
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

impl<'a> From<Decl<'a>> for Node<'a> {
  fn from(node: Decl<'a>) -> Node<'a> {
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

fn get_view_for_decl<'a>(inner: &'a swc_ast::Decl, bump: &'a Bump) -> Decl<'a> {
  match inner {
    swc_ast::Decl::Class(value) => Decl::Class(get_view_for_class_decl(value, bump)),
    swc_ast::Decl::Fn(value) => Decl::Fn(get_view_for_fn_decl(value, bump)),
    swc_ast::Decl::Var(value) => Decl::Var(get_view_for_var_decl(value, bump)),
    swc_ast::Decl::TsInterface(value) => Decl::TsInterface(get_view_for_ts_interface_decl(value, bump)),
    swc_ast::Decl::TsTypeAlias(value) => Decl::TsTypeAlias(get_view_for_ts_type_alias_decl(value, bump)),
    swc_ast::Decl::TsEnum(value) => Decl::TsEnum(get_view_for_ts_enum_decl(value, bump)),
    swc_ast::Decl::TsModule(value) => Decl::TsModule(get_view_for_ts_module_decl(value, bump)),
  }
}

fn set_parent_for_decl<'a>(node: &Decl<'a>, parent: Node<'a>) {
  match node {
    Decl::Class(value) => set_parent_for_class_decl(value, parent),
    Decl::Fn(value) => set_parent_for_fn_decl(value, parent),
    Decl::Var(value) => set_parent_for_var_decl(value, parent),
    Decl::TsInterface(value) => set_parent_for_ts_interface_decl(value, parent),
    Decl::TsTypeAlias(value) => set_parent_for_ts_type_alias_decl(value, parent),
    Decl::TsEnum(value) => set_parent_for_ts_enum_decl(value, parent),
    Decl::TsModule(value) => set_parent_for_ts_module_decl(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum DefaultDecl<'a> {
  Class(&'a ClassExpr<'a>),
  Fn(&'a FnExpr<'a>),
  TsInterfaceDecl(&'a TsInterfaceDecl<'a>),
}

impl<'a> DefaultDecl<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for DefaultDecl<'a> {
  fn start(&self) -> SourcePos {
    match self {
      DefaultDecl::Class(node) => node.start(),
      DefaultDecl::Fn(node) => node.start(),
      DefaultDecl::TsInterfaceDecl(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      DefaultDecl::Class(node) => node.end(),
      DefaultDecl::Fn(node) => node.end(),
      DefaultDecl::TsInterfaceDecl(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for DefaultDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      DefaultDecl::Class(node) => NodeTrait::parent(*node),
      DefaultDecl::Fn(node) => NodeTrait::parent(*node),
      DefaultDecl::TsInterfaceDecl(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      DefaultDecl::Class(node) => node.children(),
      DefaultDecl::Fn(node) => node.children(),
      DefaultDecl::TsInterfaceDecl(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      DefaultDecl::Class(node) => node.as_node(),
      DefaultDecl::Fn(node) => node.as_node(),
      DefaultDecl::TsInterfaceDecl(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      DefaultDecl::Class(_) => NodeKind::ClassExpr,
      DefaultDecl::Fn(_) => NodeKind::FnExpr,
      DefaultDecl::TsInterfaceDecl(_) => NodeKind::TsInterfaceDecl,
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

impl<'a> From<DefaultDecl<'a>> for Node<'a> {
  fn from(node: DefaultDecl<'a>) -> Node<'a> {
    match node {
      DefaultDecl::Class(node) => node.into(),
      DefaultDecl::Fn(node) => node.into(),
      DefaultDecl::TsInterfaceDecl(node) => node.into(),
    }
  }
}

fn get_view_for_default_decl<'a>(inner: &'a swc_ast::DefaultDecl, bump: &'a Bump) -> DefaultDecl<'a> {
  match inner {
    swc_ast::DefaultDecl::Class(value) => DefaultDecl::Class(get_view_for_class_expr(value, bump)),
    swc_ast::DefaultDecl::Fn(value) => DefaultDecl::Fn(get_view_for_fn_expr(value, bump)),
    swc_ast::DefaultDecl::TsInterfaceDecl(value) => DefaultDecl::TsInterfaceDecl(get_view_for_ts_interface_decl(value, bump)),
  }
}

fn set_parent_for_default_decl<'a>(node: &DefaultDecl<'a>, parent: Node<'a>) {
  match node {
    DefaultDecl::Class(value) => set_parent_for_class_expr(value, parent),
    DefaultDecl::Fn(value) => set_parent_for_fn_expr(value, parent),
    DefaultDecl::TsInterfaceDecl(value) => set_parent_for_ts_interface_decl(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum ExportSpecifier<'a> {
  Namespace(&'a ExportNamespaceSpecifier<'a>),
  Default(&'a ExportDefaultSpecifier<'a>),
  Named(&'a ExportNamedSpecifier<'a>),
}

impl<'a> ExportSpecifier<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> &'a NamedExport<'a> {
    NodeTrait::parent(self).unwrap().expect::<NamedExport>()
  }
}

impl<'a> SourceRanged for ExportSpecifier<'a> {
  fn start(&self) -> SourcePos {
    match self {
      ExportSpecifier::Namespace(node) => node.start(),
      ExportSpecifier::Default(node) => node.start(),
      ExportSpecifier::Named(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      ExportSpecifier::Namespace(node) => node.end(),
      ExportSpecifier::Default(node) => node.end(),
      ExportSpecifier::Named(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for ExportSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ExportSpecifier::Namespace(node) => NodeTrait::parent(*node),
      ExportSpecifier::Default(node) => NodeTrait::parent(*node),
      ExportSpecifier::Named(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ExportSpecifier::Namespace(node) => node.children(),
      ExportSpecifier::Default(node) => node.children(),
      ExportSpecifier::Named(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      ExportSpecifier::Namespace(node) => node.as_node(),
      ExportSpecifier::Default(node) => node.as_node(),
      ExportSpecifier::Named(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ExportSpecifier::Namespace(_) => NodeKind::ExportNamespaceSpecifier,
      ExportSpecifier::Default(_) => NodeKind::ExportDefaultSpecifier,
      ExportSpecifier::Named(_) => NodeKind::ExportNamedSpecifier,
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

impl<'a> From<ExportSpecifier<'a>> for Node<'a> {
  fn from(node: ExportSpecifier<'a>) -> Node<'a> {
    match node {
      ExportSpecifier::Namespace(node) => node.into(),
      ExportSpecifier::Default(node) => node.into(),
      ExportSpecifier::Named(node) => node.into(),
    }
  }
}

fn get_view_for_export_specifier<'a>(inner: &'a swc_ast::ExportSpecifier, bump: &'a Bump) -> ExportSpecifier<'a> {
  match inner {
    swc_ast::ExportSpecifier::Namespace(value) => ExportSpecifier::Namespace(get_view_for_export_namespace_specifier(value, bump)),
    swc_ast::ExportSpecifier::Default(value) => ExportSpecifier::Default(get_view_for_export_default_specifier(value, bump)),
    swc_ast::ExportSpecifier::Named(value) => ExportSpecifier::Named(get_view_for_export_named_specifier(value, bump)),
  }
}

fn set_parent_for_export_specifier<'a>(node: &ExportSpecifier<'a>, parent: Node<'a>) {
  match node {
    ExportSpecifier::Namespace(value) => set_parent_for_export_namespace_specifier(value, parent),
    ExportSpecifier::Default(value) => set_parent_for_export_default_specifier(value, parent),
    ExportSpecifier::Named(value) => set_parent_for_export_named_specifier(value, parent),
  }
}

#[derive(Copy, Clone)]
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
  SuperProp(&'a SuperPropExpr<'a>),
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
  TsAs(&'a TsAsExpr<'a>),
  TsInstantiation(&'a TsInstantiation<'a>),
  PrivateName(&'a PrivateName<'a>),
  OptChain(&'a OptChainExpr<'a>),
  Invalid(&'a Invalid<'a>),
}

impl<'a> Expr<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for Expr<'a> {
  fn start(&self) -> SourcePos {
    match self {
      Expr::This(node) => node.start(),
      Expr::Array(node) => node.start(),
      Expr::Object(node) => node.start(),
      Expr::Fn(node) => node.start(),
      Expr::Unary(node) => node.start(),
      Expr::Update(node) => node.start(),
      Expr::Bin(node) => node.start(),
      Expr::Assign(node) => node.start(),
      Expr::Member(node) => node.start(),
      Expr::SuperProp(node) => node.start(),
      Expr::Cond(node) => node.start(),
      Expr::Call(node) => node.start(),
      Expr::New(node) => node.start(),
      Expr::Seq(node) => node.start(),
      Expr::Ident(node) => node.start(),
      Expr::Lit(node) => node.start(),
      Expr::Tpl(node) => node.start(),
      Expr::TaggedTpl(node) => node.start(),
      Expr::Arrow(node) => node.start(),
      Expr::Class(node) => node.start(),
      Expr::Yield(node) => node.start(),
      Expr::MetaProp(node) => node.start(),
      Expr::Await(node) => node.start(),
      Expr::Paren(node) => node.start(),
      Expr::JSXMember(node) => node.start(),
      Expr::JSXNamespacedName(node) => node.start(),
      Expr::JSXEmpty(node) => node.start(),
      Expr::JSXElement(node) => node.start(),
      Expr::JSXFragment(node) => node.start(),
      Expr::TsTypeAssertion(node) => node.start(),
      Expr::TsConstAssertion(node) => node.start(),
      Expr::TsNonNull(node) => node.start(),
      Expr::TsAs(node) => node.start(),
      Expr::TsInstantiation(node) => node.start(),
      Expr::PrivateName(node) => node.start(),
      Expr::OptChain(node) => node.start(),
      Expr::Invalid(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      Expr::This(node) => node.end(),
      Expr::Array(node) => node.end(),
      Expr::Object(node) => node.end(),
      Expr::Fn(node) => node.end(),
      Expr::Unary(node) => node.end(),
      Expr::Update(node) => node.end(),
      Expr::Bin(node) => node.end(),
      Expr::Assign(node) => node.end(),
      Expr::Member(node) => node.end(),
      Expr::SuperProp(node) => node.end(),
      Expr::Cond(node) => node.end(),
      Expr::Call(node) => node.end(),
      Expr::New(node) => node.end(),
      Expr::Seq(node) => node.end(),
      Expr::Ident(node) => node.end(),
      Expr::Lit(node) => node.end(),
      Expr::Tpl(node) => node.end(),
      Expr::TaggedTpl(node) => node.end(),
      Expr::Arrow(node) => node.end(),
      Expr::Class(node) => node.end(),
      Expr::Yield(node) => node.end(),
      Expr::MetaProp(node) => node.end(),
      Expr::Await(node) => node.end(),
      Expr::Paren(node) => node.end(),
      Expr::JSXMember(node) => node.end(),
      Expr::JSXNamespacedName(node) => node.end(),
      Expr::JSXEmpty(node) => node.end(),
      Expr::JSXElement(node) => node.end(),
      Expr::JSXFragment(node) => node.end(),
      Expr::TsTypeAssertion(node) => node.end(),
      Expr::TsConstAssertion(node) => node.end(),
      Expr::TsNonNull(node) => node.end(),
      Expr::TsAs(node) => node.end(),
      Expr::TsInstantiation(node) => node.end(),
      Expr::PrivateName(node) => node.end(),
      Expr::OptChain(node) => node.end(),
      Expr::Invalid(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for Expr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      Expr::This(node) => NodeTrait::parent(*node),
      Expr::Array(node) => NodeTrait::parent(*node),
      Expr::Object(node) => NodeTrait::parent(*node),
      Expr::Fn(node) => NodeTrait::parent(*node),
      Expr::Unary(node) => NodeTrait::parent(*node),
      Expr::Update(node) => NodeTrait::parent(*node),
      Expr::Bin(node) => NodeTrait::parent(*node),
      Expr::Assign(node) => NodeTrait::parent(*node),
      Expr::Member(node) => NodeTrait::parent(*node),
      Expr::SuperProp(node) => NodeTrait::parent(*node),
      Expr::Cond(node) => NodeTrait::parent(*node),
      Expr::Call(node) => NodeTrait::parent(*node),
      Expr::New(node) => NodeTrait::parent(*node),
      Expr::Seq(node) => NodeTrait::parent(*node),
      Expr::Ident(node) => NodeTrait::parent(*node),
      Expr::Lit(node) => NodeTrait::parent(node),
      Expr::Tpl(node) => NodeTrait::parent(*node),
      Expr::TaggedTpl(node) => NodeTrait::parent(*node),
      Expr::Arrow(node) => NodeTrait::parent(*node),
      Expr::Class(node) => NodeTrait::parent(*node),
      Expr::Yield(node) => NodeTrait::parent(*node),
      Expr::MetaProp(node) => NodeTrait::parent(*node),
      Expr::Await(node) => NodeTrait::parent(*node),
      Expr::Paren(node) => NodeTrait::parent(*node),
      Expr::JSXMember(node) => NodeTrait::parent(*node),
      Expr::JSXNamespacedName(node) => NodeTrait::parent(*node),
      Expr::JSXEmpty(node) => NodeTrait::parent(*node),
      Expr::JSXElement(node) => NodeTrait::parent(*node),
      Expr::JSXFragment(node) => NodeTrait::parent(*node),
      Expr::TsTypeAssertion(node) => NodeTrait::parent(*node),
      Expr::TsConstAssertion(node) => NodeTrait::parent(*node),
      Expr::TsNonNull(node) => NodeTrait::parent(*node),
      Expr::TsAs(node) => NodeTrait::parent(*node),
      Expr::TsInstantiation(node) => NodeTrait::parent(*node),
      Expr::PrivateName(node) => NodeTrait::parent(*node),
      Expr::OptChain(node) => NodeTrait::parent(*node),
      Expr::Invalid(node) => NodeTrait::parent(*node),
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
      Expr::SuperProp(node) => node.children(),
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
      Expr::TsAs(node) => node.children(),
      Expr::TsInstantiation(node) => node.children(),
      Expr::PrivateName(node) => node.children(),
      Expr::OptChain(node) => node.children(),
      Expr::Invalid(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      Expr::This(node) => node.as_node(),
      Expr::Array(node) => node.as_node(),
      Expr::Object(node) => node.as_node(),
      Expr::Fn(node) => node.as_node(),
      Expr::Unary(node) => node.as_node(),
      Expr::Update(node) => node.as_node(),
      Expr::Bin(node) => node.as_node(),
      Expr::Assign(node) => node.as_node(),
      Expr::Member(node) => node.as_node(),
      Expr::SuperProp(node) => node.as_node(),
      Expr::Cond(node) => node.as_node(),
      Expr::Call(node) => node.as_node(),
      Expr::New(node) => node.as_node(),
      Expr::Seq(node) => node.as_node(),
      Expr::Ident(node) => node.as_node(),
      Expr::Lit(node) => node.as_node(),
      Expr::Tpl(node) => node.as_node(),
      Expr::TaggedTpl(node) => node.as_node(),
      Expr::Arrow(node) => node.as_node(),
      Expr::Class(node) => node.as_node(),
      Expr::Yield(node) => node.as_node(),
      Expr::MetaProp(node) => node.as_node(),
      Expr::Await(node) => node.as_node(),
      Expr::Paren(node) => node.as_node(),
      Expr::JSXMember(node) => node.as_node(),
      Expr::JSXNamespacedName(node) => node.as_node(),
      Expr::JSXEmpty(node) => node.as_node(),
      Expr::JSXElement(node) => node.as_node(),
      Expr::JSXFragment(node) => node.as_node(),
      Expr::TsTypeAssertion(node) => node.as_node(),
      Expr::TsConstAssertion(node) => node.as_node(),
      Expr::TsNonNull(node) => node.as_node(),
      Expr::TsAs(node) => node.as_node(),
      Expr::TsInstantiation(node) => node.as_node(),
      Expr::PrivateName(node) => node.as_node(),
      Expr::OptChain(node) => node.as_node(),
      Expr::Invalid(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Expr::This(_) => NodeKind::ThisExpr,
      Expr::Array(_) => NodeKind::ArrayLit,
      Expr::Object(_) => NodeKind::ObjectLit,
      Expr::Fn(_) => NodeKind::FnExpr,
      Expr::Unary(_) => NodeKind::UnaryExpr,
      Expr::Update(_) => NodeKind::UpdateExpr,
      Expr::Bin(_) => NodeKind::BinExpr,
      Expr::Assign(_) => NodeKind::AssignExpr,
      Expr::Member(_) => NodeKind::MemberExpr,
      Expr::SuperProp(_) => NodeKind::SuperPropExpr,
      Expr::Cond(_) => NodeKind::CondExpr,
      Expr::Call(_) => NodeKind::CallExpr,
      Expr::New(_) => NodeKind::NewExpr,
      Expr::Seq(_) => NodeKind::SeqExpr,
      Expr::Ident(_) => NodeKind::Ident,
      Expr::Lit(node) => node.kind(),
      Expr::Tpl(_) => NodeKind::Tpl,
      Expr::TaggedTpl(_) => NodeKind::TaggedTpl,
      Expr::Arrow(_) => NodeKind::ArrowExpr,
      Expr::Class(_) => NodeKind::ClassExpr,
      Expr::Yield(_) => NodeKind::YieldExpr,
      Expr::MetaProp(_) => NodeKind::MetaPropExpr,
      Expr::Await(_) => NodeKind::AwaitExpr,
      Expr::Paren(_) => NodeKind::ParenExpr,
      Expr::JSXMember(_) => NodeKind::JSXMemberExpr,
      Expr::JSXNamespacedName(_) => NodeKind::JSXNamespacedName,
      Expr::JSXEmpty(_) => NodeKind::JSXEmptyExpr,
      Expr::JSXElement(_) => NodeKind::JSXElement,
      Expr::JSXFragment(_) => NodeKind::JSXFragment,
      Expr::TsTypeAssertion(_) => NodeKind::TsTypeAssertion,
      Expr::TsConstAssertion(_) => NodeKind::TsConstAssertion,
      Expr::TsNonNull(_) => NodeKind::TsNonNullExpr,
      Expr::TsAs(_) => NodeKind::TsAsExpr,
      Expr::TsInstantiation(_) => NodeKind::TsInstantiation,
      Expr::PrivateName(_) => NodeKind::PrivateName,
      Expr::OptChain(_) => NodeKind::OptChainExpr,
      Expr::Invalid(_) => NodeKind::Invalid,
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
      Expr::SuperProp(node) => (*node).into(),
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
      Expr::TsAs(node) => (*node).into(),
      Expr::TsInstantiation(node) => (*node).into(),
      Expr::PrivateName(node) => (*node).into(),
      Expr::OptChain(node) => (*node).into(),
      Expr::Invalid(node) => (*node).into(),
    }
  }
}

impl<'a> From<Expr<'a>> for Node<'a> {
  fn from(node: Expr<'a>) -> Node<'a> {
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
      Expr::SuperProp(node) => node.into(),
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
      Expr::TsAs(node) => node.into(),
      Expr::TsInstantiation(node) => node.into(),
      Expr::PrivateName(node) => node.into(),
      Expr::OptChain(node) => node.into(),
      Expr::Invalid(node) => node.into(),
    }
  }
}

fn get_view_for_expr<'a>(inner: &'a swc_ast::Expr, bump: &'a Bump) -> Expr<'a> {
  match inner {
    swc_ast::Expr::This(value) => Expr::This(get_view_for_this_expr(value, bump)),
    swc_ast::Expr::Array(value) => Expr::Array(get_view_for_array_lit(value, bump)),
    swc_ast::Expr::Object(value) => Expr::Object(get_view_for_object_lit(value, bump)),
    swc_ast::Expr::Fn(value) => Expr::Fn(get_view_for_fn_expr(value, bump)),
    swc_ast::Expr::Unary(value) => Expr::Unary(get_view_for_unary_expr(value, bump)),
    swc_ast::Expr::Update(value) => Expr::Update(get_view_for_update_expr(value, bump)),
    swc_ast::Expr::Bin(value) => Expr::Bin(get_view_for_bin_expr(value, bump)),
    swc_ast::Expr::Assign(value) => Expr::Assign(get_view_for_assign_expr(value, bump)),
    swc_ast::Expr::Member(value) => Expr::Member(get_view_for_member_expr(value, bump)),
    swc_ast::Expr::SuperProp(value) => Expr::SuperProp(get_view_for_super_prop_expr(value, bump)),
    swc_ast::Expr::Cond(value) => Expr::Cond(get_view_for_cond_expr(value, bump)),
    swc_ast::Expr::Call(value) => Expr::Call(get_view_for_call_expr(value, bump)),
    swc_ast::Expr::New(value) => Expr::New(get_view_for_new_expr(value, bump)),
    swc_ast::Expr::Seq(value) => Expr::Seq(get_view_for_seq_expr(value, bump)),
    swc_ast::Expr::Ident(value) => Expr::Ident(get_view_for_ident(value, bump)),
    swc_ast::Expr::Lit(value) => Expr::Lit(get_view_for_lit(value, bump)),
    swc_ast::Expr::Tpl(value) => Expr::Tpl(get_view_for_tpl(value, bump)),
    swc_ast::Expr::TaggedTpl(value) => Expr::TaggedTpl(get_view_for_tagged_tpl(value, bump)),
    swc_ast::Expr::Arrow(value) => Expr::Arrow(get_view_for_arrow_expr(value, bump)),
    swc_ast::Expr::Class(value) => Expr::Class(get_view_for_class_expr(value, bump)),
    swc_ast::Expr::Yield(value) => Expr::Yield(get_view_for_yield_expr(value, bump)),
    swc_ast::Expr::MetaProp(value) => Expr::MetaProp(get_view_for_meta_prop_expr(value, bump)),
    swc_ast::Expr::Await(value) => Expr::Await(get_view_for_await_expr(value, bump)),
    swc_ast::Expr::Paren(value) => Expr::Paren(get_view_for_paren_expr(value, bump)),
    swc_ast::Expr::JSXMember(value) => Expr::JSXMember(get_view_for_jsxmember_expr(value, bump)),
    swc_ast::Expr::JSXNamespacedName(value) => Expr::JSXNamespacedName(get_view_for_jsxnamespaced_name(value, bump)),
    swc_ast::Expr::JSXEmpty(value) => Expr::JSXEmpty(get_view_for_jsxempty_expr(value, bump)),
    swc_ast::Expr::JSXElement(value) => Expr::JSXElement(get_view_for_jsxelement(value, bump)),
    swc_ast::Expr::JSXFragment(value) => Expr::JSXFragment(get_view_for_jsxfragment(value, bump)),
    swc_ast::Expr::TsTypeAssertion(value) => Expr::TsTypeAssertion(get_view_for_ts_type_assertion(value, bump)),
    swc_ast::Expr::TsConstAssertion(value) => Expr::TsConstAssertion(get_view_for_ts_const_assertion(value, bump)),
    swc_ast::Expr::TsNonNull(value) => Expr::TsNonNull(get_view_for_ts_non_null_expr(value, bump)),
    swc_ast::Expr::TsAs(value) => Expr::TsAs(get_view_for_ts_as_expr(value, bump)),
    swc_ast::Expr::TsInstantiation(value) => Expr::TsInstantiation(get_view_for_ts_instantiation(value, bump)),
    swc_ast::Expr::PrivateName(value) => Expr::PrivateName(get_view_for_private_name(value, bump)),
    swc_ast::Expr::OptChain(value) => Expr::OptChain(get_view_for_opt_chain_expr(value, bump)),
    swc_ast::Expr::Invalid(value) => Expr::Invalid(get_view_for_invalid(value, bump)),
  }
}

fn set_parent_for_expr<'a>(node: &Expr<'a>, parent: Node<'a>) {
  match node {
    Expr::This(value) => set_parent_for_this_expr(value, parent),
    Expr::Array(value) => set_parent_for_array_lit(value, parent),
    Expr::Object(value) => set_parent_for_object_lit(value, parent),
    Expr::Fn(value) => set_parent_for_fn_expr(value, parent),
    Expr::Unary(value) => set_parent_for_unary_expr(value, parent),
    Expr::Update(value) => set_parent_for_update_expr(value, parent),
    Expr::Bin(value) => set_parent_for_bin_expr(value, parent),
    Expr::Assign(value) => set_parent_for_assign_expr(value, parent),
    Expr::Member(value) => set_parent_for_member_expr(value, parent),
    Expr::SuperProp(value) => set_parent_for_super_prop_expr(value, parent),
    Expr::Cond(value) => set_parent_for_cond_expr(value, parent),
    Expr::Call(value) => set_parent_for_call_expr(value, parent),
    Expr::New(value) => set_parent_for_new_expr(value, parent),
    Expr::Seq(value) => set_parent_for_seq_expr(value, parent),
    Expr::Ident(value) => set_parent_for_ident(value, parent),
    Expr::Lit(value) => set_parent_for_lit(value, parent),
    Expr::Tpl(value) => set_parent_for_tpl(value, parent),
    Expr::TaggedTpl(value) => set_parent_for_tagged_tpl(value, parent),
    Expr::Arrow(value) => set_parent_for_arrow_expr(value, parent),
    Expr::Class(value) => set_parent_for_class_expr(value, parent),
    Expr::Yield(value) => set_parent_for_yield_expr(value, parent),
    Expr::MetaProp(value) => set_parent_for_meta_prop_expr(value, parent),
    Expr::Await(value) => set_parent_for_await_expr(value, parent),
    Expr::Paren(value) => set_parent_for_paren_expr(value, parent),
    Expr::JSXMember(value) => set_parent_for_jsxmember_expr(value, parent),
    Expr::JSXNamespacedName(value) => set_parent_for_jsxnamespaced_name(value, parent),
    Expr::JSXEmpty(value) => set_parent_for_jsxempty_expr(value, parent),
    Expr::JSXElement(value) => set_parent_for_jsxelement(value, parent),
    Expr::JSXFragment(value) => set_parent_for_jsxfragment(value, parent),
    Expr::TsTypeAssertion(value) => set_parent_for_ts_type_assertion(value, parent),
    Expr::TsConstAssertion(value) => set_parent_for_ts_const_assertion(value, parent),
    Expr::TsNonNull(value) => set_parent_for_ts_non_null_expr(value, parent),
    Expr::TsAs(value) => set_parent_for_ts_as_expr(value, parent),
    Expr::TsInstantiation(value) => set_parent_for_ts_instantiation(value, parent),
    Expr::PrivateName(value) => set_parent_for_private_name(value, parent),
    Expr::OptChain(value) => set_parent_for_opt_chain_expr(value, parent),
    Expr::Invalid(value) => set_parent_for_invalid(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum ImportSpecifier<'a> {
  Named(&'a ImportNamedSpecifier<'a>),
  Default(&'a ImportDefaultSpecifier<'a>),
  Namespace(&'a ImportStarAsSpecifier<'a>),
}

impl<'a> ImportSpecifier<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> &'a ImportDecl<'a> {
    NodeTrait::parent(self).unwrap().expect::<ImportDecl>()
  }
}

impl<'a> SourceRanged for ImportSpecifier<'a> {
  fn start(&self) -> SourcePos {
    match self {
      ImportSpecifier::Named(node) => node.start(),
      ImportSpecifier::Default(node) => node.start(),
      ImportSpecifier::Namespace(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      ImportSpecifier::Named(node) => node.end(),
      ImportSpecifier::Default(node) => node.end(),
      ImportSpecifier::Namespace(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for ImportSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ImportSpecifier::Named(node) => NodeTrait::parent(*node),
      ImportSpecifier::Default(node) => NodeTrait::parent(*node),
      ImportSpecifier::Namespace(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ImportSpecifier::Named(node) => node.children(),
      ImportSpecifier::Default(node) => node.children(),
      ImportSpecifier::Namespace(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      ImportSpecifier::Named(node) => node.as_node(),
      ImportSpecifier::Default(node) => node.as_node(),
      ImportSpecifier::Namespace(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ImportSpecifier::Named(_) => NodeKind::ImportNamedSpecifier,
      ImportSpecifier::Default(_) => NodeKind::ImportDefaultSpecifier,
      ImportSpecifier::Namespace(_) => NodeKind::ImportStarAsSpecifier,
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

impl<'a> From<ImportSpecifier<'a>> for Node<'a> {
  fn from(node: ImportSpecifier<'a>) -> Node<'a> {
    match node {
      ImportSpecifier::Named(node) => node.into(),
      ImportSpecifier::Default(node) => node.into(),
      ImportSpecifier::Namespace(node) => node.into(),
    }
  }
}

fn get_view_for_import_specifier<'a>(inner: &'a swc_ast::ImportSpecifier, bump: &'a Bump) -> ImportSpecifier<'a> {
  match inner {
    swc_ast::ImportSpecifier::Named(value) => ImportSpecifier::Named(get_view_for_import_named_specifier(value, bump)),
    swc_ast::ImportSpecifier::Default(value) => ImportSpecifier::Default(get_view_for_import_default_specifier(value, bump)),
    swc_ast::ImportSpecifier::Namespace(value) => ImportSpecifier::Namespace(get_view_for_import_star_as_specifier(value, bump)),
  }
}

fn set_parent_for_import_specifier<'a>(node: &ImportSpecifier<'a>, parent: Node<'a>) {
  match node {
    ImportSpecifier::Named(value) => set_parent_for_import_named_specifier(value, parent),
    ImportSpecifier::Default(value) => set_parent_for_import_default_specifier(value, parent),
    ImportSpecifier::Namespace(value) => set_parent_for_import_star_as_specifier(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum JSXAttrName<'a> {
  Ident(&'a Ident<'a>),
  JSXNamespacedName(&'a JSXNamespacedName<'a>),
}

impl<'a> JSXAttrName<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for JSXAttrName<'a> {
  fn start(&self) -> SourcePos {
    match self {
      JSXAttrName::Ident(node) => node.start(),
      JSXAttrName::JSXNamespacedName(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      JSXAttrName::Ident(node) => node.end(),
      JSXAttrName::JSXNamespacedName(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXAttrName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXAttrName::Ident(node) => NodeTrait::parent(*node),
      JSXAttrName::JSXNamespacedName(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXAttrName::Ident(node) => node.children(),
      JSXAttrName::JSXNamespacedName(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      JSXAttrName::Ident(node) => node.as_node(),
      JSXAttrName::JSXNamespacedName(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXAttrName::Ident(_) => NodeKind::Ident,
      JSXAttrName::JSXNamespacedName(_) => NodeKind::JSXNamespacedName,
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

impl<'a> From<JSXAttrName<'a>> for Node<'a> {
  fn from(node: JSXAttrName<'a>) -> Node<'a> {
    match node {
      JSXAttrName::Ident(node) => node.into(),
      JSXAttrName::JSXNamespacedName(node) => node.into(),
    }
  }
}

fn get_view_for_jsxattr_name<'a>(inner: &'a swc_ast::JSXAttrName, bump: &'a Bump) -> JSXAttrName<'a> {
  match inner {
    swc_ast::JSXAttrName::Ident(value) => JSXAttrName::Ident(get_view_for_ident(value, bump)),
    swc_ast::JSXAttrName::JSXNamespacedName(value) => JSXAttrName::JSXNamespacedName(get_view_for_jsxnamespaced_name(value, bump)),
  }
}

fn set_parent_for_jsxattr_name<'a>(node: &JSXAttrName<'a>, parent: Node<'a>) {
  match node {
    JSXAttrName::Ident(value) => set_parent_for_ident(value, parent),
    JSXAttrName::JSXNamespacedName(value) => set_parent_for_jsxnamespaced_name(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum JSXAttrOrSpread<'a> {
  JSXAttr(&'a JSXAttr<'a>),
  SpreadElement(&'a SpreadElement<'a>),
}

impl<'a> JSXAttrOrSpread<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for JSXAttrOrSpread<'a> {
  fn start(&self) -> SourcePos {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.start(),
      JSXAttrOrSpread::SpreadElement(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.end(),
      JSXAttrOrSpread::SpreadElement(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXAttrOrSpread<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => NodeTrait::parent(*node),
      JSXAttrOrSpread::SpreadElement(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.children(),
      JSXAttrOrSpread::SpreadElement(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      JSXAttrOrSpread::JSXAttr(node) => node.as_node(),
      JSXAttrOrSpread::SpreadElement(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXAttrOrSpread::JSXAttr(_) => NodeKind::JSXAttr,
      JSXAttrOrSpread::SpreadElement(_) => NodeKind::SpreadElement,
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

impl<'a> From<JSXAttrOrSpread<'a>> for Node<'a> {
  fn from(node: JSXAttrOrSpread<'a>) -> Node<'a> {
    match node {
      JSXAttrOrSpread::JSXAttr(node) => node.into(),
      JSXAttrOrSpread::SpreadElement(node) => node.into(),
    }
  }
}

fn get_view_for_jsxattr_or_spread<'a>(inner: &'a swc_ast::JSXAttrOrSpread, bump: &'a Bump) -> JSXAttrOrSpread<'a> {
  match inner {
    swc_ast::JSXAttrOrSpread::JSXAttr(value) => JSXAttrOrSpread::JSXAttr(get_view_for_jsxattr(value, bump)),
    swc_ast::JSXAttrOrSpread::SpreadElement(value) => JSXAttrOrSpread::SpreadElement(get_view_for_spread_element(value, bump)),
  }
}

fn set_parent_for_jsxattr_or_spread<'a>(node: &JSXAttrOrSpread<'a>, parent: Node<'a>) {
  match node {
    JSXAttrOrSpread::JSXAttr(value) => set_parent_for_jsxattr(value, parent),
    JSXAttrOrSpread::SpreadElement(value) => set_parent_for_spread_element(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum JSXAttrValue<'a> {
  Lit(Lit<'a>),
  JSXExprContainer(&'a JSXExprContainer<'a>),
  JSXElement(&'a JSXElement<'a>),
  JSXFragment(&'a JSXFragment<'a>),
}

impl<'a> JSXAttrValue<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for JSXAttrValue<'a> {
  fn start(&self) -> SourcePos {
    match self {
      JSXAttrValue::Lit(node) => node.start(),
      JSXAttrValue::JSXExprContainer(node) => node.start(),
      JSXAttrValue::JSXElement(node) => node.start(),
      JSXAttrValue::JSXFragment(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      JSXAttrValue::Lit(node) => node.end(),
      JSXAttrValue::JSXExprContainer(node) => node.end(),
      JSXAttrValue::JSXElement(node) => node.end(),
      JSXAttrValue::JSXFragment(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXAttrValue<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXAttrValue::Lit(node) => NodeTrait::parent(node),
      JSXAttrValue::JSXExprContainer(node) => NodeTrait::parent(*node),
      JSXAttrValue::JSXElement(node) => NodeTrait::parent(*node),
      JSXAttrValue::JSXFragment(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      JSXAttrValue::Lit(node) => node.as_node(),
      JSXAttrValue::JSXExprContainer(node) => node.as_node(),
      JSXAttrValue::JSXElement(node) => node.as_node(),
      JSXAttrValue::JSXFragment(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXAttrValue::Lit(node) => node.kind(),
      JSXAttrValue::JSXExprContainer(_) => NodeKind::JSXExprContainer,
      JSXAttrValue::JSXElement(_) => NodeKind::JSXElement,
      JSXAttrValue::JSXFragment(_) => NodeKind::JSXFragment,
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

impl<'a> From<JSXAttrValue<'a>> for Node<'a> {
  fn from(node: JSXAttrValue<'a>) -> Node<'a> {
    match node {
      JSXAttrValue::Lit(node) => node.into(),
      JSXAttrValue::JSXExprContainer(node) => node.into(),
      JSXAttrValue::JSXElement(node) => node.into(),
      JSXAttrValue::JSXFragment(node) => node.into(),
    }
  }
}

fn get_view_for_jsxattr_value<'a>(inner: &'a swc_ast::JSXAttrValue, bump: &'a Bump) -> JSXAttrValue<'a> {
  match inner {
    swc_ast::JSXAttrValue::Lit(value) => JSXAttrValue::Lit(get_view_for_lit(value, bump)),
    swc_ast::JSXAttrValue::JSXExprContainer(value) => JSXAttrValue::JSXExprContainer(get_view_for_jsxexpr_container(value, bump)),
    swc_ast::JSXAttrValue::JSXElement(value) => JSXAttrValue::JSXElement(get_view_for_jsxelement(value, bump)),
    swc_ast::JSXAttrValue::JSXFragment(value) => JSXAttrValue::JSXFragment(get_view_for_jsxfragment(value, bump)),
  }
}

fn set_parent_for_jsxattr_value<'a>(node: &JSXAttrValue<'a>, parent: Node<'a>) {
  match node {
    JSXAttrValue::Lit(value) => set_parent_for_lit(value, parent),
    JSXAttrValue::JSXExprContainer(value) => set_parent_for_jsxexpr_container(value, parent),
    JSXAttrValue::JSXElement(value) => set_parent_for_jsxelement(value, parent),
    JSXAttrValue::JSXFragment(value) => set_parent_for_jsxfragment(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum JSXElementChild<'a> {
  JSXText(&'a JSXText<'a>),
  JSXExprContainer(&'a JSXExprContainer<'a>),
  JSXSpreadChild(&'a JSXSpreadChild<'a>),
  JSXElement(&'a JSXElement<'a>),
  JSXFragment(&'a JSXFragment<'a>),
}

impl<'a> JSXElementChild<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for JSXElementChild<'a> {
  fn start(&self) -> SourcePos {
    match self {
      JSXElementChild::JSXText(node) => node.start(),
      JSXElementChild::JSXExprContainer(node) => node.start(),
      JSXElementChild::JSXSpreadChild(node) => node.start(),
      JSXElementChild::JSXElement(node) => node.start(),
      JSXElementChild::JSXFragment(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      JSXElementChild::JSXText(node) => node.end(),
      JSXElementChild::JSXExprContainer(node) => node.end(),
      JSXElementChild::JSXSpreadChild(node) => node.end(),
      JSXElementChild::JSXElement(node) => node.end(),
      JSXElementChild::JSXFragment(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXElementChild<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXElementChild::JSXText(node) => NodeTrait::parent(*node),
      JSXElementChild::JSXExprContainer(node) => NodeTrait::parent(*node),
      JSXElementChild::JSXSpreadChild(node) => NodeTrait::parent(*node),
      JSXElementChild::JSXElement(node) => NodeTrait::parent(*node),
      JSXElementChild::JSXFragment(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      JSXElementChild::JSXText(node) => node.as_node(),
      JSXElementChild::JSXExprContainer(node) => node.as_node(),
      JSXElementChild::JSXSpreadChild(node) => node.as_node(),
      JSXElementChild::JSXElement(node) => node.as_node(),
      JSXElementChild::JSXFragment(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXElementChild::JSXText(_) => NodeKind::JSXText,
      JSXElementChild::JSXExprContainer(_) => NodeKind::JSXExprContainer,
      JSXElementChild::JSXSpreadChild(_) => NodeKind::JSXSpreadChild,
      JSXElementChild::JSXElement(_) => NodeKind::JSXElement,
      JSXElementChild::JSXFragment(_) => NodeKind::JSXFragment,
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

impl<'a> From<JSXElementChild<'a>> for Node<'a> {
  fn from(node: JSXElementChild<'a>) -> Node<'a> {
    match node {
      JSXElementChild::JSXText(node) => node.into(),
      JSXElementChild::JSXExprContainer(node) => node.into(),
      JSXElementChild::JSXSpreadChild(node) => node.into(),
      JSXElementChild::JSXElement(node) => node.into(),
      JSXElementChild::JSXFragment(node) => node.into(),
    }
  }
}

fn get_view_for_jsxelement_child<'a>(inner: &'a swc_ast::JSXElementChild, bump: &'a Bump) -> JSXElementChild<'a> {
  match inner {
    swc_ast::JSXElementChild::JSXText(value) => JSXElementChild::JSXText(get_view_for_jsxtext(value, bump)),
    swc_ast::JSXElementChild::JSXExprContainer(value) => JSXElementChild::JSXExprContainer(get_view_for_jsxexpr_container(value, bump)),
    swc_ast::JSXElementChild::JSXSpreadChild(value) => JSXElementChild::JSXSpreadChild(get_view_for_jsxspread_child(value, bump)),
    swc_ast::JSXElementChild::JSXElement(value) => JSXElementChild::JSXElement(get_view_for_jsxelement(value, bump)),
    swc_ast::JSXElementChild::JSXFragment(value) => JSXElementChild::JSXFragment(get_view_for_jsxfragment(value, bump)),
  }
}

fn set_parent_for_jsxelement_child<'a>(node: &JSXElementChild<'a>, parent: Node<'a>) {
  match node {
    JSXElementChild::JSXText(value) => set_parent_for_jsxtext(value, parent),
    JSXElementChild::JSXExprContainer(value) => set_parent_for_jsxexpr_container(value, parent),
    JSXElementChild::JSXSpreadChild(value) => set_parent_for_jsxspread_child(value, parent),
    JSXElementChild::JSXElement(value) => set_parent_for_jsxelement(value, parent),
    JSXElementChild::JSXFragment(value) => set_parent_for_jsxfragment(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum JSXElementName<'a> {
  Ident(&'a Ident<'a>),
  JSXMemberExpr(&'a JSXMemberExpr<'a>),
  JSXNamespacedName(&'a JSXNamespacedName<'a>),
}

impl<'a> JSXElementName<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for JSXElementName<'a> {
  fn start(&self) -> SourcePos {
    match self {
      JSXElementName::Ident(node) => node.start(),
      JSXElementName::JSXMemberExpr(node) => node.start(),
      JSXElementName::JSXNamespacedName(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      JSXElementName::Ident(node) => node.end(),
      JSXElementName::JSXMemberExpr(node) => node.end(),
      JSXElementName::JSXNamespacedName(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXElementName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXElementName::Ident(node) => NodeTrait::parent(*node),
      JSXElementName::JSXMemberExpr(node) => NodeTrait::parent(*node),
      JSXElementName::JSXNamespacedName(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXElementName::Ident(node) => node.children(),
      JSXElementName::JSXMemberExpr(node) => node.children(),
      JSXElementName::JSXNamespacedName(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      JSXElementName::Ident(node) => node.as_node(),
      JSXElementName::JSXMemberExpr(node) => node.as_node(),
      JSXElementName::JSXNamespacedName(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXElementName::Ident(_) => NodeKind::Ident,
      JSXElementName::JSXMemberExpr(_) => NodeKind::JSXMemberExpr,
      JSXElementName::JSXNamespacedName(_) => NodeKind::JSXNamespacedName,
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

impl<'a> From<JSXElementName<'a>> for Node<'a> {
  fn from(node: JSXElementName<'a>) -> Node<'a> {
    match node {
      JSXElementName::Ident(node) => node.into(),
      JSXElementName::JSXMemberExpr(node) => node.into(),
      JSXElementName::JSXNamespacedName(node) => node.into(),
    }
  }
}

fn get_view_for_jsxelement_name<'a>(inner: &'a swc_ast::JSXElementName, bump: &'a Bump) -> JSXElementName<'a> {
  match inner {
    swc_ast::JSXElementName::Ident(value) => JSXElementName::Ident(get_view_for_ident(value, bump)),
    swc_ast::JSXElementName::JSXMemberExpr(value) => JSXElementName::JSXMemberExpr(get_view_for_jsxmember_expr(value, bump)),
    swc_ast::JSXElementName::JSXNamespacedName(value) => JSXElementName::JSXNamespacedName(get_view_for_jsxnamespaced_name(value, bump)),
  }
}

fn set_parent_for_jsxelement_name<'a>(node: &JSXElementName<'a>, parent: Node<'a>) {
  match node {
    JSXElementName::Ident(value) => set_parent_for_ident(value, parent),
    JSXElementName::JSXMemberExpr(value) => set_parent_for_jsxmember_expr(value, parent),
    JSXElementName::JSXNamespacedName(value) => set_parent_for_jsxnamespaced_name(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum JSXExpr<'a> {
  JSXEmptyExpr(&'a JSXEmptyExpr<'a>),
  Expr(Expr<'a>),
}

impl<'a> JSXExpr<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for JSXExpr<'a> {
  fn start(&self) -> SourcePos {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.start(),
      JSXExpr::Expr(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.end(),
      JSXExpr::Expr(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXExpr::JSXEmptyExpr(node) => NodeTrait::parent(*node),
      JSXExpr::Expr(node) => NodeTrait::parent(node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.children(),
      JSXExpr::Expr(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      JSXExpr::JSXEmptyExpr(node) => node.as_node(),
      JSXExpr::Expr(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXExpr::JSXEmptyExpr(_) => NodeKind::JSXEmptyExpr,
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

impl<'a> From<JSXExpr<'a>> for Node<'a> {
  fn from(node: JSXExpr<'a>) -> Node<'a> {
    match node {
      JSXExpr::JSXEmptyExpr(node) => node.into(),
      JSXExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_jsxexpr<'a>(inner: &'a swc_ast::JSXExpr, bump: &'a Bump) -> JSXExpr<'a> {
  match inner {
    swc_ast::JSXExpr::JSXEmptyExpr(value) => JSXExpr::JSXEmptyExpr(get_view_for_jsxempty_expr(value, bump)),
    swc_ast::JSXExpr::Expr(value) => JSXExpr::Expr(get_view_for_expr(value, bump)),
  }
}

fn set_parent_for_jsxexpr<'a>(node: &JSXExpr<'a>, parent: Node<'a>) {
  match node {
    JSXExpr::JSXEmptyExpr(value) => set_parent_for_jsxempty_expr(value, parent),
    JSXExpr::Expr(value) => set_parent_for_expr(value, parent),
  }
}

/// Used for `obj` property of `JSXMemberExpr`.
#[derive(Copy, Clone)]
pub enum JSXObject<'a> {
  JSXMemberExpr(&'a JSXMemberExpr<'a>),
  Ident(&'a Ident<'a>),
}

impl<'a> JSXObject<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for JSXObject<'a> {
  fn start(&self) -> SourcePos {
    match self {
      JSXObject::JSXMemberExpr(node) => node.start(),
      JSXObject::Ident(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      JSXObject::JSXMemberExpr(node) => node.end(),
      JSXObject::Ident(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for JSXObject<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      JSXObject::JSXMemberExpr(node) => NodeTrait::parent(*node),
      JSXObject::Ident(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      JSXObject::JSXMemberExpr(node) => node.children(),
      JSXObject::Ident(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      JSXObject::JSXMemberExpr(node) => node.as_node(),
      JSXObject::Ident(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      JSXObject::JSXMemberExpr(_) => NodeKind::JSXMemberExpr,
      JSXObject::Ident(_) => NodeKind::Ident,
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

impl<'a> From<JSXObject<'a>> for Node<'a> {
  fn from(node: JSXObject<'a>) -> Node<'a> {
    match node {
      JSXObject::JSXMemberExpr(node) => node.into(),
      JSXObject::Ident(node) => node.into(),
    }
  }
}

fn get_view_for_jsxobject<'a>(inner: &'a swc_ast::JSXObject, bump: &'a Bump) -> JSXObject<'a> {
  match inner {
    swc_ast::JSXObject::JSXMemberExpr(value) => JSXObject::JSXMemberExpr(get_view_for_jsxmember_expr(value, bump)),
    swc_ast::JSXObject::Ident(value) => JSXObject::Ident(get_view_for_ident(value, bump)),
  }
}

fn set_parent_for_jsxobject<'a>(node: &JSXObject<'a>, parent: Node<'a>) {
  match node {
    JSXObject::JSXMemberExpr(value) => set_parent_for_jsxmember_expr(value, parent),
    JSXObject::Ident(value) => set_parent_for_ident(value, parent),
  }
}

#[derive(Copy, Clone)]
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
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for Lit<'a> {
  fn start(&self) -> SourcePos {
    match self {
      Lit::Str(node) => node.start(),
      Lit::Bool(node) => node.start(),
      Lit::Null(node) => node.start(),
      Lit::Num(node) => node.start(),
      Lit::BigInt(node) => node.start(),
      Lit::Regex(node) => node.start(),
      Lit::JSXText(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      Lit::Str(node) => node.end(),
      Lit::Bool(node) => node.end(),
      Lit::Null(node) => node.end(),
      Lit::Num(node) => node.end(),
      Lit::BigInt(node) => node.end(),
      Lit::Regex(node) => node.end(),
      Lit::JSXText(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for Lit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      Lit::Str(node) => NodeTrait::parent(*node),
      Lit::Bool(node) => NodeTrait::parent(*node),
      Lit::Null(node) => NodeTrait::parent(*node),
      Lit::Num(node) => NodeTrait::parent(*node),
      Lit::BigInt(node) => NodeTrait::parent(*node),
      Lit::Regex(node) => NodeTrait::parent(*node),
      Lit::JSXText(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      Lit::Str(node) => node.as_node(),
      Lit::Bool(node) => node.as_node(),
      Lit::Null(node) => node.as_node(),
      Lit::Num(node) => node.as_node(),
      Lit::BigInt(node) => node.as_node(),
      Lit::Regex(node) => node.as_node(),
      Lit::JSXText(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Lit::Str(_) => NodeKind::Str,
      Lit::Bool(_) => NodeKind::Bool,
      Lit::Null(_) => NodeKind::Null,
      Lit::Num(_) => NodeKind::Number,
      Lit::BigInt(_) => NodeKind::BigInt,
      Lit::Regex(_) => NodeKind::Regex,
      Lit::JSXText(_) => NodeKind::JSXText,
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

impl<'a> From<Lit<'a>> for Node<'a> {
  fn from(node: Lit<'a>) -> Node<'a> {
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

fn get_view_for_lit<'a>(inner: &'a swc_ast::Lit, bump: &'a Bump) -> Lit<'a> {
  match inner {
    swc_ast::Lit::Str(value) => Lit::Str(get_view_for_str(value, bump)),
    swc_ast::Lit::Bool(value) => Lit::Bool(get_view_for_bool(value, bump)),
    swc_ast::Lit::Null(value) => Lit::Null(get_view_for_null(value, bump)),
    swc_ast::Lit::Num(value) => Lit::Num(get_view_for_number(value, bump)),
    swc_ast::Lit::BigInt(value) => Lit::BigInt(get_view_for_big_int(value, bump)),
    swc_ast::Lit::Regex(value) => Lit::Regex(get_view_for_regex(value, bump)),
    swc_ast::Lit::JSXText(value) => Lit::JSXText(get_view_for_jsxtext(value, bump)),
  }
}

fn set_parent_for_lit<'a>(node: &Lit<'a>, parent: Node<'a>) {
  match node {
    Lit::Str(value) => set_parent_for_str(value, parent),
    Lit::Bool(value) => set_parent_for_bool(value, parent),
    Lit::Null(value) => set_parent_for_null(value, parent),
    Lit::Num(value) => set_parent_for_number(value, parent),
    Lit::BigInt(value) => set_parent_for_big_int(value, parent),
    Lit::Regex(value) => set_parent_for_regex(value, parent),
    Lit::JSXText(value) => set_parent_for_jsxtext(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum MemberProp<'a> {
  Ident(&'a Ident<'a>),
  PrivateName(&'a PrivateName<'a>),
  Computed(&'a ComputedPropName<'a>),
}

impl<'a> MemberProp<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for MemberProp<'a> {
  fn start(&self) -> SourcePos {
    match self {
      MemberProp::Ident(node) => node.start(),
      MemberProp::PrivateName(node) => node.start(),
      MemberProp::Computed(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      MemberProp::Ident(node) => node.end(),
      MemberProp::PrivateName(node) => node.end(),
      MemberProp::Computed(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for MemberProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      MemberProp::Ident(node) => NodeTrait::parent(*node),
      MemberProp::PrivateName(node) => NodeTrait::parent(*node),
      MemberProp::Computed(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      MemberProp::Ident(node) => node.children(),
      MemberProp::PrivateName(node) => node.children(),
      MemberProp::Computed(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      MemberProp::Ident(node) => node.as_node(),
      MemberProp::PrivateName(node) => node.as_node(),
      MemberProp::Computed(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      MemberProp::Ident(_) => NodeKind::Ident,
      MemberProp::PrivateName(_) => NodeKind::PrivateName,
      MemberProp::Computed(_) => NodeKind::ComputedPropName,
    }
  }
}

impl<'a> From<&MemberProp<'a>> for Node<'a> {
  fn from(node: &MemberProp<'a>) -> Node<'a> {
    match node {
      MemberProp::Ident(node) => (*node).into(),
      MemberProp::PrivateName(node) => (*node).into(),
      MemberProp::Computed(node) => (*node).into(),
    }
  }
}

impl<'a> From<MemberProp<'a>> for Node<'a> {
  fn from(node: MemberProp<'a>) -> Node<'a> {
    match node {
      MemberProp::Ident(node) => node.into(),
      MemberProp::PrivateName(node) => node.into(),
      MemberProp::Computed(node) => node.into(),
    }
  }
}

fn get_view_for_member_prop<'a>(inner: &'a swc_ast::MemberProp, bump: &'a Bump) -> MemberProp<'a> {
  match inner {
    swc_ast::MemberProp::Ident(value) => MemberProp::Ident(get_view_for_ident(value, bump)),
    swc_ast::MemberProp::PrivateName(value) => MemberProp::PrivateName(get_view_for_private_name(value, bump)),
    swc_ast::MemberProp::Computed(value) => MemberProp::Computed(get_view_for_computed_prop_name(value, bump)),
  }
}

fn set_parent_for_member_prop<'a>(node: &MemberProp<'a>, parent: Node<'a>) {
  match node {
    MemberProp::Ident(value) => set_parent_for_ident(value, parent),
    MemberProp::PrivateName(value) => set_parent_for_private_name(value, parent),
    MemberProp::Computed(value) => set_parent_for_computed_prop_name(value, parent),
  }
}

#[derive(Copy, Clone)]
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
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for ModuleDecl<'a> {
  fn start(&self) -> SourcePos {
    match self {
      ModuleDecl::Import(node) => node.start(),
      ModuleDecl::ExportDecl(node) => node.start(),
      ModuleDecl::ExportNamed(node) => node.start(),
      ModuleDecl::ExportDefaultDecl(node) => node.start(),
      ModuleDecl::ExportDefaultExpr(node) => node.start(),
      ModuleDecl::ExportAll(node) => node.start(),
      ModuleDecl::TsImportEquals(node) => node.start(),
      ModuleDecl::TsExportAssignment(node) => node.start(),
      ModuleDecl::TsNamespaceExport(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      ModuleDecl::Import(node) => node.end(),
      ModuleDecl::ExportDecl(node) => node.end(),
      ModuleDecl::ExportNamed(node) => node.end(),
      ModuleDecl::ExportDefaultDecl(node) => node.end(),
      ModuleDecl::ExportDefaultExpr(node) => node.end(),
      ModuleDecl::ExportAll(node) => node.end(),
      ModuleDecl::TsImportEquals(node) => node.end(),
      ModuleDecl::TsExportAssignment(node) => node.end(),
      ModuleDecl::TsNamespaceExport(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for ModuleDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ModuleDecl::Import(node) => NodeTrait::parent(*node),
      ModuleDecl::ExportDecl(node) => NodeTrait::parent(*node),
      ModuleDecl::ExportNamed(node) => NodeTrait::parent(*node),
      ModuleDecl::ExportDefaultDecl(node) => NodeTrait::parent(*node),
      ModuleDecl::ExportDefaultExpr(node) => NodeTrait::parent(*node),
      ModuleDecl::ExportAll(node) => NodeTrait::parent(*node),
      ModuleDecl::TsImportEquals(node) => NodeTrait::parent(*node),
      ModuleDecl::TsExportAssignment(node) => NodeTrait::parent(*node),
      ModuleDecl::TsNamespaceExport(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      ModuleDecl::Import(node) => node.as_node(),
      ModuleDecl::ExportDecl(node) => node.as_node(),
      ModuleDecl::ExportNamed(node) => node.as_node(),
      ModuleDecl::ExportDefaultDecl(node) => node.as_node(),
      ModuleDecl::ExportDefaultExpr(node) => node.as_node(),
      ModuleDecl::ExportAll(node) => node.as_node(),
      ModuleDecl::TsImportEquals(node) => node.as_node(),
      ModuleDecl::TsExportAssignment(node) => node.as_node(),
      ModuleDecl::TsNamespaceExport(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ModuleDecl::Import(_) => NodeKind::ImportDecl,
      ModuleDecl::ExportDecl(_) => NodeKind::ExportDecl,
      ModuleDecl::ExportNamed(_) => NodeKind::NamedExport,
      ModuleDecl::ExportDefaultDecl(_) => NodeKind::ExportDefaultDecl,
      ModuleDecl::ExportDefaultExpr(_) => NodeKind::ExportDefaultExpr,
      ModuleDecl::ExportAll(_) => NodeKind::ExportAll,
      ModuleDecl::TsImportEquals(_) => NodeKind::TsImportEqualsDecl,
      ModuleDecl::TsExportAssignment(_) => NodeKind::TsExportAssignment,
      ModuleDecl::TsNamespaceExport(_) => NodeKind::TsNamespaceExportDecl,
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

impl<'a> From<ModuleDecl<'a>> for Node<'a> {
  fn from(node: ModuleDecl<'a>) -> Node<'a> {
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

fn get_view_for_module_decl<'a>(inner: &'a swc_ast::ModuleDecl, bump: &'a Bump) -> ModuleDecl<'a> {
  match inner {
    swc_ast::ModuleDecl::Import(value) => ModuleDecl::Import(get_view_for_import_decl(value, bump)),
    swc_ast::ModuleDecl::ExportDecl(value) => ModuleDecl::ExportDecl(get_view_for_export_decl(value, bump)),
    swc_ast::ModuleDecl::ExportNamed(value) => ModuleDecl::ExportNamed(get_view_for_named_export(value, bump)),
    swc_ast::ModuleDecl::ExportDefaultDecl(value) => ModuleDecl::ExportDefaultDecl(get_view_for_export_default_decl(value, bump)),
    swc_ast::ModuleDecl::ExportDefaultExpr(value) => ModuleDecl::ExportDefaultExpr(get_view_for_export_default_expr(value, bump)),
    swc_ast::ModuleDecl::ExportAll(value) => ModuleDecl::ExportAll(get_view_for_export_all(value, bump)),
    swc_ast::ModuleDecl::TsImportEquals(value) => ModuleDecl::TsImportEquals(get_view_for_ts_import_equals_decl(value, bump)),
    swc_ast::ModuleDecl::TsExportAssignment(value) => ModuleDecl::TsExportAssignment(get_view_for_ts_export_assignment(value, bump)),
    swc_ast::ModuleDecl::TsNamespaceExport(value) => ModuleDecl::TsNamespaceExport(get_view_for_ts_namespace_export_decl(value, bump)),
  }
}

fn set_parent_for_module_decl<'a>(node: &ModuleDecl<'a>, parent: Node<'a>) {
  match node {
    ModuleDecl::Import(value) => set_parent_for_import_decl(value, parent),
    ModuleDecl::ExportDecl(value) => set_parent_for_export_decl(value, parent),
    ModuleDecl::ExportNamed(value) => set_parent_for_named_export(value, parent),
    ModuleDecl::ExportDefaultDecl(value) => set_parent_for_export_default_decl(value, parent),
    ModuleDecl::ExportDefaultExpr(value) => set_parent_for_export_default_expr(value, parent),
    ModuleDecl::ExportAll(value) => set_parent_for_export_all(value, parent),
    ModuleDecl::TsImportEquals(value) => set_parent_for_ts_import_equals_decl(value, parent),
    ModuleDecl::TsExportAssignment(value) => set_parent_for_ts_export_assignment(value, parent),
    ModuleDecl::TsNamespaceExport(value) => set_parent_for_ts_namespace_export_decl(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum ModuleExportName<'a> {
  Ident(&'a Ident<'a>),
  Str(&'a Str<'a>),
}

impl<'a> ModuleExportName<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for ModuleExportName<'a> {
  fn start(&self) -> SourcePos {
    match self {
      ModuleExportName::Ident(node) => node.start(),
      ModuleExportName::Str(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      ModuleExportName::Ident(node) => node.end(),
      ModuleExportName::Str(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for ModuleExportName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ModuleExportName::Ident(node) => NodeTrait::parent(*node),
      ModuleExportName::Str(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ModuleExportName::Ident(node) => node.children(),
      ModuleExportName::Str(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      ModuleExportName::Ident(node) => node.as_node(),
      ModuleExportName::Str(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ModuleExportName::Ident(_) => NodeKind::Ident,
      ModuleExportName::Str(_) => NodeKind::Str,
    }
  }
}

impl<'a> From<&ModuleExportName<'a>> for Node<'a> {
  fn from(node: &ModuleExportName<'a>) -> Node<'a> {
    match node {
      ModuleExportName::Ident(node) => (*node).into(),
      ModuleExportName::Str(node) => (*node).into(),
    }
  }
}

impl<'a> From<ModuleExportName<'a>> for Node<'a> {
  fn from(node: ModuleExportName<'a>) -> Node<'a> {
    match node {
      ModuleExportName::Ident(node) => node.into(),
      ModuleExportName::Str(node) => node.into(),
    }
  }
}

fn get_view_for_module_export_name<'a>(inner: &'a swc_ast::ModuleExportName, bump: &'a Bump) -> ModuleExportName<'a> {
  match inner {
    swc_ast::ModuleExportName::Ident(value) => ModuleExportName::Ident(get_view_for_ident(value, bump)),
    swc_ast::ModuleExportName::Str(value) => ModuleExportName::Str(get_view_for_str(value, bump)),
  }
}

fn set_parent_for_module_export_name<'a>(node: &ModuleExportName<'a>, parent: Node<'a>) {
  match node {
    ModuleExportName::Ident(value) => set_parent_for_ident(value, parent),
    ModuleExportName::Str(value) => set_parent_for_str(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum ModuleItem<'a> {
  ModuleDecl(ModuleDecl<'a>),
  Stmt(Stmt<'a>),
}

impl<'a> ModuleItem<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for ModuleItem<'a> {
  fn start(&self) -> SourcePos {
    match self {
      ModuleItem::ModuleDecl(node) => node.start(),
      ModuleItem::Stmt(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      ModuleItem::ModuleDecl(node) => node.end(),
      ModuleItem::Stmt(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for ModuleItem<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ModuleItem::ModuleDecl(node) => NodeTrait::parent(node),
      ModuleItem::Stmt(node) => NodeTrait::parent(node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ModuleItem::ModuleDecl(node) => node.children(),
      ModuleItem::Stmt(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      ModuleItem::ModuleDecl(node) => node.as_node(),
      ModuleItem::Stmt(node) => node.as_node(),
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

impl<'a> From<ModuleItem<'a>> for Node<'a> {
  fn from(node: ModuleItem<'a>) -> Node<'a> {
    match node {
      ModuleItem::ModuleDecl(node) => node.into(),
      ModuleItem::Stmt(node) => node.into(),
    }
  }
}

fn get_view_for_module_item<'a>(inner: &'a swc_ast::ModuleItem, bump: &'a Bump) -> ModuleItem<'a> {
  match inner {
    swc_ast::ModuleItem::ModuleDecl(value) => ModuleItem::ModuleDecl(get_view_for_module_decl(value, bump)),
    swc_ast::ModuleItem::Stmt(value) => ModuleItem::Stmt(get_view_for_stmt(value, bump)),
  }
}

fn set_parent_for_module_item<'a>(node: &ModuleItem<'a>, parent: Node<'a>) {
  match node {
    ModuleItem::ModuleDecl(value) => set_parent_for_module_decl(value, parent),
    ModuleItem::Stmt(value) => set_parent_for_stmt(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum ObjectPatProp<'a> {
  KeyValue(&'a KeyValuePatProp<'a>),
  Assign(&'a AssignPatProp<'a>),
  Rest(&'a RestPat<'a>),
}

impl<'a> ObjectPatProp<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for ObjectPatProp<'a> {
  fn start(&self) -> SourcePos {
    match self {
      ObjectPatProp::KeyValue(node) => node.start(),
      ObjectPatProp::Assign(node) => node.start(),
      ObjectPatProp::Rest(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      ObjectPatProp::KeyValue(node) => node.end(),
      ObjectPatProp::Assign(node) => node.end(),
      ObjectPatProp::Rest(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for ObjectPatProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ObjectPatProp::KeyValue(node) => NodeTrait::parent(*node),
      ObjectPatProp::Assign(node) => NodeTrait::parent(*node),
      ObjectPatProp::Rest(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ObjectPatProp::KeyValue(node) => node.children(),
      ObjectPatProp::Assign(node) => node.children(),
      ObjectPatProp::Rest(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      ObjectPatProp::KeyValue(node) => node.as_node(),
      ObjectPatProp::Assign(node) => node.as_node(),
      ObjectPatProp::Rest(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ObjectPatProp::KeyValue(_) => NodeKind::KeyValuePatProp,
      ObjectPatProp::Assign(_) => NodeKind::AssignPatProp,
      ObjectPatProp::Rest(_) => NodeKind::RestPat,
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

impl<'a> From<ObjectPatProp<'a>> for Node<'a> {
  fn from(node: ObjectPatProp<'a>) -> Node<'a> {
    match node {
      ObjectPatProp::KeyValue(node) => node.into(),
      ObjectPatProp::Assign(node) => node.into(),
      ObjectPatProp::Rest(node) => node.into(),
    }
  }
}

fn get_view_for_object_pat_prop<'a>(inner: &'a swc_ast::ObjectPatProp, bump: &'a Bump) -> ObjectPatProp<'a> {
  match inner {
    swc_ast::ObjectPatProp::KeyValue(value) => ObjectPatProp::KeyValue(get_view_for_key_value_pat_prop(value, bump)),
    swc_ast::ObjectPatProp::Assign(value) => ObjectPatProp::Assign(get_view_for_assign_pat_prop(value, bump)),
    swc_ast::ObjectPatProp::Rest(value) => ObjectPatProp::Rest(get_view_for_rest_pat(value, bump)),
  }
}

fn set_parent_for_object_pat_prop<'a>(node: &ObjectPatProp<'a>, parent: Node<'a>) {
  match node {
    ObjectPatProp::KeyValue(value) => set_parent_for_key_value_pat_prop(value, parent),
    ObjectPatProp::Assign(value) => set_parent_for_assign_pat_prop(value, parent),
    ObjectPatProp::Rest(value) => set_parent_for_rest_pat(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum OptChainBase<'a> {
  Member(&'a MemberExpr<'a>),
  Call(&'a OptCall<'a>),
}

impl<'a> OptChainBase<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for OptChainBase<'a> {
  fn start(&self) -> SourcePos {
    match self {
      OptChainBase::Member(node) => node.start(),
      OptChainBase::Call(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      OptChainBase::Member(node) => node.end(),
      OptChainBase::Call(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for OptChainBase<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      OptChainBase::Member(node) => NodeTrait::parent(*node),
      OptChainBase::Call(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      OptChainBase::Member(node) => node.children(),
      OptChainBase::Call(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      OptChainBase::Member(node) => node.as_node(),
      OptChainBase::Call(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      OptChainBase::Member(_) => NodeKind::MemberExpr,
      OptChainBase::Call(_) => NodeKind::OptCall,
    }
  }
}

impl<'a> From<&OptChainBase<'a>> for Node<'a> {
  fn from(node: &OptChainBase<'a>) -> Node<'a> {
    match node {
      OptChainBase::Member(node) => (*node).into(),
      OptChainBase::Call(node) => (*node).into(),
    }
  }
}

impl<'a> From<OptChainBase<'a>> for Node<'a> {
  fn from(node: OptChainBase<'a>) -> Node<'a> {
    match node {
      OptChainBase::Member(node) => node.into(),
      OptChainBase::Call(node) => node.into(),
    }
  }
}

fn get_view_for_opt_chain_base<'a>(inner: &'a swc_ast::OptChainBase, bump: &'a Bump) -> OptChainBase<'a> {
  match inner {
    swc_ast::OptChainBase::Member(value) => OptChainBase::Member(get_view_for_member_expr(value, bump)),
    swc_ast::OptChainBase::Call(value) => OptChainBase::Call(get_view_for_opt_call(value, bump)),
  }
}

fn set_parent_for_opt_chain_base<'a>(node: &OptChainBase<'a>, parent: Node<'a>) {
  match node {
    OptChainBase::Member(value) => set_parent_for_member_expr(value, parent),
    OptChainBase::Call(value) => set_parent_for_opt_call(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum ParamOrTsParamProp<'a> {
  TsParamProp(&'a TsParamProp<'a>),
  Param(&'a Param<'a>),
}

impl<'a> ParamOrTsParamProp<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for ParamOrTsParamProp<'a> {
  fn start(&self) -> SourcePos {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.start(),
      ParamOrTsParamProp::Param(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.end(),
      ParamOrTsParamProp::Param(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for ParamOrTsParamProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => NodeTrait::parent(*node),
      ParamOrTsParamProp::Param(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.children(),
      ParamOrTsParamProp::Param(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      ParamOrTsParamProp::TsParamProp(node) => node.as_node(),
      ParamOrTsParamProp::Param(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      ParamOrTsParamProp::TsParamProp(_) => NodeKind::TsParamProp,
      ParamOrTsParamProp::Param(_) => NodeKind::Param,
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

impl<'a> From<ParamOrTsParamProp<'a>> for Node<'a> {
  fn from(node: ParamOrTsParamProp<'a>) -> Node<'a> {
    match node {
      ParamOrTsParamProp::TsParamProp(node) => node.into(),
      ParamOrTsParamProp::Param(node) => node.into(),
    }
  }
}

fn get_view_for_param_or_ts_param_prop<'a>(inner: &'a swc_ast::ParamOrTsParamProp, bump: &'a Bump) -> ParamOrTsParamProp<'a> {
  match inner {
    swc_ast::ParamOrTsParamProp::TsParamProp(value) => ParamOrTsParamProp::TsParamProp(get_view_for_ts_param_prop(value, bump)),
    swc_ast::ParamOrTsParamProp::Param(value) => ParamOrTsParamProp::Param(get_view_for_param(value, bump)),
  }
}

fn set_parent_for_param_or_ts_param_prop<'a>(node: &ParamOrTsParamProp<'a>, parent: Node<'a>) {
  match node {
    ParamOrTsParamProp::TsParamProp(value) => set_parent_for_ts_param_prop(value, parent),
    ParamOrTsParamProp::Param(value) => set_parent_for_param(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum Pat<'a> {
  Ident(&'a BindingIdent<'a>),
  Array(&'a ArrayPat<'a>),
  Rest(&'a RestPat<'a>),
  Object(&'a ObjectPat<'a>),
  Assign(&'a AssignPat<'a>),
  Invalid(&'a Invalid<'a>),
  /// Only for for-in / for-of loops. This is *syntactically* valid.
  Expr(Expr<'a>),
}

impl<'a> Pat<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for Pat<'a> {
  fn start(&self) -> SourcePos {
    match self {
      Pat::Ident(node) => node.start(),
      Pat::Array(node) => node.start(),
      Pat::Rest(node) => node.start(),
      Pat::Object(node) => node.start(),
      Pat::Assign(node) => node.start(),
      Pat::Invalid(node) => node.start(),
      Pat::Expr(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      Pat::Ident(node) => node.end(),
      Pat::Array(node) => node.end(),
      Pat::Rest(node) => node.end(),
      Pat::Object(node) => node.end(),
      Pat::Assign(node) => node.end(),
      Pat::Invalid(node) => node.end(),
      Pat::Expr(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for Pat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      Pat::Ident(node) => NodeTrait::parent(*node),
      Pat::Array(node) => NodeTrait::parent(*node),
      Pat::Rest(node) => NodeTrait::parent(*node),
      Pat::Object(node) => NodeTrait::parent(*node),
      Pat::Assign(node) => NodeTrait::parent(*node),
      Pat::Invalid(node) => NodeTrait::parent(*node),
      Pat::Expr(node) => NodeTrait::parent(node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      Pat::Ident(node) => node.as_node(),
      Pat::Array(node) => node.as_node(),
      Pat::Rest(node) => node.as_node(),
      Pat::Object(node) => node.as_node(),
      Pat::Assign(node) => node.as_node(),
      Pat::Invalid(node) => node.as_node(),
      Pat::Expr(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Pat::Ident(_) => NodeKind::BindingIdent,
      Pat::Array(_) => NodeKind::ArrayPat,
      Pat::Rest(_) => NodeKind::RestPat,
      Pat::Object(_) => NodeKind::ObjectPat,
      Pat::Assign(_) => NodeKind::AssignPat,
      Pat::Invalid(_) => NodeKind::Invalid,
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

impl<'a> From<Pat<'a>> for Node<'a> {
  fn from(node: Pat<'a>) -> Node<'a> {
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

fn get_view_for_pat<'a>(inner: &'a swc_ast::Pat, bump: &'a Bump) -> Pat<'a> {
  match inner {
    swc_ast::Pat::Ident(value) => Pat::Ident(get_view_for_binding_ident(value, bump)),
    swc_ast::Pat::Array(value) => Pat::Array(get_view_for_array_pat(value, bump)),
    swc_ast::Pat::Rest(value) => Pat::Rest(get_view_for_rest_pat(value, bump)),
    swc_ast::Pat::Object(value) => Pat::Object(get_view_for_object_pat(value, bump)),
    swc_ast::Pat::Assign(value) => Pat::Assign(get_view_for_assign_pat(value, bump)),
    swc_ast::Pat::Invalid(value) => Pat::Invalid(get_view_for_invalid(value, bump)),
    swc_ast::Pat::Expr(value) => Pat::Expr(get_view_for_expr(value, bump)),
  }
}

fn set_parent_for_pat<'a>(node: &Pat<'a>, parent: Node<'a>) {
  match node {
    Pat::Ident(value) => set_parent_for_binding_ident(value, parent),
    Pat::Array(value) => set_parent_for_array_pat(value, parent),
    Pat::Rest(value) => set_parent_for_rest_pat(value, parent),
    Pat::Object(value) => set_parent_for_object_pat(value, parent),
    Pat::Assign(value) => set_parent_for_assign_pat(value, parent),
    Pat::Invalid(value) => set_parent_for_invalid(value, parent),
    Pat::Expr(value) => set_parent_for_expr(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum PatOrExpr<'a> {
  Expr(Expr<'a>),
  Pat(Pat<'a>),
}

impl<'a> PatOrExpr<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for PatOrExpr<'a> {
  fn start(&self) -> SourcePos {
    match self {
      PatOrExpr::Expr(node) => node.start(),
      PatOrExpr::Pat(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      PatOrExpr::Expr(node) => node.end(),
      PatOrExpr::Pat(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for PatOrExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      PatOrExpr::Expr(node) => NodeTrait::parent(node),
      PatOrExpr::Pat(node) => NodeTrait::parent(node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      PatOrExpr::Expr(node) => node.children(),
      PatOrExpr::Pat(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      PatOrExpr::Expr(node) => node.as_node(),
      PatOrExpr::Pat(node) => node.as_node(),
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

impl<'a> From<PatOrExpr<'a>> for Node<'a> {
  fn from(node: PatOrExpr<'a>) -> Node<'a> {
    match node {
      PatOrExpr::Expr(node) => node.into(),
      PatOrExpr::Pat(node) => node.into(),
    }
  }
}

fn get_view_for_pat_or_expr<'a>(inner: &'a swc_ast::PatOrExpr, bump: &'a Bump) -> PatOrExpr<'a> {
  match inner {
    swc_ast::PatOrExpr::Expr(value) => PatOrExpr::Expr(get_view_for_expr(value, bump)),
    swc_ast::PatOrExpr::Pat(value) => PatOrExpr::Pat(get_view_for_pat(value, bump)),
  }
}

fn set_parent_for_pat_or_expr<'a>(node: &PatOrExpr<'a>, parent: Node<'a>) {
  match node {
    PatOrExpr::Expr(value) => set_parent_for_expr(value, parent),
    PatOrExpr::Pat(value) => set_parent_for_pat(value, parent),
  }
}

#[derive(Copy, Clone)]
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
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for Prop<'a> {
  fn start(&self) -> SourcePos {
    match self {
      Prop::Shorthand(node) => node.start(),
      Prop::KeyValue(node) => node.start(),
      Prop::Assign(node) => node.start(),
      Prop::Getter(node) => node.start(),
      Prop::Setter(node) => node.start(),
      Prop::Method(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      Prop::Shorthand(node) => node.end(),
      Prop::KeyValue(node) => node.end(),
      Prop::Assign(node) => node.end(),
      Prop::Getter(node) => node.end(),
      Prop::Setter(node) => node.end(),
      Prop::Method(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for Prop<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      Prop::Shorthand(node) => NodeTrait::parent(*node),
      Prop::KeyValue(node) => NodeTrait::parent(*node),
      Prop::Assign(node) => NodeTrait::parent(*node),
      Prop::Getter(node) => NodeTrait::parent(*node),
      Prop::Setter(node) => NodeTrait::parent(*node),
      Prop::Method(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      Prop::Shorthand(node) => node.as_node(),
      Prop::KeyValue(node) => node.as_node(),
      Prop::Assign(node) => node.as_node(),
      Prop::Getter(node) => node.as_node(),
      Prop::Setter(node) => node.as_node(),
      Prop::Method(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Prop::Shorthand(_) => NodeKind::Ident,
      Prop::KeyValue(_) => NodeKind::KeyValueProp,
      Prop::Assign(_) => NodeKind::AssignProp,
      Prop::Getter(_) => NodeKind::GetterProp,
      Prop::Setter(_) => NodeKind::SetterProp,
      Prop::Method(_) => NodeKind::MethodProp,
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

impl<'a> From<Prop<'a>> for Node<'a> {
  fn from(node: Prop<'a>) -> Node<'a> {
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

fn get_view_for_prop<'a>(inner: &'a swc_ast::Prop, bump: &'a Bump) -> Prop<'a> {
  match inner {
    swc_ast::Prop::Shorthand(value) => Prop::Shorthand(get_view_for_ident(value, bump)),
    swc_ast::Prop::KeyValue(value) => Prop::KeyValue(get_view_for_key_value_prop(value, bump)),
    swc_ast::Prop::Assign(value) => Prop::Assign(get_view_for_assign_prop(value, bump)),
    swc_ast::Prop::Getter(value) => Prop::Getter(get_view_for_getter_prop(value, bump)),
    swc_ast::Prop::Setter(value) => Prop::Setter(get_view_for_setter_prop(value, bump)),
    swc_ast::Prop::Method(value) => Prop::Method(get_view_for_method_prop(value, bump)),
  }
}

fn set_parent_for_prop<'a>(node: &Prop<'a>, parent: Node<'a>) {
  match node {
    Prop::Shorthand(value) => set_parent_for_ident(value, parent),
    Prop::KeyValue(value) => set_parent_for_key_value_prop(value, parent),
    Prop::Assign(value) => set_parent_for_assign_prop(value, parent),
    Prop::Getter(value) => set_parent_for_getter_prop(value, parent),
    Prop::Setter(value) => set_parent_for_setter_prop(value, parent),
    Prop::Method(value) => set_parent_for_method_prop(value, parent),
  }
}

#[derive(Copy, Clone)]
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
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for PropName<'a> {
  fn start(&self) -> SourcePos {
    match self {
      PropName::Ident(node) => node.start(),
      PropName::Str(node) => node.start(),
      PropName::Num(node) => node.start(),
      PropName::Computed(node) => node.start(),
      PropName::BigInt(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      PropName::Ident(node) => node.end(),
      PropName::Str(node) => node.end(),
      PropName::Num(node) => node.end(),
      PropName::Computed(node) => node.end(),
      PropName::BigInt(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for PropName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      PropName::Ident(node) => NodeTrait::parent(*node),
      PropName::Str(node) => NodeTrait::parent(*node),
      PropName::Num(node) => NodeTrait::parent(*node),
      PropName::Computed(node) => NodeTrait::parent(*node),
      PropName::BigInt(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      PropName::Ident(node) => node.as_node(),
      PropName::Str(node) => node.as_node(),
      PropName::Num(node) => node.as_node(),
      PropName::Computed(node) => node.as_node(),
      PropName::BigInt(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      PropName::Ident(_) => NodeKind::Ident,
      PropName::Str(_) => NodeKind::Str,
      PropName::Num(_) => NodeKind::Number,
      PropName::Computed(_) => NodeKind::ComputedPropName,
      PropName::BigInt(_) => NodeKind::BigInt,
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

impl<'a> From<PropName<'a>> for Node<'a> {
  fn from(node: PropName<'a>) -> Node<'a> {
    match node {
      PropName::Ident(node) => node.into(),
      PropName::Str(node) => node.into(),
      PropName::Num(node) => node.into(),
      PropName::Computed(node) => node.into(),
      PropName::BigInt(node) => node.into(),
    }
  }
}

fn get_view_for_prop_name<'a>(inner: &'a swc_ast::PropName, bump: &'a Bump) -> PropName<'a> {
  match inner {
    swc_ast::PropName::Ident(value) => PropName::Ident(get_view_for_ident(value, bump)),
    swc_ast::PropName::Str(value) => PropName::Str(get_view_for_str(value, bump)),
    swc_ast::PropName::Num(value) => PropName::Num(get_view_for_number(value, bump)),
    swc_ast::PropName::Computed(value) => PropName::Computed(get_view_for_computed_prop_name(value, bump)),
    swc_ast::PropName::BigInt(value) => PropName::BigInt(get_view_for_big_int(value, bump)),
  }
}

fn set_parent_for_prop_name<'a>(node: &PropName<'a>, parent: Node<'a>) {
  match node {
    PropName::Ident(value) => set_parent_for_ident(value, parent),
    PropName::Str(value) => set_parent_for_str(value, parent),
    PropName::Num(value) => set_parent_for_number(value, parent),
    PropName::Computed(value) => set_parent_for_computed_prop_name(value, parent),
    PropName::BigInt(value) => set_parent_for_big_int(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum PropOrSpread<'a> {
  /// Spread properties, e.g., `{a: 1, ...obj, b: 2}`.
  Spread(&'a SpreadElement<'a>),
  Prop(Prop<'a>),
}

impl<'a> PropOrSpread<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for PropOrSpread<'a> {
  fn start(&self) -> SourcePos {
    match self {
      PropOrSpread::Spread(node) => node.start(),
      PropOrSpread::Prop(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      PropOrSpread::Spread(node) => node.end(),
      PropOrSpread::Prop(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for PropOrSpread<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      PropOrSpread::Spread(node) => NodeTrait::parent(*node),
      PropOrSpread::Prop(node) => NodeTrait::parent(node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      PropOrSpread::Spread(node) => node.children(),
      PropOrSpread::Prop(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      PropOrSpread::Spread(node) => node.as_node(),
      PropOrSpread::Prop(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      PropOrSpread::Spread(_) => NodeKind::SpreadElement,
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

impl<'a> From<PropOrSpread<'a>> for Node<'a> {
  fn from(node: PropOrSpread<'a>) -> Node<'a> {
    match node {
      PropOrSpread::Spread(node) => node.into(),
      PropOrSpread::Prop(node) => node.into(),
    }
  }
}

fn get_view_for_prop_or_spread<'a>(inner: &'a swc_ast::PropOrSpread, bump: &'a Bump) -> PropOrSpread<'a> {
  match inner {
    swc_ast::PropOrSpread::Spread(value) => PropOrSpread::Spread(get_view_for_spread_element(value, bump)),
    swc_ast::PropOrSpread::Prop(value) => PropOrSpread::Prop(get_view_for_prop(value, bump)),
  }
}

fn set_parent_for_prop_or_spread<'a>(node: &PropOrSpread<'a>, parent: Node<'a>) {
  match node {
    PropOrSpread::Spread(value) => set_parent_for_spread_element(value, parent),
    PropOrSpread::Prop(value) => set_parent_for_prop(value, parent),
  }
}

#[derive(Copy, Clone)]
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
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for Stmt<'a> {
  fn start(&self) -> SourcePos {
    match self {
      Stmt::Block(node) => node.start(),
      Stmt::Empty(node) => node.start(),
      Stmt::Debugger(node) => node.start(),
      Stmt::With(node) => node.start(),
      Stmt::Return(node) => node.start(),
      Stmt::Labeled(node) => node.start(),
      Stmt::Break(node) => node.start(),
      Stmt::Continue(node) => node.start(),
      Stmt::If(node) => node.start(),
      Stmt::Switch(node) => node.start(),
      Stmt::Throw(node) => node.start(),
      Stmt::Try(node) => node.start(),
      Stmt::While(node) => node.start(),
      Stmt::DoWhile(node) => node.start(),
      Stmt::For(node) => node.start(),
      Stmt::ForIn(node) => node.start(),
      Stmt::ForOf(node) => node.start(),
      Stmt::Decl(node) => node.start(),
      Stmt::Expr(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      Stmt::Block(node) => node.end(),
      Stmt::Empty(node) => node.end(),
      Stmt::Debugger(node) => node.end(),
      Stmt::With(node) => node.end(),
      Stmt::Return(node) => node.end(),
      Stmt::Labeled(node) => node.end(),
      Stmt::Break(node) => node.end(),
      Stmt::Continue(node) => node.end(),
      Stmt::If(node) => node.end(),
      Stmt::Switch(node) => node.end(),
      Stmt::Throw(node) => node.end(),
      Stmt::Try(node) => node.end(),
      Stmt::While(node) => node.end(),
      Stmt::DoWhile(node) => node.end(),
      Stmt::For(node) => node.end(),
      Stmt::ForIn(node) => node.end(),
      Stmt::ForOf(node) => node.end(),
      Stmt::Decl(node) => node.end(),
      Stmt::Expr(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for Stmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      Stmt::Block(node) => NodeTrait::parent(*node),
      Stmt::Empty(node) => NodeTrait::parent(*node),
      Stmt::Debugger(node) => NodeTrait::parent(*node),
      Stmt::With(node) => NodeTrait::parent(*node),
      Stmt::Return(node) => NodeTrait::parent(*node),
      Stmt::Labeled(node) => NodeTrait::parent(*node),
      Stmt::Break(node) => NodeTrait::parent(*node),
      Stmt::Continue(node) => NodeTrait::parent(*node),
      Stmt::If(node) => NodeTrait::parent(*node),
      Stmt::Switch(node) => NodeTrait::parent(*node),
      Stmt::Throw(node) => NodeTrait::parent(*node),
      Stmt::Try(node) => NodeTrait::parent(*node),
      Stmt::While(node) => NodeTrait::parent(*node),
      Stmt::DoWhile(node) => NodeTrait::parent(*node),
      Stmt::For(node) => NodeTrait::parent(*node),
      Stmt::ForIn(node) => NodeTrait::parent(*node),
      Stmt::ForOf(node) => NodeTrait::parent(*node),
      Stmt::Decl(node) => NodeTrait::parent(node),
      Stmt::Expr(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      Stmt::Block(node) => node.as_node(),
      Stmt::Empty(node) => node.as_node(),
      Stmt::Debugger(node) => node.as_node(),
      Stmt::With(node) => node.as_node(),
      Stmt::Return(node) => node.as_node(),
      Stmt::Labeled(node) => node.as_node(),
      Stmt::Break(node) => node.as_node(),
      Stmt::Continue(node) => node.as_node(),
      Stmt::If(node) => node.as_node(),
      Stmt::Switch(node) => node.as_node(),
      Stmt::Throw(node) => node.as_node(),
      Stmt::Try(node) => node.as_node(),
      Stmt::While(node) => node.as_node(),
      Stmt::DoWhile(node) => node.as_node(),
      Stmt::For(node) => node.as_node(),
      Stmt::ForIn(node) => node.as_node(),
      Stmt::ForOf(node) => node.as_node(),
      Stmt::Decl(node) => node.as_node(),
      Stmt::Expr(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      Stmt::Block(_) => NodeKind::BlockStmt,
      Stmt::Empty(_) => NodeKind::EmptyStmt,
      Stmt::Debugger(_) => NodeKind::DebuggerStmt,
      Stmt::With(_) => NodeKind::WithStmt,
      Stmt::Return(_) => NodeKind::ReturnStmt,
      Stmt::Labeled(_) => NodeKind::LabeledStmt,
      Stmt::Break(_) => NodeKind::BreakStmt,
      Stmt::Continue(_) => NodeKind::ContinueStmt,
      Stmt::If(_) => NodeKind::IfStmt,
      Stmt::Switch(_) => NodeKind::SwitchStmt,
      Stmt::Throw(_) => NodeKind::ThrowStmt,
      Stmt::Try(_) => NodeKind::TryStmt,
      Stmt::While(_) => NodeKind::WhileStmt,
      Stmt::DoWhile(_) => NodeKind::DoWhileStmt,
      Stmt::For(_) => NodeKind::ForStmt,
      Stmt::ForIn(_) => NodeKind::ForInStmt,
      Stmt::ForOf(_) => NodeKind::ForOfStmt,
      Stmt::Decl(node) => node.kind(),
      Stmt::Expr(_) => NodeKind::ExprStmt,
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

impl<'a> From<Stmt<'a>> for Node<'a> {
  fn from(node: Stmt<'a>) -> Node<'a> {
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

fn get_view_for_stmt<'a>(inner: &'a swc_ast::Stmt, bump: &'a Bump) -> Stmt<'a> {
  match inner {
    swc_ast::Stmt::Block(value) => Stmt::Block(get_view_for_block_stmt(value, bump)),
    swc_ast::Stmt::Empty(value) => Stmt::Empty(get_view_for_empty_stmt(value, bump)),
    swc_ast::Stmt::Debugger(value) => Stmt::Debugger(get_view_for_debugger_stmt(value, bump)),
    swc_ast::Stmt::With(value) => Stmt::With(get_view_for_with_stmt(value, bump)),
    swc_ast::Stmt::Return(value) => Stmt::Return(get_view_for_return_stmt(value, bump)),
    swc_ast::Stmt::Labeled(value) => Stmt::Labeled(get_view_for_labeled_stmt(value, bump)),
    swc_ast::Stmt::Break(value) => Stmt::Break(get_view_for_break_stmt(value, bump)),
    swc_ast::Stmt::Continue(value) => Stmt::Continue(get_view_for_continue_stmt(value, bump)),
    swc_ast::Stmt::If(value) => Stmt::If(get_view_for_if_stmt(value, bump)),
    swc_ast::Stmt::Switch(value) => Stmt::Switch(get_view_for_switch_stmt(value, bump)),
    swc_ast::Stmt::Throw(value) => Stmt::Throw(get_view_for_throw_stmt(value, bump)),
    swc_ast::Stmt::Try(value) => Stmt::Try(get_view_for_try_stmt(value, bump)),
    swc_ast::Stmt::While(value) => Stmt::While(get_view_for_while_stmt(value, bump)),
    swc_ast::Stmt::DoWhile(value) => Stmt::DoWhile(get_view_for_do_while_stmt(value, bump)),
    swc_ast::Stmt::For(value) => Stmt::For(get_view_for_for_stmt(value, bump)),
    swc_ast::Stmt::ForIn(value) => Stmt::ForIn(get_view_for_for_in_stmt(value, bump)),
    swc_ast::Stmt::ForOf(value) => Stmt::ForOf(get_view_for_for_of_stmt(value, bump)),
    swc_ast::Stmt::Decl(value) => Stmt::Decl(get_view_for_decl(value, bump)),
    swc_ast::Stmt::Expr(value) => Stmt::Expr(get_view_for_expr_stmt(value, bump)),
  }
}

fn set_parent_for_stmt<'a>(node: &Stmt<'a>, parent: Node<'a>) {
  match node {
    Stmt::Block(value) => set_parent_for_block_stmt(value, parent),
    Stmt::Empty(value) => set_parent_for_empty_stmt(value, parent),
    Stmt::Debugger(value) => set_parent_for_debugger_stmt(value, parent),
    Stmt::With(value) => set_parent_for_with_stmt(value, parent),
    Stmt::Return(value) => set_parent_for_return_stmt(value, parent),
    Stmt::Labeled(value) => set_parent_for_labeled_stmt(value, parent),
    Stmt::Break(value) => set_parent_for_break_stmt(value, parent),
    Stmt::Continue(value) => set_parent_for_continue_stmt(value, parent),
    Stmt::If(value) => set_parent_for_if_stmt(value, parent),
    Stmt::Switch(value) => set_parent_for_switch_stmt(value, parent),
    Stmt::Throw(value) => set_parent_for_throw_stmt(value, parent),
    Stmt::Try(value) => set_parent_for_try_stmt(value, parent),
    Stmt::While(value) => set_parent_for_while_stmt(value, parent),
    Stmt::DoWhile(value) => set_parent_for_do_while_stmt(value, parent),
    Stmt::For(value) => set_parent_for_for_stmt(value, parent),
    Stmt::ForIn(value) => set_parent_for_for_in_stmt(value, parent),
    Stmt::ForOf(value) => set_parent_for_for_of_stmt(value, parent),
    Stmt::Decl(value) => set_parent_for_decl(value, parent),
    Stmt::Expr(value) => set_parent_for_expr_stmt(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum SuperProp<'a> {
  Ident(&'a Ident<'a>),
  Computed(&'a ComputedPropName<'a>),
}

impl<'a> SuperProp<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for SuperProp<'a> {
  fn start(&self) -> SourcePos {
    match self {
      SuperProp::Ident(node) => node.start(),
      SuperProp::Computed(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      SuperProp::Ident(node) => node.end(),
      SuperProp::Computed(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for SuperProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      SuperProp::Ident(node) => NodeTrait::parent(*node),
      SuperProp::Computed(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      SuperProp::Ident(node) => node.children(),
      SuperProp::Computed(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      SuperProp::Ident(node) => node.as_node(),
      SuperProp::Computed(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      SuperProp::Ident(_) => NodeKind::Ident,
      SuperProp::Computed(_) => NodeKind::ComputedPropName,
    }
  }
}

impl<'a> From<&SuperProp<'a>> for Node<'a> {
  fn from(node: &SuperProp<'a>) -> Node<'a> {
    match node {
      SuperProp::Ident(node) => (*node).into(),
      SuperProp::Computed(node) => (*node).into(),
    }
  }
}

impl<'a> From<SuperProp<'a>> for Node<'a> {
  fn from(node: SuperProp<'a>) -> Node<'a> {
    match node {
      SuperProp::Ident(node) => node.into(),
      SuperProp::Computed(node) => node.into(),
    }
  }
}

fn get_view_for_super_prop<'a>(inner: &'a swc_ast::SuperProp, bump: &'a Bump) -> SuperProp<'a> {
  match inner {
    swc_ast::SuperProp::Ident(value) => SuperProp::Ident(get_view_for_ident(value, bump)),
    swc_ast::SuperProp::Computed(value) => SuperProp::Computed(get_view_for_computed_prop_name(value, bump)),
  }
}

fn set_parent_for_super_prop<'a>(node: &SuperProp<'a>, parent: Node<'a>) {
  match node {
    SuperProp::Ident(value) => set_parent_for_ident(value, parent),
    SuperProp::Computed(value) => set_parent_for_computed_prop_name(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsEntityName<'a> {
  TsQualifiedName(&'a TsQualifiedName<'a>),
  Ident(&'a Ident<'a>),
}

impl<'a> TsEntityName<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsEntityName<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsEntityName::TsQualifiedName(node) => node.start(),
      TsEntityName::Ident(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsEntityName::TsQualifiedName(node) => node.end(),
      TsEntityName::Ident(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsEntityName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsEntityName::TsQualifiedName(node) => NodeTrait::parent(*node),
      TsEntityName::Ident(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsEntityName::TsQualifiedName(node) => node.children(),
      TsEntityName::Ident(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsEntityName::TsQualifiedName(node) => node.as_node(),
      TsEntityName::Ident(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsEntityName::TsQualifiedName(_) => NodeKind::TsQualifiedName,
      TsEntityName::Ident(_) => NodeKind::Ident,
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

impl<'a> From<TsEntityName<'a>> for Node<'a> {
  fn from(node: TsEntityName<'a>) -> Node<'a> {
    match node {
      TsEntityName::TsQualifiedName(node) => node.into(),
      TsEntityName::Ident(node) => node.into(),
    }
  }
}

fn get_view_for_ts_entity_name<'a>(inner: &'a swc_ast::TsEntityName, bump: &'a Bump) -> TsEntityName<'a> {
  match inner {
    swc_ast::TsEntityName::TsQualifiedName(value) => TsEntityName::TsQualifiedName(get_view_for_ts_qualified_name(value, bump)),
    swc_ast::TsEntityName::Ident(value) => TsEntityName::Ident(get_view_for_ident(value, bump)),
  }
}

fn set_parent_for_ts_entity_name<'a>(node: &TsEntityName<'a>, parent: Node<'a>) {
  match node {
    TsEntityName::TsQualifiedName(value) => set_parent_for_ts_qualified_name(value, parent),
    TsEntityName::Ident(value) => set_parent_for_ident(value, parent),
  }
}

///
/// - Invalid: [Ident] with empty symbol.
#[derive(Copy, Clone)]
pub enum TsEnumMemberId<'a> {
  Ident(&'a Ident<'a>),
  Str(&'a Str<'a>),
}

impl<'a> TsEnumMemberId<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsEnumMemberId<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsEnumMemberId::Ident(node) => node.start(),
      TsEnumMemberId::Str(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsEnumMemberId::Ident(node) => node.end(),
      TsEnumMemberId::Str(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsEnumMemberId<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsEnumMemberId::Ident(node) => NodeTrait::parent(*node),
      TsEnumMemberId::Str(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsEnumMemberId::Ident(node) => node.children(),
      TsEnumMemberId::Str(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsEnumMemberId::Ident(node) => node.as_node(),
      TsEnumMemberId::Str(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsEnumMemberId::Ident(_) => NodeKind::Ident,
      TsEnumMemberId::Str(_) => NodeKind::Str,
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

impl<'a> From<TsEnumMemberId<'a>> for Node<'a> {
  fn from(node: TsEnumMemberId<'a>) -> Node<'a> {
    match node {
      TsEnumMemberId::Ident(node) => node.into(),
      TsEnumMemberId::Str(node) => node.into(),
    }
  }
}

fn get_view_for_ts_enum_member_id<'a>(inner: &'a swc_ast::TsEnumMemberId, bump: &'a Bump) -> TsEnumMemberId<'a> {
  match inner {
    swc_ast::TsEnumMemberId::Ident(value) => TsEnumMemberId::Ident(get_view_for_ident(value, bump)),
    swc_ast::TsEnumMemberId::Str(value) => TsEnumMemberId::Str(get_view_for_str(value, bump)),
  }
}

fn set_parent_for_ts_enum_member_id<'a>(node: &TsEnumMemberId<'a>, parent: Node<'a>) {
  match node {
    TsEnumMemberId::Ident(value) => set_parent_for_ident(value, parent),
    TsEnumMemberId::Str(value) => set_parent_for_str(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsFnOrConstructorType<'a> {
  TsFnType(&'a TsFnType<'a>),
  TsConstructorType(&'a TsConstructorType<'a>),
}

impl<'a> TsFnOrConstructorType<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsFnOrConstructorType<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.start(),
      TsFnOrConstructorType::TsConstructorType(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.end(),
      TsFnOrConstructorType::TsConstructorType(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsFnOrConstructorType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsFnOrConstructorType::TsFnType(node) => NodeTrait::parent(*node),
      TsFnOrConstructorType::TsConstructorType(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.children(),
      TsFnOrConstructorType::TsConstructorType(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsFnOrConstructorType::TsFnType(node) => node.as_node(),
      TsFnOrConstructorType::TsConstructorType(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsFnOrConstructorType::TsFnType(_) => NodeKind::TsFnType,
      TsFnOrConstructorType::TsConstructorType(_) => NodeKind::TsConstructorType,
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

impl<'a> From<TsFnOrConstructorType<'a>> for Node<'a> {
  fn from(node: TsFnOrConstructorType<'a>) -> Node<'a> {
    match node {
      TsFnOrConstructorType::TsFnType(node) => node.into(),
      TsFnOrConstructorType::TsConstructorType(node) => node.into(),
    }
  }
}

fn get_view_for_ts_fn_or_constructor_type<'a>(inner: &'a swc_ast::TsFnOrConstructorType, bump: &'a Bump) -> TsFnOrConstructorType<'a> {
  match inner {
    swc_ast::TsFnOrConstructorType::TsFnType(value) => TsFnOrConstructorType::TsFnType(get_view_for_ts_fn_type(value, bump)),
    swc_ast::TsFnOrConstructorType::TsConstructorType(value) => TsFnOrConstructorType::TsConstructorType(get_view_for_ts_constructor_type(value, bump)),
  }
}

fn set_parent_for_ts_fn_or_constructor_type<'a>(node: &TsFnOrConstructorType<'a>, parent: Node<'a>) {
  match node {
    TsFnOrConstructorType::TsFnType(value) => set_parent_for_ts_fn_type(value, parent),
    TsFnOrConstructorType::TsConstructorType(value) => set_parent_for_ts_constructor_type(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsFnParam<'a> {
  Ident(&'a BindingIdent<'a>),
  Array(&'a ArrayPat<'a>),
  Rest(&'a RestPat<'a>),
  Object(&'a ObjectPat<'a>),
}

impl<'a> TsFnParam<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsFnParam<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsFnParam::Ident(node) => node.start(),
      TsFnParam::Array(node) => node.start(),
      TsFnParam::Rest(node) => node.start(),
      TsFnParam::Object(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsFnParam::Ident(node) => node.end(),
      TsFnParam::Array(node) => node.end(),
      TsFnParam::Rest(node) => node.end(),
      TsFnParam::Object(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsFnParam<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsFnParam::Ident(node) => NodeTrait::parent(*node),
      TsFnParam::Array(node) => NodeTrait::parent(*node),
      TsFnParam::Rest(node) => NodeTrait::parent(*node),
      TsFnParam::Object(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      TsFnParam::Ident(node) => node.as_node(),
      TsFnParam::Array(node) => node.as_node(),
      TsFnParam::Rest(node) => node.as_node(),
      TsFnParam::Object(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsFnParam::Ident(_) => NodeKind::BindingIdent,
      TsFnParam::Array(_) => NodeKind::ArrayPat,
      TsFnParam::Rest(_) => NodeKind::RestPat,
      TsFnParam::Object(_) => NodeKind::ObjectPat,
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

impl<'a> From<TsFnParam<'a>> for Node<'a> {
  fn from(node: TsFnParam<'a>) -> Node<'a> {
    match node {
      TsFnParam::Ident(node) => node.into(),
      TsFnParam::Array(node) => node.into(),
      TsFnParam::Rest(node) => node.into(),
      TsFnParam::Object(node) => node.into(),
    }
  }
}

fn get_view_for_ts_fn_param<'a>(inner: &'a swc_ast::TsFnParam, bump: &'a Bump) -> TsFnParam<'a> {
  match inner {
    swc_ast::TsFnParam::Ident(value) => TsFnParam::Ident(get_view_for_binding_ident(value, bump)),
    swc_ast::TsFnParam::Array(value) => TsFnParam::Array(get_view_for_array_pat(value, bump)),
    swc_ast::TsFnParam::Rest(value) => TsFnParam::Rest(get_view_for_rest_pat(value, bump)),
    swc_ast::TsFnParam::Object(value) => TsFnParam::Object(get_view_for_object_pat(value, bump)),
  }
}

fn set_parent_for_ts_fn_param<'a>(node: &TsFnParam<'a>, parent: Node<'a>) {
  match node {
    TsFnParam::Ident(value) => set_parent_for_binding_ident(value, parent),
    TsFnParam::Array(value) => set_parent_for_array_pat(value, parent),
    TsFnParam::Rest(value) => set_parent_for_rest_pat(value, parent),
    TsFnParam::Object(value) => set_parent_for_object_pat(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsLit<'a> {
  Number(&'a Number<'a>),
  Str(&'a Str<'a>),
  Bool(&'a Bool<'a>),
  BigInt(&'a BigInt<'a>),
  Tpl(&'a TsTplLitType<'a>),
}

impl<'a> TsLit<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsLit<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsLit::Number(node) => node.start(),
      TsLit::Str(node) => node.start(),
      TsLit::Bool(node) => node.start(),
      TsLit::BigInt(node) => node.start(),
      TsLit::Tpl(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsLit::Number(node) => node.end(),
      TsLit::Str(node) => node.end(),
      TsLit::Bool(node) => node.end(),
      TsLit::BigInt(node) => node.end(),
      TsLit::Tpl(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsLit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsLit::Number(node) => NodeTrait::parent(*node),
      TsLit::Str(node) => NodeTrait::parent(*node),
      TsLit::Bool(node) => NodeTrait::parent(*node),
      TsLit::BigInt(node) => NodeTrait::parent(*node),
      TsLit::Tpl(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      TsLit::Number(node) => node.as_node(),
      TsLit::Str(node) => node.as_node(),
      TsLit::Bool(node) => node.as_node(),
      TsLit::BigInt(node) => node.as_node(),
      TsLit::Tpl(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsLit::Number(_) => NodeKind::Number,
      TsLit::Str(_) => NodeKind::Str,
      TsLit::Bool(_) => NodeKind::Bool,
      TsLit::BigInt(_) => NodeKind::BigInt,
      TsLit::Tpl(_) => NodeKind::TsTplLitType,
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

impl<'a> From<TsLit<'a>> for Node<'a> {
  fn from(node: TsLit<'a>) -> Node<'a> {
    match node {
      TsLit::Number(node) => node.into(),
      TsLit::Str(node) => node.into(),
      TsLit::Bool(node) => node.into(),
      TsLit::BigInt(node) => node.into(),
      TsLit::Tpl(node) => node.into(),
    }
  }
}

fn get_view_for_ts_lit<'a>(inner: &'a swc_ast::TsLit, bump: &'a Bump) -> TsLit<'a> {
  match inner {
    swc_ast::TsLit::Number(value) => TsLit::Number(get_view_for_number(value, bump)),
    swc_ast::TsLit::Str(value) => TsLit::Str(get_view_for_str(value, bump)),
    swc_ast::TsLit::Bool(value) => TsLit::Bool(get_view_for_bool(value, bump)),
    swc_ast::TsLit::BigInt(value) => TsLit::BigInt(get_view_for_big_int(value, bump)),
    swc_ast::TsLit::Tpl(value) => TsLit::Tpl(get_view_for_ts_tpl_lit_type(value, bump)),
  }
}

fn set_parent_for_ts_lit<'a>(node: &TsLit<'a>, parent: Node<'a>) {
  match node {
    TsLit::Number(value) => set_parent_for_number(value, parent),
    TsLit::Str(value) => set_parent_for_str(value, parent),
    TsLit::Bool(value) => set_parent_for_bool(value, parent),
    TsLit::BigInt(value) => set_parent_for_big_int(value, parent),
    TsLit::Tpl(value) => set_parent_for_ts_tpl_lit_type(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsModuleName<'a> {
  Ident(&'a Ident<'a>),
  Str(&'a Str<'a>),
}

impl<'a> TsModuleName<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsModuleName<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsModuleName::Ident(node) => node.start(),
      TsModuleName::Str(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsModuleName::Ident(node) => node.end(),
      TsModuleName::Str(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsModuleName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsModuleName::Ident(node) => NodeTrait::parent(*node),
      TsModuleName::Str(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsModuleName::Ident(node) => node.children(),
      TsModuleName::Str(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsModuleName::Ident(node) => node.as_node(),
      TsModuleName::Str(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsModuleName::Ident(_) => NodeKind::Ident,
      TsModuleName::Str(_) => NodeKind::Str,
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

impl<'a> From<TsModuleName<'a>> for Node<'a> {
  fn from(node: TsModuleName<'a>) -> Node<'a> {
    match node {
      TsModuleName::Ident(node) => node.into(),
      TsModuleName::Str(node) => node.into(),
    }
  }
}

fn get_view_for_ts_module_name<'a>(inner: &'a swc_ast::TsModuleName, bump: &'a Bump) -> TsModuleName<'a> {
  match inner {
    swc_ast::TsModuleName::Ident(value) => TsModuleName::Ident(get_view_for_ident(value, bump)),
    swc_ast::TsModuleName::Str(value) => TsModuleName::Str(get_view_for_str(value, bump)),
  }
}

fn set_parent_for_ts_module_name<'a>(node: &TsModuleName<'a>, parent: Node<'a>) {
  match node {
    TsModuleName::Ident(value) => set_parent_for_ident(value, parent),
    TsModuleName::Str(value) => set_parent_for_str(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsModuleRef<'a> {
  TsEntityName(TsEntityName<'a>),
  TsExternalModuleRef(&'a TsExternalModuleRef<'a>),
}

impl<'a> TsModuleRef<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for TsModuleRef<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsModuleRef::TsEntityName(node) => node.start(),
      TsModuleRef::TsExternalModuleRef(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsModuleRef::TsEntityName(node) => node.end(),
      TsModuleRef::TsExternalModuleRef(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsModuleRef<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsModuleRef::TsEntityName(node) => NodeTrait::parent(node),
      TsModuleRef::TsExternalModuleRef(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsModuleRef::TsEntityName(node) => node.children(),
      TsModuleRef::TsExternalModuleRef(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsModuleRef::TsEntityName(node) => node.as_node(),
      TsModuleRef::TsExternalModuleRef(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsModuleRef::TsEntityName(node) => node.kind(),
      TsModuleRef::TsExternalModuleRef(_) => NodeKind::TsExternalModuleRef,
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

impl<'a> From<TsModuleRef<'a>> for Node<'a> {
  fn from(node: TsModuleRef<'a>) -> Node<'a> {
    match node {
      TsModuleRef::TsEntityName(node) => node.into(),
      TsModuleRef::TsExternalModuleRef(node) => node.into(),
    }
  }
}

fn get_view_for_ts_module_ref<'a>(inner: &'a swc_ast::TsModuleRef, bump: &'a Bump) -> TsModuleRef<'a> {
  match inner {
    swc_ast::TsModuleRef::TsEntityName(value) => TsModuleRef::TsEntityName(get_view_for_ts_entity_name(value, bump)),
    swc_ast::TsModuleRef::TsExternalModuleRef(value) => TsModuleRef::TsExternalModuleRef(get_view_for_ts_external_module_ref(value, bump)),
  }
}

fn set_parent_for_ts_module_ref<'a>(node: &TsModuleRef<'a>, parent: Node<'a>) {
  match node {
    TsModuleRef::TsEntityName(value) => set_parent_for_ts_entity_name(value, parent),
    TsModuleRef::TsExternalModuleRef(value) => set_parent_for_ts_external_module_ref(value, parent),
  }
}

/// `namespace A.B { }` is a namespace named `A` with another TsNamespaceDecl as
/// its body.
#[derive(Copy, Clone)]
pub enum TsNamespaceBody<'a> {
  TsModuleBlock(&'a TsModuleBlock<'a>),
  TsNamespaceDecl(&'a TsNamespaceDecl<'a>),
}

impl<'a> TsNamespaceBody<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsNamespaceBody<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.start(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.end(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsNamespaceBody<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => NodeTrait::parent(*node),
      TsNamespaceBody::TsNamespaceDecl(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.children(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsNamespaceBody::TsModuleBlock(node) => node.as_node(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsNamespaceBody::TsModuleBlock(_) => NodeKind::TsModuleBlock,
      TsNamespaceBody::TsNamespaceDecl(_) => NodeKind::TsNamespaceDecl,
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

impl<'a> From<TsNamespaceBody<'a>> for Node<'a> {
  fn from(node: TsNamespaceBody<'a>) -> Node<'a> {
    match node {
      TsNamespaceBody::TsModuleBlock(node) => node.into(),
      TsNamespaceBody::TsNamespaceDecl(node) => node.into(),
    }
  }
}

fn get_view_for_ts_namespace_body<'a>(inner: &'a swc_ast::TsNamespaceBody, bump: &'a Bump) -> TsNamespaceBody<'a> {
  match inner {
    swc_ast::TsNamespaceBody::TsModuleBlock(value) => TsNamespaceBody::TsModuleBlock(get_view_for_ts_module_block(value, bump)),
    swc_ast::TsNamespaceBody::TsNamespaceDecl(value) => TsNamespaceBody::TsNamespaceDecl(get_view_for_ts_namespace_decl(value, bump)),
  }
}

fn set_parent_for_ts_namespace_body<'a>(node: &TsNamespaceBody<'a>, parent: Node<'a>) {
  match node {
    TsNamespaceBody::TsModuleBlock(value) => set_parent_for_ts_module_block(value, parent),
    TsNamespaceBody::TsNamespaceDecl(value) => set_parent_for_ts_namespace_decl(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsParamPropParam<'a> {
  Ident(&'a BindingIdent<'a>),
  Assign(&'a AssignPat<'a>),
}

impl<'a> TsParamPropParam<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsParamPropParam<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsParamPropParam::Ident(node) => node.start(),
      TsParamPropParam::Assign(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsParamPropParam::Ident(node) => node.end(),
      TsParamPropParam::Assign(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsParamPropParam<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsParamPropParam::Ident(node) => NodeTrait::parent(*node),
      TsParamPropParam::Assign(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsParamPropParam::Ident(node) => node.children(),
      TsParamPropParam::Assign(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsParamPropParam::Ident(node) => node.as_node(),
      TsParamPropParam::Assign(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsParamPropParam::Ident(_) => NodeKind::BindingIdent,
      TsParamPropParam::Assign(_) => NodeKind::AssignPat,
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

impl<'a> From<TsParamPropParam<'a>> for Node<'a> {
  fn from(node: TsParamPropParam<'a>) -> Node<'a> {
    match node {
      TsParamPropParam::Ident(node) => node.into(),
      TsParamPropParam::Assign(node) => node.into(),
    }
  }
}

fn get_view_for_ts_param_prop_param<'a>(inner: &'a swc_ast::TsParamPropParam, bump: &'a Bump) -> TsParamPropParam<'a> {
  match inner {
    swc_ast::TsParamPropParam::Ident(value) => TsParamPropParam::Ident(get_view_for_binding_ident(value, bump)),
    swc_ast::TsParamPropParam::Assign(value) => TsParamPropParam::Assign(get_view_for_assign_pat(value, bump)),
  }
}

fn set_parent_for_ts_param_prop_param<'a>(node: &TsParamPropParam<'a>, parent: Node<'a>) {
  match node {
    TsParamPropParam::Ident(value) => set_parent_for_binding_ident(value, parent),
    TsParamPropParam::Assign(value) => set_parent_for_assign_pat(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsThisTypeOrIdent<'a> {
  TsThisType(&'a TsThisType<'a>),
  Ident(&'a Ident<'a>),
}

impl<'a> TsThisTypeOrIdent<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsThisTypeOrIdent<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.start(),
      TsThisTypeOrIdent::Ident(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.end(),
      TsThisTypeOrIdent::Ident(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsThisTypeOrIdent<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => NodeTrait::parent(*node),
      TsThisTypeOrIdent::Ident(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.children(),
      TsThisTypeOrIdent::Ident(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsThisTypeOrIdent::TsThisType(node) => node.as_node(),
      TsThisTypeOrIdent::Ident(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsThisTypeOrIdent::TsThisType(_) => NodeKind::TsThisType,
      TsThisTypeOrIdent::Ident(_) => NodeKind::Ident,
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

impl<'a> From<TsThisTypeOrIdent<'a>> for Node<'a> {
  fn from(node: TsThisTypeOrIdent<'a>) -> Node<'a> {
    match node {
      TsThisTypeOrIdent::TsThisType(node) => node.into(),
      TsThisTypeOrIdent::Ident(node) => node.into(),
    }
  }
}

fn get_view_for_ts_this_type_or_ident<'a>(inner: &'a swc_ast::TsThisTypeOrIdent, bump: &'a Bump) -> TsThisTypeOrIdent<'a> {
  match inner {
    swc_ast::TsThisTypeOrIdent::TsThisType(value) => TsThisTypeOrIdent::TsThisType(get_view_for_ts_this_type(value, bump)),
    swc_ast::TsThisTypeOrIdent::Ident(value) => TsThisTypeOrIdent::Ident(get_view_for_ident(value, bump)),
  }
}

fn set_parent_for_ts_this_type_or_ident<'a>(node: &TsThisTypeOrIdent<'a>, parent: Node<'a>) {
  match node {
    TsThisTypeOrIdent::TsThisType(value) => set_parent_for_ts_this_type(value, parent),
    TsThisTypeOrIdent::Ident(value) => set_parent_for_ident(value, parent),
  }
}

#[derive(Copy, Clone)]
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
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for TsType<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsType::TsKeywordType(node) => node.start(),
      TsType::TsThisType(node) => node.start(),
      TsType::TsFnOrConstructorType(node) => node.start(),
      TsType::TsTypeRef(node) => node.start(),
      TsType::TsTypeQuery(node) => node.start(),
      TsType::TsTypeLit(node) => node.start(),
      TsType::TsArrayType(node) => node.start(),
      TsType::TsTupleType(node) => node.start(),
      TsType::TsOptionalType(node) => node.start(),
      TsType::TsRestType(node) => node.start(),
      TsType::TsUnionOrIntersectionType(node) => node.start(),
      TsType::TsConditionalType(node) => node.start(),
      TsType::TsInferType(node) => node.start(),
      TsType::TsParenthesizedType(node) => node.start(),
      TsType::TsTypeOperator(node) => node.start(),
      TsType::TsIndexedAccessType(node) => node.start(),
      TsType::TsMappedType(node) => node.start(),
      TsType::TsLitType(node) => node.start(),
      TsType::TsTypePredicate(node) => node.start(),
      TsType::TsImportType(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsType::TsKeywordType(node) => node.end(),
      TsType::TsThisType(node) => node.end(),
      TsType::TsFnOrConstructorType(node) => node.end(),
      TsType::TsTypeRef(node) => node.end(),
      TsType::TsTypeQuery(node) => node.end(),
      TsType::TsTypeLit(node) => node.end(),
      TsType::TsArrayType(node) => node.end(),
      TsType::TsTupleType(node) => node.end(),
      TsType::TsOptionalType(node) => node.end(),
      TsType::TsRestType(node) => node.end(),
      TsType::TsUnionOrIntersectionType(node) => node.end(),
      TsType::TsConditionalType(node) => node.end(),
      TsType::TsInferType(node) => node.end(),
      TsType::TsParenthesizedType(node) => node.end(),
      TsType::TsTypeOperator(node) => node.end(),
      TsType::TsIndexedAccessType(node) => node.end(),
      TsType::TsMappedType(node) => node.end(),
      TsType::TsLitType(node) => node.end(),
      TsType::TsTypePredicate(node) => node.end(),
      TsType::TsImportType(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsType::TsKeywordType(node) => NodeTrait::parent(*node),
      TsType::TsThisType(node) => NodeTrait::parent(*node),
      TsType::TsFnOrConstructorType(node) => NodeTrait::parent(node),
      TsType::TsTypeRef(node) => NodeTrait::parent(*node),
      TsType::TsTypeQuery(node) => NodeTrait::parent(*node),
      TsType::TsTypeLit(node) => NodeTrait::parent(*node),
      TsType::TsArrayType(node) => NodeTrait::parent(*node),
      TsType::TsTupleType(node) => NodeTrait::parent(*node),
      TsType::TsOptionalType(node) => NodeTrait::parent(*node),
      TsType::TsRestType(node) => NodeTrait::parent(*node),
      TsType::TsUnionOrIntersectionType(node) => NodeTrait::parent(node),
      TsType::TsConditionalType(node) => NodeTrait::parent(*node),
      TsType::TsInferType(node) => NodeTrait::parent(*node),
      TsType::TsParenthesizedType(node) => NodeTrait::parent(*node),
      TsType::TsTypeOperator(node) => NodeTrait::parent(*node),
      TsType::TsIndexedAccessType(node) => NodeTrait::parent(*node),
      TsType::TsMappedType(node) => NodeTrait::parent(*node),
      TsType::TsLitType(node) => NodeTrait::parent(*node),
      TsType::TsTypePredicate(node) => NodeTrait::parent(*node),
      TsType::TsImportType(node) => NodeTrait::parent(*node),
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

  fn as_node(&self) -> Node<'a> {
    match self {
      TsType::TsKeywordType(node) => node.as_node(),
      TsType::TsThisType(node) => node.as_node(),
      TsType::TsFnOrConstructorType(node) => node.as_node(),
      TsType::TsTypeRef(node) => node.as_node(),
      TsType::TsTypeQuery(node) => node.as_node(),
      TsType::TsTypeLit(node) => node.as_node(),
      TsType::TsArrayType(node) => node.as_node(),
      TsType::TsTupleType(node) => node.as_node(),
      TsType::TsOptionalType(node) => node.as_node(),
      TsType::TsRestType(node) => node.as_node(),
      TsType::TsUnionOrIntersectionType(node) => node.as_node(),
      TsType::TsConditionalType(node) => node.as_node(),
      TsType::TsInferType(node) => node.as_node(),
      TsType::TsParenthesizedType(node) => node.as_node(),
      TsType::TsTypeOperator(node) => node.as_node(),
      TsType::TsIndexedAccessType(node) => node.as_node(),
      TsType::TsMappedType(node) => node.as_node(),
      TsType::TsLitType(node) => node.as_node(),
      TsType::TsTypePredicate(node) => node.as_node(),
      TsType::TsImportType(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsType::TsKeywordType(_) => NodeKind::TsKeywordType,
      TsType::TsThisType(_) => NodeKind::TsThisType,
      TsType::TsFnOrConstructorType(node) => node.kind(),
      TsType::TsTypeRef(_) => NodeKind::TsTypeRef,
      TsType::TsTypeQuery(_) => NodeKind::TsTypeQuery,
      TsType::TsTypeLit(_) => NodeKind::TsTypeLit,
      TsType::TsArrayType(_) => NodeKind::TsArrayType,
      TsType::TsTupleType(_) => NodeKind::TsTupleType,
      TsType::TsOptionalType(_) => NodeKind::TsOptionalType,
      TsType::TsRestType(_) => NodeKind::TsRestType,
      TsType::TsUnionOrIntersectionType(node) => node.kind(),
      TsType::TsConditionalType(_) => NodeKind::TsConditionalType,
      TsType::TsInferType(_) => NodeKind::TsInferType,
      TsType::TsParenthesizedType(_) => NodeKind::TsParenthesizedType,
      TsType::TsTypeOperator(_) => NodeKind::TsTypeOperator,
      TsType::TsIndexedAccessType(_) => NodeKind::TsIndexedAccessType,
      TsType::TsMappedType(_) => NodeKind::TsMappedType,
      TsType::TsLitType(_) => NodeKind::TsLitType,
      TsType::TsTypePredicate(_) => NodeKind::TsTypePredicate,
      TsType::TsImportType(_) => NodeKind::TsImportType,
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

impl<'a> From<TsType<'a>> for Node<'a> {
  fn from(node: TsType<'a>) -> Node<'a> {
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

fn get_view_for_ts_type<'a>(inner: &'a swc_ast::TsType, bump: &'a Bump) -> TsType<'a> {
  match inner {
    swc_ast::TsType::TsKeywordType(value) => TsType::TsKeywordType(get_view_for_ts_keyword_type(value, bump)),
    swc_ast::TsType::TsThisType(value) => TsType::TsThisType(get_view_for_ts_this_type(value, bump)),
    swc_ast::TsType::TsFnOrConstructorType(value) => TsType::TsFnOrConstructorType(get_view_for_ts_fn_or_constructor_type(value, bump)),
    swc_ast::TsType::TsTypeRef(value) => TsType::TsTypeRef(get_view_for_ts_type_ref(value, bump)),
    swc_ast::TsType::TsTypeQuery(value) => TsType::TsTypeQuery(get_view_for_ts_type_query(value, bump)),
    swc_ast::TsType::TsTypeLit(value) => TsType::TsTypeLit(get_view_for_ts_type_lit(value, bump)),
    swc_ast::TsType::TsArrayType(value) => TsType::TsArrayType(get_view_for_ts_array_type(value, bump)),
    swc_ast::TsType::TsTupleType(value) => TsType::TsTupleType(get_view_for_ts_tuple_type(value, bump)),
    swc_ast::TsType::TsOptionalType(value) => TsType::TsOptionalType(get_view_for_ts_optional_type(value, bump)),
    swc_ast::TsType::TsRestType(value) => TsType::TsRestType(get_view_for_ts_rest_type(value, bump)),
    swc_ast::TsType::TsUnionOrIntersectionType(value) => TsType::TsUnionOrIntersectionType(get_view_for_ts_union_or_intersection_type(value, bump)),
    swc_ast::TsType::TsConditionalType(value) => TsType::TsConditionalType(get_view_for_ts_conditional_type(value, bump)),
    swc_ast::TsType::TsInferType(value) => TsType::TsInferType(get_view_for_ts_infer_type(value, bump)),
    swc_ast::TsType::TsParenthesizedType(value) => TsType::TsParenthesizedType(get_view_for_ts_parenthesized_type(value, bump)),
    swc_ast::TsType::TsTypeOperator(value) => TsType::TsTypeOperator(get_view_for_ts_type_operator(value, bump)),
    swc_ast::TsType::TsIndexedAccessType(value) => TsType::TsIndexedAccessType(get_view_for_ts_indexed_access_type(value, bump)),
    swc_ast::TsType::TsMappedType(value) => TsType::TsMappedType(get_view_for_ts_mapped_type(value, bump)),
    swc_ast::TsType::TsLitType(value) => TsType::TsLitType(get_view_for_ts_lit_type(value, bump)),
    swc_ast::TsType::TsTypePredicate(value) => TsType::TsTypePredicate(get_view_for_ts_type_predicate(value, bump)),
    swc_ast::TsType::TsImportType(value) => TsType::TsImportType(get_view_for_ts_import_type(value, bump)),
  }
}

fn set_parent_for_ts_type<'a>(node: &TsType<'a>, parent: Node<'a>) {
  match node {
    TsType::TsKeywordType(value) => set_parent_for_ts_keyword_type(value, parent),
    TsType::TsThisType(value) => set_parent_for_ts_this_type(value, parent),
    TsType::TsFnOrConstructorType(value) => set_parent_for_ts_fn_or_constructor_type(value, parent),
    TsType::TsTypeRef(value) => set_parent_for_ts_type_ref(value, parent),
    TsType::TsTypeQuery(value) => set_parent_for_ts_type_query(value, parent),
    TsType::TsTypeLit(value) => set_parent_for_ts_type_lit(value, parent),
    TsType::TsArrayType(value) => set_parent_for_ts_array_type(value, parent),
    TsType::TsTupleType(value) => set_parent_for_ts_tuple_type(value, parent),
    TsType::TsOptionalType(value) => set_parent_for_ts_optional_type(value, parent),
    TsType::TsRestType(value) => set_parent_for_ts_rest_type(value, parent),
    TsType::TsUnionOrIntersectionType(value) => set_parent_for_ts_union_or_intersection_type(value, parent),
    TsType::TsConditionalType(value) => set_parent_for_ts_conditional_type(value, parent),
    TsType::TsInferType(value) => set_parent_for_ts_infer_type(value, parent),
    TsType::TsParenthesizedType(value) => set_parent_for_ts_parenthesized_type(value, parent),
    TsType::TsTypeOperator(value) => set_parent_for_ts_type_operator(value, parent),
    TsType::TsIndexedAccessType(value) => set_parent_for_ts_indexed_access_type(value, parent),
    TsType::TsMappedType(value) => set_parent_for_ts_mapped_type(value, parent),
    TsType::TsLitType(value) => set_parent_for_ts_lit_type(value, parent),
    TsType::TsTypePredicate(value) => set_parent_for_ts_type_predicate(value, parent),
    TsType::TsImportType(value) => set_parent_for_ts_import_type(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsTypeElement<'a> {
  TsCallSignatureDecl(&'a TsCallSignatureDecl<'a>),
  TsConstructSignatureDecl(&'a TsConstructSignatureDecl<'a>),
  TsPropertySignature(&'a TsPropertySignature<'a>),
  TsGetterSignature(&'a TsGetterSignature<'a>),
  TsSetterSignature(&'a TsSetterSignature<'a>),
  TsMethodSignature(&'a TsMethodSignature<'a>),
  TsIndexSignature(&'a TsIndexSignature<'a>),
}

impl<'a> TsTypeElement<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsTypeElement<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.start(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.start(),
      TsTypeElement::TsPropertySignature(node) => node.start(),
      TsTypeElement::TsGetterSignature(node) => node.start(),
      TsTypeElement::TsSetterSignature(node) => node.start(),
      TsTypeElement::TsMethodSignature(node) => node.start(),
      TsTypeElement::TsIndexSignature(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.end(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.end(),
      TsTypeElement::TsPropertySignature(node) => node.end(),
      TsTypeElement::TsGetterSignature(node) => node.end(),
      TsTypeElement::TsSetterSignature(node) => node.end(),
      TsTypeElement::TsMethodSignature(node) => node.end(),
      TsTypeElement::TsIndexSignature(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsTypeElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => NodeTrait::parent(*node),
      TsTypeElement::TsConstructSignatureDecl(node) => NodeTrait::parent(*node),
      TsTypeElement::TsPropertySignature(node) => NodeTrait::parent(*node),
      TsTypeElement::TsGetterSignature(node) => NodeTrait::parent(*node),
      TsTypeElement::TsSetterSignature(node) => NodeTrait::parent(*node),
      TsTypeElement::TsMethodSignature(node) => NodeTrait::parent(*node),
      TsTypeElement::TsIndexSignature(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.children(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.children(),
      TsTypeElement::TsPropertySignature(node) => node.children(),
      TsTypeElement::TsGetterSignature(node) => node.children(),
      TsTypeElement::TsSetterSignature(node) => node.children(),
      TsTypeElement::TsMethodSignature(node) => node.children(),
      TsTypeElement::TsIndexSignature(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsTypeElement::TsCallSignatureDecl(node) => node.as_node(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.as_node(),
      TsTypeElement::TsPropertySignature(node) => node.as_node(),
      TsTypeElement::TsGetterSignature(node) => node.as_node(),
      TsTypeElement::TsSetterSignature(node) => node.as_node(),
      TsTypeElement::TsMethodSignature(node) => node.as_node(),
      TsTypeElement::TsIndexSignature(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsTypeElement::TsCallSignatureDecl(_) => NodeKind::TsCallSignatureDecl,
      TsTypeElement::TsConstructSignatureDecl(_) => NodeKind::TsConstructSignatureDecl,
      TsTypeElement::TsPropertySignature(_) => NodeKind::TsPropertySignature,
      TsTypeElement::TsGetterSignature(_) => NodeKind::TsGetterSignature,
      TsTypeElement::TsSetterSignature(_) => NodeKind::TsSetterSignature,
      TsTypeElement::TsMethodSignature(_) => NodeKind::TsMethodSignature,
      TsTypeElement::TsIndexSignature(_) => NodeKind::TsIndexSignature,
    }
  }
}

impl<'a> From<&TsTypeElement<'a>> for Node<'a> {
  fn from(node: &TsTypeElement<'a>) -> Node<'a> {
    match node {
      TsTypeElement::TsCallSignatureDecl(node) => (*node).into(),
      TsTypeElement::TsConstructSignatureDecl(node) => (*node).into(),
      TsTypeElement::TsPropertySignature(node) => (*node).into(),
      TsTypeElement::TsGetterSignature(node) => (*node).into(),
      TsTypeElement::TsSetterSignature(node) => (*node).into(),
      TsTypeElement::TsMethodSignature(node) => (*node).into(),
      TsTypeElement::TsIndexSignature(node) => (*node).into(),
    }
  }
}

impl<'a> From<TsTypeElement<'a>> for Node<'a> {
  fn from(node: TsTypeElement<'a>) -> Node<'a> {
    match node {
      TsTypeElement::TsCallSignatureDecl(node) => node.into(),
      TsTypeElement::TsConstructSignatureDecl(node) => node.into(),
      TsTypeElement::TsPropertySignature(node) => node.into(),
      TsTypeElement::TsGetterSignature(node) => node.into(),
      TsTypeElement::TsSetterSignature(node) => node.into(),
      TsTypeElement::TsMethodSignature(node) => node.into(),
      TsTypeElement::TsIndexSignature(node) => node.into(),
    }
  }
}

fn get_view_for_ts_type_element<'a>(inner: &'a swc_ast::TsTypeElement, bump: &'a Bump) -> TsTypeElement<'a> {
  match inner {
    swc_ast::TsTypeElement::TsCallSignatureDecl(value) => TsTypeElement::TsCallSignatureDecl(get_view_for_ts_call_signature_decl(value, bump)),
    swc_ast::TsTypeElement::TsConstructSignatureDecl(value) => TsTypeElement::TsConstructSignatureDecl(get_view_for_ts_construct_signature_decl(value, bump)),
    swc_ast::TsTypeElement::TsPropertySignature(value) => TsTypeElement::TsPropertySignature(get_view_for_ts_property_signature(value, bump)),
    swc_ast::TsTypeElement::TsGetterSignature(value) => TsTypeElement::TsGetterSignature(get_view_for_ts_getter_signature(value, bump)),
    swc_ast::TsTypeElement::TsSetterSignature(value) => TsTypeElement::TsSetterSignature(get_view_for_ts_setter_signature(value, bump)),
    swc_ast::TsTypeElement::TsMethodSignature(value) => TsTypeElement::TsMethodSignature(get_view_for_ts_method_signature(value, bump)),
    swc_ast::TsTypeElement::TsIndexSignature(value) => TsTypeElement::TsIndexSignature(get_view_for_ts_index_signature(value, bump)),
  }
}

fn set_parent_for_ts_type_element<'a>(node: &TsTypeElement<'a>, parent: Node<'a>) {
  match node {
    TsTypeElement::TsCallSignatureDecl(value) => set_parent_for_ts_call_signature_decl(value, parent),
    TsTypeElement::TsConstructSignatureDecl(value) => set_parent_for_ts_construct_signature_decl(value, parent),
    TsTypeElement::TsPropertySignature(value) => set_parent_for_ts_property_signature(value, parent),
    TsTypeElement::TsGetterSignature(value) => set_parent_for_ts_getter_signature(value, parent),
    TsTypeElement::TsSetterSignature(value) => set_parent_for_ts_setter_signature(value, parent),
    TsTypeElement::TsMethodSignature(value) => set_parent_for_ts_method_signature(value, parent),
    TsTypeElement::TsIndexSignature(value) => set_parent_for_ts_index_signature(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsTypeQueryExpr<'a> {
  TsEntityName(TsEntityName<'a>),
  Import(&'a TsImportType<'a>),
}

impl<'a> TsTypeQueryExpr<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for TsTypeQueryExpr<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.start(),
      TsTypeQueryExpr::Import(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.end(),
      TsTypeQueryExpr::Import(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsTypeQueryExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => NodeTrait::parent(node),
      TsTypeQueryExpr::Import(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.children(),
      TsTypeQueryExpr::Import(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.as_node(),
      TsTypeQueryExpr::Import(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsTypeQueryExpr::TsEntityName(node) => node.kind(),
      TsTypeQueryExpr::Import(_) => NodeKind::TsImportType,
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

impl<'a> From<TsTypeQueryExpr<'a>> for Node<'a> {
  fn from(node: TsTypeQueryExpr<'a>) -> Node<'a> {
    match node {
      TsTypeQueryExpr::TsEntityName(node) => node.into(),
      TsTypeQueryExpr::Import(node) => node.into(),
    }
  }
}

fn get_view_for_ts_type_query_expr<'a>(inner: &'a swc_ast::TsTypeQueryExpr, bump: &'a Bump) -> TsTypeQueryExpr<'a> {
  match inner {
    swc_ast::TsTypeQueryExpr::TsEntityName(value) => TsTypeQueryExpr::TsEntityName(get_view_for_ts_entity_name(value, bump)),
    swc_ast::TsTypeQueryExpr::Import(value) => TsTypeQueryExpr::Import(get_view_for_ts_import_type(value, bump)),
  }
}

fn set_parent_for_ts_type_query_expr<'a>(node: &TsTypeQueryExpr<'a>, parent: Node<'a>) {
  match node {
    TsTypeQueryExpr::TsEntityName(value) => set_parent_for_ts_entity_name(value, parent),
    TsTypeQueryExpr::Import(value) => set_parent_for_ts_import_type(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum TsUnionOrIntersectionType<'a> {
  TsUnionType(&'a TsUnionType<'a>),
  TsIntersectionType(&'a TsIntersectionType<'a>),
}

impl<'a> TsUnionOrIntersectionType<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
  pub fn parent(&self) -> Node<'a> {
    NodeTrait::parent(self).unwrap()
  }
}

impl<'a> SourceRanged for TsUnionOrIntersectionType<'a> {
  fn start(&self) -> SourcePos {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.start(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.end(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for TsUnionOrIntersectionType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => NodeTrait::parent(*node),
      TsUnionOrIntersectionType::TsIntersectionType(node) => NodeTrait::parent(*node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.children(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      TsUnionOrIntersectionType::TsUnionType(node) => node.as_node(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      TsUnionOrIntersectionType::TsUnionType(_) => NodeKind::TsUnionType,
      TsUnionOrIntersectionType::TsIntersectionType(_) => NodeKind::TsIntersectionType,
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

impl<'a> From<TsUnionOrIntersectionType<'a>> for Node<'a> {
  fn from(node: TsUnionOrIntersectionType<'a>) -> Node<'a> {
    match node {
      TsUnionOrIntersectionType::TsUnionType(node) => node.into(),
      TsUnionOrIntersectionType::TsIntersectionType(node) => node.into(),
    }
  }
}

fn get_view_for_ts_union_or_intersection_type<'a>(inner: &'a swc_ast::TsUnionOrIntersectionType, bump: &'a Bump) -> TsUnionOrIntersectionType<'a> {
  match inner {
    swc_ast::TsUnionOrIntersectionType::TsUnionType(value) => TsUnionOrIntersectionType::TsUnionType(get_view_for_ts_union_type(value, bump)),
    swc_ast::TsUnionOrIntersectionType::TsIntersectionType(value) => TsUnionOrIntersectionType::TsIntersectionType(get_view_for_ts_intersection_type(value, bump)),
  }
}

fn set_parent_for_ts_union_or_intersection_type<'a>(node: &TsUnionOrIntersectionType<'a>, parent: Node<'a>) {
  match node {
    TsUnionOrIntersectionType::TsUnionType(value) => set_parent_for_ts_union_type(value, parent),
    TsUnionOrIntersectionType::TsIntersectionType(value) => set_parent_for_ts_intersection_type(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum VarDeclOrExpr<'a> {
  VarDecl(&'a VarDecl<'a>),
  Expr(Expr<'a>),
}

impl<'a> VarDeclOrExpr<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for VarDeclOrExpr<'a> {
  fn start(&self) -> SourcePos {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.start(),
      VarDeclOrExpr::Expr(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.end(),
      VarDeclOrExpr::Expr(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for VarDeclOrExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      VarDeclOrExpr::VarDecl(node) => NodeTrait::parent(*node),
      VarDeclOrExpr::Expr(node) => NodeTrait::parent(node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.children(),
      VarDeclOrExpr::Expr(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      VarDeclOrExpr::VarDecl(node) => node.as_node(),
      VarDeclOrExpr::Expr(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      VarDeclOrExpr::VarDecl(_) => NodeKind::VarDecl,
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

impl<'a> From<VarDeclOrExpr<'a>> for Node<'a> {
  fn from(node: VarDeclOrExpr<'a>) -> Node<'a> {
    match node {
      VarDeclOrExpr::VarDecl(node) => node.into(),
      VarDeclOrExpr::Expr(node) => node.into(),
    }
  }
}

fn get_view_for_var_decl_or_expr<'a>(inner: &'a swc_ast::VarDeclOrExpr, bump: &'a Bump) -> VarDeclOrExpr<'a> {
  match inner {
    swc_ast::VarDeclOrExpr::VarDecl(value) => VarDeclOrExpr::VarDecl(get_view_for_var_decl(value, bump)),
    swc_ast::VarDeclOrExpr::Expr(value) => VarDeclOrExpr::Expr(get_view_for_expr(value, bump)),
  }
}

fn set_parent_for_var_decl_or_expr<'a>(node: &VarDeclOrExpr<'a>, parent: Node<'a>) {
  match node {
    VarDeclOrExpr::VarDecl(value) => set_parent_for_var_decl(value, parent),
    VarDeclOrExpr::Expr(value) => set_parent_for_expr(value, parent),
  }
}

#[derive(Copy, Clone)]
pub enum VarDeclOrPat<'a> {
  VarDecl(&'a VarDecl<'a>),
  Pat(Pat<'a>),
}

impl<'a> VarDeclOrPat<'a> {
  pub fn to<T: CastableNode<'a>>(&self) -> Option<&'a T> {
    T::to(&self.into())
  }

  pub fn expect<T: CastableNode<'a>>(&self) -> &'a T {
    let node: Node<'a> = self.into();
    if let Some(result) = T::to(&node) {
      result
    } else {
      panic!("Tried to cast node of type {} to {}.", node.kind(), T::kind())
    }
  }

  pub fn is<T: CastableNode<'a>>(&self) -> bool {
    self.kind() == T::kind()
  }
}

impl<'a> SourceRanged for VarDeclOrPat<'a> {
  fn start(&self) -> SourcePos {
    match self {
      VarDeclOrPat::VarDecl(node) => node.start(),
      VarDeclOrPat::Pat(node) => node.start(),
    }
  }
  fn end(&self) -> SourcePos {
    match self {
      VarDeclOrPat::VarDecl(node) => node.end(),
      VarDeclOrPat::Pat(node) => node.end(),
    }
  }
}

impl<'a> NodeTrait<'a> for VarDeclOrPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    match self {
      VarDeclOrPat::VarDecl(node) => NodeTrait::parent(*node),
      VarDeclOrPat::Pat(node) => NodeTrait::parent(node),
    }
  }

  fn children(&self) -> Vec<Node<'a>> {
    match self {
      VarDeclOrPat::VarDecl(node) => node.children(),
      VarDeclOrPat::Pat(node) => node.children(),
    }
  }

  fn as_node(&self) -> Node<'a> {
    match self {
      VarDeclOrPat::VarDecl(node) => node.as_node(),
      VarDeclOrPat::Pat(node) => node.as_node(),
    }
  }

  fn kind(&self) -> NodeKind {
    match self {
      VarDeclOrPat::VarDecl(_) => NodeKind::VarDecl,
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

impl<'a> From<VarDeclOrPat<'a>> for Node<'a> {
  fn from(node: VarDeclOrPat<'a>) -> Node<'a> {
    match node {
      VarDeclOrPat::VarDecl(node) => node.into(),
      VarDeclOrPat::Pat(node) => node.into(),
    }
  }
}

fn get_view_for_var_decl_or_pat<'a>(inner: &'a swc_ast::VarDeclOrPat, bump: &'a Bump) -> VarDeclOrPat<'a> {
  match inner {
    swc_ast::VarDeclOrPat::VarDecl(value) => VarDeclOrPat::VarDecl(get_view_for_var_decl(value, bump)),
    swc_ast::VarDeclOrPat::Pat(value) => VarDeclOrPat::Pat(get_view_for_pat(value, bump)),
  }
}

fn set_parent_for_var_decl_or_pat<'a>(node: &VarDeclOrPat<'a>, parent: Node<'a>) {
  match node {
    VarDeclOrPat::VarDecl(value) => set_parent_for_var_decl(value, parent),
    VarDeclOrPat::Pat(value) => set_parent_for_pat(value, parent),
  }
}

/// Array literal.
#[derive(Clone)]
pub struct ArrayLit<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ArrayLit,
  pub elems: Vec<Option<&'a ExprOrSpread<'a>>>,
}

impl<'a> ArrayLit<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ArrayLit<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ArrayLit<'a>> for Node<'a> {
  fn from(node: &ArrayLit<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ArrayLit<'a>, &'a ArrayLit<'a>>(node) };
    Node::ArrayLit(node)
  }
}

impl<'a> NodeTrait<'a> for ArrayLit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ArrayLit
  }
}

impl<'a> CastableNode<'a> for ArrayLit<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ArrayLit(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ArrayLit
  }
}

fn get_view_for_array_lit<'a>(inner: &'a swc_ast::ArrayLit, bump: &'a Bump) -> &'a ArrayLit<'a> {
  let node = bump.alloc(ArrayLit {
    inner,
    parent: None,
    elems: inner.elems.iter().map(|value| match value {
      Some(value) => Some(get_view_for_expr_or_spread(value, bump)),
      None => None,
    }).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.elems.iter() {
    if let Some(value) = value {
      set_parent_for_expr_or_spread(value, parent)
    }
  }
  node
}

fn set_parent_for_array_lit<'a>(node: &ArrayLit<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ArrayLit<'a> as *mut ArrayLit<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ArrayPat<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ArrayPat,
  pub elems: Vec<Option<Pat<'a>>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> ArrayPat<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  /// Only in an ambient context
  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl<'a> SourceRanged for ArrayPat<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ArrayPat<'a>> for Node<'a> {
  fn from(node: &ArrayPat<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ArrayPat<'a>, &'a ArrayPat<'a>>(node) };
    Node::ArrayPat(node)
  }
}

impl<'a> NodeTrait<'a> for ArrayPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ArrayPat
  }
}

impl<'a> CastableNode<'a> for ArrayPat<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ArrayPat(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ArrayPat
  }
}

fn get_view_for_array_pat<'a>(inner: &'a swc_ast::ArrayPat, bump: &'a Bump) -> &'a ArrayPat<'a> {
  let node = bump.alloc(ArrayPat {
    inner,
    parent: None,
    elems: inner.elems.iter().map(|value| match value {
      Some(value) => Some(get_view_for_pat(value, bump)),
      None => None,
    }).collect(),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.elems.iter() {
    if let Some(value) = value {
      set_parent_for_pat(value, parent)
    }
  }
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  node
}

fn set_parent_for_array_pat<'a>(node: &ArrayPat<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ArrayPat<'a> as *mut ArrayPat<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ArrowExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ArrowExpr,
  pub params: Vec<Pat<'a>>,
  pub body: BlockStmtOrExpr<'a>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub return_type: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> ArrowExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn is_async(&self) -> bool {
    self.inner.is_async
  }

  pub fn is_generator(&self) -> bool {
    self.inner.is_generator
  }
}

impl<'a> SourceRanged for ArrowExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ArrowExpr<'a>> for Node<'a> {
  fn from(node: &ArrowExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ArrowExpr<'a>, &'a ArrowExpr<'a>>(node) };
    Node::ArrowExpr(node)
  }
}

impl<'a> NodeTrait<'a> for ArrowExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ArrowExpr
  }
}

impl<'a> CastableNode<'a> for ArrowExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ArrowExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ArrowExpr
  }
}

fn get_view_for_arrow_expr<'a>(inner: &'a swc_ast::ArrowExpr, bump: &'a Bump) -> &'a ArrowExpr<'a> {
  let node = bump.alloc(ArrowExpr {
    inner,
    parent: None,
    params: inner.params.iter().map(|value| get_view_for_pat(value, bump)).collect(),
    body: get_view_for_block_stmt_or_expr(&inner.body, bump),
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
    return_type: match &inner.return_type {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.params.iter() {
    set_parent_for_pat(value, parent)
  }
  set_parent_for_block_stmt_or_expr(&node.body, parent);
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  if let Some(value) = &node.return_type {
    set_parent_for_ts_type_ann(value, parent)
  };
  node
}

fn set_parent_for_arrow_expr<'a>(node: &ArrowExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ArrowExpr<'a> as *mut ArrowExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct AssignExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::AssignExpr,
  pub left: PatOrExpr<'a>,
  pub right: Expr<'a>,
}

impl<'a> AssignExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn op(&self) -> AssignOp {
    self.inner.op
  }
}

impl<'a> SourceRanged for AssignExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&AssignExpr<'a>> for Node<'a> {
  fn from(node: &AssignExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&AssignExpr<'a>, &'a AssignExpr<'a>>(node) };
    Node::AssignExpr(node)
  }
}

impl<'a> NodeTrait<'a> for AssignExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::AssignExpr
  }
}

impl<'a> CastableNode<'a> for AssignExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::AssignExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::AssignExpr
  }
}

fn get_view_for_assign_expr<'a>(inner: &'a swc_ast::AssignExpr, bump: &'a Bump) -> &'a AssignExpr<'a> {
  let node = bump.alloc(AssignExpr {
    inner,
    parent: None,
    left: get_view_for_pat_or_expr(&inner.left, bump),
    right: get_view_for_expr(&inner.right, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_pat_or_expr(&node.left, parent);
  set_parent_for_expr(&node.right, parent);
  node
}

fn set_parent_for_assign_expr<'a>(node: &AssignExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const AssignExpr<'a> as *mut AssignExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct AssignPat<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::AssignPat,
  pub left: Pat<'a>,
  pub right: Expr<'a>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> AssignPat<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for AssignPat<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&AssignPat<'a>> for Node<'a> {
  fn from(node: &AssignPat<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&AssignPat<'a>, &'a AssignPat<'a>>(node) };
    Node::AssignPat(node)
  }
}

impl<'a> NodeTrait<'a> for AssignPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::AssignPat
  }
}

impl<'a> CastableNode<'a> for AssignPat<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::AssignPat(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::AssignPat
  }
}

fn get_view_for_assign_pat<'a>(inner: &'a swc_ast::AssignPat, bump: &'a Bump) -> &'a AssignPat<'a> {
  let node = bump.alloc(AssignPat {
    inner,
    parent: None,
    left: get_view_for_pat(&inner.left, bump),
    right: get_view_for_expr(&inner.right, bump),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_pat(&node.left, parent);
  set_parent_for_expr(&node.right, parent);
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  node
}

fn set_parent_for_assign_pat<'a>(node: &AssignPat<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const AssignPat<'a> as *mut AssignPat<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// `{key}` or `{key = value}`
#[derive(Clone)]
pub struct AssignPatProp<'a> {
  parent: Option<&'a ObjectPat<'a>>,
  pub inner: &'a swc_ast::AssignPatProp,
  pub key: &'a Ident<'a>,
  pub value: Option<Expr<'a>>,
}

impl<'a> AssignPatProp<'a> {
  pub fn parent(&self) -> &'a ObjectPat<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for AssignPatProp<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&AssignPatProp<'a>> for Node<'a> {
  fn from(node: &AssignPatProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&AssignPatProp<'a>, &'a AssignPatProp<'a>>(node) };
    Node::AssignPatProp(node)
  }
}

impl<'a> NodeTrait<'a> for AssignPatProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, });
    children.push(self.key.into());
    if let Some(child) = self.value.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::AssignPatProp
  }
}

impl<'a> CastableNode<'a> for AssignPatProp<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::AssignPatProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::AssignPatProp
  }
}

fn get_view_for_assign_pat_prop<'a>(inner: &'a swc_ast::AssignPatProp, bump: &'a Bump) -> &'a AssignPatProp<'a> {
  let node = bump.alloc(AssignPatProp {
    inner,
    parent: None,
    key: get_view_for_ident(&inner.key, bump),
    value: match &inner.value {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.key, parent);
  if let Some(value) = &node.value {
    set_parent_for_expr(value, parent)
  };
  node
}

fn set_parent_for_assign_pat_prop<'a>(node: &AssignPatProp<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const AssignPatProp<'a> as *mut AssignPatProp<'a>;
    (*node_ptr).parent.replace(parent.expect::<ObjectPat>());
  }
}

#[derive(Clone)]
pub struct AssignProp<'a> {
  parent: Option<&'a ObjectLit<'a>>,
  pub inner: &'a swc_ast::AssignProp,
  pub key: &'a Ident<'a>,
  pub value: Expr<'a>,
}

impl<'a> AssignProp<'a> {
  pub fn parent(&self) -> &'a ObjectLit<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for AssignProp<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&AssignProp<'a>> for Node<'a> {
  fn from(node: &AssignProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&AssignProp<'a>, &'a AssignProp<'a>>(node) };
    Node::AssignProp(node)
  }
}

impl<'a> NodeTrait<'a> for AssignProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.key.into());
    children.push((&self.value).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::AssignProp
  }
}

impl<'a> CastableNode<'a> for AssignProp<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::AssignProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::AssignProp
  }
}

fn get_view_for_assign_prop<'a>(inner: &'a swc_ast::AssignProp, bump: &'a Bump) -> &'a AssignProp<'a> {
  let node = bump.alloc(AssignProp {
    inner,
    parent: None,
    key: get_view_for_ident(&inner.key, bump),
    value: get_view_for_expr(&inner.value, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.key, parent);
  set_parent_for_expr(&node.value, parent);
  node
}

fn set_parent_for_assign_prop<'a>(node: &AssignProp<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const AssignProp<'a> as *mut AssignProp<'a>;
    (*node_ptr).parent.replace(parent.expect::<ObjectLit>());
  }
}

#[derive(Clone)]
pub struct AwaitExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::AwaitExpr,
  pub arg: Expr<'a>,
}

impl<'a> AwaitExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for AwaitExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&AwaitExpr<'a>> for Node<'a> {
  fn from(node: &AwaitExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&AwaitExpr<'a>, &'a AwaitExpr<'a>>(node) };
    Node::AwaitExpr(node)
  }
}

impl<'a> NodeTrait<'a> for AwaitExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::AwaitExpr
  }
}

impl<'a> CastableNode<'a> for AwaitExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::AwaitExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::AwaitExpr
  }
}

fn get_view_for_await_expr<'a>(inner: &'a swc_ast::AwaitExpr, bump: &'a Bump) -> &'a AwaitExpr<'a> {
  let node = bump.alloc(AwaitExpr {
    inner,
    parent: None,
    arg: get_view_for_expr(&inner.arg, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.arg, parent);
  node
}

fn set_parent_for_await_expr<'a>(node: &AwaitExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const AwaitExpr<'a> as *mut AwaitExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct BigInt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::BigInt,
}

impl<'a> BigInt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn value(&self) -> &num_bigint::BigInt {
    &self.inner.value
  }

  /// Use `None` value only for transformations to avoid recalculate
  /// characters in big integer
  pub fn raw(&self) -> &Option<swc_atoms::JsWord> {
    &self.inner.raw
  }
}

impl<'a> SourceRanged for BigInt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&BigInt<'a>> for Node<'a> {
  fn from(node: &BigInt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&BigInt<'a>, &'a BigInt<'a>>(node) };
    Node::BigInt(node)
  }
}

impl<'a> NodeTrait<'a> for BigInt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::BigInt
  }
}

impl<'a> CastableNode<'a> for BigInt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::BigInt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::BigInt
  }
}

fn get_view_for_big_int<'a>(inner: &'a swc_ast::BigInt, bump: &'a Bump) -> &'a BigInt<'a> {
  let node = bump.alloc(BigInt {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_big_int<'a>(node: &BigInt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const BigInt<'a> as *mut BigInt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct BinExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::BinExpr,
  pub left: Expr<'a>,
  pub right: Expr<'a>,
}

impl<'a> BinExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn op(&self) -> BinaryOp {
    self.inner.op
  }
}

impl<'a> SourceRanged for BinExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&BinExpr<'a>> for Node<'a> {
  fn from(node: &BinExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&BinExpr<'a>, &'a BinExpr<'a>>(node) };
    Node::BinExpr(node)
  }
}

impl<'a> NodeTrait<'a> for BinExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::BinExpr
  }
}

impl<'a> CastableNode<'a> for BinExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::BinExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::BinExpr
  }
}

fn get_view_for_bin_expr<'a>(inner: &'a swc_ast::BinExpr, bump: &'a Bump) -> &'a BinExpr<'a> {
  let node = bump.alloc(BinExpr {
    inner,
    parent: None,
    left: get_view_for_expr(&inner.left, bump),
    right: get_view_for_expr(&inner.right, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.left, parent);
  set_parent_for_expr(&node.right, parent);
  node
}

fn set_parent_for_bin_expr<'a>(node: &BinExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const BinExpr<'a> as *mut BinExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// Identifier used as a pattern.
#[derive(Clone)]
pub struct BindingIdent<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::BindingIdent,
  pub id: &'a Ident<'a>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> BindingIdent<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for BindingIdent<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&BindingIdent<'a>> for Node<'a> {
  fn from(node: &BindingIdent<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&BindingIdent<'a>, &'a BindingIdent<'a>>(node) };
    Node::BindingIdent(node)
  }
}

impl<'a> NodeTrait<'a> for BindingIdent<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push(self.id.into());
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::BindingIdent
  }
}

impl<'a> CastableNode<'a> for BindingIdent<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::BindingIdent(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::BindingIdent
  }
}

fn get_view_for_binding_ident<'a>(inner: &'a swc_ast::BindingIdent, bump: &'a Bump) -> &'a BindingIdent<'a> {
  let node = bump.alloc(BindingIdent {
    inner,
    parent: None,
    id: get_view_for_ident(&inner.id, bump),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.id, parent);
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  node
}

fn set_parent_for_binding_ident<'a>(node: &BindingIdent<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const BindingIdent<'a> as *mut BindingIdent<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// Use when only block statements are allowed.
#[derive(Clone)]
pub struct BlockStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::BlockStmt,
  pub stmts: Vec<Stmt<'a>>,
}

impl<'a> BlockStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for BlockStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&BlockStmt<'a>> for Node<'a> {
  fn from(node: &BlockStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&BlockStmt<'a>, &'a BlockStmt<'a>>(node) };
    Node::BlockStmt(node)
  }
}

impl<'a> NodeTrait<'a> for BlockStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.stmts.len());
    for child in self.stmts.iter() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::BlockStmt
  }
}

impl<'a> CastableNode<'a> for BlockStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::BlockStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::BlockStmt
  }
}

fn get_view_for_block_stmt<'a>(inner: &'a swc_ast::BlockStmt, bump: &'a Bump) -> &'a BlockStmt<'a> {
  let node = bump.alloc(BlockStmt {
    inner,
    parent: None,
    stmts: inner.stmts.iter().map(|value| get_view_for_stmt(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.stmts.iter() {
    set_parent_for_stmt(value, parent)
  }
  node
}

fn set_parent_for_block_stmt<'a>(node: &BlockStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const BlockStmt<'a> as *mut BlockStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// A boolean literal.
///
///
/// # Creation
///
/// If you are creating a boolean literal with a dummy span, please use
/// `true.into()` or `false.into()`, instead of creating this struct directly.
///
/// All of `Box<Expr>`, `Expr`, `Lit`, `Bool` implements `From<bool>`.
#[derive(Clone)]
pub struct Bool<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Bool,
}

impl<'a> Bool<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn value(&self) -> bool {
    self.inner.value
  }
}

impl<'a> SourceRanged for Bool<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Bool<'a>> for Node<'a> {
  fn from(node: &Bool<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Bool<'a>, &'a Bool<'a>>(node) };
    Node::Bool(node)
  }
}

impl<'a> NodeTrait<'a> for Bool<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Bool
  }
}

impl<'a> CastableNode<'a> for Bool<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Bool(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Bool
  }
}

fn get_view_for_bool<'a>(inner: &'a swc_ast::Bool, bump: &'a Bump) -> &'a Bool<'a> {
  let node = bump.alloc(Bool {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_bool<'a>(node: &Bool<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Bool<'a> as *mut Bool<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct BreakStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::BreakStmt,
  pub label: Option<&'a Ident<'a>>,
}

impl<'a> BreakStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for BreakStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&BreakStmt<'a>> for Node<'a> {
  fn from(node: &BreakStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&BreakStmt<'a>, &'a BreakStmt<'a>>(node) };
    Node::BreakStmt(node)
  }
}

impl<'a> NodeTrait<'a> for BreakStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = self.label {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::BreakStmt
  }
}

impl<'a> CastableNode<'a> for BreakStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::BreakStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::BreakStmt
  }
}

fn get_view_for_break_stmt<'a>(inner: &'a swc_ast::BreakStmt, bump: &'a Bump) -> &'a BreakStmt<'a> {
  let node = bump.alloc(BreakStmt {
    inner,
    parent: None,
    label: match &inner.label {
      Some(value) => Some(get_view_for_ident(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  if let Some(value) = &node.label {
    set_parent_for_ident(value, parent)
  };
  node
}

fn set_parent_for_break_stmt<'a>(node: &BreakStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const BreakStmt<'a> as *mut BreakStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct CallExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::CallExpr,
  pub callee: Callee<'a>,
  pub args: Vec<&'a ExprOrSpread<'a>>,
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> CallExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for CallExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&CallExpr<'a>> for Node<'a> {
  fn from(node: &CallExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&CallExpr<'a>, &'a CallExpr<'a>>(node) };
    Node::CallExpr(node)
  }
}

impl<'a> NodeTrait<'a> for CallExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::CallExpr
  }
}

impl<'a> CastableNode<'a> for CallExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::CallExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::CallExpr
  }
}

fn get_view_for_call_expr<'a>(inner: &'a swc_ast::CallExpr, bump: &'a Bump) -> &'a CallExpr<'a> {
  let node = bump.alloc(CallExpr {
    inner,
    parent: None,
    callee: get_view_for_callee(&inner.callee, bump),
    args: inner.args.iter().map(|value| get_view_for_expr_or_spread(value, bump)).collect(),
    type_args: match &inner.type_args {
      Some(value) => Some(get_view_for_ts_type_param_instantiation(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_callee(&node.callee, parent);
  for value in node.args.iter() {
    set_parent_for_expr_or_spread(value, parent)
  }
  if let Some(value) = &node.type_args {
    set_parent_for_ts_type_param_instantiation(value, parent)
  };
  node
}

fn set_parent_for_call_expr<'a>(node: &CallExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const CallExpr<'a> as *mut CallExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct CatchClause<'a> {
  parent: Option<&'a TryStmt<'a>>,
  pub inner: &'a swc_ast::CatchClause,
  /// es2019
  ///
  /// The param is null if the catch binding is omitted. E.g., try { foo() }
  /// catch { bar() }
  pub param: Option<Pat<'a>>,
  pub body: &'a BlockStmt<'a>,
}

impl<'a> CatchClause<'a> {
  pub fn parent(&self) -> &'a TryStmt<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for CatchClause<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&CatchClause<'a>> for Node<'a> {
  fn from(node: &CatchClause<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&CatchClause<'a>, &'a CatchClause<'a>>(node) };
    Node::CatchClause(node)
  }
}

impl<'a> NodeTrait<'a> for CatchClause<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.param { Some(_value) => 1, None => 0, });
    if let Some(child) = self.param.as_ref() {
      children.push(child.into());
    }
    children.push(self.body.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::CatchClause
  }
}

impl<'a> CastableNode<'a> for CatchClause<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::CatchClause(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::CatchClause
  }
}

fn get_view_for_catch_clause<'a>(inner: &'a swc_ast::CatchClause, bump: &'a Bump) -> &'a CatchClause<'a> {
  let node = bump.alloc(CatchClause {
    inner,
    parent: None,
    param: match &inner.param {
      Some(value) => Some(get_view_for_pat(value, bump)),
      None => None,
    },
    body: get_view_for_block_stmt(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  if let Some(value) = &node.param {
    set_parent_for_pat(value, parent)
  };
  set_parent_for_block_stmt(&node.body, parent);
  node
}

fn set_parent_for_catch_clause<'a>(node: &CatchClause<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const CatchClause<'a> as *mut CatchClause<'a>;
    (*node_ptr).parent.replace(parent.expect::<TryStmt>());
  }
}

#[derive(Clone)]
pub struct Class<'a> {
  parent: Option<Node<'a>>,
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
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }
}

impl<'a> SourceRanged for Class<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Class<'a>> for Node<'a> {
  fn from(node: &Class<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Class<'a>, &'a Class<'a>>(node) };
    Node::Class(node)
  }
}

impl<'a> NodeTrait<'a> for Class<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Class
  }
}

impl<'a> CastableNode<'a> for Class<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Class(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Class
  }
}

fn get_view_for_class<'a>(inner: &'a swc_ast::Class, bump: &'a Bump) -> &'a Class<'a> {
  let node = bump.alloc(Class {
    inner,
    parent: None,
    decorators: inner.decorators.iter().map(|value| get_view_for_decorator(value, bump)).collect(),
    body: inner.body.iter().map(|value| get_view_for_class_member(value, bump)).collect(),
    super_class: match &inner.super_class {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
    super_type_params: match &inner.super_type_params {
      Some(value) => Some(get_view_for_ts_type_param_instantiation(value, bump)),
      None => None,
    },
    implements: inner.implements.iter().map(|value| get_view_for_ts_expr_with_type_args(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.decorators.iter() {
    set_parent_for_decorator(value, parent)
  }
  for value in node.body.iter() {
    set_parent_for_class_member(value, parent)
  }
  if let Some(value) = &node.super_class {
    set_parent_for_expr(value, parent)
  };
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  if let Some(value) = &node.super_type_params {
    set_parent_for_ts_type_param_instantiation(value, parent)
  };
  for value in node.implements.iter() {
    set_parent_for_ts_expr_with_type_args(value, parent)
  }
  node
}

fn set_parent_for_class<'a>(node: &Class<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Class<'a> as *mut Class<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ClassDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ClassDecl,
  pub ident: &'a Ident<'a>,
  pub class: &'a Class<'a>,
}

impl<'a> ClassDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl<'a> SourceRanged for ClassDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ClassDecl<'a>> for Node<'a> {
  fn from(node: &ClassDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ClassDecl<'a>, &'a ClassDecl<'a>>(node) };
    Node::ClassDecl(node)
  }
}

impl<'a> NodeTrait<'a> for ClassDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.ident.into());
    children.push(self.class.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ClassDecl
  }
}

impl<'a> CastableNode<'a> for ClassDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ClassDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ClassDecl
  }
}

fn get_view_for_class_decl<'a>(inner: &'a swc_ast::ClassDecl, bump: &'a Bump) -> &'a ClassDecl<'a> {
  let node = bump.alloc(ClassDecl {
    inner,
    parent: None,
    ident: get_view_for_ident(&inner.ident, bump),
    class: get_view_for_class(&inner.class, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.ident, parent);
  set_parent_for_class(&node.class, parent);
  node
}

fn set_parent_for_class_decl<'a>(node: &ClassDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ClassDecl<'a> as *mut ClassDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// Class expression.
#[derive(Clone)]
pub struct ClassExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ClassExpr,
  pub ident: Option<&'a Ident<'a>>,
  pub class: &'a Class<'a>,
}

impl<'a> ClassExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ClassExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ClassExpr<'a>> for Node<'a> {
  fn from(node: &ClassExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ClassExpr<'a>, &'a ClassExpr<'a>>(node) };
    Node::ClassExpr(node)
  }
}

impl<'a> NodeTrait<'a> for ClassExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.ident { Some(_value) => 1, None => 0, });
    if let Some(child) = self.ident {
      children.push(child.into());
    }
    children.push(self.class.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ClassExpr
  }
}

impl<'a> CastableNode<'a> for ClassExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ClassExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ClassExpr
  }
}

fn get_view_for_class_expr<'a>(inner: &'a swc_ast::ClassExpr, bump: &'a Bump) -> &'a ClassExpr<'a> {
  let node = bump.alloc(ClassExpr {
    inner,
    parent: None,
    ident: match &inner.ident {
      Some(value) => Some(get_view_for_ident(value, bump)),
      None => None,
    },
    class: get_view_for_class(&inner.class, bump),
  });
  let parent: Node<'a> = (&*node).into();
  if let Some(value) = &node.ident {
    set_parent_for_ident(value, parent)
  };
  set_parent_for_class(&node.class, parent);
  node
}

fn set_parent_for_class_expr<'a>(node: &ClassExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ClassExpr<'a> as *mut ClassExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ClassMethod<'a> {
  parent: Option<&'a Class<'a>>,
  pub inner: &'a swc_ast::ClassMethod,
  pub key: PropName<'a>,
  pub function: &'a Function<'a>,
}

impl<'a> ClassMethod<'a> {
  pub fn parent(&self) -> &'a Class<'a> {
    self.parent.unwrap()
  }

  pub fn method_kind(&self) -> MethodKind {
    self.inner.kind
  }

  pub fn is_static(&self) -> bool {
    self.inner.is_static
  }

  /// Typescript extension.
  pub fn accessibility(&self) -> Option<Accessibility> {
    self.inner.accessibility
  }

  /// Typescript extension.
  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }

  pub fn is_override(&self) -> bool {
    self.inner.is_override
  }
}

impl<'a> SourceRanged for ClassMethod<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ClassMethod<'a>> for Node<'a> {
  fn from(node: &ClassMethod<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ClassMethod<'a>, &'a ClassMethod<'a>>(node) };
    Node::ClassMethod(node)
  }
}

impl<'a> NodeTrait<'a> for ClassMethod<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push(self.function.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ClassMethod
  }
}

impl<'a> CastableNode<'a> for ClassMethod<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ClassMethod(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ClassMethod
  }
}

fn get_view_for_class_method<'a>(inner: &'a swc_ast::ClassMethod, bump: &'a Bump) -> &'a ClassMethod<'a> {
  let node = bump.alloc(ClassMethod {
    inner,
    parent: None,
    key: get_view_for_prop_name(&inner.key, bump),
    function: get_view_for_function(&inner.function, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_prop_name(&node.key, parent);
  set_parent_for_function(&node.function, parent);
  node
}

fn set_parent_for_class_method<'a>(node: &ClassMethod<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ClassMethod<'a> as *mut ClassMethod<'a>;
    (*node_ptr).parent.replace(parent.expect::<Class>());
  }
}

#[derive(Clone)]
pub struct ClassProp<'a> {
  parent: Option<&'a Class<'a>>,
  pub inner: &'a swc_ast::ClassProp,
  pub key: PropName<'a>,
  pub value: Option<Expr<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub decorators: Vec<&'a Decorator<'a>>,
}

impl<'a> ClassProp<'a> {
  pub fn parent(&self) -> &'a Class<'a> {
    self.parent.unwrap()
  }

  pub fn is_static(&self) -> bool {
    self.inner.is_static
  }

  /// Typescript extension.
  pub fn accessibility(&self) -> Option<Accessibility> {
    self.inner.accessibility
  }

  /// Typescript extension.
  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }

  pub fn is_override(&self) -> bool {
    self.inner.is_override
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

impl<'a> SourceRanged for ClassProp<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ClassProp<'a>> for Node<'a> {
  fn from(node: &ClassProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ClassProp<'a>, &'a ClassProp<'a>>(node) };
    Node::ClassProp(node)
  }
}

impl<'a> NodeTrait<'a> for ClassProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ClassProp
  }
}

impl<'a> CastableNode<'a> for ClassProp<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ClassProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ClassProp
  }
}

fn get_view_for_class_prop<'a>(inner: &'a swc_ast::ClassProp, bump: &'a Bump) -> &'a ClassProp<'a> {
  let node = bump.alloc(ClassProp {
    inner,
    parent: None,
    key: get_view_for_prop_name(&inner.key, bump),
    value: match &inner.value {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
    decorators: inner.decorators.iter().map(|value| get_view_for_decorator(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_prop_name(&node.key, parent);
  if let Some(value) = &node.value {
    set_parent_for_expr(value, parent)
  };
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  for value in node.decorators.iter() {
    set_parent_for_decorator(value, parent)
  }
  node
}

fn set_parent_for_class_prop<'a>(node: &ClassProp<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ClassProp<'a> as *mut ClassProp<'a>;
    (*node_ptr).parent.replace(parent.expect::<Class>());
  }
}

#[derive(Clone)]
pub struct ComputedPropName<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ComputedPropName,
  pub expr: Expr<'a>,
}

impl<'a> ComputedPropName<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ComputedPropName<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ComputedPropName<'a>> for Node<'a> {
  fn from(node: &ComputedPropName<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ComputedPropName<'a>, &'a ComputedPropName<'a>>(node) };
    Node::ComputedPropName(node)
  }
}

impl<'a> NodeTrait<'a> for ComputedPropName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ComputedPropName
  }
}

impl<'a> CastableNode<'a> for ComputedPropName<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ComputedPropName(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ComputedPropName
  }
}

fn get_view_for_computed_prop_name<'a>(inner: &'a swc_ast::ComputedPropName, bump: &'a Bump) -> &'a ComputedPropName<'a> {
  let node = bump.alloc(ComputedPropName {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_computed_prop_name<'a>(node: &ComputedPropName<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ComputedPropName<'a> as *mut ComputedPropName<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct CondExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::CondExpr,
  pub test: Expr<'a>,
  pub cons: Expr<'a>,
  pub alt: Expr<'a>,
}

impl<'a> CondExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for CondExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&CondExpr<'a>> for Node<'a> {
  fn from(node: &CondExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&CondExpr<'a>, &'a CondExpr<'a>>(node) };
    Node::CondExpr(node)
  }
}

impl<'a> NodeTrait<'a> for CondExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(3);
    children.push((&self.test).into());
    children.push((&self.cons).into());
    children.push((&self.alt).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::CondExpr
  }
}

impl<'a> CastableNode<'a> for CondExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::CondExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::CondExpr
  }
}

fn get_view_for_cond_expr<'a>(inner: &'a swc_ast::CondExpr, bump: &'a Bump) -> &'a CondExpr<'a> {
  let node = bump.alloc(CondExpr {
    inner,
    parent: None,
    test: get_view_for_expr(&inner.test, bump),
    cons: get_view_for_expr(&inner.cons, bump),
    alt: get_view_for_expr(&inner.alt, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.test, parent);
  set_parent_for_expr(&node.cons, parent);
  set_parent_for_expr(&node.alt, parent);
  node
}

fn set_parent_for_cond_expr<'a>(node: &CondExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const CondExpr<'a> as *mut CondExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct Constructor<'a> {
  parent: Option<&'a Class<'a>>,
  pub inner: &'a swc_ast::Constructor,
  pub key: PropName<'a>,
  pub params: Vec<ParamOrTsParamProp<'a>>,
  pub body: Option<&'a BlockStmt<'a>>,
}

impl<'a> Constructor<'a> {
  pub fn parent(&self) -> &'a Class<'a> {
    self.parent.unwrap()
  }

  pub fn accessibility(&self) -> Option<Accessibility> {
    self.inner.accessibility
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }
}

impl<'a> SourceRanged for Constructor<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Constructor<'a>> for Node<'a> {
  fn from(node: &Constructor<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Constructor<'a>, &'a Constructor<'a>>(node) };
    Node::Constructor(node)
  }
}

impl<'a> NodeTrait<'a> for Constructor<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Constructor
  }
}

impl<'a> CastableNode<'a> for Constructor<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Constructor(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Constructor
  }
}

fn get_view_for_constructor<'a>(inner: &'a swc_ast::Constructor, bump: &'a Bump) -> &'a Constructor<'a> {
  let node = bump.alloc(Constructor {
    inner,
    parent: None,
    key: get_view_for_prop_name(&inner.key, bump),
    params: inner.params.iter().map(|value| get_view_for_param_or_ts_param_prop(value, bump)).collect(),
    body: match &inner.body {
      Some(value) => Some(get_view_for_block_stmt(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_prop_name(&node.key, parent);
  for value in node.params.iter() {
    set_parent_for_param_or_ts_param_prop(value, parent)
  }
  if let Some(value) = &node.body {
    set_parent_for_block_stmt(value, parent)
  };
  node
}

fn set_parent_for_constructor<'a>(node: &Constructor<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Constructor<'a> as *mut Constructor<'a>;
    (*node_ptr).parent.replace(parent.expect::<Class>());
  }
}

#[derive(Clone)]
pub struct ContinueStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ContinueStmt,
  pub label: Option<&'a Ident<'a>>,
}

impl<'a> ContinueStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ContinueStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ContinueStmt<'a>> for Node<'a> {
  fn from(node: &ContinueStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ContinueStmt<'a>, &'a ContinueStmt<'a>>(node) };
    Node::ContinueStmt(node)
  }
}

impl<'a> NodeTrait<'a> for ContinueStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = self.label {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ContinueStmt
  }
}

impl<'a> CastableNode<'a> for ContinueStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ContinueStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ContinueStmt
  }
}

fn get_view_for_continue_stmt<'a>(inner: &'a swc_ast::ContinueStmt, bump: &'a Bump) -> &'a ContinueStmt<'a> {
  let node = bump.alloc(ContinueStmt {
    inner,
    parent: None,
    label: match &inner.label {
      Some(value) => Some(get_view_for_ident(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  if let Some(value) = &node.label {
    set_parent_for_ident(value, parent)
  };
  node
}

fn set_parent_for_continue_stmt<'a>(node: &ContinueStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ContinueStmt<'a> as *mut ContinueStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct DebuggerStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::DebuggerStmt,
}

impl<'a> DebuggerStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for DebuggerStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&DebuggerStmt<'a>> for Node<'a> {
  fn from(node: &DebuggerStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&DebuggerStmt<'a>, &'a DebuggerStmt<'a>>(node) };
    Node::DebuggerStmt(node)
  }
}

impl<'a> NodeTrait<'a> for DebuggerStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::DebuggerStmt
  }
}

impl<'a> CastableNode<'a> for DebuggerStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::DebuggerStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::DebuggerStmt
  }
}

fn get_view_for_debugger_stmt<'a>(inner: &'a swc_ast::DebuggerStmt, bump: &'a Bump) -> &'a DebuggerStmt<'a> {
  let node = bump.alloc(DebuggerStmt {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_debugger_stmt<'a>(node: &DebuggerStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const DebuggerStmt<'a> as *mut DebuggerStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct Decorator<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Decorator,
  pub expr: Expr<'a>,
}

impl<'a> Decorator<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for Decorator<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Decorator<'a>> for Node<'a> {
  fn from(node: &Decorator<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Decorator<'a>, &'a Decorator<'a>>(node) };
    Node::Decorator(node)
  }
}

impl<'a> NodeTrait<'a> for Decorator<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Decorator
  }
}

impl<'a> CastableNode<'a> for Decorator<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Decorator(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Decorator
  }
}

fn get_view_for_decorator<'a>(inner: &'a swc_ast::Decorator, bump: &'a Bump) -> &'a Decorator<'a> {
  let node = bump.alloc(Decorator {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_decorator<'a>(node: &Decorator<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Decorator<'a> as *mut Decorator<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct DoWhileStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::DoWhileStmt,
  pub test: Expr<'a>,
  pub body: Stmt<'a>,
}

impl<'a> DoWhileStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for DoWhileStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&DoWhileStmt<'a>> for Node<'a> {
  fn from(node: &DoWhileStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&DoWhileStmt<'a>, &'a DoWhileStmt<'a>>(node) };
    Node::DoWhileStmt(node)
  }
}

impl<'a> NodeTrait<'a> for DoWhileStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.test).into());
    children.push((&self.body).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::DoWhileStmt
  }
}

impl<'a> CastableNode<'a> for DoWhileStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::DoWhileStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::DoWhileStmt
  }
}

fn get_view_for_do_while_stmt<'a>(inner: &'a swc_ast::DoWhileStmt, bump: &'a Bump) -> &'a DoWhileStmt<'a> {
  let node = bump.alloc(DoWhileStmt {
    inner,
    parent: None,
    test: get_view_for_expr(&inner.test, bump),
    body: get_view_for_stmt(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.test, parent);
  set_parent_for_stmt(&node.body, parent);
  node
}

fn set_parent_for_do_while_stmt<'a>(node: &DoWhileStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const DoWhileStmt<'a> as *mut DoWhileStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct EmptyStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::EmptyStmt,
}

impl<'a> EmptyStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for EmptyStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&EmptyStmt<'a>> for Node<'a> {
  fn from(node: &EmptyStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&EmptyStmt<'a>, &'a EmptyStmt<'a>>(node) };
    Node::EmptyStmt(node)
  }
}

impl<'a> NodeTrait<'a> for EmptyStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::EmptyStmt
  }
}

impl<'a> CastableNode<'a> for EmptyStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::EmptyStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::EmptyStmt
  }
}

fn get_view_for_empty_stmt<'a>(inner: &'a swc_ast::EmptyStmt, bump: &'a Bump) -> &'a EmptyStmt<'a> {
  let node = bump.alloc(EmptyStmt {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_empty_stmt<'a>(node: &EmptyStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const EmptyStmt<'a> as *mut EmptyStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// `export * from 'mod'`
#[derive(Clone)]
pub struct ExportAll<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ExportAll,
  pub src: &'a Str<'a>,
  pub asserts: Option<&'a ObjectLit<'a>>,
}

impl<'a> ExportAll<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ExportAll<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ExportAll<'a>> for Node<'a> {
  fn from(node: &ExportAll<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportAll<'a>, &'a ExportAll<'a>>(node) };
    Node::ExportAll(node)
  }
}

impl<'a> NodeTrait<'a> for ExportAll<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.asserts { Some(_value) => 1, None => 0, });
    children.push(self.src.into());
    if let Some(child) = self.asserts {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportAll
  }
}

impl<'a> CastableNode<'a> for ExportAll<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportAll(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ExportAll
  }
}

fn get_view_for_export_all<'a>(inner: &'a swc_ast::ExportAll, bump: &'a Bump) -> &'a ExportAll<'a> {
  let node = bump.alloc(ExportAll {
    inner,
    parent: None,
    src: get_view_for_str(&inner.src, bump),
    asserts: match &inner.asserts {
      Some(value) => Some(get_view_for_object_lit(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_str(&node.src, parent);
  if let Some(value) = &node.asserts {
    set_parent_for_object_lit(value, parent)
  };
  node
}

fn set_parent_for_export_all<'a>(node: &ExportAll<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ExportAll<'a> as *mut ExportAll<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ExportDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ExportDecl,
  pub decl: Decl<'a>,
}

impl<'a> ExportDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ExportDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ExportDecl<'a>> for Node<'a> {
  fn from(node: &ExportDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportDecl<'a>, &'a ExportDecl<'a>>(node) };
    Node::ExportDecl(node)
  }
}

impl<'a> NodeTrait<'a> for ExportDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.decl).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportDecl
  }
}

impl<'a> CastableNode<'a> for ExportDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ExportDecl
  }
}

fn get_view_for_export_decl<'a>(inner: &'a swc_ast::ExportDecl, bump: &'a Bump) -> &'a ExportDecl<'a> {
  let node = bump.alloc(ExportDecl {
    inner,
    parent: None,
    decl: get_view_for_decl(&inner.decl, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_decl(&node.decl, parent);
  node
}

fn set_parent_for_export_decl<'a>(node: &ExportDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ExportDecl<'a> as *mut ExportDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ExportDefaultDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ExportDefaultDecl,
  pub decl: DefaultDecl<'a>,
}

impl<'a> ExportDefaultDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ExportDefaultDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ExportDefaultDecl<'a>> for Node<'a> {
  fn from(node: &ExportDefaultDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportDefaultDecl<'a>, &'a ExportDefaultDecl<'a>>(node) };
    Node::ExportDefaultDecl(node)
  }
}

impl<'a> NodeTrait<'a> for ExportDefaultDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.decl).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportDefaultDecl
  }
}

impl<'a> CastableNode<'a> for ExportDefaultDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportDefaultDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ExportDefaultDecl
  }
}

fn get_view_for_export_default_decl<'a>(inner: &'a swc_ast::ExportDefaultDecl, bump: &'a Bump) -> &'a ExportDefaultDecl<'a> {
  let node = bump.alloc(ExportDefaultDecl {
    inner,
    parent: None,
    decl: get_view_for_default_decl(&inner.decl, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_default_decl(&node.decl, parent);
  node
}

fn set_parent_for_export_default_decl<'a>(node: &ExportDefaultDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ExportDefaultDecl<'a> as *mut ExportDefaultDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ExportDefaultExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ExportDefaultExpr,
  pub expr: Expr<'a>,
}

impl<'a> ExportDefaultExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ExportDefaultExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ExportDefaultExpr<'a>> for Node<'a> {
  fn from(node: &ExportDefaultExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportDefaultExpr<'a>, &'a ExportDefaultExpr<'a>>(node) };
    Node::ExportDefaultExpr(node)
  }
}

impl<'a> NodeTrait<'a> for ExportDefaultExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportDefaultExpr
  }
}

impl<'a> CastableNode<'a> for ExportDefaultExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportDefaultExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ExportDefaultExpr
  }
}

fn get_view_for_export_default_expr<'a>(inner: &'a swc_ast::ExportDefaultExpr, bump: &'a Bump) -> &'a ExportDefaultExpr<'a> {
  let node = bump.alloc(ExportDefaultExpr {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_export_default_expr<'a>(node: &ExportDefaultExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ExportDefaultExpr<'a> as *mut ExportDefaultExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ExportDefaultSpecifier<'a> {
  parent: Option<&'a NamedExport<'a>>,
  pub inner: &'a swc_ast::ExportDefaultSpecifier,
  pub exported: &'a Ident<'a>,
}

impl<'a> ExportDefaultSpecifier<'a> {
  pub fn parent(&self) -> &'a NamedExport<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ExportDefaultSpecifier<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ExportDefaultSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportDefaultSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportDefaultSpecifier<'a>, &'a ExportDefaultSpecifier<'a>>(node) };
    Node::ExportDefaultSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for ExportDefaultSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.exported.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportDefaultSpecifier
  }
}

impl<'a> CastableNode<'a> for ExportDefaultSpecifier<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportDefaultSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ExportDefaultSpecifier
  }
}

fn get_view_for_export_default_specifier<'a>(inner: &'a swc_ast::ExportDefaultSpecifier, bump: &'a Bump) -> &'a ExportDefaultSpecifier<'a> {
  let node = bump.alloc(ExportDefaultSpecifier {
    inner,
    parent: None,
    exported: get_view_for_ident(&inner.exported, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.exported, parent);
  node
}

fn set_parent_for_export_default_specifier<'a>(node: &ExportDefaultSpecifier<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ExportDefaultSpecifier<'a> as *mut ExportDefaultSpecifier<'a>;
    (*node_ptr).parent.replace(parent.expect::<NamedExport>());
  }
}

#[derive(Clone)]
pub struct ExportNamedSpecifier<'a> {
  parent: Option<&'a NamedExport<'a>>,
  pub inner: &'a swc_ast::ExportNamedSpecifier,
  /// `foo` in `export { foo as bar }`
  pub orig: ModuleExportName<'a>,
  /// `Some(bar)` in `export { foo as bar }`
  pub exported: Option<ModuleExportName<'a>>,
}

impl<'a> ExportNamedSpecifier<'a> {
  pub fn parent(&self) -> &'a NamedExport<'a> {
    self.parent.unwrap()
  }

  /// `type` in `export { type foo as bar }`
  pub fn is_type_only(&self) -> bool {
    self.inner.is_type_only
  }
}

impl<'a> SourceRanged for ExportNamedSpecifier<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ExportNamedSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportNamedSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportNamedSpecifier<'a>, &'a ExportNamedSpecifier<'a>>(node) };
    Node::ExportNamedSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for ExportNamedSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.exported { Some(_value) => 1, None => 0, });
    children.push((&self.orig).into());
    if let Some(child) = self.exported.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportNamedSpecifier
  }
}

impl<'a> CastableNode<'a> for ExportNamedSpecifier<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportNamedSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ExportNamedSpecifier
  }
}

fn get_view_for_export_named_specifier<'a>(inner: &'a swc_ast::ExportNamedSpecifier, bump: &'a Bump) -> &'a ExportNamedSpecifier<'a> {
  let node = bump.alloc(ExportNamedSpecifier {
    inner,
    parent: None,
    orig: get_view_for_module_export_name(&inner.orig, bump),
    exported: match &inner.exported {
      Some(value) => Some(get_view_for_module_export_name(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_module_export_name(&node.orig, parent);
  if let Some(value) = &node.exported {
    set_parent_for_module_export_name(value, parent)
  };
  node
}

fn set_parent_for_export_named_specifier<'a>(node: &ExportNamedSpecifier<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ExportNamedSpecifier<'a> as *mut ExportNamedSpecifier<'a>;
    (*node_ptr).parent.replace(parent.expect::<NamedExport>());
  }
}

/// `export * as foo from 'src';`
#[derive(Clone)]
pub struct ExportNamespaceSpecifier<'a> {
  parent: Option<&'a NamedExport<'a>>,
  pub inner: &'a swc_ast::ExportNamespaceSpecifier,
  pub name: ModuleExportName<'a>,
}

impl<'a> ExportNamespaceSpecifier<'a> {
  pub fn parent(&self) -> &'a NamedExport<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ExportNamespaceSpecifier<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ExportNamespaceSpecifier<'a>> for Node<'a> {
  fn from(node: &ExportNamespaceSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExportNamespaceSpecifier<'a>, &'a ExportNamespaceSpecifier<'a>>(node) };
    Node::ExportNamespaceSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for ExportNamespaceSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.name).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExportNamespaceSpecifier
  }
}

impl<'a> CastableNode<'a> for ExportNamespaceSpecifier<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExportNamespaceSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ExportNamespaceSpecifier
  }
}

fn get_view_for_export_namespace_specifier<'a>(inner: &'a swc_ast::ExportNamespaceSpecifier, bump: &'a Bump) -> &'a ExportNamespaceSpecifier<'a> {
  let node = bump.alloc(ExportNamespaceSpecifier {
    inner,
    parent: None,
    name: get_view_for_module_export_name(&inner.name, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_module_export_name(&node.name, parent);
  node
}

fn set_parent_for_export_namespace_specifier<'a>(node: &ExportNamespaceSpecifier<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ExportNamespaceSpecifier<'a> as *mut ExportNamespaceSpecifier<'a>;
    (*node_ptr).parent.replace(parent.expect::<NamedExport>());
  }
}

#[derive(Clone)]
pub struct ExprOrSpread<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ExprOrSpread,
  pub expr: Expr<'a>,
}

impl<'a> ExprOrSpread<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn spread(&self) -> &Option<swc_common::Span> {
    &self.inner.spread
  }
}

impl<'a> SourceRanged for ExprOrSpread<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ExprOrSpread<'a>> for Node<'a> {
  fn from(node: &ExprOrSpread<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExprOrSpread<'a>, &'a ExprOrSpread<'a>>(node) };
    Node::ExprOrSpread(node)
  }
}

impl<'a> NodeTrait<'a> for ExprOrSpread<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExprOrSpread
  }
}

impl<'a> CastableNode<'a> for ExprOrSpread<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExprOrSpread(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ExprOrSpread
  }
}

fn get_view_for_expr_or_spread<'a>(inner: &'a swc_ast::ExprOrSpread, bump: &'a Bump) -> &'a ExprOrSpread<'a> {
  let node = bump.alloc(ExprOrSpread {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_expr_or_spread<'a>(node: &ExprOrSpread<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ExprOrSpread<'a> as *mut ExprOrSpread<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ExprStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ExprStmt,
  pub expr: Expr<'a>,
}

impl<'a> ExprStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ExprStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ExprStmt<'a>> for Node<'a> {
  fn from(node: &ExprStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ExprStmt<'a>, &'a ExprStmt<'a>>(node) };
    Node::ExprStmt(node)
  }
}

impl<'a> NodeTrait<'a> for ExprStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ExprStmt
  }
}

impl<'a> CastableNode<'a> for ExprStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ExprStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ExprStmt
  }
}

fn get_view_for_expr_stmt<'a>(inner: &'a swc_ast::ExprStmt, bump: &'a Bump) -> &'a ExprStmt<'a> {
  let node = bump.alloc(ExprStmt {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_expr_stmt<'a>(node: &ExprStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ExprStmt<'a> as *mut ExprStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct FnDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::FnDecl,
  pub ident: &'a Ident<'a>,
  pub function: &'a Function<'a>,
}

impl<'a> FnDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl<'a> SourceRanged for FnDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&FnDecl<'a>> for Node<'a> {
  fn from(node: &FnDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&FnDecl<'a>, &'a FnDecl<'a>>(node) };
    Node::FnDecl(node)
  }
}

impl<'a> NodeTrait<'a> for FnDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.ident.into());
    children.push(self.function.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::FnDecl
  }
}

impl<'a> CastableNode<'a> for FnDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::FnDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::FnDecl
  }
}

fn get_view_for_fn_decl<'a>(inner: &'a swc_ast::FnDecl, bump: &'a Bump) -> &'a FnDecl<'a> {
  let node = bump.alloc(FnDecl {
    inner,
    parent: None,
    ident: get_view_for_ident(&inner.ident, bump),
    function: get_view_for_function(&inner.function, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.ident, parent);
  set_parent_for_function(&node.function, parent);
  node
}

fn set_parent_for_fn_decl<'a>(node: &FnDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const FnDecl<'a> as *mut FnDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// Function expression.
#[derive(Clone)]
pub struct FnExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::FnExpr,
  pub ident: Option<&'a Ident<'a>>,
  pub function: &'a Function<'a>,
}

impl<'a> FnExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for FnExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&FnExpr<'a>> for Node<'a> {
  fn from(node: &FnExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&FnExpr<'a>, &'a FnExpr<'a>>(node) };
    Node::FnExpr(node)
  }
}

impl<'a> NodeTrait<'a> for FnExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.ident { Some(_value) => 1, None => 0, });
    if let Some(child) = self.ident {
      children.push(child.into());
    }
    children.push(self.function.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::FnExpr
  }
}

impl<'a> CastableNode<'a> for FnExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::FnExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::FnExpr
  }
}

fn get_view_for_fn_expr<'a>(inner: &'a swc_ast::FnExpr, bump: &'a Bump) -> &'a FnExpr<'a> {
  let node = bump.alloc(FnExpr {
    inner,
    parent: None,
    ident: match &inner.ident {
      Some(value) => Some(get_view_for_ident(value, bump)),
      None => None,
    },
    function: get_view_for_function(&inner.function, bump),
  });
  let parent: Node<'a> = (&*node).into();
  if let Some(value) = &node.ident {
    set_parent_for_ident(value, parent)
  };
  set_parent_for_function(&node.function, parent);
  node
}

fn set_parent_for_fn_expr<'a>(node: &FnExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const FnExpr<'a> as *mut FnExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ForInStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ForInStmt,
  pub left: VarDeclOrPat<'a>,
  pub right: Expr<'a>,
  pub body: Stmt<'a>,
}

impl<'a> ForInStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ForInStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ForInStmt<'a>> for Node<'a> {
  fn from(node: &ForInStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ForInStmt<'a>, &'a ForInStmt<'a>>(node) };
    Node::ForInStmt(node)
  }
}

impl<'a> NodeTrait<'a> for ForInStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(3);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children.push((&self.body).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ForInStmt
  }
}

impl<'a> CastableNode<'a> for ForInStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ForInStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ForInStmt
  }
}

fn get_view_for_for_in_stmt<'a>(inner: &'a swc_ast::ForInStmt, bump: &'a Bump) -> &'a ForInStmt<'a> {
  let node = bump.alloc(ForInStmt {
    inner,
    parent: None,
    left: get_view_for_var_decl_or_pat(&inner.left, bump),
    right: get_view_for_expr(&inner.right, bump),
    body: get_view_for_stmt(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_var_decl_or_pat(&node.left, parent);
  set_parent_for_expr(&node.right, parent);
  set_parent_for_stmt(&node.body, parent);
  node
}

fn set_parent_for_for_in_stmt<'a>(node: &ForInStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ForInStmt<'a> as *mut ForInStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ForOfStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ForOfStmt,
  pub left: VarDeclOrPat<'a>,
  pub right: Expr<'a>,
  pub body: Stmt<'a>,
}

impl<'a> ForOfStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  /// Span of the await token.
  ///
  /// es2018
  ///
  /// for-await-of statements, e.g., `for await (const x of xs) {`
  pub fn await_token(&self) -> &Option<swc_common::Span> {
    &self.inner.await_token
  }
}

impl<'a> SourceRanged for ForOfStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ForOfStmt<'a>> for Node<'a> {
  fn from(node: &ForOfStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ForOfStmt<'a>, &'a ForOfStmt<'a>>(node) };
    Node::ForOfStmt(node)
  }
}

impl<'a> NodeTrait<'a> for ForOfStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(3);
    children.push((&self.left).into());
    children.push((&self.right).into());
    children.push((&self.body).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ForOfStmt
  }
}

impl<'a> CastableNode<'a> for ForOfStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ForOfStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ForOfStmt
  }
}

fn get_view_for_for_of_stmt<'a>(inner: &'a swc_ast::ForOfStmt, bump: &'a Bump) -> &'a ForOfStmt<'a> {
  let node = bump.alloc(ForOfStmt {
    inner,
    parent: None,
    left: get_view_for_var_decl_or_pat(&inner.left, bump),
    right: get_view_for_expr(&inner.right, bump),
    body: get_view_for_stmt(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_var_decl_or_pat(&node.left, parent);
  set_parent_for_expr(&node.right, parent);
  set_parent_for_stmt(&node.body, parent);
  node
}

fn set_parent_for_for_of_stmt<'a>(node: &ForOfStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ForOfStmt<'a> as *mut ForOfStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ForStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ForStmt,
  pub init: Option<VarDeclOrExpr<'a>>,
  pub test: Option<Expr<'a>>,
  pub update: Option<Expr<'a>>,
  pub body: Stmt<'a>,
}

impl<'a> ForStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ForStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ForStmt<'a>> for Node<'a> {
  fn from(node: &ForStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ForStmt<'a>, &'a ForStmt<'a>>(node) };
    Node::ForStmt(node)
  }
}

impl<'a> NodeTrait<'a> for ForStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ForStmt
  }
}

impl<'a> CastableNode<'a> for ForStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ForStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ForStmt
  }
}

fn get_view_for_for_stmt<'a>(inner: &'a swc_ast::ForStmt, bump: &'a Bump) -> &'a ForStmt<'a> {
  let node = bump.alloc(ForStmt {
    inner,
    parent: None,
    init: match &inner.init {
      Some(value) => Some(get_view_for_var_decl_or_expr(value, bump)),
      None => None,
    },
    test: match &inner.test {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
    update: match &inner.update {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
    body: get_view_for_stmt(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  if let Some(value) = &node.init {
    set_parent_for_var_decl_or_expr(value, parent)
  };
  if let Some(value) = &node.test {
    set_parent_for_expr(value, parent)
  };
  if let Some(value) = &node.update {
    set_parent_for_expr(value, parent)
  };
  set_parent_for_stmt(&node.body, parent);
  node
}

fn set_parent_for_for_stmt<'a>(node: &ForStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ForStmt<'a> as *mut ForStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// Common parts of function and method.
#[derive(Clone)]
pub struct Function<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Function,
  pub params: Vec<&'a Param<'a>>,
  pub decorators: Vec<&'a Decorator<'a>>,
  pub body: Option<&'a BlockStmt<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub return_type: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> Function<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  /// if it's a generator.
  pub fn is_generator(&self) -> bool {
    self.inner.is_generator
  }

  /// if it's an async function.
  pub fn is_async(&self) -> bool {
    self.inner.is_async
  }
}

impl<'a> SourceRanged for Function<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Function<'a>> for Node<'a> {
  fn from(node: &Function<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Function<'a>, &'a Function<'a>>(node) };
    Node::Function(node)
  }
}

impl<'a> NodeTrait<'a> for Function<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Function
  }
}

impl<'a> CastableNode<'a> for Function<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Function(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Function
  }
}

fn get_view_for_function<'a>(inner: &'a swc_ast::Function, bump: &'a Bump) -> &'a Function<'a> {
  let node = bump.alloc(Function {
    inner,
    parent: None,
    params: inner.params.iter().map(|value| get_view_for_param(value, bump)).collect(),
    decorators: inner.decorators.iter().map(|value| get_view_for_decorator(value, bump)).collect(),
    body: match &inner.body {
      Some(value) => Some(get_view_for_block_stmt(value, bump)),
      None => None,
    },
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
    return_type: match &inner.return_type {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.params.iter() {
    set_parent_for_param(value, parent)
  }
  for value in node.decorators.iter() {
    set_parent_for_decorator(value, parent)
  }
  if let Some(value) = &node.body {
    set_parent_for_block_stmt(value, parent)
  };
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  if let Some(value) = &node.return_type {
    set_parent_for_ts_type_ann(value, parent)
  };
  node
}

fn set_parent_for_function<'a>(node: &Function<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Function<'a> as *mut Function<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct GetterProp<'a> {
  parent: Option<&'a ObjectLit<'a>>,
  pub inner: &'a swc_ast::GetterProp,
  pub key: PropName<'a>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub body: Option<&'a BlockStmt<'a>>,
}

impl<'a> GetterProp<'a> {
  pub fn parent(&self) -> &'a ObjectLit<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for GetterProp<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&GetterProp<'a>> for Node<'a> {
  fn from(node: &GetterProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&GetterProp<'a>, &'a GetterProp<'a>>(node) };
    Node::GetterProp(node)
  }
}

impl<'a> NodeTrait<'a> for GetterProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::GetterProp
  }
}

impl<'a> CastableNode<'a> for GetterProp<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::GetterProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::GetterProp
  }
}

fn get_view_for_getter_prop<'a>(inner: &'a swc_ast::GetterProp, bump: &'a Bump) -> &'a GetterProp<'a> {
  let node = bump.alloc(GetterProp {
    inner,
    parent: None,
    key: get_view_for_prop_name(&inner.key, bump),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
    body: match &inner.body {
      Some(value) => Some(get_view_for_block_stmt(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_prop_name(&node.key, parent);
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  if let Some(value) = &node.body {
    set_parent_for_block_stmt(value, parent)
  };
  node
}

fn set_parent_for_getter_prop<'a>(node: &GetterProp<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const GetterProp<'a> as *mut GetterProp<'a>;
    (*node_ptr).parent.replace(parent.expect::<ObjectLit>());
  }
}

/// A complete identifier with span.
///
/// Identifier of swc consists of two parts. The first one is symbol, which is
/// stored using an interned string, [JsWord] . The second
/// one is [SyntaxContext][swc_common::SyntaxContext], which can be
/// used to distinguish identifier with same symbol.
///
/// Let me explain this with an example.
///
/// ```ts
/// let a = 5
/// {
///     let a = 3;
/// }
/// ```
/// In the code above, there are two variables with the symbol a.
///
///
/// Other compilers typically uses type like `Scope`, and store them nested, but
/// in rust, type like `Scope`  requires [Arc<Mutex<Scope>>] so swc uses
/// different approach. Instead of passing scopes, swc annotates two variables
/// with different tag, which is named
/// [SyntaxContext]. The notation for the syntax
/// context is #n where n is a number. e.g. `foo#1`
///
/// For the example above, after applying resolver pass, it becomes.
///
/// ```ts
/// let a#1 = 5
/// {
///     let a#2 = 3;
/// }
/// ```
///
/// Thanks to the `tag` we attached, we can now distinguish them.
///
/// ([JsWord], [SyntaxContext])
///
/// See [Id], which is a type alias for this.
///
/// This can be used to store all variables in a module to single hash map.
///
/// # Comparison
///
/// While comparing two identifiers, you can use `.to_id()`.
///
/// # HashMap
///
/// There's a type named [Id] which only contains minimal information to
/// distinguish identifiers.
#[derive(Clone)]
pub struct Ident<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Ident,
}

impl<'a> Ident<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn sym(&self) -> &swc_atoms::JsWord {
    &self.inner.sym
  }

  /// TypeScript only. Used in case of an optional parameter.
  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl<'a> SourceRanged for Ident<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Ident<'a>> for Node<'a> {
  fn from(node: &Ident<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Ident<'a>, &'a Ident<'a>>(node) };
    Node::Ident(node)
  }
}

impl<'a> NodeTrait<'a> for Ident<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Ident
  }
}

impl<'a> CastableNode<'a> for Ident<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Ident(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Ident
  }
}

fn get_view_for_ident<'a>(inner: &'a swc_ast::Ident, bump: &'a Bump) -> &'a Ident<'a> {
  let node = bump.alloc(Ident {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_ident<'a>(node: &Ident<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Ident<'a> as *mut Ident<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct IfStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::IfStmt,
  pub test: Expr<'a>,
  pub cons: Stmt<'a>,
  pub alt: Option<Stmt<'a>>,
}

impl<'a> IfStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for IfStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&IfStmt<'a>> for Node<'a> {
  fn from(node: &IfStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&IfStmt<'a>, &'a IfStmt<'a>>(node) };
    Node::IfStmt(node)
  }
}

impl<'a> NodeTrait<'a> for IfStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::IfStmt
  }
}

impl<'a> CastableNode<'a> for IfStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::IfStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::IfStmt
  }
}

fn get_view_for_if_stmt<'a>(inner: &'a swc_ast::IfStmt, bump: &'a Bump) -> &'a IfStmt<'a> {
  let node = bump.alloc(IfStmt {
    inner,
    parent: None,
    test: get_view_for_expr(&inner.test, bump),
    cons: get_view_for_stmt(&inner.cons, bump),
    alt: match &inner.alt {
      Some(value) => Some(get_view_for_stmt(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.test, parent);
  set_parent_for_stmt(&node.cons, parent);
  if let Some(value) = &node.alt {
    set_parent_for_stmt(value, parent)
  };
  node
}

fn set_parent_for_if_stmt<'a>(node: &IfStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const IfStmt<'a> as *mut IfStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct Import<'a> {
  parent: Option<&'a CallExpr<'a>>,
  pub inner: &'a swc_ast::Import,
}

impl<'a> Import<'a> {
  pub fn parent(&self) -> &'a CallExpr<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for Import<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Import<'a>> for Node<'a> {
  fn from(node: &Import<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Import<'a>, &'a Import<'a>>(node) };
    Node::Import(node)
  }
}

impl<'a> NodeTrait<'a> for Import<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Import
  }
}

impl<'a> CastableNode<'a> for Import<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Import(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Import
  }
}

fn get_view_for_import<'a>(inner: &'a swc_ast::Import, bump: &'a Bump) -> &'a Import<'a> {
  let node = bump.alloc(Import {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_import<'a>(node: &Import<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Import<'a> as *mut Import<'a>;
    (*node_ptr).parent.replace(parent.expect::<CallExpr>());
  }
}

#[derive(Clone)]
pub struct ImportDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ImportDecl,
  pub specifiers: Vec<ImportSpecifier<'a>>,
  pub src: &'a Str<'a>,
  pub asserts: Option<&'a ObjectLit<'a>>,
}

impl<'a> ImportDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn type_only(&self) -> bool {
    self.inner.type_only
  }
}

impl<'a> SourceRanged for ImportDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ImportDecl<'a>> for Node<'a> {
  fn from(node: &ImportDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ImportDecl<'a>, &'a ImportDecl<'a>>(node) };
    Node::ImportDecl(node)
  }
}

impl<'a> NodeTrait<'a> for ImportDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ImportDecl
  }
}

impl<'a> CastableNode<'a> for ImportDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ImportDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ImportDecl
  }
}

fn get_view_for_import_decl<'a>(inner: &'a swc_ast::ImportDecl, bump: &'a Bump) -> &'a ImportDecl<'a> {
  let node = bump.alloc(ImportDecl {
    inner,
    parent: None,
    specifiers: inner.specifiers.iter().map(|value| get_view_for_import_specifier(value, bump)).collect(),
    src: get_view_for_str(&inner.src, bump),
    asserts: match &inner.asserts {
      Some(value) => Some(get_view_for_object_lit(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.specifiers.iter() {
    set_parent_for_import_specifier(value, parent)
  }
  set_parent_for_str(&node.src, parent);
  if let Some(value) = &node.asserts {
    set_parent_for_object_lit(value, parent)
  };
  node
}

fn set_parent_for_import_decl<'a>(node: &ImportDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ImportDecl<'a> as *mut ImportDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// e.g. `import foo from 'mod.js'`
#[derive(Clone)]
pub struct ImportDefaultSpecifier<'a> {
  parent: Option<&'a ImportDecl<'a>>,
  pub inner: &'a swc_ast::ImportDefaultSpecifier,
  pub local: &'a Ident<'a>,
}

impl<'a> ImportDefaultSpecifier<'a> {
  pub fn parent(&self) -> &'a ImportDecl<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ImportDefaultSpecifier<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ImportDefaultSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportDefaultSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ImportDefaultSpecifier<'a>, &'a ImportDefaultSpecifier<'a>>(node) };
    Node::ImportDefaultSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for ImportDefaultSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.local.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ImportDefaultSpecifier
  }
}

impl<'a> CastableNode<'a> for ImportDefaultSpecifier<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ImportDefaultSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ImportDefaultSpecifier
  }
}

fn get_view_for_import_default_specifier<'a>(inner: &'a swc_ast::ImportDefaultSpecifier, bump: &'a Bump) -> &'a ImportDefaultSpecifier<'a> {
  let node = bump.alloc(ImportDefaultSpecifier {
    inner,
    parent: None,
    local: get_view_for_ident(&inner.local, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.local, parent);
  node
}

fn set_parent_for_import_default_specifier<'a>(node: &ImportDefaultSpecifier<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ImportDefaultSpecifier<'a> as *mut ImportDefaultSpecifier<'a>;
    (*node_ptr).parent.replace(parent.expect::<ImportDecl>());
  }
}

/// e.g. local = foo, imported = None `import { foo } from 'mod.js'`
/// e.g. local = bar, imported = Some(foo) for `import { foo as bar } from
/// 'mod.js'`
#[derive(Clone)]
pub struct ImportNamedSpecifier<'a> {
  parent: Option<&'a ImportDecl<'a>>,
  pub inner: &'a swc_ast::ImportNamedSpecifier,
  pub local: &'a Ident<'a>,
  pub imported: Option<ModuleExportName<'a>>,
}

impl<'a> ImportNamedSpecifier<'a> {
  pub fn parent(&self) -> &'a ImportDecl<'a> {
    self.parent.unwrap()
  }

  pub fn is_type_only(&self) -> bool {
    self.inner.is_type_only
  }
}

impl<'a> SourceRanged for ImportNamedSpecifier<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ImportNamedSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportNamedSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ImportNamedSpecifier<'a>, &'a ImportNamedSpecifier<'a>>(node) };
    Node::ImportNamedSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for ImportNamedSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.imported { Some(_value) => 1, None => 0, });
    children.push(self.local.into());
    if let Some(child) = self.imported.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ImportNamedSpecifier
  }
}

impl<'a> CastableNode<'a> for ImportNamedSpecifier<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ImportNamedSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ImportNamedSpecifier
  }
}

fn get_view_for_import_named_specifier<'a>(inner: &'a swc_ast::ImportNamedSpecifier, bump: &'a Bump) -> &'a ImportNamedSpecifier<'a> {
  let node = bump.alloc(ImportNamedSpecifier {
    inner,
    parent: None,
    local: get_view_for_ident(&inner.local, bump),
    imported: match &inner.imported {
      Some(value) => Some(get_view_for_module_export_name(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.local, parent);
  if let Some(value) = &node.imported {
    set_parent_for_module_export_name(value, parent)
  };
  node
}

fn set_parent_for_import_named_specifier<'a>(node: &ImportNamedSpecifier<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ImportNamedSpecifier<'a> as *mut ImportNamedSpecifier<'a>;
    (*node_ptr).parent.replace(parent.expect::<ImportDecl>());
  }
}

/// e.g. `import * as foo from 'mod.js'`.
#[derive(Clone)]
pub struct ImportStarAsSpecifier<'a> {
  parent: Option<&'a ImportDecl<'a>>,
  pub inner: &'a swc_ast::ImportStarAsSpecifier,
  pub local: &'a Ident<'a>,
}

impl<'a> ImportStarAsSpecifier<'a> {
  pub fn parent(&self) -> &'a ImportDecl<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ImportStarAsSpecifier<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ImportStarAsSpecifier<'a>> for Node<'a> {
  fn from(node: &ImportStarAsSpecifier<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ImportStarAsSpecifier<'a>, &'a ImportStarAsSpecifier<'a>>(node) };
    Node::ImportStarAsSpecifier(node)
  }
}

impl<'a> NodeTrait<'a> for ImportStarAsSpecifier<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.local.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ImportStarAsSpecifier
  }
}

impl<'a> CastableNode<'a> for ImportStarAsSpecifier<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ImportStarAsSpecifier(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ImportStarAsSpecifier
  }
}

fn get_view_for_import_star_as_specifier<'a>(inner: &'a swc_ast::ImportStarAsSpecifier, bump: &'a Bump) -> &'a ImportStarAsSpecifier<'a> {
  let node = bump.alloc(ImportStarAsSpecifier {
    inner,
    parent: None,
    local: get_view_for_ident(&inner.local, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.local, parent);
  node
}

fn set_parent_for_import_star_as_specifier<'a>(node: &ImportStarAsSpecifier<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ImportStarAsSpecifier<'a> as *mut ImportStarAsSpecifier<'a>;
    (*node_ptr).parent.replace(parent.expect::<ImportDecl>());
  }
}

/// Represents a invalid node.
#[derive(Clone)]
pub struct Invalid<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Invalid,
}

impl<'a> Invalid<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for Invalid<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Invalid<'a>> for Node<'a> {
  fn from(node: &Invalid<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Invalid<'a>, &'a Invalid<'a>>(node) };
    Node::Invalid(node)
  }
}

impl<'a> NodeTrait<'a> for Invalid<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Invalid
  }
}

impl<'a> CastableNode<'a> for Invalid<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Invalid(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Invalid
  }
}

fn get_view_for_invalid<'a>(inner: &'a swc_ast::Invalid, bump: &'a Bump) -> &'a Invalid<'a> {
  let node = bump.alloc(Invalid {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_invalid<'a>(node: &Invalid<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Invalid<'a> as *mut Invalid<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct JSXAttr<'a> {
  parent: Option<&'a JSXOpeningElement<'a>>,
  pub inner: &'a swc_ast::JSXAttr,
  pub name: JSXAttrName<'a>,
  /// Babel uses Expr instead of JSXAttrValue
  pub value: Option<JSXAttrValue<'a>>,
}

impl<'a> JSXAttr<'a> {
  pub fn parent(&self) -> &'a JSXOpeningElement<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXAttr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXAttr<'a>> for Node<'a> {
  fn from(node: &JSXAttr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXAttr<'a>, &'a JSXAttr<'a>>(node) };
    Node::JSXAttr(node)
  }
}

impl<'a> NodeTrait<'a> for JSXAttr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.value { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    if let Some(child) = self.value.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXAttr
  }
}

impl<'a> CastableNode<'a> for JSXAttr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXAttr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXAttr
  }
}

fn get_view_for_jsxattr<'a>(inner: &'a swc_ast::JSXAttr, bump: &'a Bump) -> &'a JSXAttr<'a> {
  let node = bump.alloc(JSXAttr {
    inner,
    parent: None,
    name: get_view_for_jsxattr_name(&inner.name, bump),
    value: match &inner.value {
      Some(value) => Some(get_view_for_jsxattr_value(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_jsxattr_name(&node.name, parent);
  if let Some(value) = &node.value {
    set_parent_for_jsxattr_value(value, parent)
  };
  node
}

fn set_parent_for_jsxattr<'a>(node: &JSXAttr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXAttr<'a> as *mut JSXAttr<'a>;
    (*node_ptr).parent.replace(parent.expect::<JSXOpeningElement>());
  }
}

#[derive(Clone)]
pub struct JSXClosingElement<'a> {
  parent: Option<&'a JSXElement<'a>>,
  pub inner: &'a swc_ast::JSXClosingElement,
  pub name: JSXElementName<'a>,
}

impl<'a> JSXClosingElement<'a> {
  pub fn parent(&self) -> &'a JSXElement<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXClosingElement<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXClosingElement<'a>> for Node<'a> {
  fn from(node: &JSXClosingElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXClosingElement<'a>, &'a JSXClosingElement<'a>>(node) };
    Node::JSXClosingElement(node)
  }
}

impl<'a> NodeTrait<'a> for JSXClosingElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.name).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXClosingElement
  }
}

impl<'a> CastableNode<'a> for JSXClosingElement<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXClosingElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXClosingElement
  }
}

fn get_view_for_jsxclosing_element<'a>(inner: &'a swc_ast::JSXClosingElement, bump: &'a Bump) -> &'a JSXClosingElement<'a> {
  let node = bump.alloc(JSXClosingElement {
    inner,
    parent: None,
    name: get_view_for_jsxelement_name(&inner.name, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_jsxelement_name(&node.name, parent);
  node
}

fn set_parent_for_jsxclosing_element<'a>(node: &JSXClosingElement<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXClosingElement<'a> as *mut JSXClosingElement<'a>;
    (*node_ptr).parent.replace(parent.expect::<JSXElement>());
  }
}

#[derive(Clone)]
pub struct JSXClosingFragment<'a> {
  parent: Option<&'a JSXFragment<'a>>,
  pub inner: &'a swc_ast::JSXClosingFragment,
}

impl<'a> JSXClosingFragment<'a> {
  pub fn parent(&self) -> &'a JSXFragment<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXClosingFragment<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXClosingFragment<'a>> for Node<'a> {
  fn from(node: &JSXClosingFragment<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXClosingFragment<'a>, &'a JSXClosingFragment<'a>>(node) };
    Node::JSXClosingFragment(node)
  }
}

impl<'a> NodeTrait<'a> for JSXClosingFragment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXClosingFragment
  }
}

impl<'a> CastableNode<'a> for JSXClosingFragment<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXClosingFragment(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXClosingFragment
  }
}

fn get_view_for_jsxclosing_fragment<'a>(inner: &'a swc_ast::JSXClosingFragment, bump: &'a Bump) -> &'a JSXClosingFragment<'a> {
  let node = bump.alloc(JSXClosingFragment {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_jsxclosing_fragment<'a>(node: &JSXClosingFragment<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXClosingFragment<'a> as *mut JSXClosingFragment<'a>;
    (*node_ptr).parent.replace(parent.expect::<JSXFragment>());
  }
}

#[derive(Clone)]
pub struct JSXElement<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::JSXElement,
  pub opening: &'a JSXOpeningElement<'a>,
  pub children: Vec<JSXElementChild<'a>>,
  pub closing: Option<&'a JSXClosingElement<'a>>,
}

impl<'a> JSXElement<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXElement<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXElement<'a>> for Node<'a> {
  fn from(node: &JSXElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXElement<'a>, &'a JSXElement<'a>>(node) };
    Node::JSXElement(node)
  }
}

impl<'a> NodeTrait<'a> for JSXElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXElement
  }
}

impl<'a> CastableNode<'a> for JSXElement<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXElement
  }
}

fn get_view_for_jsxelement<'a>(inner: &'a swc_ast::JSXElement, bump: &'a Bump) -> &'a JSXElement<'a> {
  let node = bump.alloc(JSXElement {
    inner,
    parent: None,
    opening: get_view_for_jsxopening_element(&inner.opening, bump),
    children: inner.children.iter().map(|value| get_view_for_jsxelement_child(value, bump)).collect(),
    closing: match &inner.closing {
      Some(value) => Some(get_view_for_jsxclosing_element(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_jsxopening_element(&node.opening, parent);
  for value in node.children.iter() {
    set_parent_for_jsxelement_child(value, parent)
  }
  if let Some(value) = &node.closing {
    set_parent_for_jsxclosing_element(value, parent)
  };
  node
}

fn set_parent_for_jsxelement<'a>(node: &JSXElement<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXElement<'a> as *mut JSXElement<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct JSXEmptyExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::JSXEmptyExpr,
}

impl<'a> JSXEmptyExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXEmptyExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXEmptyExpr<'a>> for Node<'a> {
  fn from(node: &JSXEmptyExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXEmptyExpr<'a>, &'a JSXEmptyExpr<'a>>(node) };
    Node::JSXEmptyExpr(node)
  }
}

impl<'a> NodeTrait<'a> for JSXEmptyExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXEmptyExpr
  }
}

impl<'a> CastableNode<'a> for JSXEmptyExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXEmptyExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXEmptyExpr
  }
}

fn get_view_for_jsxempty_expr<'a>(inner: &'a swc_ast::JSXEmptyExpr, bump: &'a Bump) -> &'a JSXEmptyExpr<'a> {
  let node = bump.alloc(JSXEmptyExpr {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_jsxempty_expr<'a>(node: &JSXEmptyExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXEmptyExpr<'a> as *mut JSXEmptyExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct JSXExprContainer<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::JSXExprContainer,
  pub expr: JSXExpr<'a>,
}

impl<'a> JSXExprContainer<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXExprContainer<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXExprContainer<'a>> for Node<'a> {
  fn from(node: &JSXExprContainer<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXExprContainer<'a>, &'a JSXExprContainer<'a>>(node) };
    Node::JSXExprContainer(node)
  }
}

impl<'a> NodeTrait<'a> for JSXExprContainer<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXExprContainer
  }
}

impl<'a> CastableNode<'a> for JSXExprContainer<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXExprContainer(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXExprContainer
  }
}

fn get_view_for_jsxexpr_container<'a>(inner: &'a swc_ast::JSXExprContainer, bump: &'a Bump) -> &'a JSXExprContainer<'a> {
  let node = bump.alloc(JSXExprContainer {
    inner,
    parent: None,
    expr: get_view_for_jsxexpr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_jsxexpr(&node.expr, parent);
  node
}

fn set_parent_for_jsxexpr_container<'a>(node: &JSXExprContainer<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXExprContainer<'a> as *mut JSXExprContainer<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct JSXFragment<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::JSXFragment,
  pub opening: &'a JSXOpeningFragment<'a>,
  pub children: Vec<JSXElementChild<'a>>,
  pub closing: &'a JSXClosingFragment<'a>,
}

impl<'a> JSXFragment<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXFragment<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXFragment<'a>> for Node<'a> {
  fn from(node: &JSXFragment<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXFragment<'a>, &'a JSXFragment<'a>>(node) };
    Node::JSXFragment(node)
  }
}

impl<'a> NodeTrait<'a> for JSXFragment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXFragment
  }
}

impl<'a> CastableNode<'a> for JSXFragment<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXFragment(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXFragment
  }
}

fn get_view_for_jsxfragment<'a>(inner: &'a swc_ast::JSXFragment, bump: &'a Bump) -> &'a JSXFragment<'a> {
  let node = bump.alloc(JSXFragment {
    inner,
    parent: None,
    opening: get_view_for_jsxopening_fragment(&inner.opening, bump),
    children: inner.children.iter().map(|value| get_view_for_jsxelement_child(value, bump)).collect(),
    closing: get_view_for_jsxclosing_fragment(&inner.closing, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_jsxopening_fragment(&node.opening, parent);
  for value in node.children.iter() {
    set_parent_for_jsxelement_child(value, parent)
  }
  set_parent_for_jsxclosing_fragment(&node.closing, parent);
  node
}

fn set_parent_for_jsxfragment<'a>(node: &JSXFragment<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXFragment<'a> as *mut JSXFragment<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct JSXMemberExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::JSXMemberExpr,
  pub obj: JSXObject<'a>,
  pub prop: &'a Ident<'a>,
}

impl<'a> JSXMemberExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXMemberExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXMemberExpr<'a>> for Node<'a> {
  fn from(node: &JSXMemberExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXMemberExpr<'a>, &'a JSXMemberExpr<'a>>(node) };
    Node::JSXMemberExpr(node)
  }
}

impl<'a> NodeTrait<'a> for JSXMemberExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push(self.prop.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXMemberExpr
  }
}

impl<'a> CastableNode<'a> for JSXMemberExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXMemberExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXMemberExpr
  }
}

fn get_view_for_jsxmember_expr<'a>(inner: &'a swc_ast::JSXMemberExpr, bump: &'a Bump) -> &'a JSXMemberExpr<'a> {
  let node = bump.alloc(JSXMemberExpr {
    inner,
    parent: None,
    obj: get_view_for_jsxobject(&inner.obj, bump),
    prop: get_view_for_ident(&inner.prop, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_jsxobject(&node.obj, parent);
  set_parent_for_ident(&node.prop, parent);
  node
}

fn set_parent_for_jsxmember_expr<'a>(node: &JSXMemberExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXMemberExpr<'a> as *mut JSXMemberExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// XML-based namespace syntax:
#[derive(Clone)]
pub struct JSXNamespacedName<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::JSXNamespacedName,
  pub ns: &'a Ident<'a>,
  pub name: &'a Ident<'a>,
}

impl<'a> JSXNamespacedName<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXNamespacedName<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXNamespacedName<'a>> for Node<'a> {
  fn from(node: &JSXNamespacedName<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXNamespacedName<'a>, &'a JSXNamespacedName<'a>>(node) };
    Node::JSXNamespacedName(node)
  }
}

impl<'a> NodeTrait<'a> for JSXNamespacedName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.ns.into());
    children.push(self.name.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXNamespacedName
  }
}

impl<'a> CastableNode<'a> for JSXNamespacedName<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXNamespacedName(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXNamespacedName
  }
}

fn get_view_for_jsxnamespaced_name<'a>(inner: &'a swc_ast::JSXNamespacedName, bump: &'a Bump) -> &'a JSXNamespacedName<'a> {
  let node = bump.alloc(JSXNamespacedName {
    inner,
    parent: None,
    ns: get_view_for_ident(&inner.ns, bump),
    name: get_view_for_ident(&inner.name, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.ns, parent);
  set_parent_for_ident(&node.name, parent);
  node
}

fn set_parent_for_jsxnamespaced_name<'a>(node: &JSXNamespacedName<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXNamespacedName<'a> as *mut JSXNamespacedName<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct JSXOpeningElement<'a> {
  parent: Option<&'a JSXElement<'a>>,
  pub inner: &'a swc_ast::JSXOpeningElement,
  pub name: JSXElementName<'a>,
  pub attrs: Vec<JSXAttrOrSpread<'a>>,
  /// Note: This field's name is different from one from babel because it is
  /// misleading
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> JSXOpeningElement<'a> {
  pub fn parent(&self) -> &'a JSXElement<'a> {
    self.parent.unwrap()
  }

  pub fn self_closing(&self) -> bool {
    self.inner.self_closing
  }
}

impl<'a> SourceRanged for JSXOpeningElement<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXOpeningElement<'a>> for Node<'a> {
  fn from(node: &JSXOpeningElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXOpeningElement<'a>, &'a JSXOpeningElement<'a>>(node) };
    Node::JSXOpeningElement(node)
  }
}

impl<'a> NodeTrait<'a> for JSXOpeningElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXOpeningElement
  }
}

impl<'a> CastableNode<'a> for JSXOpeningElement<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXOpeningElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXOpeningElement
  }
}

fn get_view_for_jsxopening_element<'a>(inner: &'a swc_ast::JSXOpeningElement, bump: &'a Bump) -> &'a JSXOpeningElement<'a> {
  let node = bump.alloc(JSXOpeningElement {
    inner,
    parent: None,
    name: get_view_for_jsxelement_name(&inner.name, bump),
    attrs: inner.attrs.iter().map(|value| get_view_for_jsxattr_or_spread(value, bump)).collect(),
    type_args: match &inner.type_args {
      Some(value) => Some(get_view_for_ts_type_param_instantiation(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_jsxelement_name(&node.name, parent);
  for value in node.attrs.iter() {
    set_parent_for_jsxattr_or_spread(value, parent)
  }
  if let Some(value) = &node.type_args {
    set_parent_for_ts_type_param_instantiation(value, parent)
  };
  node
}

fn set_parent_for_jsxopening_element<'a>(node: &JSXOpeningElement<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXOpeningElement<'a> as *mut JSXOpeningElement<'a>;
    (*node_ptr).parent.replace(parent.expect::<JSXElement>());
  }
}

#[derive(Clone)]
pub struct JSXOpeningFragment<'a> {
  parent: Option<&'a JSXFragment<'a>>,
  pub inner: &'a swc_ast::JSXOpeningFragment,
}

impl<'a> JSXOpeningFragment<'a> {
  pub fn parent(&self) -> &'a JSXFragment<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXOpeningFragment<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXOpeningFragment<'a>> for Node<'a> {
  fn from(node: &JSXOpeningFragment<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXOpeningFragment<'a>, &'a JSXOpeningFragment<'a>>(node) };
    Node::JSXOpeningFragment(node)
  }
}

impl<'a> NodeTrait<'a> for JSXOpeningFragment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXOpeningFragment
  }
}

impl<'a> CastableNode<'a> for JSXOpeningFragment<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXOpeningFragment(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXOpeningFragment
  }
}

fn get_view_for_jsxopening_fragment<'a>(inner: &'a swc_ast::JSXOpeningFragment, bump: &'a Bump) -> &'a JSXOpeningFragment<'a> {
  let node = bump.alloc(JSXOpeningFragment {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_jsxopening_fragment<'a>(node: &JSXOpeningFragment<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXOpeningFragment<'a> as *mut JSXOpeningFragment<'a>;
    (*node_ptr).parent.replace(parent.expect::<JSXFragment>());
  }
}

#[derive(Clone)]
pub struct JSXSpreadChild<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::JSXSpreadChild,
  pub expr: Expr<'a>,
}

impl<'a> JSXSpreadChild<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for JSXSpreadChild<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXSpreadChild<'a>> for Node<'a> {
  fn from(node: &JSXSpreadChild<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXSpreadChild<'a>, &'a JSXSpreadChild<'a>>(node) };
    Node::JSXSpreadChild(node)
  }
}

impl<'a> NodeTrait<'a> for JSXSpreadChild<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXSpreadChild
  }
}

impl<'a> CastableNode<'a> for JSXSpreadChild<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXSpreadChild(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXSpreadChild
  }
}

fn get_view_for_jsxspread_child<'a>(inner: &'a swc_ast::JSXSpreadChild, bump: &'a Bump) -> &'a JSXSpreadChild<'a> {
  let node = bump.alloc(JSXSpreadChild {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_jsxspread_child<'a>(node: &JSXSpreadChild<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXSpreadChild<'a> as *mut JSXSpreadChild<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct JSXText<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::JSXText,
}

impl<'a> JSXText<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn value(&self) -> &swc_atoms::JsWord {
    &self.inner.value
  }

  pub fn raw(&self) -> &swc_atoms::JsWord {
    &self.inner.raw
  }
}

impl<'a> SourceRanged for JSXText<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&JSXText<'a>> for Node<'a> {
  fn from(node: &JSXText<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&JSXText<'a>, &'a JSXText<'a>>(node) };
    Node::JSXText(node)
  }
}

impl<'a> NodeTrait<'a> for JSXText<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::JSXText
  }
}

impl<'a> CastableNode<'a> for JSXText<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::JSXText(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::JSXText
  }
}

fn get_view_for_jsxtext<'a>(inner: &'a swc_ast::JSXText, bump: &'a Bump) -> &'a JSXText<'a> {
  let node = bump.alloc(JSXText {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_jsxtext<'a>(node: &JSXText<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const JSXText<'a> as *mut JSXText<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// `{key: value}`
#[derive(Clone)]
pub struct KeyValuePatProp<'a> {
  parent: Option<&'a ObjectPat<'a>>,
  pub inner: &'a swc_ast::KeyValuePatProp,
  pub key: PropName<'a>,
  pub value: Pat<'a>,
}

impl<'a> KeyValuePatProp<'a> {
  pub fn parent(&self) -> &'a ObjectPat<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for KeyValuePatProp<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&KeyValuePatProp<'a>> for Node<'a> {
  fn from(node: &KeyValuePatProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&KeyValuePatProp<'a>, &'a KeyValuePatProp<'a>>(node) };
    Node::KeyValuePatProp(node)
  }
}

impl<'a> NodeTrait<'a> for KeyValuePatProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.value).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::KeyValuePatProp
  }
}

impl<'a> CastableNode<'a> for KeyValuePatProp<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::KeyValuePatProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::KeyValuePatProp
  }
}

fn get_view_for_key_value_pat_prop<'a>(inner: &'a swc_ast::KeyValuePatProp, bump: &'a Bump) -> &'a KeyValuePatProp<'a> {
  let node = bump.alloc(KeyValuePatProp {
    inner,
    parent: None,
    key: get_view_for_prop_name(&inner.key, bump),
    value: get_view_for_pat(&inner.value, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_prop_name(&node.key, parent);
  set_parent_for_pat(&node.value, parent);
  node
}

fn set_parent_for_key_value_pat_prop<'a>(node: &KeyValuePatProp<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const KeyValuePatProp<'a> as *mut KeyValuePatProp<'a>;
    (*node_ptr).parent.replace(parent.expect::<ObjectPat>());
  }
}

#[derive(Clone)]
pub struct KeyValueProp<'a> {
  parent: Option<&'a ObjectLit<'a>>,
  pub inner: &'a swc_ast::KeyValueProp,
  pub key: PropName<'a>,
  pub value: Expr<'a>,
}

impl<'a> KeyValueProp<'a> {
  pub fn parent(&self) -> &'a ObjectLit<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for KeyValueProp<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&KeyValueProp<'a>> for Node<'a> {
  fn from(node: &KeyValueProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&KeyValueProp<'a>, &'a KeyValueProp<'a>>(node) };
    Node::KeyValueProp(node)
  }
}

impl<'a> NodeTrait<'a> for KeyValueProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.value).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::KeyValueProp
  }
}

impl<'a> CastableNode<'a> for KeyValueProp<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::KeyValueProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::KeyValueProp
  }
}

fn get_view_for_key_value_prop<'a>(inner: &'a swc_ast::KeyValueProp, bump: &'a Bump) -> &'a KeyValueProp<'a> {
  let node = bump.alloc(KeyValueProp {
    inner,
    parent: None,
    key: get_view_for_prop_name(&inner.key, bump),
    value: get_view_for_expr(&inner.value, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_prop_name(&node.key, parent);
  set_parent_for_expr(&node.value, parent);
  node
}

fn set_parent_for_key_value_prop<'a>(node: &KeyValueProp<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const KeyValueProp<'a> as *mut KeyValueProp<'a>;
    (*node_ptr).parent.replace(parent.expect::<ObjectLit>());
  }
}

#[derive(Clone)]
pub struct LabeledStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::LabeledStmt,
  pub label: &'a Ident<'a>,
  pub body: Stmt<'a>,
}

impl<'a> LabeledStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for LabeledStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&LabeledStmt<'a>> for Node<'a> {
  fn from(node: &LabeledStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&LabeledStmt<'a>, &'a LabeledStmt<'a>>(node) };
    Node::LabeledStmt(node)
  }
}

impl<'a> NodeTrait<'a> for LabeledStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.label.into());
    children.push((&self.body).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::LabeledStmt
  }
}

impl<'a> CastableNode<'a> for LabeledStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::LabeledStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::LabeledStmt
  }
}

fn get_view_for_labeled_stmt<'a>(inner: &'a swc_ast::LabeledStmt, bump: &'a Bump) -> &'a LabeledStmt<'a> {
  let node = bump.alloc(LabeledStmt {
    inner,
    parent: None,
    label: get_view_for_ident(&inner.label, bump),
    body: get_view_for_stmt(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.label, parent);
  set_parent_for_stmt(&node.body, parent);
  node
}

fn set_parent_for_labeled_stmt<'a>(node: &LabeledStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const LabeledStmt<'a> as *mut LabeledStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct MemberExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::MemberExpr,
  pub obj: Expr<'a>,
  pub prop: MemberProp<'a>,
}

impl<'a> MemberExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for MemberExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&MemberExpr<'a>> for Node<'a> {
  fn from(node: &MemberExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&MemberExpr<'a>, &'a MemberExpr<'a>>(node) };
    Node::MemberExpr(node)
  }
}

impl<'a> NodeTrait<'a> for MemberExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push((&self.prop).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::MemberExpr
  }
}

impl<'a> CastableNode<'a> for MemberExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::MemberExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::MemberExpr
  }
}

fn get_view_for_member_expr<'a>(inner: &'a swc_ast::MemberExpr, bump: &'a Bump) -> &'a MemberExpr<'a> {
  let node = bump.alloc(MemberExpr {
    inner,
    parent: None,
    obj: get_view_for_expr(&inner.obj, bump),
    prop: get_view_for_member_prop(&inner.prop, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.obj, parent);
  set_parent_for_member_prop(&node.prop, parent);
  node
}

fn set_parent_for_member_expr<'a>(node: &MemberExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const MemberExpr<'a> as *mut MemberExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct MetaPropExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::MetaPropExpr,
}

impl<'a> MetaPropExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn prop_kind(&self) -> MetaPropKind {
    self.inner.kind
  }
}

impl<'a> SourceRanged for MetaPropExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&MetaPropExpr<'a>> for Node<'a> {
  fn from(node: &MetaPropExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&MetaPropExpr<'a>, &'a MetaPropExpr<'a>>(node) };
    Node::MetaPropExpr(node)
  }
}

impl<'a> NodeTrait<'a> for MetaPropExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::MetaPropExpr
  }
}

impl<'a> CastableNode<'a> for MetaPropExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::MetaPropExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::MetaPropExpr
  }
}

fn get_view_for_meta_prop_expr<'a>(inner: &'a swc_ast::MetaPropExpr, bump: &'a Bump) -> &'a MetaPropExpr<'a> {
  let node = bump.alloc(MetaPropExpr {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_meta_prop_expr<'a>(node: &MetaPropExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const MetaPropExpr<'a> as *mut MetaPropExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct MethodProp<'a> {
  parent: Option<&'a ObjectLit<'a>>,
  pub inner: &'a swc_ast::MethodProp,
  pub key: PropName<'a>,
  pub function: &'a Function<'a>,
}

impl<'a> MethodProp<'a> {
  pub fn parent(&self) -> &'a ObjectLit<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for MethodProp<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&MethodProp<'a>> for Node<'a> {
  fn from(node: &MethodProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&MethodProp<'a>, &'a MethodProp<'a>>(node) };
    Node::MethodProp(node)
  }
}

impl<'a> NodeTrait<'a> for MethodProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push(self.function.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::MethodProp
  }
}

impl<'a> CastableNode<'a> for MethodProp<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::MethodProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::MethodProp
  }
}

fn get_view_for_method_prop<'a>(inner: &'a swc_ast::MethodProp, bump: &'a Bump) -> &'a MethodProp<'a> {
  let node = bump.alloc(MethodProp {
    inner,
    parent: None,
    key: get_view_for_prop_name(&inner.key, bump),
    function: get_view_for_function(&inner.function, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_prop_name(&node.key, parent);
  set_parent_for_function(&node.function, parent);
  node
}

fn set_parent_for_method_prop<'a>(node: &MethodProp<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const MethodProp<'a> as *mut MethodProp<'a>;
    (*node_ptr).parent.replace(parent.expect::<ObjectLit>());
  }
}

#[derive(Clone)]
pub struct Module<'a> {
  pub text_info: Option<&'a SourceTextInfo>,
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

impl<'a> SourceRanged for Module<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Module<'a>> for Node<'a> {
  fn from(node: &Module<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Module<'a>, &'a Module<'a>>(node) };
    Node::Module(node)
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Module
  }
}

impl<'a> CastableNode<'a> for Module<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Module(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Module
  }
}

fn get_view_for_module<'a>(source_file_info: &'a ModuleInfo<'a>, bump: &'a Bump) -> &'a Module<'a> {
  let inner = source_file_info.module;
  let tokens = source_file_info.tokens.map(|t| &*bump.alloc(TokenContainer::new(t)));
  let comments = source_file_info.comments.map(|c| &*bump.alloc(CommentContainer::new(
    c.leading,
    c.trailing,
    tokens.expect("Tokens must be provided when using comments."),
    source_file_info.text_info.expect("Text info must be provided when using comments"),
  )));
  let node = bump.alloc(Module {
    inner,
    text_info: source_file_info.text_info,
    tokens,
    comments,
    body: inner.body.iter().map(|value| get_view_for_module_item(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.body.iter() {
    set_parent_for_module_item(value, parent)
  }
  node
}

/// `export { foo } from 'mod'`
/// `export { foo as bar } from 'mod'`
#[derive(Clone)]
pub struct NamedExport<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::NamedExport,
  pub specifiers: Vec<ExportSpecifier<'a>>,
  pub src: Option<&'a Str<'a>>,
  pub asserts: Option<&'a ObjectLit<'a>>,
}

impl<'a> NamedExport<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn type_only(&self) -> bool {
    self.inner.type_only
  }
}

impl<'a> SourceRanged for NamedExport<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&NamedExport<'a>> for Node<'a> {
  fn from(node: &NamedExport<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&NamedExport<'a>, &'a NamedExport<'a>>(node) };
    Node::NamedExport(node)
  }
}

impl<'a> NodeTrait<'a> for NamedExport<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.specifiers.len() + match &self.src { Some(_value) => 1, None => 0, } + match &self.asserts { Some(_value) => 1, None => 0, });
    for child in self.specifiers.iter() {
      children.push(child.into());
    }
    if let Some(child) = self.src {
      children.push(child.into());
    }
    if let Some(child) = self.asserts {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::NamedExport
  }
}

impl<'a> CastableNode<'a> for NamedExport<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::NamedExport(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::NamedExport
  }
}

fn get_view_for_named_export<'a>(inner: &'a swc_ast::NamedExport, bump: &'a Bump) -> &'a NamedExport<'a> {
  let node = bump.alloc(NamedExport {
    inner,
    parent: None,
    specifiers: inner.specifiers.iter().map(|value| get_view_for_export_specifier(value, bump)).collect(),
    src: match &inner.src {
      Some(value) => Some(get_view_for_str(value, bump)),
      None => None,
    },
    asserts: match &inner.asserts {
      Some(value) => Some(get_view_for_object_lit(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.specifiers.iter() {
    set_parent_for_export_specifier(value, parent)
  }
  if let Some(value) = &node.src {
    set_parent_for_str(value, parent)
  };
  if let Some(value) = &node.asserts {
    set_parent_for_object_lit(value, parent)
  };
  node
}

fn set_parent_for_named_export<'a>(node: &NamedExport<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const NamedExport<'a> as *mut NamedExport<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct NewExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::NewExpr,
  pub callee: Expr<'a>,
  pub args: Option<Vec<&'a ExprOrSpread<'a>>>,
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> NewExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for NewExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&NewExpr<'a>> for Node<'a> {
  fn from(node: &NewExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&NewExpr<'a>, &'a NewExpr<'a>>(node) };
    Node::NewExpr(node)
  }
}

impl<'a> NodeTrait<'a> for NewExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::NewExpr
  }
}

impl<'a> CastableNode<'a> for NewExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::NewExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::NewExpr
  }
}

fn get_view_for_new_expr<'a>(inner: &'a swc_ast::NewExpr, bump: &'a Bump) -> &'a NewExpr<'a> {
  let node = bump.alloc(NewExpr {
    inner,
    parent: None,
    callee: get_view_for_expr(&inner.callee, bump),
    args: match &inner.args {
      Some(value) => Some(value.iter().map(|value| get_view_for_expr_or_spread(value, bump)).collect()),
      None => None,
    },
    type_args: match &inner.type_args {
      Some(value) => Some(get_view_for_ts_type_param_instantiation(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.callee, parent);
  if let Some(value) = &node.args {
    for value in value.iter() {
      set_parent_for_expr_or_spread(value, parent)
    }
  };
  if let Some(value) = &node.type_args {
    set_parent_for_ts_type_param_instantiation(value, parent)
  };
  node
}

fn set_parent_for_new_expr<'a>(node: &NewExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const NewExpr<'a> as *mut NewExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct Null<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Null,
}

impl<'a> Null<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for Null<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Null<'a>> for Node<'a> {
  fn from(node: &Null<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Null<'a>, &'a Null<'a>>(node) };
    Node::Null(node)
  }
}

impl<'a> NodeTrait<'a> for Null<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Null
  }
}

impl<'a> CastableNode<'a> for Null<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Null(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Null
  }
}

fn get_view_for_null<'a>(inner: &'a swc_ast::Null, bump: &'a Bump) -> &'a Null<'a> {
  let node = bump.alloc(Null {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_null<'a>(node: &Null<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Null<'a> as *mut Null<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// A numeric literal.
///
///
/// # Creation
///
/// If you are creating a numeric literal with a dummy span, please use
/// `literal.into()`, instead of creating this struct directly.
///
/// All of `Box<Expr>`, `Expr`, `Lit`, `Number` implements `From<64>` and
/// `From<usize>`.
#[derive(Clone)]
pub struct Number<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Number,
}

impl<'a> Number<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  /// **Note**: This should not be `NaN`. Use [crate::Ident] to represent NaN.
  ///
  /// If you store `NaN` in this field, a hash map will behave strangely.
  pub fn value(&self) -> f64 {
    self.inner.value
  }

  /// Use `None` value only for transformations to avoid recalculate
  /// characters in number literal
  pub fn raw(&self) -> &Option<swc_atoms::JsWord> {
    &self.inner.raw
  }
}

impl<'a> SourceRanged for Number<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Number<'a>> for Node<'a> {
  fn from(node: &Number<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Number<'a>, &'a Number<'a>>(node) };
    Node::Number(node)
  }
}

impl<'a> NodeTrait<'a> for Number<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Number
  }
}

impl<'a> CastableNode<'a> for Number<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Number(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Number
  }
}

fn get_view_for_number<'a>(inner: &'a swc_ast::Number, bump: &'a Bump) -> &'a Number<'a> {
  let node = bump.alloc(Number {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_number<'a>(node: &Number<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Number<'a> as *mut Number<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// Object literal.
#[derive(Clone)]
pub struct ObjectLit<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ObjectLit,
  pub props: Vec<PropOrSpread<'a>>,
}

impl<'a> ObjectLit<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ObjectLit<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ObjectLit<'a>> for Node<'a> {
  fn from(node: &ObjectLit<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ObjectLit<'a>, &'a ObjectLit<'a>>(node) };
    Node::ObjectLit(node)
  }
}

impl<'a> NodeTrait<'a> for ObjectLit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.props.len());
    for child in self.props.iter() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ObjectLit
  }
}

impl<'a> CastableNode<'a> for ObjectLit<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ObjectLit(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ObjectLit
  }
}

fn get_view_for_object_lit<'a>(inner: &'a swc_ast::ObjectLit, bump: &'a Bump) -> &'a ObjectLit<'a> {
  let node = bump.alloc(ObjectLit {
    inner,
    parent: None,
    props: inner.props.iter().map(|value| get_view_for_prop_or_spread(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.props.iter() {
    set_parent_for_prop_or_spread(value, parent)
  }
  node
}

fn set_parent_for_object_lit<'a>(node: &ObjectLit<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ObjectLit<'a> as *mut ObjectLit<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ObjectPat<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ObjectPat,
  pub props: Vec<ObjectPatProp<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> ObjectPat<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  /// Only in an ambient context
  pub fn optional(&self) -> bool {
    self.inner.optional
  }
}

impl<'a> SourceRanged for ObjectPat<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ObjectPat<'a>> for Node<'a> {
  fn from(node: &ObjectPat<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ObjectPat<'a>, &'a ObjectPat<'a>>(node) };
    Node::ObjectPat(node)
  }
}

impl<'a> NodeTrait<'a> for ObjectPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ObjectPat
  }
}

impl<'a> CastableNode<'a> for ObjectPat<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ObjectPat(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ObjectPat
  }
}

fn get_view_for_object_pat<'a>(inner: &'a swc_ast::ObjectPat, bump: &'a Bump) -> &'a ObjectPat<'a> {
  let node = bump.alloc(ObjectPat {
    inner,
    parent: None,
    props: inner.props.iter().map(|value| get_view_for_object_pat_prop(value, bump)).collect(),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.props.iter() {
    set_parent_for_object_pat_prop(value, parent)
  }
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  node
}

fn set_parent_for_object_pat<'a>(node: &ObjectPat<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ObjectPat<'a> as *mut ObjectPat<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct OptCall<'a> {
  parent: Option<&'a OptChainExpr<'a>>,
  pub inner: &'a swc_ast::OptCall,
  pub callee: Expr<'a>,
  pub args: Vec<&'a ExprOrSpread<'a>>,
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> OptCall<'a> {
  pub fn parent(&self) -> &'a OptChainExpr<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for OptCall<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&OptCall<'a>> for Node<'a> {
  fn from(node: &OptCall<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&OptCall<'a>, &'a OptCall<'a>>(node) };
    Node::OptCall(node)
  }
}

impl<'a> NodeTrait<'a> for OptCall<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::OptCall
  }
}

impl<'a> CastableNode<'a> for OptCall<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::OptCall(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::OptCall
  }
}

fn get_view_for_opt_call<'a>(inner: &'a swc_ast::OptCall, bump: &'a Bump) -> &'a OptCall<'a> {
  let node = bump.alloc(OptCall {
    inner,
    parent: None,
    callee: get_view_for_expr(&inner.callee, bump),
    args: inner.args.iter().map(|value| get_view_for_expr_or_spread(value, bump)).collect(),
    type_args: match &inner.type_args {
      Some(value) => Some(get_view_for_ts_type_param_instantiation(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.callee, parent);
  for value in node.args.iter() {
    set_parent_for_expr_or_spread(value, parent)
  }
  if let Some(value) = &node.type_args {
    set_parent_for_ts_type_param_instantiation(value, parent)
  };
  node
}

fn set_parent_for_opt_call<'a>(node: &OptCall<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const OptCall<'a> as *mut OptCall<'a>;
    (*node_ptr).parent.replace(parent.expect::<OptChainExpr>());
  }
}

#[derive(Clone)]
pub struct OptChainExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::OptChainExpr,
  pub base: OptChainBase<'a>,
}

impl<'a> OptChainExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn question_dot_token(&self) -> &swc_common::Span {
    &self.inner.question_dot_token
  }
}

impl<'a> SourceRanged for OptChainExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&OptChainExpr<'a>> for Node<'a> {
  fn from(node: &OptChainExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&OptChainExpr<'a>, &'a OptChainExpr<'a>>(node) };
    Node::OptChainExpr(node)
  }
}

impl<'a> NodeTrait<'a> for OptChainExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.base).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::OptChainExpr
  }
}

impl<'a> CastableNode<'a> for OptChainExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::OptChainExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::OptChainExpr
  }
}

fn get_view_for_opt_chain_expr<'a>(inner: &'a swc_ast::OptChainExpr, bump: &'a Bump) -> &'a OptChainExpr<'a> {
  let node = bump.alloc(OptChainExpr {
    inner,
    parent: None,
    base: get_view_for_opt_chain_base(&inner.base, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_opt_chain_base(&node.base, parent);
  node
}

fn set_parent_for_opt_chain_expr<'a>(node: &OptChainExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const OptChainExpr<'a> as *mut OptChainExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct Param<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Param,
  pub decorators: Vec<&'a Decorator<'a>>,
  pub pat: Pat<'a>,
}

impl<'a> Param<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for Param<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Param<'a>> for Node<'a> {
  fn from(node: &Param<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Param<'a>, &'a Param<'a>>(node) };
    Node::Param(node)
  }
}

impl<'a> NodeTrait<'a> for Param<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.decorators.len());
    for child in self.decorators.iter() {
      children.push((*child).into());
    }
    children.push((&self.pat).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Param
  }
}

impl<'a> CastableNode<'a> for Param<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Param(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Param
  }
}

fn get_view_for_param<'a>(inner: &'a swc_ast::Param, bump: &'a Bump) -> &'a Param<'a> {
  let node = bump.alloc(Param {
    inner,
    parent: None,
    decorators: inner.decorators.iter().map(|value| get_view_for_decorator(value, bump)).collect(),
    pat: get_view_for_pat(&inner.pat, bump),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.decorators.iter() {
    set_parent_for_decorator(value, parent)
  }
  set_parent_for_pat(&node.pat, parent);
  node
}

fn set_parent_for_param<'a>(node: &Param<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Param<'a> as *mut Param<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ParenExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ParenExpr,
  pub expr: Expr<'a>,
}

impl<'a> ParenExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ParenExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ParenExpr<'a>> for Node<'a> {
  fn from(node: &ParenExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ParenExpr<'a>, &'a ParenExpr<'a>>(node) };
    Node::ParenExpr(node)
  }
}

impl<'a> NodeTrait<'a> for ParenExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ParenExpr
  }
}

impl<'a> CastableNode<'a> for ParenExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ParenExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ParenExpr
  }
}

fn get_view_for_paren_expr<'a>(inner: &'a swc_ast::ParenExpr, bump: &'a Bump) -> &'a ParenExpr<'a> {
  let node = bump.alloc(ParenExpr {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_paren_expr<'a>(node: &ParenExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ParenExpr<'a> as *mut ParenExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct PrivateMethod<'a> {
  parent: Option<&'a Class<'a>>,
  pub inner: &'a swc_ast::PrivateMethod,
  pub key: &'a PrivateName<'a>,
  pub function: &'a Function<'a>,
}

impl<'a> PrivateMethod<'a> {
  pub fn parent(&self) -> &'a Class<'a> {
    self.parent.unwrap()
  }

  pub fn method_kind(&self) -> MethodKind {
    self.inner.kind
  }

  pub fn is_static(&self) -> bool {
    self.inner.is_static
  }

  /// Typescript extension.
  pub fn accessibility(&self) -> Option<Accessibility> {
    self.inner.accessibility
  }

  /// Typescript extension.
  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }

  pub fn is_override(&self) -> bool {
    self.inner.is_override
  }
}

impl<'a> SourceRanged for PrivateMethod<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&PrivateMethod<'a>> for Node<'a> {
  fn from(node: &PrivateMethod<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&PrivateMethod<'a>, &'a PrivateMethod<'a>>(node) };
    Node::PrivateMethod(node)
  }
}

impl<'a> NodeTrait<'a> for PrivateMethod<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.key.into());
    children.push(self.function.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::PrivateMethod
  }
}

impl<'a> CastableNode<'a> for PrivateMethod<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::PrivateMethod(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::PrivateMethod
  }
}

fn get_view_for_private_method<'a>(inner: &'a swc_ast::PrivateMethod, bump: &'a Bump) -> &'a PrivateMethod<'a> {
  let node = bump.alloc(PrivateMethod {
    inner,
    parent: None,
    key: get_view_for_private_name(&inner.key, bump),
    function: get_view_for_function(&inner.function, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_private_name(&node.key, parent);
  set_parent_for_function(&node.function, parent);
  node
}

fn set_parent_for_private_method<'a>(node: &PrivateMethod<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const PrivateMethod<'a> as *mut PrivateMethod<'a>;
    (*node_ptr).parent.replace(parent.expect::<Class>());
  }
}

#[derive(Clone)]
pub struct PrivateName<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::PrivateName,
  pub id: &'a Ident<'a>,
}

impl<'a> PrivateName<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for PrivateName<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&PrivateName<'a>> for Node<'a> {
  fn from(node: &PrivateName<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&PrivateName<'a>, &'a PrivateName<'a>>(node) };
    Node::PrivateName(node)
  }
}

impl<'a> NodeTrait<'a> for PrivateName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.id.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::PrivateName
  }
}

impl<'a> CastableNode<'a> for PrivateName<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::PrivateName(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::PrivateName
  }
}

fn get_view_for_private_name<'a>(inner: &'a swc_ast::PrivateName, bump: &'a Bump) -> &'a PrivateName<'a> {
  let node = bump.alloc(PrivateName {
    inner,
    parent: None,
    id: get_view_for_ident(&inner.id, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.id, parent);
  node
}

fn set_parent_for_private_name<'a>(node: &PrivateName<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const PrivateName<'a> as *mut PrivateName<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct PrivateProp<'a> {
  parent: Option<&'a Class<'a>>,
  pub inner: &'a swc_ast::PrivateProp,
  pub key: &'a PrivateName<'a>,
  pub value: Option<Expr<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub decorators: Vec<&'a Decorator<'a>>,
}

impl<'a> PrivateProp<'a> {
  pub fn parent(&self) -> &'a Class<'a> {
    self.parent.unwrap()
  }

  pub fn is_static(&self) -> bool {
    self.inner.is_static
  }

  /// Typescript extension.
  pub fn accessibility(&self) -> Option<Accessibility> {
    self.inner.accessibility
  }

  pub fn is_optional(&self) -> bool {
    self.inner.is_optional
  }

  pub fn is_override(&self) -> bool {
    self.inner.is_override
  }

  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }

  pub fn definite(&self) -> bool {
    self.inner.definite
  }
}

impl<'a> SourceRanged for PrivateProp<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&PrivateProp<'a>> for Node<'a> {
  fn from(node: &PrivateProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&PrivateProp<'a>, &'a PrivateProp<'a>>(node) };
    Node::PrivateProp(node)
  }
}

impl<'a> NodeTrait<'a> for PrivateProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::PrivateProp
  }
}

impl<'a> CastableNode<'a> for PrivateProp<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::PrivateProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::PrivateProp
  }
}

fn get_view_for_private_prop<'a>(inner: &'a swc_ast::PrivateProp, bump: &'a Bump) -> &'a PrivateProp<'a> {
  let node = bump.alloc(PrivateProp {
    inner,
    parent: None,
    key: get_view_for_private_name(&inner.key, bump),
    value: match &inner.value {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
    decorators: inner.decorators.iter().map(|value| get_view_for_decorator(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_private_name(&node.key, parent);
  if let Some(value) = &node.value {
    set_parent_for_expr(value, parent)
  };
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  for value in node.decorators.iter() {
    set_parent_for_decorator(value, parent)
  }
  node
}

fn set_parent_for_private_prop<'a>(node: &PrivateProp<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const PrivateProp<'a> as *mut PrivateProp<'a>;
    (*node_ptr).parent.replace(parent.expect::<Class>());
  }
}

#[derive(Clone)]
pub struct Regex<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Regex,
}

impl<'a> Regex<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn exp(&self) -> &swc_atoms::JsWord {
    &self.inner.exp
  }

  pub fn flags(&self) -> &swc_atoms::JsWord {
    &self.inner.flags
  }
}

impl<'a> SourceRanged for Regex<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Regex<'a>> for Node<'a> {
  fn from(node: &Regex<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Regex<'a>, &'a Regex<'a>>(node) };
    Node::Regex(node)
  }
}

impl<'a> NodeTrait<'a> for Regex<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Regex
  }
}

impl<'a> CastableNode<'a> for Regex<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Regex(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Regex
  }
}

fn get_view_for_regex<'a>(inner: &'a swc_ast::Regex, bump: &'a Bump) -> &'a Regex<'a> {
  let node = bump.alloc(Regex {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_regex<'a>(node: &Regex<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Regex<'a> as *mut Regex<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// EsTree `RestElement`
#[derive(Clone)]
pub struct RestPat<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::RestPat,
  pub arg: Pat<'a>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> RestPat<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn dot3_token(&self) -> &swc_common::Span {
    &self.inner.dot3_token
  }
}

impl<'a> SourceRanged for RestPat<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&RestPat<'a>> for Node<'a> {
  fn from(node: &RestPat<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&RestPat<'a>, &'a RestPat<'a>>(node) };
    Node::RestPat(node)
  }
}

impl<'a> NodeTrait<'a> for RestPat<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.arg).into());
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::RestPat
  }
}

impl<'a> CastableNode<'a> for RestPat<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::RestPat(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::RestPat
  }
}

fn get_view_for_rest_pat<'a>(inner: &'a swc_ast::RestPat, bump: &'a Bump) -> &'a RestPat<'a> {
  let node = bump.alloc(RestPat {
    inner,
    parent: None,
    arg: get_view_for_pat(&inner.arg, bump),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_pat(&node.arg, parent);
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  node
}

fn set_parent_for_rest_pat<'a>(node: &RestPat<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const RestPat<'a> as *mut RestPat<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ReturnStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ReturnStmt,
  pub arg: Option<Expr<'a>>,
}

impl<'a> ReturnStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ReturnStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ReturnStmt<'a>> for Node<'a> {
  fn from(node: &ReturnStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ReturnStmt<'a>, &'a ReturnStmt<'a>>(node) };
    Node::ReturnStmt(node)
  }
}

impl<'a> NodeTrait<'a> for ReturnStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.arg { Some(_value) => 1, None => 0, });
    if let Some(child) = self.arg.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ReturnStmt
  }
}

impl<'a> CastableNode<'a> for ReturnStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ReturnStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ReturnStmt
  }
}

fn get_view_for_return_stmt<'a>(inner: &'a swc_ast::ReturnStmt, bump: &'a Bump) -> &'a ReturnStmt<'a> {
  let node = bump.alloc(ReturnStmt {
    inner,
    parent: None,
    arg: match &inner.arg {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  if let Some(value) = &node.arg {
    set_parent_for_expr(value, parent)
  };
  node
}

fn set_parent_for_return_stmt<'a>(node: &ReturnStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ReturnStmt<'a> as *mut ReturnStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct Script<'a> {
  pub text_info: Option<&'a SourceTextInfo>,
  pub tokens: Option<&'a TokenContainer<'a>>,
  pub comments: Option<&'a CommentContainer<'a>>,
  pub inner: &'a swc_ast::Script,
  pub body: Vec<Stmt<'a>>,
}

impl<'a> Script<'a> {
  pub fn shebang(&self) -> &Option<swc_atoms::JsWord> {
    &self.inner.shebang
  }
}

impl<'a> SourceRanged for Script<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Script<'a>> for Node<'a> {
  fn from(node: &Script<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Script<'a>, &'a Script<'a>>(node) };
    Node::Script(node)
  }
}

impl<'a> NodeTrait<'a> for Script<'a> {
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Script
  }
}

impl<'a> CastableNode<'a> for Script<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Script(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Script
  }
}

fn get_view_for_script<'a>(source_file_info: &'a ScriptInfo<'a>, bump: &'a Bump) -> &'a Script<'a> {
  let inner = source_file_info.script;
  let tokens = source_file_info.tokens.map(|t| &*bump.alloc(TokenContainer::new(t)));
  let comments = source_file_info.comments.map(|c| &*bump.alloc(CommentContainer::new(
    c.leading,
    c.trailing,
    tokens.expect("Tokens must be provided when using comments."),
    source_file_info.text_info.expect("Text info must be provided when using comments"),
  )));
  let node = bump.alloc(Script {
    inner,
    text_info: source_file_info.text_info,
    tokens,
    comments,
    body: inner.body.iter().map(|value| get_view_for_stmt(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.body.iter() {
    set_parent_for_stmt(value, parent)
  }
  node
}

#[derive(Clone)]
pub struct SeqExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::SeqExpr,
  pub exprs: Vec<Expr<'a>>,
}

impl<'a> SeqExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for SeqExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&SeqExpr<'a>> for Node<'a> {
  fn from(node: &SeqExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SeqExpr<'a>, &'a SeqExpr<'a>>(node) };
    Node::SeqExpr(node)
  }
}

impl<'a> NodeTrait<'a> for SeqExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.exprs.len());
    for child in self.exprs.iter() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SeqExpr
  }
}

impl<'a> CastableNode<'a> for SeqExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SeqExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::SeqExpr
  }
}

fn get_view_for_seq_expr<'a>(inner: &'a swc_ast::SeqExpr, bump: &'a Bump) -> &'a SeqExpr<'a> {
  let node = bump.alloc(SeqExpr {
    inner,
    parent: None,
    exprs: inner.exprs.iter().map(|value| get_view_for_expr(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.exprs.iter() {
    set_parent_for_expr(value, parent)
  }
  node
}

fn set_parent_for_seq_expr<'a>(node: &SeqExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const SeqExpr<'a> as *mut SeqExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct SetterProp<'a> {
  parent: Option<&'a ObjectLit<'a>>,
  pub inner: &'a swc_ast::SetterProp,
  pub key: PropName<'a>,
  pub param: Pat<'a>,
  pub body: Option<&'a BlockStmt<'a>>,
}

impl<'a> SetterProp<'a> {
  pub fn parent(&self) -> &'a ObjectLit<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for SetterProp<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&SetterProp<'a>> for Node<'a> {
  fn from(node: &SetterProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SetterProp<'a>, &'a SetterProp<'a>>(node) };
    Node::SetterProp(node)
  }
}

impl<'a> NodeTrait<'a> for SetterProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SetterProp
  }
}

impl<'a> CastableNode<'a> for SetterProp<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SetterProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::SetterProp
  }
}

fn get_view_for_setter_prop<'a>(inner: &'a swc_ast::SetterProp, bump: &'a Bump) -> &'a SetterProp<'a> {
  let node = bump.alloc(SetterProp {
    inner,
    parent: None,
    key: get_view_for_prop_name(&inner.key, bump),
    param: get_view_for_pat(&inner.param, bump),
    body: match &inner.body {
      Some(value) => Some(get_view_for_block_stmt(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_prop_name(&node.key, parent);
  set_parent_for_pat(&node.param, parent);
  if let Some(value) = &node.body {
    set_parent_for_block_stmt(value, parent)
  };
  node
}

fn set_parent_for_setter_prop<'a>(node: &SetterProp<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const SetterProp<'a> as *mut SetterProp<'a>;
    (*node_ptr).parent.replace(parent.expect::<ObjectLit>());
  }
}

#[derive(Clone)]
pub struct SpreadElement<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::SpreadElement,
  pub expr: Expr<'a>,
}

impl<'a> SpreadElement<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn dot3_token(&self) -> &swc_common::Span {
    &self.inner.dot3_token
  }
}

impl<'a> SourceRanged for SpreadElement<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&SpreadElement<'a>> for Node<'a> {
  fn from(node: &SpreadElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SpreadElement<'a>, &'a SpreadElement<'a>>(node) };
    Node::SpreadElement(node)
  }
}

impl<'a> NodeTrait<'a> for SpreadElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SpreadElement
  }
}

impl<'a> CastableNode<'a> for SpreadElement<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SpreadElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::SpreadElement
  }
}

fn get_view_for_spread_element<'a>(inner: &'a swc_ast::SpreadElement, bump: &'a Bump) -> &'a SpreadElement<'a> {
  let node = bump.alloc(SpreadElement {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_spread_element<'a>(node: &SpreadElement<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const SpreadElement<'a> as *mut SpreadElement<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct StaticBlock<'a> {
  parent: Option<&'a Class<'a>>,
  pub inner: &'a swc_ast::StaticBlock,
  pub body: &'a BlockStmt<'a>,
}

impl<'a> StaticBlock<'a> {
  pub fn parent(&self) -> &'a Class<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for StaticBlock<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&StaticBlock<'a>> for Node<'a> {
  fn from(node: &StaticBlock<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&StaticBlock<'a>, &'a StaticBlock<'a>>(node) };
    Node::StaticBlock(node)
  }
}

impl<'a> NodeTrait<'a> for StaticBlock<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.body.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::StaticBlock
  }
}

impl<'a> CastableNode<'a> for StaticBlock<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::StaticBlock(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::StaticBlock
  }
}

fn get_view_for_static_block<'a>(inner: &'a swc_ast::StaticBlock, bump: &'a Bump) -> &'a StaticBlock<'a> {
  let node = bump.alloc(StaticBlock {
    inner,
    parent: None,
    body: get_view_for_block_stmt(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_block_stmt(&node.body, parent);
  node
}

fn set_parent_for_static_block<'a>(node: &StaticBlock<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const StaticBlock<'a> as *mut StaticBlock<'a>;
    (*node_ptr).parent.replace(parent.expect::<Class>());
  }
}

/// A string literal.
#[derive(Clone)]
pub struct Str<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Str,
}

impl<'a> Str<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn value(&self) -> &swc_atoms::JsWord {
    &self.inner.value
  }

  /// Use `None` value only for transformations to avoid recalculate escaped
  /// characters in strings
  pub fn raw(&self) -> &Option<swc_atoms::JsWord> {
    &self.inner.raw
  }
}

impl<'a> SourceRanged for Str<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Str<'a>> for Node<'a> {
  fn from(node: &Str<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Str<'a>, &'a Str<'a>>(node) };
    Node::Str(node)
  }
}

impl<'a> NodeTrait<'a> for Str<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Str
  }
}

impl<'a> CastableNode<'a> for Str<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Str(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Str
  }
}

fn get_view_for_str<'a>(inner: &'a swc_ast::Str, bump: &'a Bump) -> &'a Str<'a> {
  let node = bump.alloc(Str {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_str<'a>(node: &Str<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Str<'a> as *mut Str<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct Super<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Super,
}

impl<'a> Super<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for Super<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Super<'a>> for Node<'a> {
  fn from(node: &Super<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Super<'a>, &'a Super<'a>>(node) };
    Node::Super(node)
  }
}

impl<'a> NodeTrait<'a> for Super<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Super
  }
}

impl<'a> CastableNode<'a> for Super<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Super(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Super
  }
}

fn get_view_for_super<'a>(inner: &'a swc_ast::Super, bump: &'a Bump) -> &'a Super<'a> {
  let node = bump.alloc(Super {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_super<'a>(node: &Super<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Super<'a> as *mut Super<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct SuperPropExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::SuperPropExpr,
  pub obj: &'a Super<'a>,
  pub prop: SuperProp<'a>,
}

impl<'a> SuperPropExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for SuperPropExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&SuperPropExpr<'a>> for Node<'a> {
  fn from(node: &SuperPropExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SuperPropExpr<'a>, &'a SuperPropExpr<'a>>(node) };
    Node::SuperPropExpr(node)
  }
}

impl<'a> NodeTrait<'a> for SuperPropExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.obj.into());
    children.push((&self.prop).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SuperPropExpr
  }
}

impl<'a> CastableNode<'a> for SuperPropExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SuperPropExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::SuperPropExpr
  }
}

fn get_view_for_super_prop_expr<'a>(inner: &'a swc_ast::SuperPropExpr, bump: &'a Bump) -> &'a SuperPropExpr<'a> {
  let node = bump.alloc(SuperPropExpr {
    inner,
    parent: None,
    obj: get_view_for_super(&inner.obj, bump),
    prop: get_view_for_super_prop(&inner.prop, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_super(&node.obj, parent);
  set_parent_for_super_prop(&node.prop, parent);
  node
}

fn set_parent_for_super_prop_expr<'a>(node: &SuperPropExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const SuperPropExpr<'a> as *mut SuperPropExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct SwitchCase<'a> {
  parent: Option<&'a SwitchStmt<'a>>,
  pub inner: &'a swc_ast::SwitchCase,
  /// None for `default:`
  pub test: Option<Expr<'a>>,
  pub cons: Vec<Stmt<'a>>,
}

impl<'a> SwitchCase<'a> {
  pub fn parent(&self) -> &'a SwitchStmt<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for SwitchCase<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&SwitchCase<'a>> for Node<'a> {
  fn from(node: &SwitchCase<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SwitchCase<'a>, &'a SwitchCase<'a>>(node) };
    Node::SwitchCase(node)
  }
}

impl<'a> NodeTrait<'a> for SwitchCase<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SwitchCase
  }
}

impl<'a> CastableNode<'a> for SwitchCase<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SwitchCase(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::SwitchCase
  }
}

fn get_view_for_switch_case<'a>(inner: &'a swc_ast::SwitchCase, bump: &'a Bump) -> &'a SwitchCase<'a> {
  let node = bump.alloc(SwitchCase {
    inner,
    parent: None,
    test: match &inner.test {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
    cons: inner.cons.iter().map(|value| get_view_for_stmt(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  if let Some(value) = &node.test {
    set_parent_for_expr(value, parent)
  };
  for value in node.cons.iter() {
    set_parent_for_stmt(value, parent)
  }
  node
}

fn set_parent_for_switch_case<'a>(node: &SwitchCase<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const SwitchCase<'a> as *mut SwitchCase<'a>;
    (*node_ptr).parent.replace(parent.expect::<SwitchStmt>());
  }
}

#[derive(Clone)]
pub struct SwitchStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::SwitchStmt,
  pub discriminant: Expr<'a>,
  pub cases: Vec<&'a SwitchCase<'a>>,
}

impl<'a> SwitchStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for SwitchStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&SwitchStmt<'a>> for Node<'a> {
  fn from(node: &SwitchStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&SwitchStmt<'a>, &'a SwitchStmt<'a>>(node) };
    Node::SwitchStmt(node)
  }
}

impl<'a> NodeTrait<'a> for SwitchStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.cases.len());
    children.push((&self.discriminant).into());
    for child in self.cases.iter() {
      children.push((*child).into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::SwitchStmt
  }
}

impl<'a> CastableNode<'a> for SwitchStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::SwitchStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::SwitchStmt
  }
}

fn get_view_for_switch_stmt<'a>(inner: &'a swc_ast::SwitchStmt, bump: &'a Bump) -> &'a SwitchStmt<'a> {
  let node = bump.alloc(SwitchStmt {
    inner,
    parent: None,
    discriminant: get_view_for_expr(&inner.discriminant, bump),
    cases: inner.cases.iter().map(|value| get_view_for_switch_case(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.discriminant, parent);
  for value in node.cases.iter() {
    set_parent_for_switch_case(value, parent)
  }
  node
}

fn set_parent_for_switch_stmt<'a>(node: &SwitchStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const SwitchStmt<'a> as *mut SwitchStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TaggedTpl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TaggedTpl,
  pub tag: Expr<'a>,
  pub type_params: Option<&'a TsTypeParamInstantiation<'a>>,
  pub tpl: &'a Tpl<'a>,
}

impl<'a> TaggedTpl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TaggedTpl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TaggedTpl<'a>> for Node<'a> {
  fn from(node: &TaggedTpl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TaggedTpl<'a>, &'a TaggedTpl<'a>>(node) };
    Node::TaggedTpl(node)
  }
}

impl<'a> NodeTrait<'a> for TaggedTpl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2 + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.tag).into());
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children.push(self.tpl.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TaggedTpl
  }
}

impl<'a> CastableNode<'a> for TaggedTpl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TaggedTpl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TaggedTpl
  }
}

fn get_view_for_tagged_tpl<'a>(inner: &'a swc_ast::TaggedTpl, bump: &'a Bump) -> &'a TaggedTpl<'a> {
  let node = bump.alloc(TaggedTpl {
    inner,
    parent: None,
    tag: get_view_for_expr(&inner.tag, bump),
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_instantiation(value, bump)),
      None => None,
    },
    tpl: get_view_for_tpl(&inner.tpl, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.tag, parent);
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_instantiation(value, parent)
  };
  set_parent_for_tpl(&node.tpl, parent);
  node
}

fn set_parent_for_tagged_tpl<'a>(node: &TaggedTpl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TaggedTpl<'a> as *mut TaggedTpl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ThisExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ThisExpr,
}

impl<'a> ThisExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ThisExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ThisExpr<'a>> for Node<'a> {
  fn from(node: &ThisExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ThisExpr<'a>, &'a ThisExpr<'a>>(node) };
    Node::ThisExpr(node)
  }
}

impl<'a> NodeTrait<'a> for ThisExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ThisExpr
  }
}

impl<'a> CastableNode<'a> for ThisExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ThisExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ThisExpr
  }
}

fn get_view_for_this_expr<'a>(inner: &'a swc_ast::ThisExpr, bump: &'a Bump) -> &'a ThisExpr<'a> {
  let node = bump.alloc(ThisExpr {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_this_expr<'a>(node: &ThisExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ThisExpr<'a> as *mut ThisExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct ThrowStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::ThrowStmt,
  pub arg: Expr<'a>,
}

impl<'a> ThrowStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for ThrowStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&ThrowStmt<'a>> for Node<'a> {
  fn from(node: &ThrowStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&ThrowStmt<'a>, &'a ThrowStmt<'a>>(node) };
    Node::ThrowStmt(node)
  }
}

impl<'a> NodeTrait<'a> for ThrowStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::ThrowStmt
  }
}

impl<'a> CastableNode<'a> for ThrowStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::ThrowStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::ThrowStmt
  }
}

fn get_view_for_throw_stmt<'a>(inner: &'a swc_ast::ThrowStmt, bump: &'a Bump) -> &'a ThrowStmt<'a> {
  let node = bump.alloc(ThrowStmt {
    inner,
    parent: None,
    arg: get_view_for_expr(&inner.arg, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.arg, parent);
  node
}

fn set_parent_for_throw_stmt<'a>(node: &ThrowStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const ThrowStmt<'a> as *mut ThrowStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct Tpl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::Tpl,
  pub exprs: Vec<Expr<'a>>,
  pub quasis: Vec<&'a TplElement<'a>>,
}

impl<'a> Tpl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for Tpl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&Tpl<'a>> for Node<'a> {
  fn from(node: &Tpl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&Tpl<'a>, &'a Tpl<'a>>(node) };
    Node::Tpl(node)
  }
}

impl<'a> NodeTrait<'a> for Tpl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::Tpl
  }
}

impl<'a> CastableNode<'a> for Tpl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::Tpl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::Tpl
  }
}

fn get_view_for_tpl<'a>(inner: &'a swc_ast::Tpl, bump: &'a Bump) -> &'a Tpl<'a> {
  let node = bump.alloc(Tpl {
    inner,
    parent: None,
    exprs: inner.exprs.iter().map(|value| get_view_for_expr(value, bump)).collect(),
    quasis: inner.quasis.iter().map(|value| get_view_for_tpl_element(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.exprs.iter() {
    set_parent_for_expr(value, parent)
  }
  for value in node.quasis.iter() {
    set_parent_for_tpl_element(value, parent)
  }
  node
}

fn set_parent_for_tpl<'a>(node: &Tpl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const Tpl<'a> as *mut Tpl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TplElement<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TplElement,
}

impl<'a> TplElement<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn tail(&self) -> bool {
    self.inner.tail
  }

  /// This value is never used by `swc_ecma_codegen`, and this fact is
  /// considered as a public API.
  ///
  /// If you are going to use codegen right after creating a [TplElement], you
  /// don't have to worry about this value.
  pub fn cooked(&self) -> &Option<swc_atoms::JsWord> {
    &self.inner.cooked
  }

  pub fn raw(&self) -> &swc_atoms::JsWord {
    &self.inner.raw
  }
}

impl<'a> SourceRanged for TplElement<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TplElement<'a>> for Node<'a> {
  fn from(node: &TplElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TplElement<'a>, &'a TplElement<'a>>(node) };
    Node::TplElement(node)
  }
}

impl<'a> NodeTrait<'a> for TplElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TplElement
  }
}

impl<'a> CastableNode<'a> for TplElement<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TplElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TplElement
  }
}

fn get_view_for_tpl_element<'a>(inner: &'a swc_ast::TplElement, bump: &'a Bump) -> &'a TplElement<'a> {
  let node = bump.alloc(TplElement {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_tpl_element<'a>(node: &TplElement<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TplElement<'a> as *mut TplElement<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TryStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TryStmt,
  pub block: &'a BlockStmt<'a>,
  pub handler: Option<&'a CatchClause<'a>>,
  pub finalizer: Option<&'a BlockStmt<'a>>,
}

impl<'a> TryStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TryStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TryStmt<'a>> for Node<'a> {
  fn from(node: &TryStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TryStmt<'a>, &'a TryStmt<'a>>(node) };
    Node::TryStmt(node)
  }
}

impl<'a> NodeTrait<'a> for TryStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TryStmt
  }
}

impl<'a> CastableNode<'a> for TryStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TryStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TryStmt
  }
}

fn get_view_for_try_stmt<'a>(inner: &'a swc_ast::TryStmt, bump: &'a Bump) -> &'a TryStmt<'a> {
  let node = bump.alloc(TryStmt {
    inner,
    parent: None,
    block: get_view_for_block_stmt(&inner.block, bump),
    handler: match &inner.handler {
      Some(value) => Some(get_view_for_catch_clause(value, bump)),
      None => None,
    },
    finalizer: match &inner.finalizer {
      Some(value) => Some(get_view_for_block_stmt(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_block_stmt(&node.block, parent);
  if let Some(value) = &node.handler {
    set_parent_for_catch_clause(value, parent)
  };
  if let Some(value) = &node.finalizer {
    set_parent_for_block_stmt(value, parent)
  };
  node
}

fn set_parent_for_try_stmt<'a>(node: &TryStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TryStmt<'a> as *mut TryStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsArrayType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsArrayType,
  pub elem_type: TsType<'a>,
}

impl<'a> TsArrayType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsArrayType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsArrayType<'a>> for Node<'a> {
  fn from(node: &TsArrayType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsArrayType<'a>, &'a TsArrayType<'a>>(node) };
    Node::TsArrayType(node)
  }
}

impl<'a> NodeTrait<'a> for TsArrayType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.elem_type).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsArrayType
  }
}

impl<'a> CastableNode<'a> for TsArrayType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsArrayType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsArrayType
  }
}

fn get_view_for_ts_array_type<'a>(inner: &'a swc_ast::TsArrayType, bump: &'a Bump) -> &'a TsArrayType<'a> {
  let node = bump.alloc(TsArrayType {
    inner,
    parent: None,
    elem_type: get_view_for_ts_type(&inner.elem_type, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type(&node.elem_type, parent);
  node
}

fn set_parent_for_ts_array_type<'a>(node: &TsArrayType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsArrayType<'a> as *mut TsArrayType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsAsExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsAsExpr,
  pub expr: Expr<'a>,
  pub type_ann: TsType<'a>,
}

impl<'a> TsAsExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsAsExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsAsExpr<'a>> for Node<'a> {
  fn from(node: &TsAsExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsAsExpr<'a>, &'a TsAsExpr<'a>>(node) };
    Node::TsAsExpr(node)
  }
}

impl<'a> NodeTrait<'a> for TsAsExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push((&self.type_ann).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsAsExpr
  }
}

impl<'a> CastableNode<'a> for TsAsExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsAsExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsAsExpr
  }
}

fn get_view_for_ts_as_expr<'a>(inner: &'a swc_ast::TsAsExpr, bump: &'a Bump) -> &'a TsAsExpr<'a> {
  let node = bump.alloc(TsAsExpr {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
    type_ann: get_view_for_ts_type(&inner.type_ann, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  set_parent_for_ts_type(&node.type_ann, parent);
  node
}

fn set_parent_for_ts_as_expr<'a>(node: &TsAsExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsAsExpr<'a> as *mut TsAsExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsCallSignatureDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsCallSignatureDecl,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
}

impl<'a> TsCallSignatureDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsCallSignatureDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsCallSignatureDecl<'a>> for Node<'a> {
  fn from(node: &TsCallSignatureDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsCallSignatureDecl<'a>, &'a TsCallSignatureDecl<'a>>(node) };
    Node::TsCallSignatureDecl(node)
  }
}

impl<'a> NodeTrait<'a> for TsCallSignatureDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsCallSignatureDecl
  }
}

impl<'a> CastableNode<'a> for TsCallSignatureDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsCallSignatureDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsCallSignatureDecl
  }
}

fn get_view_for_ts_call_signature_decl<'a>(inner: &'a swc_ast::TsCallSignatureDecl, bump: &'a Bump) -> &'a TsCallSignatureDecl<'a> {
  let node = bump.alloc(TsCallSignatureDecl {
    inner,
    parent: None,
    params: inner.params.iter().map(|value| get_view_for_ts_fn_param(value, bump)).collect(),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.params.iter() {
    set_parent_for_ts_fn_param(value, parent)
  }
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  node
}

fn set_parent_for_ts_call_signature_decl<'a>(node: &TsCallSignatureDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsCallSignatureDecl<'a> as *mut TsCallSignatureDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsConditionalType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsConditionalType,
  pub check_type: TsType<'a>,
  pub extends_type: TsType<'a>,
  pub true_type: TsType<'a>,
  pub false_type: TsType<'a>,
}

impl<'a> TsConditionalType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsConditionalType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsConditionalType<'a>> for Node<'a> {
  fn from(node: &TsConditionalType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsConditionalType<'a>, &'a TsConditionalType<'a>>(node) };
    Node::TsConditionalType(node)
  }
}

impl<'a> NodeTrait<'a> for TsConditionalType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(4);
    children.push((&self.check_type).into());
    children.push((&self.extends_type).into());
    children.push((&self.true_type).into());
    children.push((&self.false_type).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsConditionalType
  }
}

impl<'a> CastableNode<'a> for TsConditionalType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsConditionalType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsConditionalType
  }
}

fn get_view_for_ts_conditional_type<'a>(inner: &'a swc_ast::TsConditionalType, bump: &'a Bump) -> &'a TsConditionalType<'a> {
  let node = bump.alloc(TsConditionalType {
    inner,
    parent: None,
    check_type: get_view_for_ts_type(&inner.check_type, bump),
    extends_type: get_view_for_ts_type(&inner.extends_type, bump),
    true_type: get_view_for_ts_type(&inner.true_type, bump),
    false_type: get_view_for_ts_type(&inner.false_type, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type(&node.check_type, parent);
  set_parent_for_ts_type(&node.extends_type, parent);
  set_parent_for_ts_type(&node.true_type, parent);
  set_parent_for_ts_type(&node.false_type, parent);
  node
}

fn set_parent_for_ts_conditional_type<'a>(node: &TsConditionalType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsConditionalType<'a> as *mut TsConditionalType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsConstAssertion<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsConstAssertion,
  pub expr: Expr<'a>,
}

impl<'a> TsConstAssertion<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsConstAssertion<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsConstAssertion<'a>> for Node<'a> {
  fn from(node: &TsConstAssertion<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsConstAssertion<'a>, &'a TsConstAssertion<'a>>(node) };
    Node::TsConstAssertion(node)
  }
}

impl<'a> NodeTrait<'a> for TsConstAssertion<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsConstAssertion
  }
}

impl<'a> CastableNode<'a> for TsConstAssertion<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsConstAssertion(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsConstAssertion
  }
}

fn get_view_for_ts_const_assertion<'a>(inner: &'a swc_ast::TsConstAssertion, bump: &'a Bump) -> &'a TsConstAssertion<'a> {
  let node = bump.alloc(TsConstAssertion {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_ts_const_assertion<'a>(node: &TsConstAssertion<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsConstAssertion<'a> as *mut TsConstAssertion<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsConstructSignatureDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsConstructSignatureDecl,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
}

impl<'a> TsConstructSignatureDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsConstructSignatureDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsConstructSignatureDecl<'a>> for Node<'a> {
  fn from(node: &TsConstructSignatureDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsConstructSignatureDecl<'a>, &'a TsConstructSignatureDecl<'a>>(node) };
    Node::TsConstructSignatureDecl(node)
  }
}

impl<'a> NodeTrait<'a> for TsConstructSignatureDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsConstructSignatureDecl
  }
}

impl<'a> CastableNode<'a> for TsConstructSignatureDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsConstructSignatureDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsConstructSignatureDecl
  }
}

fn get_view_for_ts_construct_signature_decl<'a>(inner: &'a swc_ast::TsConstructSignatureDecl, bump: &'a Bump) -> &'a TsConstructSignatureDecl<'a> {
  let node = bump.alloc(TsConstructSignatureDecl {
    inner,
    parent: None,
    params: inner.params.iter().map(|value| get_view_for_ts_fn_param(value, bump)).collect(),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.params.iter() {
    set_parent_for_ts_fn_param(value, parent)
  }
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  node
}

fn set_parent_for_ts_construct_signature_decl<'a>(node: &TsConstructSignatureDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsConstructSignatureDecl<'a> as *mut TsConstructSignatureDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsConstructorType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsConstructorType,
  pub params: Vec<TsFnParam<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub type_ann: &'a TsTypeAnn<'a>,
}

impl<'a> TsConstructorType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn is_abstract(&self) -> bool {
    self.inner.is_abstract
  }
}

impl<'a> SourceRanged for TsConstructorType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsConstructorType<'a>> for Node<'a> {
  fn from(node: &TsConstructorType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsConstructorType<'a>, &'a TsConstructorType<'a>>(node) };
    Node::TsConstructorType(node)
  }
}

impl<'a> NodeTrait<'a> for TsConstructorType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsConstructorType
  }
}

impl<'a> CastableNode<'a> for TsConstructorType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsConstructorType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsConstructorType
  }
}

fn get_view_for_ts_constructor_type<'a>(inner: &'a swc_ast::TsConstructorType, bump: &'a Bump) -> &'a TsConstructorType<'a> {
  let node = bump.alloc(TsConstructorType {
    inner,
    parent: None,
    params: inner.params.iter().map(|value| get_view_for_ts_fn_param(value, bump)).collect(),
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
    type_ann: get_view_for_ts_type_ann(&inner.type_ann, bump),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.params.iter() {
    set_parent_for_ts_fn_param(value, parent)
  }
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  set_parent_for_ts_type_ann(&node.type_ann, parent);
  node
}

fn set_parent_for_ts_constructor_type<'a>(node: &TsConstructorType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsConstructorType<'a> as *mut TsConstructorType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsEnumDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsEnumDecl,
  pub id: &'a Ident<'a>,
  pub members: Vec<&'a TsEnumMember<'a>>,
}

impl<'a> TsEnumDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  pub fn is_const(&self) -> bool {
    self.inner.is_const
  }
}

impl<'a> SourceRanged for TsEnumDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsEnumDecl<'a>> for Node<'a> {
  fn from(node: &TsEnumDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsEnumDecl<'a>, &'a TsEnumDecl<'a>>(node) };
    Node::TsEnumDecl(node)
  }
}

impl<'a> NodeTrait<'a> for TsEnumDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.members.len());
    children.push(self.id.into());
    for child in self.members.iter() {
      children.push((*child).into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsEnumDecl
  }
}

impl<'a> CastableNode<'a> for TsEnumDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsEnumDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsEnumDecl
  }
}

fn get_view_for_ts_enum_decl<'a>(inner: &'a swc_ast::TsEnumDecl, bump: &'a Bump) -> &'a TsEnumDecl<'a> {
  let node = bump.alloc(TsEnumDecl {
    inner,
    parent: None,
    id: get_view_for_ident(&inner.id, bump),
    members: inner.members.iter().map(|value| get_view_for_ts_enum_member(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.id, parent);
  for value in node.members.iter() {
    set_parent_for_ts_enum_member(value, parent)
  }
  node
}

fn set_parent_for_ts_enum_decl<'a>(node: &TsEnumDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsEnumDecl<'a> as *mut TsEnumDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsEnumMember<'a> {
  parent: Option<&'a TsEnumDecl<'a>>,
  pub inner: &'a swc_ast::TsEnumMember,
  pub id: TsEnumMemberId<'a>,
  pub init: Option<Expr<'a>>,
}

impl<'a> TsEnumMember<'a> {
  pub fn parent(&self) -> &'a TsEnumDecl<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsEnumMember<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsEnumMember<'a>> for Node<'a> {
  fn from(node: &TsEnumMember<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsEnumMember<'a>, &'a TsEnumMember<'a>>(node) };
    Node::TsEnumMember(node)
  }
}

impl<'a> NodeTrait<'a> for TsEnumMember<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, });
    children.push((&self.id).into());
    if let Some(child) = self.init.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsEnumMember
  }
}

impl<'a> CastableNode<'a> for TsEnumMember<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsEnumMember(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsEnumMember
  }
}

fn get_view_for_ts_enum_member<'a>(inner: &'a swc_ast::TsEnumMember, bump: &'a Bump) -> &'a TsEnumMember<'a> {
  let node = bump.alloc(TsEnumMember {
    inner,
    parent: None,
    id: get_view_for_ts_enum_member_id(&inner.id, bump),
    init: match &inner.init {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_enum_member_id(&node.id, parent);
  if let Some(value) = &node.init {
    set_parent_for_expr(value, parent)
  };
  node
}

fn set_parent_for_ts_enum_member<'a>(node: &TsEnumMember<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsEnumMember<'a> as *mut TsEnumMember<'a>;
    (*node_ptr).parent.replace(parent.expect::<TsEnumDecl>());
  }
}

/// TypeScript's own parser uses ExportAssignment for both `export default` and
/// `export =`. But for @babel/parser, `export default` is an ExportDefaultDecl,
/// so a TsExportAssignment is always `export =`.
#[derive(Clone)]
pub struct TsExportAssignment<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsExportAssignment,
  pub expr: Expr<'a>,
}

impl<'a> TsExportAssignment<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsExportAssignment<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsExportAssignment<'a>> for Node<'a> {
  fn from(node: &TsExportAssignment<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsExportAssignment<'a>, &'a TsExportAssignment<'a>>(node) };
    Node::TsExportAssignment(node)
  }
}

impl<'a> NodeTrait<'a> for TsExportAssignment<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsExportAssignment
  }
}

impl<'a> CastableNode<'a> for TsExportAssignment<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsExportAssignment(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsExportAssignment
  }
}

fn get_view_for_ts_export_assignment<'a>(inner: &'a swc_ast::TsExportAssignment, bump: &'a Bump) -> &'a TsExportAssignment<'a> {
  let node = bump.alloc(TsExportAssignment {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_ts_export_assignment<'a>(node: &TsExportAssignment<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsExportAssignment<'a> as *mut TsExportAssignment<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsExprWithTypeArgs<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsExprWithTypeArgs,
  pub expr: Expr<'a>,
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> TsExprWithTypeArgs<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsExprWithTypeArgs<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsExprWithTypeArgs<'a>> for Node<'a> {
  fn from(node: &TsExprWithTypeArgs<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsExprWithTypeArgs<'a>, &'a TsExprWithTypeArgs<'a>>(node) };
    Node::TsExprWithTypeArgs(node)
  }
}

impl<'a> NodeTrait<'a> for TsExprWithTypeArgs<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.expr).into());
    if let Some(child) = self.type_args {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsExprWithTypeArgs
  }
}

impl<'a> CastableNode<'a> for TsExprWithTypeArgs<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsExprWithTypeArgs(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsExprWithTypeArgs
  }
}

fn get_view_for_ts_expr_with_type_args<'a>(inner: &'a swc_ast::TsExprWithTypeArgs, bump: &'a Bump) -> &'a TsExprWithTypeArgs<'a> {
  let node = bump.alloc(TsExprWithTypeArgs {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
    type_args: match &inner.type_args {
      Some(value) => Some(get_view_for_ts_type_param_instantiation(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  if let Some(value) = &node.type_args {
    set_parent_for_ts_type_param_instantiation(value, parent)
  };
  node
}

fn set_parent_for_ts_expr_with_type_args<'a>(node: &TsExprWithTypeArgs<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsExprWithTypeArgs<'a> as *mut TsExprWithTypeArgs<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsExternalModuleRef<'a> {
  parent: Option<&'a TsImportEqualsDecl<'a>>,
  pub inner: &'a swc_ast::TsExternalModuleRef,
  pub expr: &'a Str<'a>,
}

impl<'a> TsExternalModuleRef<'a> {
  pub fn parent(&self) -> &'a TsImportEqualsDecl<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsExternalModuleRef<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsExternalModuleRef<'a>> for Node<'a> {
  fn from(node: &TsExternalModuleRef<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsExternalModuleRef<'a>, &'a TsExternalModuleRef<'a>>(node) };
    Node::TsExternalModuleRef(node)
  }
}

impl<'a> NodeTrait<'a> for TsExternalModuleRef<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.expr.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsExternalModuleRef
  }
}

impl<'a> CastableNode<'a> for TsExternalModuleRef<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsExternalModuleRef(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsExternalModuleRef
  }
}

fn get_view_for_ts_external_module_ref<'a>(inner: &'a swc_ast::TsExternalModuleRef, bump: &'a Bump) -> &'a TsExternalModuleRef<'a> {
  let node = bump.alloc(TsExternalModuleRef {
    inner,
    parent: None,
    expr: get_view_for_str(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_str(&node.expr, parent);
  node
}

fn set_parent_for_ts_external_module_ref<'a>(node: &TsExternalModuleRef<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsExternalModuleRef<'a> as *mut TsExternalModuleRef<'a>;
    (*node_ptr).parent.replace(parent.expect::<TsImportEqualsDecl>());
  }
}

#[derive(Clone)]
pub struct TsFnType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsFnType,
  pub params: Vec<TsFnParam<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub type_ann: &'a TsTypeAnn<'a>,
}

impl<'a> TsFnType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsFnType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsFnType<'a>> for Node<'a> {
  fn from(node: &TsFnType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsFnType<'a>, &'a TsFnType<'a>>(node) };
    Node::TsFnType(node)
  }
}

impl<'a> NodeTrait<'a> for TsFnType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsFnType
  }
}

impl<'a> CastableNode<'a> for TsFnType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsFnType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsFnType
  }
}

fn get_view_for_ts_fn_type<'a>(inner: &'a swc_ast::TsFnType, bump: &'a Bump) -> &'a TsFnType<'a> {
  let node = bump.alloc(TsFnType {
    inner,
    parent: None,
    params: inner.params.iter().map(|value| get_view_for_ts_fn_param(value, bump)).collect(),
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
    type_ann: get_view_for_ts_type_ann(&inner.type_ann, bump),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.params.iter() {
    set_parent_for_ts_fn_param(value, parent)
  }
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  set_parent_for_ts_type_ann(&node.type_ann, parent);
  node
}

fn set_parent_for_ts_fn_type<'a>(node: &TsFnType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsFnType<'a> as *mut TsFnType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsGetterSignature<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsGetterSignature,
  pub key: Expr<'a>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> TsGetterSignature<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

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

impl<'a> SourceRanged for TsGetterSignature<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsGetterSignature<'a>> for Node<'a> {
  fn from(node: &TsGetterSignature<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsGetterSignature<'a>, &'a TsGetterSignature<'a>>(node) };
    Node::TsGetterSignature(node)
  }
}

impl<'a> NodeTrait<'a> for TsGetterSignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.key).into());
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsGetterSignature
  }
}

impl<'a> CastableNode<'a> for TsGetterSignature<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsGetterSignature(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsGetterSignature
  }
}

fn get_view_for_ts_getter_signature<'a>(inner: &'a swc_ast::TsGetterSignature, bump: &'a Bump) -> &'a TsGetterSignature<'a> {
  let node = bump.alloc(TsGetterSignature {
    inner,
    parent: None,
    key: get_view_for_expr(&inner.key, bump),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.key, parent);
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  node
}

fn set_parent_for_ts_getter_signature<'a>(node: &TsGetterSignature<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsGetterSignature<'a> as *mut TsGetterSignature<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsImportEqualsDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsImportEqualsDecl,
  pub id: &'a Ident<'a>,
  pub module_ref: TsModuleRef<'a>,
}

impl<'a> TsImportEqualsDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  pub fn is_export(&self) -> bool {
    self.inner.is_export
  }

  pub fn is_type_only(&self) -> bool {
    self.inner.is_type_only
  }
}

impl<'a> SourceRanged for TsImportEqualsDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsImportEqualsDecl<'a>> for Node<'a> {
  fn from(node: &TsImportEqualsDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsImportEqualsDecl<'a>, &'a TsImportEqualsDecl<'a>>(node) };
    Node::TsImportEqualsDecl(node)
  }
}

impl<'a> NodeTrait<'a> for TsImportEqualsDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.id.into());
    children.push((&self.module_ref).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsImportEqualsDecl
  }
}

impl<'a> CastableNode<'a> for TsImportEqualsDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsImportEqualsDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsImportEqualsDecl
  }
}

fn get_view_for_ts_import_equals_decl<'a>(inner: &'a swc_ast::TsImportEqualsDecl, bump: &'a Bump) -> &'a TsImportEqualsDecl<'a> {
  let node = bump.alloc(TsImportEqualsDecl {
    inner,
    parent: None,
    id: get_view_for_ident(&inner.id, bump),
    module_ref: get_view_for_ts_module_ref(&inner.module_ref, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.id, parent);
  set_parent_for_ts_module_ref(&node.module_ref, parent);
  node
}

fn set_parent_for_ts_import_equals_decl<'a>(node: &TsImportEqualsDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsImportEqualsDecl<'a> as *mut TsImportEqualsDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsImportType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsImportType,
  pub arg: &'a Str<'a>,
  pub qualifier: Option<TsEntityName<'a>>,
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> TsImportType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsImportType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsImportType<'a>> for Node<'a> {
  fn from(node: &TsImportType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsImportType<'a>, &'a TsImportType<'a>>(node) };
    Node::TsImportType(node)
  }
}

impl<'a> NodeTrait<'a> for TsImportType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsImportType
  }
}

impl<'a> CastableNode<'a> for TsImportType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsImportType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsImportType
  }
}

fn get_view_for_ts_import_type<'a>(inner: &'a swc_ast::TsImportType, bump: &'a Bump) -> &'a TsImportType<'a> {
  let node = bump.alloc(TsImportType {
    inner,
    parent: None,
    arg: get_view_for_str(&inner.arg, bump),
    qualifier: match &inner.qualifier {
      Some(value) => Some(get_view_for_ts_entity_name(value, bump)),
      None => None,
    },
    type_args: match &inner.type_args {
      Some(value) => Some(get_view_for_ts_type_param_instantiation(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_str(&node.arg, parent);
  if let Some(value) = &node.qualifier {
    set_parent_for_ts_entity_name(value, parent)
  };
  if let Some(value) = &node.type_args {
    set_parent_for_ts_type_param_instantiation(value, parent)
  };
  node
}

fn set_parent_for_ts_import_type<'a>(node: &TsImportType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsImportType<'a> as *mut TsImportType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsIndexSignature<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsIndexSignature,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> TsIndexSignature<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }

  pub fn is_static(&self) -> bool {
    self.inner.is_static
  }
}

impl<'a> SourceRanged for TsIndexSignature<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsIndexSignature<'a>> for Node<'a> {
  fn from(node: &TsIndexSignature<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsIndexSignature<'a>, &'a TsIndexSignature<'a>>(node) };
    Node::TsIndexSignature(node)
  }
}

impl<'a> NodeTrait<'a> for TsIndexSignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsIndexSignature
  }
}

impl<'a> CastableNode<'a> for TsIndexSignature<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsIndexSignature(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsIndexSignature
  }
}

fn get_view_for_ts_index_signature<'a>(inner: &'a swc_ast::TsIndexSignature, bump: &'a Bump) -> &'a TsIndexSignature<'a> {
  let node = bump.alloc(TsIndexSignature {
    inner,
    parent: None,
    params: inner.params.iter().map(|value| get_view_for_ts_fn_param(value, bump)).collect(),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.params.iter() {
    set_parent_for_ts_fn_param(value, parent)
  }
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  node
}

fn set_parent_for_ts_index_signature<'a>(node: &TsIndexSignature<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsIndexSignature<'a> as *mut TsIndexSignature<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsIndexedAccessType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsIndexedAccessType,
  pub obj_type: TsType<'a>,
  pub index_type: TsType<'a>,
}

impl<'a> TsIndexedAccessType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }
}

impl<'a> SourceRanged for TsIndexedAccessType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsIndexedAccessType<'a>> for Node<'a> {
  fn from(node: &TsIndexedAccessType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsIndexedAccessType<'a>, &'a TsIndexedAccessType<'a>>(node) };
    Node::TsIndexedAccessType(node)
  }
}

impl<'a> NodeTrait<'a> for TsIndexedAccessType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj_type).into());
    children.push((&self.index_type).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsIndexedAccessType
  }
}

impl<'a> CastableNode<'a> for TsIndexedAccessType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsIndexedAccessType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsIndexedAccessType
  }
}

fn get_view_for_ts_indexed_access_type<'a>(inner: &'a swc_ast::TsIndexedAccessType, bump: &'a Bump) -> &'a TsIndexedAccessType<'a> {
  let node = bump.alloc(TsIndexedAccessType {
    inner,
    parent: None,
    obj_type: get_view_for_ts_type(&inner.obj_type, bump),
    index_type: get_view_for_ts_type(&inner.index_type, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type(&node.obj_type, parent);
  set_parent_for_ts_type(&node.index_type, parent);
  node
}

fn set_parent_for_ts_indexed_access_type<'a>(node: &TsIndexedAccessType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsIndexedAccessType<'a> as *mut TsIndexedAccessType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsInferType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsInferType,
  pub type_param: &'a TsTypeParam<'a>,
}

impl<'a> TsInferType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsInferType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsInferType<'a>> for Node<'a> {
  fn from(node: &TsInferType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsInferType<'a>, &'a TsInferType<'a>>(node) };
    Node::TsInferType(node)
  }
}

impl<'a> NodeTrait<'a> for TsInferType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.type_param.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsInferType
  }
}

impl<'a> CastableNode<'a> for TsInferType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsInferType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsInferType
  }
}

fn get_view_for_ts_infer_type<'a>(inner: &'a swc_ast::TsInferType, bump: &'a Bump) -> &'a TsInferType<'a> {
  let node = bump.alloc(TsInferType {
    inner,
    parent: None,
    type_param: get_view_for_ts_type_param(&inner.type_param, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type_param(&node.type_param, parent);
  node
}

fn set_parent_for_ts_infer_type<'a>(node: &TsInferType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsInferType<'a> as *mut TsInferType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsInstantiation<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsInstantiation,
  pub expr: Expr<'a>,
  pub type_args: &'a TsTypeParamInstantiation<'a>,
}

impl<'a> TsInstantiation<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsInstantiation<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsInstantiation<'a>> for Node<'a> {
  fn from(node: &TsInstantiation<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsInstantiation<'a>, &'a TsInstantiation<'a>>(node) };
    Node::TsInstantiation(node)
  }
}

impl<'a> NodeTrait<'a> for TsInstantiation<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push(self.type_args.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsInstantiation
  }
}

impl<'a> CastableNode<'a> for TsInstantiation<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsInstantiation(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsInstantiation
  }
}

fn get_view_for_ts_instantiation<'a>(inner: &'a swc_ast::TsInstantiation, bump: &'a Bump) -> &'a TsInstantiation<'a> {
  let node = bump.alloc(TsInstantiation {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
    type_args: get_view_for_ts_type_param_instantiation(&inner.type_args, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  set_parent_for_ts_type_param_instantiation(&node.type_args, parent);
  node
}

fn set_parent_for_ts_instantiation<'a>(node: &TsInstantiation<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsInstantiation<'a> as *mut TsInstantiation<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsInterfaceBody<'a> {
  parent: Option<&'a TsInterfaceDecl<'a>>,
  pub inner: &'a swc_ast::TsInterfaceBody,
  pub body: Vec<TsTypeElement<'a>>,
}

impl<'a> TsInterfaceBody<'a> {
  pub fn parent(&self) -> &'a TsInterfaceDecl<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsInterfaceBody<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsInterfaceBody<'a>> for Node<'a> {
  fn from(node: &TsInterfaceBody<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsInterfaceBody<'a>, &'a TsInterfaceBody<'a>>(node) };
    Node::TsInterfaceBody(node)
  }
}

impl<'a> NodeTrait<'a> for TsInterfaceBody<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.body.len());
    for child in self.body.iter() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsInterfaceBody
  }
}

impl<'a> CastableNode<'a> for TsInterfaceBody<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsInterfaceBody(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsInterfaceBody
  }
}

fn get_view_for_ts_interface_body<'a>(inner: &'a swc_ast::TsInterfaceBody, bump: &'a Bump) -> &'a TsInterfaceBody<'a> {
  let node = bump.alloc(TsInterfaceBody {
    inner,
    parent: None,
    body: inner.body.iter().map(|value| get_view_for_ts_type_element(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.body.iter() {
    set_parent_for_ts_type_element(value, parent)
  }
  node
}

fn set_parent_for_ts_interface_body<'a>(node: &TsInterfaceBody<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsInterfaceBody<'a> as *mut TsInterfaceBody<'a>;
    (*node_ptr).parent.replace(parent.expect::<TsInterfaceDecl>());
  }
}

#[derive(Clone)]
pub struct TsInterfaceDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsInterfaceDecl,
  pub id: &'a Ident<'a>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub extends: Vec<&'a TsExprWithTypeArgs<'a>>,
  pub body: &'a TsInterfaceBody<'a>,
}

impl<'a> TsInterfaceDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl<'a> SourceRanged for TsInterfaceDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsInterfaceDecl<'a>> for Node<'a> {
  fn from(node: &TsInterfaceDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsInterfaceDecl<'a>, &'a TsInterfaceDecl<'a>>(node) };
    Node::TsInterfaceDecl(node)
  }
}

impl<'a> NodeTrait<'a> for TsInterfaceDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsInterfaceDecl
  }
}

impl<'a> CastableNode<'a> for TsInterfaceDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsInterfaceDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsInterfaceDecl
  }
}

fn get_view_for_ts_interface_decl<'a>(inner: &'a swc_ast::TsInterfaceDecl, bump: &'a Bump) -> &'a TsInterfaceDecl<'a> {
  let node = bump.alloc(TsInterfaceDecl {
    inner,
    parent: None,
    id: get_view_for_ident(&inner.id, bump),
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
    extends: inner.extends.iter().map(|value| get_view_for_ts_expr_with_type_args(value, bump)).collect(),
    body: get_view_for_ts_interface_body(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.id, parent);
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  for value in node.extends.iter() {
    set_parent_for_ts_expr_with_type_args(value, parent)
  }
  set_parent_for_ts_interface_body(&node.body, parent);
  node
}

fn set_parent_for_ts_interface_decl<'a>(node: &TsInterfaceDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsInterfaceDecl<'a> as *mut TsInterfaceDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsIntersectionType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsIntersectionType,
  pub types: Vec<TsType<'a>>,
}

impl<'a> TsIntersectionType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsIntersectionType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsIntersectionType<'a>> for Node<'a> {
  fn from(node: &TsIntersectionType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsIntersectionType<'a>, &'a TsIntersectionType<'a>>(node) };
    Node::TsIntersectionType(node)
  }
}

impl<'a> NodeTrait<'a> for TsIntersectionType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.types.len());
    for child in self.types.iter() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsIntersectionType
  }
}

impl<'a> CastableNode<'a> for TsIntersectionType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsIntersectionType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsIntersectionType
  }
}

fn get_view_for_ts_intersection_type<'a>(inner: &'a swc_ast::TsIntersectionType, bump: &'a Bump) -> &'a TsIntersectionType<'a> {
  let node = bump.alloc(TsIntersectionType {
    inner,
    parent: None,
    types: inner.types.iter().map(|value| get_view_for_ts_type(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.types.iter() {
    set_parent_for_ts_type(value, parent)
  }
  node
}

fn set_parent_for_ts_intersection_type<'a>(node: &TsIntersectionType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsIntersectionType<'a> as *mut TsIntersectionType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsKeywordType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsKeywordType,
}

impl<'a> TsKeywordType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn keyword_kind(&self) -> TsKeywordTypeKind {
    self.inner.kind
  }
}

impl<'a> SourceRanged for TsKeywordType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsKeywordType<'a>> for Node<'a> {
  fn from(node: &TsKeywordType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsKeywordType<'a>, &'a TsKeywordType<'a>>(node) };
    Node::TsKeywordType(node)
  }
}

impl<'a> NodeTrait<'a> for TsKeywordType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsKeywordType
  }
}

impl<'a> CastableNode<'a> for TsKeywordType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsKeywordType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsKeywordType
  }
}

fn get_view_for_ts_keyword_type<'a>(inner: &'a swc_ast::TsKeywordType, bump: &'a Bump) -> &'a TsKeywordType<'a> {
  let node = bump.alloc(TsKeywordType {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_ts_keyword_type<'a>(node: &TsKeywordType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsKeywordType<'a> as *mut TsKeywordType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsLitType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsLitType,
  pub lit: TsLit<'a>,
}

impl<'a> TsLitType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsLitType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsLitType<'a>> for Node<'a> {
  fn from(node: &TsLitType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsLitType<'a>, &'a TsLitType<'a>>(node) };
    Node::TsLitType(node)
  }
}

impl<'a> NodeTrait<'a> for TsLitType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.lit).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsLitType
  }
}

impl<'a> CastableNode<'a> for TsLitType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsLitType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsLitType
  }
}

fn get_view_for_ts_lit_type<'a>(inner: &'a swc_ast::TsLitType, bump: &'a Bump) -> &'a TsLitType<'a> {
  let node = bump.alloc(TsLitType {
    inner,
    parent: None,
    lit: get_view_for_ts_lit(&inner.lit, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_lit(&node.lit, parent);
  node
}

fn set_parent_for_ts_lit_type<'a>(node: &TsLitType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsLitType<'a> as *mut TsLitType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsMappedType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsMappedType,
  pub type_param: &'a TsTypeParam<'a>,
  pub name_type: Option<TsType<'a>>,
  pub type_ann: Option<TsType<'a>>,
}

impl<'a> TsMappedType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn readonly(&self) -> Option<TruePlusMinus> {
    self.inner.readonly
  }

  pub fn optional(&self) -> Option<TruePlusMinus> {
    self.inner.optional
  }
}

impl<'a> SourceRanged for TsMappedType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsMappedType<'a>> for Node<'a> {
  fn from(node: &TsMappedType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsMappedType<'a>, &'a TsMappedType<'a>>(node) };
    Node::TsMappedType(node)
  }
}

impl<'a> NodeTrait<'a> for TsMappedType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsMappedType
  }
}

impl<'a> CastableNode<'a> for TsMappedType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsMappedType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsMappedType
  }
}

fn get_view_for_ts_mapped_type<'a>(inner: &'a swc_ast::TsMappedType, bump: &'a Bump) -> &'a TsMappedType<'a> {
  let node = bump.alloc(TsMappedType {
    inner,
    parent: None,
    type_param: get_view_for_ts_type_param(&inner.type_param, bump),
    name_type: match &inner.name_type {
      Some(value) => Some(get_view_for_ts_type(value, bump)),
      None => None,
    },
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type_param(&node.type_param, parent);
  if let Some(value) = &node.name_type {
    set_parent_for_ts_type(value, parent)
  };
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type(value, parent)
  };
  node
}

fn set_parent_for_ts_mapped_type<'a>(node: &TsMappedType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsMappedType<'a> as *mut TsMappedType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsMethodSignature<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsMethodSignature,
  pub key: Expr<'a>,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
}

impl<'a> TsMethodSignature<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

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

impl<'a> SourceRanged for TsMethodSignature<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsMethodSignature<'a>> for Node<'a> {
  fn from(node: &TsMethodSignature<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsMethodSignature<'a>, &'a TsMethodSignature<'a>>(node) };
    Node::TsMethodSignature(node)
  }
}

impl<'a> NodeTrait<'a> for TsMethodSignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsMethodSignature
  }
}

impl<'a> CastableNode<'a> for TsMethodSignature<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsMethodSignature(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsMethodSignature
  }
}

fn get_view_for_ts_method_signature<'a>(inner: &'a swc_ast::TsMethodSignature, bump: &'a Bump) -> &'a TsMethodSignature<'a> {
  let node = bump.alloc(TsMethodSignature {
    inner,
    parent: None,
    key: get_view_for_expr(&inner.key, bump),
    params: inner.params.iter().map(|value| get_view_for_ts_fn_param(value, bump)).collect(),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.key, parent);
  for value in node.params.iter() {
    set_parent_for_ts_fn_param(value, parent)
  }
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  node
}

fn set_parent_for_ts_method_signature<'a>(node: &TsMethodSignature<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsMethodSignature<'a> as *mut TsMethodSignature<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsModuleBlock<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsModuleBlock,
  pub body: Vec<ModuleItem<'a>>,
}

impl<'a> TsModuleBlock<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsModuleBlock<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsModuleBlock<'a>> for Node<'a> {
  fn from(node: &TsModuleBlock<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsModuleBlock<'a>, &'a TsModuleBlock<'a>>(node) };
    Node::TsModuleBlock(node)
  }
}

impl<'a> NodeTrait<'a> for TsModuleBlock<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.body.len());
    for child in self.body.iter() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsModuleBlock
  }
}

impl<'a> CastableNode<'a> for TsModuleBlock<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsModuleBlock(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsModuleBlock
  }
}

fn get_view_for_ts_module_block<'a>(inner: &'a swc_ast::TsModuleBlock, bump: &'a Bump) -> &'a TsModuleBlock<'a> {
  let node = bump.alloc(TsModuleBlock {
    inner,
    parent: None,
    body: inner.body.iter().map(|value| get_view_for_module_item(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.body.iter() {
    set_parent_for_module_item(value, parent)
  }
  node
}

fn set_parent_for_ts_module_block<'a>(node: &TsModuleBlock<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsModuleBlock<'a> as *mut TsModuleBlock<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsModuleDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsModuleDecl,
  pub id: TsModuleName<'a>,
  pub body: Option<TsNamespaceBody<'a>>,
}

impl<'a> TsModuleDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  /// In TypeScript, this is only available through`node.flags`.
  pub fn global(&self) -> bool {
    self.inner.global
  }
}

impl<'a> SourceRanged for TsModuleDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsModuleDecl<'a>> for Node<'a> {
  fn from(node: &TsModuleDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsModuleDecl<'a>, &'a TsModuleDecl<'a>>(node) };
    Node::TsModuleDecl(node)
  }
}

impl<'a> NodeTrait<'a> for TsModuleDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.body { Some(_value) => 1, None => 0, });
    children.push((&self.id).into());
    if let Some(child) = self.body.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsModuleDecl
  }
}

impl<'a> CastableNode<'a> for TsModuleDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsModuleDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsModuleDecl
  }
}

fn get_view_for_ts_module_decl<'a>(inner: &'a swc_ast::TsModuleDecl, bump: &'a Bump) -> &'a TsModuleDecl<'a> {
  let node = bump.alloc(TsModuleDecl {
    inner,
    parent: None,
    id: get_view_for_ts_module_name(&inner.id, bump),
    body: match &inner.body {
      Some(value) => Some(get_view_for_ts_namespace_body(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_module_name(&node.id, parent);
  if let Some(value) = &node.body {
    set_parent_for_ts_namespace_body(value, parent)
  };
  node
}

fn set_parent_for_ts_module_decl<'a>(node: &TsModuleDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsModuleDecl<'a> as *mut TsModuleDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsNamespaceDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsNamespaceDecl,
  pub id: &'a Ident<'a>,
  pub body: TsNamespaceBody<'a>,
}

impl<'a> TsNamespaceDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }

  /// In TypeScript, this is only available through`node.flags`.
  pub fn global(&self) -> bool {
    self.inner.global
  }
}

impl<'a> SourceRanged for TsNamespaceDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsNamespaceDecl<'a>> for Node<'a> {
  fn from(node: &TsNamespaceDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsNamespaceDecl<'a>, &'a TsNamespaceDecl<'a>>(node) };
    Node::TsNamespaceDecl(node)
  }
}

impl<'a> NodeTrait<'a> for TsNamespaceDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push(self.id.into());
    children.push((&self.body).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsNamespaceDecl
  }
}

impl<'a> CastableNode<'a> for TsNamespaceDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsNamespaceDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsNamespaceDecl
  }
}

fn get_view_for_ts_namespace_decl<'a>(inner: &'a swc_ast::TsNamespaceDecl, bump: &'a Bump) -> &'a TsNamespaceDecl<'a> {
  let node = bump.alloc(TsNamespaceDecl {
    inner,
    parent: None,
    id: get_view_for_ident(&inner.id, bump),
    body: get_view_for_ts_namespace_body(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.id, parent);
  set_parent_for_ts_namespace_body(&node.body, parent);
  node
}

fn set_parent_for_ts_namespace_decl<'a>(node: &TsNamespaceDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsNamespaceDecl<'a> as *mut TsNamespaceDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsNamespaceExportDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsNamespaceExportDecl,
  pub id: &'a Ident<'a>,
}

impl<'a> TsNamespaceExportDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsNamespaceExportDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsNamespaceExportDecl<'a>> for Node<'a> {
  fn from(node: &TsNamespaceExportDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsNamespaceExportDecl<'a>, &'a TsNamespaceExportDecl<'a>>(node) };
    Node::TsNamespaceExportDecl(node)
  }
}

impl<'a> NodeTrait<'a> for TsNamespaceExportDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push(self.id.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsNamespaceExportDecl
  }
}

impl<'a> CastableNode<'a> for TsNamespaceExportDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsNamespaceExportDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsNamespaceExportDecl
  }
}

fn get_view_for_ts_namespace_export_decl<'a>(inner: &'a swc_ast::TsNamespaceExportDecl, bump: &'a Bump) -> &'a TsNamespaceExportDecl<'a> {
  let node = bump.alloc(TsNamespaceExportDecl {
    inner,
    parent: None,
    id: get_view_for_ident(&inner.id, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.id, parent);
  node
}

fn set_parent_for_ts_namespace_export_decl<'a>(node: &TsNamespaceExportDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsNamespaceExportDecl<'a> as *mut TsNamespaceExportDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsNonNullExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsNonNullExpr,
  pub expr: Expr<'a>,
}

impl<'a> TsNonNullExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsNonNullExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsNonNullExpr<'a>> for Node<'a> {
  fn from(node: &TsNonNullExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsNonNullExpr<'a>, &'a TsNonNullExpr<'a>>(node) };
    Node::TsNonNullExpr(node)
  }
}

impl<'a> NodeTrait<'a> for TsNonNullExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.expr).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsNonNullExpr
  }
}

impl<'a> CastableNode<'a> for TsNonNullExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsNonNullExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsNonNullExpr
  }
}

fn get_view_for_ts_non_null_expr<'a>(inner: &'a swc_ast::TsNonNullExpr, bump: &'a Bump) -> &'a TsNonNullExpr<'a> {
  let node = bump.alloc(TsNonNullExpr {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  node
}

fn set_parent_for_ts_non_null_expr<'a>(node: &TsNonNullExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsNonNullExpr<'a> as *mut TsNonNullExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsOptionalType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsOptionalType,
  pub type_ann: TsType<'a>,
}

impl<'a> TsOptionalType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsOptionalType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsOptionalType<'a>> for Node<'a> {
  fn from(node: &TsOptionalType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsOptionalType<'a>, &'a TsOptionalType<'a>>(node) };
    Node::TsOptionalType(node)
  }
}

impl<'a> NodeTrait<'a> for TsOptionalType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsOptionalType
  }
}

impl<'a> CastableNode<'a> for TsOptionalType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsOptionalType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsOptionalType
  }
}

fn get_view_for_ts_optional_type<'a>(inner: &'a swc_ast::TsOptionalType, bump: &'a Bump) -> &'a TsOptionalType<'a> {
  let node = bump.alloc(TsOptionalType {
    inner,
    parent: None,
    type_ann: get_view_for_ts_type(&inner.type_ann, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type(&node.type_ann, parent);
  node
}

fn set_parent_for_ts_optional_type<'a>(node: &TsOptionalType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsOptionalType<'a> as *mut TsOptionalType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsParamProp<'a> {
  parent: Option<&'a Constructor<'a>>,
  pub inner: &'a swc_ast::TsParamProp,
  pub decorators: Vec<&'a Decorator<'a>>,
  pub param: TsParamPropParam<'a>,
}

impl<'a> TsParamProp<'a> {
  pub fn parent(&self) -> &'a Constructor<'a> {
    self.parent.unwrap()
  }

  /// At least one of `accessibility` or `readonly` must be set.
  pub fn accessibility(&self) -> Option<Accessibility> {
    self.inner.accessibility
  }

  pub fn is_override(&self) -> bool {
    self.inner.is_override
  }

  pub fn readonly(&self) -> bool {
    self.inner.readonly
  }
}

impl<'a> SourceRanged for TsParamProp<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsParamProp<'a>> for Node<'a> {
  fn from(node: &TsParamProp<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsParamProp<'a>, &'a TsParamProp<'a>>(node) };
    Node::TsParamProp(node)
  }
}

impl<'a> NodeTrait<'a> for TsParamProp<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + self.decorators.len());
    for child in self.decorators.iter() {
      children.push((*child).into());
    }
    children.push((&self.param).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsParamProp
  }
}

impl<'a> CastableNode<'a> for TsParamProp<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsParamProp(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsParamProp
  }
}

fn get_view_for_ts_param_prop<'a>(inner: &'a swc_ast::TsParamProp, bump: &'a Bump) -> &'a TsParamProp<'a> {
  let node = bump.alloc(TsParamProp {
    inner,
    parent: None,
    decorators: inner.decorators.iter().map(|value| get_view_for_decorator(value, bump)).collect(),
    param: get_view_for_ts_param_prop_param(&inner.param, bump),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.decorators.iter() {
    set_parent_for_decorator(value, parent)
  }
  set_parent_for_ts_param_prop_param(&node.param, parent);
  node
}

fn set_parent_for_ts_param_prop<'a>(node: &TsParamProp<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsParamProp<'a> as *mut TsParamProp<'a>;
    (*node_ptr).parent.replace(parent.expect::<Constructor>());
  }
}

#[derive(Clone)]
pub struct TsParenthesizedType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsParenthesizedType,
  pub type_ann: TsType<'a>,
}

impl<'a> TsParenthesizedType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsParenthesizedType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsParenthesizedType<'a>> for Node<'a> {
  fn from(node: &TsParenthesizedType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsParenthesizedType<'a>, &'a TsParenthesizedType<'a>>(node) };
    Node::TsParenthesizedType(node)
  }
}

impl<'a> NodeTrait<'a> for TsParenthesizedType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsParenthesizedType
  }
}

impl<'a> CastableNode<'a> for TsParenthesizedType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsParenthesizedType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsParenthesizedType
  }
}

fn get_view_for_ts_parenthesized_type<'a>(inner: &'a swc_ast::TsParenthesizedType, bump: &'a Bump) -> &'a TsParenthesizedType<'a> {
  let node = bump.alloc(TsParenthesizedType {
    inner,
    parent: None,
    type_ann: get_view_for_ts_type(&inner.type_ann, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type(&node.type_ann, parent);
  node
}

fn set_parent_for_ts_parenthesized_type<'a>(node: &TsParenthesizedType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsParenthesizedType<'a> as *mut TsParenthesizedType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsPropertySignature<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsPropertySignature,
  pub key: Expr<'a>,
  pub init: Option<Expr<'a>>,
  pub params: Vec<TsFnParam<'a>>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
}

impl<'a> TsPropertySignature<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

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

impl<'a> SourceRanged for TsPropertySignature<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsPropertySignature<'a>> for Node<'a> {
  fn from(node: &TsPropertySignature<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsPropertySignature<'a>, &'a TsPropertySignature<'a>>(node) };
    Node::TsPropertySignature(node)
  }
}

impl<'a> NodeTrait<'a> for TsPropertySignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsPropertySignature
  }
}

impl<'a> CastableNode<'a> for TsPropertySignature<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsPropertySignature(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsPropertySignature
  }
}

fn get_view_for_ts_property_signature<'a>(inner: &'a swc_ast::TsPropertySignature, bump: &'a Bump) -> &'a TsPropertySignature<'a> {
  let node = bump.alloc(TsPropertySignature {
    inner,
    parent: None,
    key: get_view_for_expr(&inner.key, bump),
    init: match &inner.init {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
    params: inner.params.iter().map(|value| get_view_for_ts_fn_param(value, bump)).collect(),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.key, parent);
  if let Some(value) = &node.init {
    set_parent_for_expr(value, parent)
  };
  for value in node.params.iter() {
    set_parent_for_ts_fn_param(value, parent)
  }
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  node
}

fn set_parent_for_ts_property_signature<'a>(node: &TsPropertySignature<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsPropertySignature<'a> as *mut TsPropertySignature<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsQualifiedName<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsQualifiedName,
  pub left: TsEntityName<'a>,
  pub right: &'a Ident<'a>,
}

impl<'a> TsQualifiedName<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsQualifiedName<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsQualifiedName<'a>> for Node<'a> {
  fn from(node: &TsQualifiedName<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsQualifiedName<'a>, &'a TsQualifiedName<'a>>(node) };
    Node::TsQualifiedName(node)
  }
}

impl<'a> NodeTrait<'a> for TsQualifiedName<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.left).into());
    children.push(self.right.into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsQualifiedName
  }
}

impl<'a> CastableNode<'a> for TsQualifiedName<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsQualifiedName(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsQualifiedName
  }
}

fn get_view_for_ts_qualified_name<'a>(inner: &'a swc_ast::TsQualifiedName, bump: &'a Bump) -> &'a TsQualifiedName<'a> {
  let node = bump.alloc(TsQualifiedName {
    inner,
    parent: None,
    left: get_view_for_ts_entity_name(&inner.left, bump),
    right: get_view_for_ident(&inner.right, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_entity_name(&node.left, parent);
  set_parent_for_ident(&node.right, parent);
  node
}

fn set_parent_for_ts_qualified_name<'a>(node: &TsQualifiedName<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsQualifiedName<'a> as *mut TsQualifiedName<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsRestType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsRestType,
  pub type_ann: TsType<'a>,
}

impl<'a> TsRestType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsRestType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsRestType<'a>> for Node<'a> {
  fn from(node: &TsRestType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsRestType<'a>, &'a TsRestType<'a>>(node) };
    Node::TsRestType(node)
  }
}

impl<'a> NodeTrait<'a> for TsRestType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsRestType
  }
}

impl<'a> CastableNode<'a> for TsRestType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsRestType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsRestType
  }
}

fn get_view_for_ts_rest_type<'a>(inner: &'a swc_ast::TsRestType, bump: &'a Bump) -> &'a TsRestType<'a> {
  let node = bump.alloc(TsRestType {
    inner,
    parent: None,
    type_ann: get_view_for_ts_type(&inner.type_ann, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type(&node.type_ann, parent);
  node
}

fn set_parent_for_ts_rest_type<'a>(node: &TsRestType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsRestType<'a> as *mut TsRestType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsSetterSignature<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsSetterSignature,
  pub key: Expr<'a>,
  pub param: TsFnParam<'a>,
}

impl<'a> TsSetterSignature<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

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

impl<'a> SourceRanged for TsSetterSignature<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsSetterSignature<'a>> for Node<'a> {
  fn from(node: &TsSetterSignature<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsSetterSignature<'a>, &'a TsSetterSignature<'a>>(node) };
    Node::TsSetterSignature(node)
  }
}

impl<'a> NodeTrait<'a> for TsSetterSignature<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.key).into());
    children.push((&self.param).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsSetterSignature
  }
}

impl<'a> CastableNode<'a> for TsSetterSignature<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsSetterSignature(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsSetterSignature
  }
}

fn get_view_for_ts_setter_signature<'a>(inner: &'a swc_ast::TsSetterSignature, bump: &'a Bump) -> &'a TsSetterSignature<'a> {
  let node = bump.alloc(TsSetterSignature {
    inner,
    parent: None,
    key: get_view_for_expr(&inner.key, bump),
    param: get_view_for_ts_fn_param(&inner.param, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.key, parent);
  set_parent_for_ts_fn_param(&node.param, parent);
  node
}

fn set_parent_for_ts_setter_signature<'a>(node: &TsSetterSignature<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsSetterSignature<'a> as *mut TsSetterSignature<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsThisType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsThisType,
}

impl<'a> TsThisType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsThisType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsThisType<'a>> for Node<'a> {
  fn from(node: &TsThisType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsThisType<'a>, &'a TsThisType<'a>>(node) };
    Node::TsThisType(node)
  }
}

impl<'a> NodeTrait<'a> for TsThisType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    Vec::with_capacity(0)
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsThisType
  }
}

impl<'a> CastableNode<'a> for TsThisType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsThisType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsThisType
  }
}

fn get_view_for_ts_this_type<'a>(inner: &'a swc_ast::TsThisType, bump: &'a Bump) -> &'a TsThisType<'a> {
  let node = bump.alloc(TsThisType {
    inner,
    parent: None,
  });
  node
}

fn set_parent_for_ts_this_type<'a>(node: &TsThisType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsThisType<'a> as *mut TsThisType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTplLitType<'a> {
  parent: Option<&'a TsLitType<'a>>,
  pub inner: &'a swc_ast::TsTplLitType,
  pub types: Vec<TsType<'a>>,
  pub quasis: Vec<&'a TplElement<'a>>,
}

impl<'a> TsTplLitType<'a> {
  pub fn parent(&self) -> &'a TsLitType<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsTplLitType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTplLitType<'a>> for Node<'a> {
  fn from(node: &TsTplLitType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTplLitType<'a>, &'a TsTplLitType<'a>>(node) };
    Node::TsTplLitType(node)
  }
}

impl<'a> NodeTrait<'a> for TsTplLitType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTplLitType
  }
}

impl<'a> CastableNode<'a> for TsTplLitType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTplLitType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTplLitType
  }
}

fn get_view_for_ts_tpl_lit_type<'a>(inner: &'a swc_ast::TsTplLitType, bump: &'a Bump) -> &'a TsTplLitType<'a> {
  let node = bump.alloc(TsTplLitType {
    inner,
    parent: None,
    types: inner.types.iter().map(|value| get_view_for_ts_type(value, bump)).collect(),
    quasis: inner.quasis.iter().map(|value| get_view_for_tpl_element(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.types.iter() {
    set_parent_for_ts_type(value, parent)
  }
  for value in node.quasis.iter() {
    set_parent_for_tpl_element(value, parent)
  }
  node
}

fn set_parent_for_ts_tpl_lit_type<'a>(node: &TsTplLitType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTplLitType<'a> as *mut TsTplLitType<'a>;
    (*node_ptr).parent.replace(parent.expect::<TsLitType>());
  }
}

#[derive(Clone)]
pub struct TsTupleElement<'a> {
  parent: Option<&'a TsTupleType<'a>>,
  pub inner: &'a swc_ast::TsTupleElement,
  /// `Ident` or `RestPat { arg: Ident }`
  pub label: Option<Pat<'a>>,
  pub ty: TsType<'a>,
}

impl<'a> TsTupleElement<'a> {
  pub fn parent(&self) -> &'a TsTupleType<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsTupleElement<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTupleElement<'a>> for Node<'a> {
  fn from(node: &TsTupleElement<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTupleElement<'a>, &'a TsTupleElement<'a>>(node) };
    Node::TsTupleElement(node)
  }
}

impl<'a> NodeTrait<'a> for TsTupleElement<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.label { Some(_value) => 1, None => 0, });
    if let Some(child) = self.label.as_ref() {
      children.push(child.into());
    }
    children.push((&self.ty).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTupleElement
  }
}

impl<'a> CastableNode<'a> for TsTupleElement<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTupleElement(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTupleElement
  }
}

fn get_view_for_ts_tuple_element<'a>(inner: &'a swc_ast::TsTupleElement, bump: &'a Bump) -> &'a TsTupleElement<'a> {
  let node = bump.alloc(TsTupleElement {
    inner,
    parent: None,
    label: match &inner.label {
      Some(value) => Some(get_view_for_pat(value, bump)),
      None => None,
    },
    ty: get_view_for_ts_type(&inner.ty, bump),
  });
  let parent: Node<'a> = (&*node).into();
  if let Some(value) = &node.label {
    set_parent_for_pat(value, parent)
  };
  set_parent_for_ts_type(&node.ty, parent);
  node
}

fn set_parent_for_ts_tuple_element<'a>(node: &TsTupleElement<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTupleElement<'a> as *mut TsTupleElement<'a>;
    (*node_ptr).parent.replace(parent.expect::<TsTupleType>());
  }
}

#[derive(Clone)]
pub struct TsTupleType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTupleType,
  pub elem_types: Vec<&'a TsTupleElement<'a>>,
}

impl<'a> TsTupleType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsTupleType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTupleType<'a>> for Node<'a> {
  fn from(node: &TsTupleType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTupleType<'a>, &'a TsTupleType<'a>>(node) };
    Node::TsTupleType(node)
  }
}

impl<'a> NodeTrait<'a> for TsTupleType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.elem_types.len());
    for child in self.elem_types.iter() {
      children.push((*child).into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTupleType
  }
}

impl<'a> CastableNode<'a> for TsTupleType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTupleType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTupleType
  }
}

fn get_view_for_ts_tuple_type<'a>(inner: &'a swc_ast::TsTupleType, bump: &'a Bump) -> &'a TsTupleType<'a> {
  let node = bump.alloc(TsTupleType {
    inner,
    parent: None,
    elem_types: inner.elem_types.iter().map(|value| get_view_for_ts_tuple_element(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.elem_types.iter() {
    set_parent_for_ts_tuple_element(value, parent)
  }
  node
}

fn set_parent_for_ts_tuple_type<'a>(node: &TsTupleType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTupleType<'a> as *mut TsTupleType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTypeAliasDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypeAliasDecl,
  pub id: &'a Ident<'a>,
  pub type_params: Option<&'a TsTypeParamDecl<'a>>,
  pub type_ann: TsType<'a>,
}

impl<'a> TsTypeAliasDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl<'a> SourceRanged for TsTypeAliasDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypeAliasDecl<'a>> for Node<'a> {
  fn from(node: &TsTypeAliasDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeAliasDecl<'a>, &'a TsTypeAliasDecl<'a>>(node) };
    Node::TsTypeAliasDecl(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypeAliasDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeAliasDecl
  }
}

impl<'a> CastableNode<'a> for TsTypeAliasDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeAliasDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypeAliasDecl
  }
}

fn get_view_for_ts_type_alias_decl<'a>(inner: &'a swc_ast::TsTypeAliasDecl, bump: &'a Bump) -> &'a TsTypeAliasDecl<'a> {
  let node = bump.alloc(TsTypeAliasDecl {
    inner,
    parent: None,
    id: get_view_for_ident(&inner.id, bump),
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_decl(value, bump)),
      None => None,
    },
    type_ann: get_view_for_ts_type(&inner.type_ann, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.id, parent);
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_decl(value, parent)
  };
  set_parent_for_ts_type(&node.type_ann, parent);
  node
}

fn set_parent_for_ts_type_alias_decl<'a>(node: &TsTypeAliasDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypeAliasDecl<'a> as *mut TsTypeAliasDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTypeAnn<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypeAnn,
  pub type_ann: TsType<'a>,
}

impl<'a> TsTypeAnn<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsTypeAnn<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypeAnn<'a>> for Node<'a> {
  fn from(node: &TsTypeAnn<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeAnn<'a>, &'a TsTypeAnn<'a>>(node) };
    Node::TsTypeAnn(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypeAnn<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeAnn
  }
}

impl<'a> CastableNode<'a> for TsTypeAnn<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeAnn(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypeAnn
  }
}

fn get_view_for_ts_type_ann<'a>(inner: &'a swc_ast::TsTypeAnn, bump: &'a Bump) -> &'a TsTypeAnn<'a> {
  let node = bump.alloc(TsTypeAnn {
    inner,
    parent: None,
    type_ann: get_view_for_ts_type(&inner.type_ann, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type(&node.type_ann, parent);
  node
}

fn set_parent_for_ts_type_ann<'a>(node: &TsTypeAnn<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypeAnn<'a> as *mut TsTypeAnn<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTypeAssertion<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypeAssertion,
  pub expr: Expr<'a>,
  pub type_ann: TsType<'a>,
}

impl<'a> TsTypeAssertion<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsTypeAssertion<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypeAssertion<'a>> for Node<'a> {
  fn from(node: &TsTypeAssertion<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeAssertion<'a>, &'a TsTypeAssertion<'a>>(node) };
    Node::TsTypeAssertion(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypeAssertion<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.expr).into());
    children.push((&self.type_ann).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeAssertion
  }
}

impl<'a> CastableNode<'a> for TsTypeAssertion<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeAssertion(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypeAssertion
  }
}

fn get_view_for_ts_type_assertion<'a>(inner: &'a swc_ast::TsTypeAssertion, bump: &'a Bump) -> &'a TsTypeAssertion<'a> {
  let node = bump.alloc(TsTypeAssertion {
    inner,
    parent: None,
    expr: get_view_for_expr(&inner.expr, bump),
    type_ann: get_view_for_ts_type(&inner.type_ann, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.expr, parent);
  set_parent_for_ts_type(&node.type_ann, parent);
  node
}

fn set_parent_for_ts_type_assertion<'a>(node: &TsTypeAssertion<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypeAssertion<'a> as *mut TsTypeAssertion<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTypeLit<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypeLit,
  pub members: Vec<TsTypeElement<'a>>,
}

impl<'a> TsTypeLit<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsTypeLit<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypeLit<'a>> for Node<'a> {
  fn from(node: &TsTypeLit<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeLit<'a>, &'a TsTypeLit<'a>>(node) };
    Node::TsTypeLit(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypeLit<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.members.len());
    for child in self.members.iter() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeLit
  }
}

impl<'a> CastableNode<'a> for TsTypeLit<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeLit(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypeLit
  }
}

fn get_view_for_ts_type_lit<'a>(inner: &'a swc_ast::TsTypeLit, bump: &'a Bump) -> &'a TsTypeLit<'a> {
  let node = bump.alloc(TsTypeLit {
    inner,
    parent: None,
    members: inner.members.iter().map(|value| get_view_for_ts_type_element(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.members.iter() {
    set_parent_for_ts_type_element(value, parent)
  }
  node
}

fn set_parent_for_ts_type_lit<'a>(node: &TsTypeLit<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypeLit<'a> as *mut TsTypeLit<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTypeOperator<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypeOperator,
  pub type_ann: TsType<'a>,
}

impl<'a> TsTypeOperator<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn op(&self) -> TsTypeOperatorOp {
    self.inner.op
  }
}

impl<'a> SourceRanged for TsTypeOperator<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypeOperator<'a>> for Node<'a> {
  fn from(node: &TsTypeOperator<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeOperator<'a>, &'a TsTypeOperator<'a>>(node) };
    Node::TsTypeOperator(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypeOperator<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.type_ann).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeOperator
  }
}

impl<'a> CastableNode<'a> for TsTypeOperator<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeOperator(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypeOperator
  }
}

fn get_view_for_ts_type_operator<'a>(inner: &'a swc_ast::TsTypeOperator, bump: &'a Bump) -> &'a TsTypeOperator<'a> {
  let node = bump.alloc(TsTypeOperator {
    inner,
    parent: None,
    type_ann: get_view_for_ts_type(&inner.type_ann, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type(&node.type_ann, parent);
  node
}

fn set_parent_for_ts_type_operator<'a>(node: &TsTypeOperator<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypeOperator<'a> as *mut TsTypeOperator<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTypeParam<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypeParam,
  pub name: &'a Ident<'a>,
  pub constraint: Option<TsType<'a>>,
  pub default: Option<TsType<'a>>,
}

impl<'a> TsTypeParam<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn is_in(&self) -> bool {
    self.inner.is_in
  }

  pub fn is_out(&self) -> bool {
    self.inner.is_out
  }
}

impl<'a> SourceRanged for TsTypeParam<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypeParam<'a>> for Node<'a> {
  fn from(node: &TsTypeParam<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeParam<'a>, &'a TsTypeParam<'a>>(node) };
    Node::TsTypeParam(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypeParam<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
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

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeParam
  }
}

impl<'a> CastableNode<'a> for TsTypeParam<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeParam(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypeParam
  }
}

fn get_view_for_ts_type_param<'a>(inner: &'a swc_ast::TsTypeParam, bump: &'a Bump) -> &'a TsTypeParam<'a> {
  let node = bump.alloc(TsTypeParam {
    inner,
    parent: None,
    name: get_view_for_ident(&inner.name, bump),
    constraint: match &inner.constraint {
      Some(value) => Some(get_view_for_ts_type(value, bump)),
      None => None,
    },
    default: match &inner.default {
      Some(value) => Some(get_view_for_ts_type(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ident(&node.name, parent);
  if let Some(value) = &node.constraint {
    set_parent_for_ts_type(value, parent)
  };
  if let Some(value) = &node.default {
    set_parent_for_ts_type(value, parent)
  };
  node
}

fn set_parent_for_ts_type_param<'a>(node: &TsTypeParam<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypeParam<'a> as *mut TsTypeParam<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTypeParamDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypeParamDecl,
  pub params: Vec<&'a TsTypeParam<'a>>,
}

impl<'a> TsTypeParamDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsTypeParamDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypeParamDecl<'a>> for Node<'a> {
  fn from(node: &TsTypeParamDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeParamDecl<'a>, &'a TsTypeParamDecl<'a>>(node) };
    Node::TsTypeParamDecl(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypeParamDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.params.len());
    for child in self.params.iter() {
      children.push((*child).into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeParamDecl
  }
}

impl<'a> CastableNode<'a> for TsTypeParamDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeParamDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypeParamDecl
  }
}

fn get_view_for_ts_type_param_decl<'a>(inner: &'a swc_ast::TsTypeParamDecl, bump: &'a Bump) -> &'a TsTypeParamDecl<'a> {
  let node = bump.alloc(TsTypeParamDecl {
    inner,
    parent: None,
    params: inner.params.iter().map(|value| get_view_for_ts_type_param(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.params.iter() {
    set_parent_for_ts_type_param(value, parent)
  }
  node
}

fn set_parent_for_ts_type_param_decl<'a>(node: &TsTypeParamDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypeParamDecl<'a> as *mut TsTypeParamDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTypeParamInstantiation<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypeParamInstantiation,
  pub params: Vec<TsType<'a>>,
}

impl<'a> TsTypeParamInstantiation<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsTypeParamInstantiation<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypeParamInstantiation<'a>> for Node<'a> {
  fn from(node: &TsTypeParamInstantiation<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeParamInstantiation<'a>, &'a TsTypeParamInstantiation<'a>>(node) };
    Node::TsTypeParamInstantiation(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypeParamInstantiation<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.params.len());
    for child in self.params.iter() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeParamInstantiation
  }
}

impl<'a> CastableNode<'a> for TsTypeParamInstantiation<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeParamInstantiation(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypeParamInstantiation
  }
}

fn get_view_for_ts_type_param_instantiation<'a>(inner: &'a swc_ast::TsTypeParamInstantiation, bump: &'a Bump) -> &'a TsTypeParamInstantiation<'a> {
  let node = bump.alloc(TsTypeParamInstantiation {
    inner,
    parent: None,
    params: inner.params.iter().map(|value| get_view_for_ts_type(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.params.iter() {
    set_parent_for_ts_type(value, parent)
  }
  node
}

fn set_parent_for_ts_type_param_instantiation<'a>(node: &TsTypeParamInstantiation<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypeParamInstantiation<'a> as *mut TsTypeParamInstantiation<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTypePredicate<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypePredicate,
  pub param_name: TsThisTypeOrIdent<'a>,
  pub type_ann: Option<&'a TsTypeAnn<'a>>,
}

impl<'a> TsTypePredicate<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn asserts(&self) -> bool {
    self.inner.asserts
  }
}

impl<'a> SourceRanged for TsTypePredicate<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypePredicate<'a>> for Node<'a> {
  fn from(node: &TsTypePredicate<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypePredicate<'a>, &'a TsTypePredicate<'a>>(node) };
    Node::TsTypePredicate(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypePredicate<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_ann { Some(_value) => 1, None => 0, });
    children.push((&self.param_name).into());
    if let Some(child) = self.type_ann {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypePredicate
  }
}

impl<'a> CastableNode<'a> for TsTypePredicate<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypePredicate(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypePredicate
  }
}

fn get_view_for_ts_type_predicate<'a>(inner: &'a swc_ast::TsTypePredicate, bump: &'a Bump) -> &'a TsTypePredicate<'a> {
  let node = bump.alloc(TsTypePredicate {
    inner,
    parent: None,
    param_name: get_view_for_ts_this_type_or_ident(&inner.param_name, bump),
    type_ann: match &inner.type_ann {
      Some(value) => Some(get_view_for_ts_type_ann(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_this_type_or_ident(&node.param_name, parent);
  if let Some(value) = &node.type_ann {
    set_parent_for_ts_type_ann(value, parent)
  };
  node
}

fn set_parent_for_ts_type_predicate<'a>(node: &TsTypePredicate<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypePredicate<'a> as *mut TsTypePredicate<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

/// `typeof` operator
#[derive(Clone)]
pub struct TsTypeQuery<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypeQuery,
  pub expr_name: TsTypeQueryExpr<'a>,
  pub type_args: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> TsTypeQuery<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsTypeQuery<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypeQuery<'a>> for Node<'a> {
  fn from(node: &TsTypeQuery<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeQuery<'a>, &'a TsTypeQuery<'a>>(node) };
    Node::TsTypeQuery(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypeQuery<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_args { Some(_value) => 1, None => 0, });
    children.push((&self.expr_name).into());
    if let Some(child) = self.type_args {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeQuery
  }
}

impl<'a> CastableNode<'a> for TsTypeQuery<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeQuery(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypeQuery
  }
}

fn get_view_for_ts_type_query<'a>(inner: &'a swc_ast::TsTypeQuery, bump: &'a Bump) -> &'a TsTypeQuery<'a> {
  let node = bump.alloc(TsTypeQuery {
    inner,
    parent: None,
    expr_name: get_view_for_ts_type_query_expr(&inner.expr_name, bump),
    type_args: match &inner.type_args {
      Some(value) => Some(get_view_for_ts_type_param_instantiation(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_type_query_expr(&node.expr_name, parent);
  if let Some(value) = &node.type_args {
    set_parent_for_ts_type_param_instantiation(value, parent)
  };
  node
}

fn set_parent_for_ts_type_query<'a>(node: &TsTypeQuery<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypeQuery<'a> as *mut TsTypeQuery<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsTypeRef<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsTypeRef,
  pub type_name: TsEntityName<'a>,
  pub type_params: Option<&'a TsTypeParamInstantiation<'a>>,
}

impl<'a> TsTypeRef<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsTypeRef<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsTypeRef<'a>> for Node<'a> {
  fn from(node: &TsTypeRef<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsTypeRef<'a>, &'a TsTypeRef<'a>>(node) };
    Node::TsTypeRef(node)
  }
}

impl<'a> NodeTrait<'a> for TsTypeRef<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.type_params { Some(_value) => 1, None => 0, });
    children.push((&self.type_name).into());
    if let Some(child) = self.type_params {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsTypeRef
  }
}

impl<'a> CastableNode<'a> for TsTypeRef<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsTypeRef(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsTypeRef
  }
}

fn get_view_for_ts_type_ref<'a>(inner: &'a swc_ast::TsTypeRef, bump: &'a Bump) -> &'a TsTypeRef<'a> {
  let node = bump.alloc(TsTypeRef {
    inner,
    parent: None,
    type_name: get_view_for_ts_entity_name(&inner.type_name, bump),
    type_params: match &inner.type_params {
      Some(value) => Some(get_view_for_ts_type_param_instantiation(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_ts_entity_name(&node.type_name, parent);
  if let Some(value) = &node.type_params {
    set_parent_for_ts_type_param_instantiation(value, parent)
  };
  node
}

fn set_parent_for_ts_type_ref<'a>(node: &TsTypeRef<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsTypeRef<'a> as *mut TsTypeRef<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct TsUnionType<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::TsUnionType,
  pub types: Vec<TsType<'a>>,
}

impl<'a> TsUnionType<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for TsUnionType<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&TsUnionType<'a>> for Node<'a> {
  fn from(node: &TsUnionType<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&TsUnionType<'a>, &'a TsUnionType<'a>>(node) };
    Node::TsUnionType(node)
  }
}

impl<'a> NodeTrait<'a> for TsUnionType<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.types.len());
    for child in self.types.iter() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::TsUnionType
  }
}

impl<'a> CastableNode<'a> for TsUnionType<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::TsUnionType(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::TsUnionType
  }
}

fn get_view_for_ts_union_type<'a>(inner: &'a swc_ast::TsUnionType, bump: &'a Bump) -> &'a TsUnionType<'a> {
  let node = bump.alloc(TsUnionType {
    inner,
    parent: None,
    types: inner.types.iter().map(|value| get_view_for_ts_type(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.types.iter() {
    set_parent_for_ts_type(value, parent)
  }
  node
}

fn set_parent_for_ts_union_type<'a>(node: &TsUnionType<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const TsUnionType<'a> as *mut TsUnionType<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct UnaryExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::UnaryExpr,
  pub arg: Expr<'a>,
}

impl<'a> UnaryExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn op(&self) -> UnaryOp {
    self.inner.op
  }
}

impl<'a> SourceRanged for UnaryExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&UnaryExpr<'a>> for Node<'a> {
  fn from(node: &UnaryExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&UnaryExpr<'a>, &'a UnaryExpr<'a>>(node) };
    Node::UnaryExpr(node)
  }
}

impl<'a> NodeTrait<'a> for UnaryExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::UnaryExpr
  }
}

impl<'a> CastableNode<'a> for UnaryExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::UnaryExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::UnaryExpr
  }
}

fn get_view_for_unary_expr<'a>(inner: &'a swc_ast::UnaryExpr, bump: &'a Bump) -> &'a UnaryExpr<'a> {
  let node = bump.alloc(UnaryExpr {
    inner,
    parent: None,
    arg: get_view_for_expr(&inner.arg, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.arg, parent);
  node
}

fn set_parent_for_unary_expr<'a>(node: &UnaryExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const UnaryExpr<'a> as *mut UnaryExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct UpdateExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::UpdateExpr,
  pub arg: Expr<'a>,
}

impl<'a> UpdateExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn op(&self) -> UpdateOp {
    self.inner.op
  }

  pub fn prefix(&self) -> bool {
    self.inner.prefix
  }
}

impl<'a> SourceRanged for UpdateExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&UpdateExpr<'a>> for Node<'a> {
  fn from(node: &UpdateExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&UpdateExpr<'a>, &'a UpdateExpr<'a>>(node) };
    Node::UpdateExpr(node)
  }
}

impl<'a> NodeTrait<'a> for UpdateExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1);
    children.push((&self.arg).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::UpdateExpr
  }
}

impl<'a> CastableNode<'a> for UpdateExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::UpdateExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::UpdateExpr
  }
}

fn get_view_for_update_expr<'a>(inner: &'a swc_ast::UpdateExpr, bump: &'a Bump) -> &'a UpdateExpr<'a> {
  let node = bump.alloc(UpdateExpr {
    inner,
    parent: None,
    arg: get_view_for_expr(&inner.arg, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.arg, parent);
  node
}

fn set_parent_for_update_expr<'a>(node: &UpdateExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const UpdateExpr<'a> as *mut UpdateExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct VarDecl<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::VarDecl,
  pub decls: Vec<&'a VarDeclarator<'a>>,
}

impl<'a> VarDecl<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn decl_kind(&self) -> VarDeclKind {
    self.inner.kind
  }

  pub fn declare(&self) -> bool {
    self.inner.declare
  }
}

impl<'a> SourceRanged for VarDecl<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&VarDecl<'a>> for Node<'a> {
  fn from(node: &VarDecl<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&VarDecl<'a>, &'a VarDecl<'a>>(node) };
    Node::VarDecl(node)
  }
}

impl<'a> NodeTrait<'a> for VarDecl<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(self.decls.len());
    for child in self.decls.iter() {
      children.push((*child).into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::VarDecl
  }
}

impl<'a> CastableNode<'a> for VarDecl<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::VarDecl(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::VarDecl
  }
}

fn get_view_for_var_decl<'a>(inner: &'a swc_ast::VarDecl, bump: &'a Bump) -> &'a VarDecl<'a> {
  let node = bump.alloc(VarDecl {
    inner,
    parent: None,
    decls: inner.decls.iter().map(|value| get_view_for_var_declarator(value, bump)).collect(),
  });
  let parent: Node<'a> = (&*node).into();
  for value in node.decls.iter() {
    set_parent_for_var_declarator(value, parent)
  }
  node
}

fn set_parent_for_var_decl<'a>(node: &VarDecl<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const VarDecl<'a> as *mut VarDecl<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct VarDeclarator<'a> {
  parent: Option<&'a VarDecl<'a>>,
  pub inner: &'a swc_ast::VarDeclarator,
  pub name: Pat<'a>,
  /// Initialization expression.
  pub init: Option<Expr<'a>>,
}

impl<'a> VarDeclarator<'a> {
  pub fn parent(&self) -> &'a VarDecl<'a> {
    self.parent.unwrap()
  }

  /// Typescript only
  pub fn definite(&self) -> bool {
    self.inner.definite
  }
}

impl<'a> SourceRanged for VarDeclarator<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&VarDeclarator<'a>> for Node<'a> {
  fn from(node: &VarDeclarator<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&VarDeclarator<'a>, &'a VarDeclarator<'a>>(node) };
    Node::VarDeclarator(node)
  }
}

impl<'a> NodeTrait<'a> for VarDeclarator<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().into())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(1 + match &self.init { Some(_value) => 1, None => 0, });
    children.push((&self.name).into());
    if let Some(child) = self.init.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::VarDeclarator
  }
}

impl<'a> CastableNode<'a> for VarDeclarator<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::VarDeclarator(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::VarDeclarator
  }
}

fn get_view_for_var_declarator<'a>(inner: &'a swc_ast::VarDeclarator, bump: &'a Bump) -> &'a VarDeclarator<'a> {
  let node = bump.alloc(VarDeclarator {
    inner,
    parent: None,
    name: get_view_for_pat(&inner.name, bump),
    init: match &inner.init {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_pat(&node.name, parent);
  if let Some(value) = &node.init {
    set_parent_for_expr(value, parent)
  };
  node
}

fn set_parent_for_var_declarator<'a>(node: &VarDeclarator<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const VarDeclarator<'a> as *mut VarDeclarator<'a>;
    (*node_ptr).parent.replace(parent.expect::<VarDecl>());
  }
}

#[derive(Clone)]
pub struct WhileStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::WhileStmt,
  pub test: Expr<'a>,
  pub body: Stmt<'a>,
}

impl<'a> WhileStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for WhileStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&WhileStmt<'a>> for Node<'a> {
  fn from(node: &WhileStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&WhileStmt<'a>, &'a WhileStmt<'a>>(node) };
    Node::WhileStmt(node)
  }
}

impl<'a> NodeTrait<'a> for WhileStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.test).into());
    children.push((&self.body).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::WhileStmt
  }
}

impl<'a> CastableNode<'a> for WhileStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::WhileStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::WhileStmt
  }
}

fn get_view_for_while_stmt<'a>(inner: &'a swc_ast::WhileStmt, bump: &'a Bump) -> &'a WhileStmt<'a> {
  let node = bump.alloc(WhileStmt {
    inner,
    parent: None,
    test: get_view_for_expr(&inner.test, bump),
    body: get_view_for_stmt(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.test, parent);
  set_parent_for_stmt(&node.body, parent);
  node
}

fn set_parent_for_while_stmt<'a>(node: &WhileStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const WhileStmt<'a> as *mut WhileStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct WithStmt<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::WithStmt,
  pub obj: Expr<'a>,
  pub body: Stmt<'a>,
}

impl<'a> WithStmt<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }
}

impl<'a> SourceRanged for WithStmt<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&WithStmt<'a>> for Node<'a> {
  fn from(node: &WithStmt<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&WithStmt<'a>, &'a WithStmt<'a>>(node) };
    Node::WithStmt(node)
  }
}

impl<'a> NodeTrait<'a> for WithStmt<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(2);
    children.push((&self.obj).into());
    children.push((&self.body).into());
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::WithStmt
  }
}

impl<'a> CastableNode<'a> for WithStmt<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::WithStmt(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::WithStmt
  }
}

fn get_view_for_with_stmt<'a>(inner: &'a swc_ast::WithStmt, bump: &'a Bump) -> &'a WithStmt<'a> {
  let node = bump.alloc(WithStmt {
    inner,
    parent: None,
    obj: get_view_for_expr(&inner.obj, bump),
    body: get_view_for_stmt(&inner.body, bump),
  });
  let parent: Node<'a> = (&*node).into();
  set_parent_for_expr(&node.obj, parent);
  set_parent_for_stmt(&node.body, parent);
  node
}

fn set_parent_for_with_stmt<'a>(node: &WithStmt<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const WithStmt<'a> as *mut WithStmt<'a>;
    (*node_ptr).parent.replace(parent);
  }
}

#[derive(Clone)]
pub struct YieldExpr<'a> {
  parent: Option<Node<'a>>,
  pub inner: &'a swc_ast::YieldExpr,
  pub arg: Option<Expr<'a>>,
}

impl<'a> YieldExpr<'a> {
  pub fn parent(&self) -> Node<'a> {
    self.parent.unwrap()
  }

  pub fn delegate(&self) -> bool {
    self.inner.delegate
  }
}

impl<'a> SourceRanged for YieldExpr<'a> {
  fn start(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().lo)
  }
  fn end(&self) -> SourcePos {
    SourcePos::from_byte_pos(self.inner.span().hi)
  }
}

impl<'a> From<&YieldExpr<'a>> for Node<'a> {
  fn from(node: &YieldExpr<'a>) -> Node<'a> {
    let node = unsafe { mem::transmute::<&YieldExpr<'a>, &'a YieldExpr<'a>>(node) };
    Node::YieldExpr(node)
  }
}

impl<'a> NodeTrait<'a> for YieldExpr<'a> {
  fn parent(&self) -> Option<Node<'a>> {
    Some(self.parent.unwrap().clone())
  }

  fn children(&self) -> Vec<Node<'a>> {
    let mut children = Vec::with_capacity(match &self.arg { Some(_value) => 1, None => 0, });
    if let Some(child) = self.arg.as_ref() {
      children.push(child.into());
    }
    children
  }

  fn as_node(&self) -> Node<'a> {
    self.into()
  }

  fn kind(&self) -> NodeKind {
    NodeKind::YieldExpr
  }
}

impl<'a> CastableNode<'a> for YieldExpr<'a> {
  fn to(node: &Node<'a>) -> Option<&'a Self> {
    if let Node::YieldExpr(node) = node {
      Some(node)
    } else {
      None
    }
  }

  fn kind() -> NodeKind {
    NodeKind::YieldExpr
  }
}

fn get_view_for_yield_expr<'a>(inner: &'a swc_ast::YieldExpr, bump: &'a Bump) -> &'a YieldExpr<'a> {
  let node = bump.alloc(YieldExpr {
    inner,
    parent: None,
    arg: match &inner.arg {
      Some(value) => Some(get_view_for_expr(value, bump)),
      None => None,
    },
  });
  let parent: Node<'a> = (&*node).into();
  if let Some(value) = &node.arg {
    set_parent_for_expr(value, parent)
  };
  node
}

fn set_parent_for_yield_expr<'a>(node: &YieldExpr<'a>, parent: Node<'a>) {
  unsafe {
    let node_ptr = node as *const YieldExpr<'a> as *mut YieldExpr<'a>;
    (*node_ptr).parent.replace(parent);
  }
}
