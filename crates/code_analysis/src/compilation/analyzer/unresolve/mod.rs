use emmylua_parser::{LuaCallExpr, LuaExpr};

use crate::{db_index::{LuaDeclId, LuaSignatureId}, FileId};

#[derive(Debug)]
pub enum UnResolve{
    Decl(Box<UnResolveDecl>),
    Module(Box<UnResolveModule>),
    Return(Box<UnResolveReturn>),
    ClosureParams(Box<UnResolveClosureParams>),
}

#[derive(Debug)]
pub struct UnResolveDecl {
    decl_id: LuaDeclId,
    expr: LuaExpr,
    ret_idx: usize,
}

#[derive(Debug)]
pub struct UnResolveModule {
    file_id: FileId,
    expr: LuaExpr,
}

#[derive(Debug)]
pub struct UnResolveReturn {
    signature_id : LuaSignatureId,
    return_exprs: Vec<LuaExpr>
}

#[derive(Debug)]
pub struct UnResolveClosureParams {
    signature_id : LuaSignatureId,
    call_expr: LuaCallExpr,
    param_idx: usize,
}