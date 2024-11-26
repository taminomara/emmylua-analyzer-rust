mod merge_type;


use emmylua_parser::{LuaCallExpr, LuaExpr};
pub use merge_type::{merge_decl_expr_type, merge_member_type};
use crate::{db_index::{LuaDeclId, LuaMemberId, LuaSignatureId}, FileId};

use super::lua::LuaReturnPoint;

#[derive(Debug)]
pub enum UnResolve{
    Decl(Box<UnResolveDecl>),
    IterDecl(Box<UnResolveIterVar>),
    Member(Box<UnResolveMember>),
    Module(Box<UnResolveModule>),
    Return(Box<UnResolveReturn>),
    ClosureParams(Box<UnResolveClosureParams>),
}

#[derive(Debug)]
pub struct UnResolveDecl {
    pub decl_id: LuaDeclId,
    pub expr: LuaExpr,
    pub ret_idx: usize,
}

impl From<UnResolveDecl> for UnResolve {
    fn from(un_resolve_decl: UnResolveDecl) -> Self {
        UnResolve::Decl(Box::new(un_resolve_decl))
    }
}

#[derive(Debug)]
pub struct UnResolveMember {
    pub member_id: LuaMemberId,
    pub expr: LuaExpr,
    pub prefix: Option<LuaExpr>,
    pub ret_idx: usize,
}

impl From<UnResolveMember> for UnResolve {
    fn from(un_resolve_member: UnResolveMember) -> Self {
        UnResolve::Member(Box::new(un_resolve_member))
    }
}

#[derive(Debug)]
pub struct UnResolveModule {
    pub file_id: FileId,
    pub expr: LuaExpr,
}

impl From<UnResolveModule> for UnResolve {
    fn from(un_resolve_module: UnResolveModule) -> Self {
        UnResolve::Module(Box::new(un_resolve_module))
    }
}

#[derive(Debug)]
pub struct UnResolveReturn {
    pub signature_id : LuaSignatureId,
    pub return_exprs: Vec<LuaReturnPoint>
}

impl From<UnResolveReturn> for UnResolve {
    fn from(un_resolve_return: UnResolveReturn) -> Self {
        UnResolve::Return(Box::new(un_resolve_return))
    }
}

#[derive(Debug)]
pub struct UnResolveClosureParams {
    pub signature_id : LuaSignatureId,
    pub call_expr: LuaCallExpr,
    pub param_idx: usize,
}

impl From<UnResolveClosureParams> for UnResolve {
    fn from(un_resolve_closure_params: UnResolveClosureParams) -> Self {
        UnResolve::ClosureParams(Box::new(un_resolve_closure_params))
    }
}

#[derive(Debug)]
pub struct UnResolveIterVar {
    pub decl_id: LuaDeclId,
    pub iter_expr: LuaExpr,
    pub ret_idx: usize,
}

impl From<UnResolveIterVar> for UnResolve {
    fn from(un_resolve_iter_var: UnResolveIterVar) -> Self {
        UnResolve::IterDecl(Box::new(un_resolve_iter_var))
    }
}