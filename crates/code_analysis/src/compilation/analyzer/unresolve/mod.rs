mod infer_manager;
mod merge_type;
mod resolve;

use crate::{
    db_index::{DbIndex, LuaDeclId, LuaMemberId, LuaSignatureId},
    FileId,
};
use emmylua_parser::{LuaCallExpr, LuaExpr};
use infer_manager::InferManager;
pub use merge_type::{merge_decl_expr_type, merge_member_type};
use resolve::try_resolve_decl;

use super::{lua::LuaReturnPoint, AnalyzeContext};

pub fn analyze(db: &mut DbIndex, context: &mut AnalyzeContext) {
    let mut unresolves = std::mem::take(&mut context.unresolves);
    let mut infer_manager = InferManager::new(context.config.clone());
    while try_resolve(db, &mut infer_manager, &mut unresolves) {
        unresolves.retain(|un_resolve| match un_resolve {
            UnResolve::None => false,
            _ => true,
        });
    }

    // force_resolve(db, &mut unresolves);
}

fn try_resolve(
    db: &mut DbIndex,
    infer_manager: &mut InferManager,
    unresolves: &mut Vec<UnResolve>,
) -> bool {
    let mut changed = false;
    for i in 0..unresolves.len() {
        let un_resolve = &unresolves[i];

        let resolve = match un_resolve {
            UnResolve::Decl(un_resolve_decl) => {
                let config = infer_manager.get_infer_config(un_resolve_decl.file_id);
                try_resolve_decl(db, config, un_resolve_decl).unwrap_or(false)
            }
            UnResolve::Member(un_resolve_member) => {
                todo!();
                true
            }
            UnResolve::Module(un_resolve_module) => {
                todo!();
                true
            }
            UnResolve::Return(un_resolve_return) => {
                todo!();
                true
            }
            UnResolve::ClosureParams(un_resolve_closure_params) => {
                todo!();
                true
            }
            UnResolve::IterDecl(un_resolve_iter_var) => {
                todo!();
                true
            }
            UnResolve::None => continue,
        };

        if resolve {
            changed = true;
            unresolves[i] = UnResolve::None;
        }
    }

    changed
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum UnResolve {
    None,
    Decl(Box<UnResolveDecl>),
    IterDecl(Box<UnResolveIterVar>),
    Member(Box<UnResolveMember>),
    Module(Box<UnResolveModule>),
    Return(Box<UnResolveReturn>),
    ClosureParams(Box<UnResolveClosureParams>),
}

#[derive(Debug)]
pub struct UnResolveDecl {
    pub file_id: FileId,
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
    pub file_id: FileId,
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
    pub file_id: FileId,
    pub signature_id: LuaSignatureId,
    pub return_points: Vec<LuaReturnPoint>,
}

impl From<UnResolveReturn> for UnResolve {
    fn from(un_resolve_return: UnResolveReturn) -> Self {
        UnResolve::Return(Box::new(un_resolve_return))
    }
}

#[derive(Debug)]
pub struct UnResolveClosureParams {
    pub file_id: FileId,
    pub signature_id: LuaSignatureId,
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
    pub file_id: FileId,
    pub decl_id: LuaDeclId,
    pub iter_expr: LuaExpr,
    pub ret_idx: usize,
}

impl From<UnResolveIterVar> for UnResolve {
    fn from(un_resolve_iter_var: UnResolveIterVar) -> Self {
        UnResolve::IterDecl(Box::new(un_resolve_iter_var))
    }
}
