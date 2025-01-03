mod call_func;
mod infer;
mod instantiate;
mod member;
mod overload_resolve;
mod reference;
mod semantic_info;
mod type_calc;
mod type_compact;

use std::{collections::HashSet, sync::Arc};

use emmylua_parser::{LuaCallExpr, LuaChunk, LuaExpr, LuaSyntaxNode, LuaSyntaxToken};
use infer::InferResult;
pub use infer::LuaInferConfig;
use member::infer_members;
pub use member::LuaMemberInfo;
use reference::is_reference_to;
use rowan::{NodeOrToken, TextRange};
pub use semantic_info::SemanticInfo;
use semantic_info::{
    infer_node_property_owner, infer_node_semantic_info, infer_token_property_owner,
    infer_token_semantic_info,
};

use crate::LuaFunctionType;
use crate::{db_index::LuaTypeDeclId, Emmyrc, LuaDocument, LuaPropertyOwnerId};
use crate::{
    db_index::{DbIndex, LuaType},
    FileId,
};
pub(crate) use call_func::infer_call_expr_func;
pub(crate) use infer::{infer_expr, instantiate_doc_function};
use instantiate::instantiate_type;
use overload_resolve::resolve_signature;

#[derive(Debug)]
pub struct SemanticModel<'a> {
    file_id: FileId,
    db: &'a DbIndex,
    infer_config: LuaInferConfig,
    emmyrc: Arc<Emmyrc>,
    root: LuaChunk,
}

unsafe impl<'a> Send for SemanticModel<'a> {}
unsafe impl<'a> Sync for SemanticModel<'a> {}

impl<'a> SemanticModel<'a> {
    pub fn new(
        file_id: FileId,
        db: &'a DbIndex,
        infer_config: LuaInferConfig,
        emmyrc: Arc<Emmyrc>,
        root: LuaChunk,
    ) -> Self {
        Self {
            file_id,
            db,
            infer_config,
            emmyrc,
            root,
        }
    }

    pub fn get_document(&self) -> LuaDocument {
        self.db.get_vfs().get_document(&self.file_id).unwrap()
    }

    pub fn get_document_by_file_id(&self, file_id: FileId) -> Option<LuaDocument> {
        self.db.get_vfs().get_document(&file_id)
    }

    pub fn get_file_parse_error(&self) -> Option<Vec<(String, TextRange)>> {
        self.db.get_vfs().get_file_parse_error(&self.file_id)
    }

    pub fn infer_expr(&mut self, expr: LuaExpr) -> InferResult {
        infer_expr(self.db, &mut self.infer_config, expr)
    }

    pub fn infer_member_infos(&self, prefix_type: &LuaType) -> Option<Vec<LuaMemberInfo>> {
        infer_members(self.db, prefix_type)
    }

    pub fn infer_call_expr_func(
        &mut self,
        call_expr: LuaCallExpr,
        arg_count: Option<usize>,
    ) -> Option<Arc<LuaFunctionType>> {
        let prefix_expr = call_expr.get_prefix_expr()?;
        let call_expr_type = infer_expr(self.db, &mut self.infer_config, prefix_expr.into())?;
        infer_call_expr_func(
            self.db,
            &mut self.infer_config,
            call_expr,
            call_expr_type,
            &mut InferGuard::new(),
            arg_count
        )
    }

    pub fn get_semantic_info(
        &mut self,
        node_or_token: NodeOrToken<LuaSyntaxNode, LuaSyntaxToken>,
    ) -> Option<SemanticInfo> {
        match node_or_token {
            NodeOrToken::Node(node) => {
                infer_node_semantic_info(self.db, &mut self.infer_config, node)
            }
            NodeOrToken::Token(token) => {
                infer_token_semantic_info(self.db, &mut self.infer_config, token)
            }
        }
    }

    pub fn get_property_owner_id(
        &mut self,
        node_or_token: NodeOrToken<LuaSyntaxNode, LuaSyntaxToken>,
    ) -> Option<LuaPropertyOwnerId> {
        match node_or_token {
            NodeOrToken::Node(node) => {
                infer_node_property_owner(self.db, &mut self.infer_config, node)
            }
            NodeOrToken::Token(token) => {
                infer_token_property_owner(self.db, &mut self.infer_config, token)
            }
        }
    }

    pub fn is_reference_to(
        &mut self,
        node: LuaSyntaxNode,
        property_owner: LuaPropertyOwnerId,
    ) -> bool {
        is_reference_to(self.db, &mut self.infer_config, node, property_owner).unwrap_or(false)
    }

    pub fn get_emmyrc(&self) -> &Emmyrc {
        &self.emmyrc
    }

    pub fn get_root(&self) -> &LuaChunk {
        &self.root
    }

    pub fn get_db(&self) -> &DbIndex {
        self.db
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }
}

/// Guard to prevent infinite recursion
/// Some type may reference itself, so we need to check if we have already infered this type
#[derive(Debug)]
pub struct InferGuard {
    guard: HashSet<LuaTypeDeclId>,
}

impl InferGuard {
    pub fn new() -> Self {
        Self {
            guard: HashSet::default(),
        }
    }

    pub fn check(&mut self, type_id: &LuaTypeDeclId) -> Option<()> {
        if self.guard.contains(type_id) {
            return None;
        }
        self.guard.insert(type_id.clone());
        Some(())
    }
}
