mod cache;
mod generic;
mod infer;
mod member;
mod overload_resolve;
mod reference;
mod semantic_info;
mod type_check;
mod visibility;

use std::cell::RefCell;
use std::collections::HashMap;
use std::{collections::HashSet, sync::Arc};

pub use cache::{CacheEntry, CacheKey, CacheOptions, LuaAnalysisPhase, LuaInferCache};
use emmylua_parser::{LuaCallExpr, LuaChunk, LuaExpr, LuaSyntaxNode, LuaSyntaxToken, LuaTableExpr};
pub use infer::infer_table_field_value_should_be;
use infer::{
    infer_left_value_type_from_right_value, infer_multi_value_adjusted_expression_types,
    infer_table_should_be,
};
pub use member::LuaMemberInfo;
use member::{infer_member_map, infer_members};
use reference::is_reference_to;
use rowan::{NodeOrToken, TextRange};
pub use semantic_info::SemanticInfo;
use semantic_info::{
    infer_node_semantic_decl, infer_node_semantic_info, infer_token_semantic_decl,
    infer_token_semantic_info,
};
pub(crate) use type_check::check_type_compact;
use type_check::is_sub_type_of;
use visibility::check_visibility;

use crate::{db_index::LuaTypeDeclId, Emmyrc, LuaDocument, LuaSemanticDeclId};
use crate::{
    db_index::{DbIndex, LuaType},
    FileId,
};
use crate::{LuaFunctionType, LuaMemberKey};
pub use generic::{instantiate_type_generic, TypeSubstitutor};
pub use infer::InferFailReason;
pub(crate) use infer::{infer_call_expr_func, infer_expr};
use overload_resolve::resolve_signature;
pub use semantic_info::SemanticDeclLevel;
pub use type_check::{TypeCheckFailReason, TypeCheckResult};

#[derive(Debug)]
pub struct SemanticModel<'a> {
    file_id: FileId,
    db: &'a DbIndex,
    infer_cache: RefCell<LuaInferCache>,
    emmyrc: Arc<Emmyrc>,
    root: LuaChunk,
}

unsafe impl<'a> Send for SemanticModel<'a> {}
unsafe impl<'a> Sync for SemanticModel<'a> {}

impl<'a> SemanticModel<'a> {
    pub fn new(
        file_id: FileId,
        db: &'a DbIndex,
        infer_config: LuaInferCache,
        emmyrc: Arc<Emmyrc>,
        root: LuaChunk,
    ) -> Self {
        Self {
            file_id,
            db,
            infer_cache: RefCell::new(infer_config),
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

    pub fn get_root_by_file_id(&self, file_id: FileId) -> Option<LuaChunk> {
        Some(
            self.db
                .get_vfs()
                .get_syntax_tree(&file_id)?
                .get_chunk_node(),
        )
    }

    pub fn get_file_parse_error(&self) -> Option<Vec<(String, TextRange)>> {
        self.db.get_vfs().get_file_parse_error(&self.file_id)
    }

    pub fn infer_expr(&self, expr: LuaExpr) -> Result<LuaType, InferFailReason> {
        infer_expr(self.db, &mut self.infer_cache.borrow_mut(), expr)
    }

    pub fn infer_table_should_be(&self, table: LuaTableExpr) -> Option<LuaType> {
        infer_table_should_be(self.db, &mut self.infer_cache.borrow_mut(), table).ok()
    }

    pub fn infer_member_infos(&self, prefix_type: &LuaType) -> Option<Vec<LuaMemberInfo>> {
        infer_members(self.db, prefix_type)
    }

    pub fn infer_member_map(
        &self,
        prefix_type: &LuaType,
    ) -> Option<HashMap<LuaMemberKey, Vec<LuaMemberInfo>>> {
        infer_member_map(self.db, prefix_type)
    }

    pub fn type_check(&self, source: &LuaType, compact_type: &LuaType) -> TypeCheckResult {
        check_type_compact(self.db, source, compact_type)
    }

    pub fn infer_call_expr_func(
        &self,
        call_expr: LuaCallExpr,
        arg_count: Option<usize>,
    ) -> Option<Arc<LuaFunctionType>> {
        let prefix_expr = call_expr.get_prefix_expr()?;
        let call_expr_type = infer_expr(
            self.db,
            &mut self.infer_cache.borrow_mut(),
            prefix_expr.into(),
        )
        .ok()?;
        infer_call_expr_func(
            self.db,
            &mut self.infer_cache.borrow_mut(),
            call_expr,
            call_expr_type,
            &mut InferGuard::new(),
            arg_count,
        )
        .ok()
    }

    /// 获取赋值时所有右值类型或调用时所有参数类型或返回时所有返回值类型
    pub fn infer_multi_value_adjusted_expression_types(
        &self,
        exprs: &[LuaExpr],
        var_count: Option<usize>,
    ) -> Option<Vec<(LuaType, TextRange)>> {
        infer_multi_value_adjusted_expression_types(
            self.db,
            &mut self.infer_cache.borrow_mut(),
            exprs,
            var_count,
        )
    }

    /// 从右值推断左值已绑定的类型
    pub fn infer_left_value_type_from_right_value(&self, expr: LuaExpr) -> Option<LuaType> {
        infer_left_value_type_from_right_value(self.db, &mut self.infer_cache.borrow_mut(), expr)
    }

    pub fn get_semantic_info(
        &self,
        node_or_token: NodeOrToken<LuaSyntaxNode, LuaSyntaxToken>,
    ) -> Option<SemanticInfo> {
        match node_or_token {
            NodeOrToken::Node(node) => {
                infer_node_semantic_info(self.db, &mut self.infer_cache.borrow_mut(), node)
            }
            NodeOrToken::Token(token) => {
                infer_token_semantic_info(self.db, &mut self.infer_cache.borrow_mut(), token)
            }
        }
    }

    pub fn find_decl(
        &self,
        node_or_token: NodeOrToken<LuaSyntaxNode, LuaSyntaxToken>,
        level: SemanticDeclLevel,
    ) -> Option<LuaSemanticDeclId> {
        match node_or_token {
            NodeOrToken::Node(node) => {
                infer_node_semantic_decl(self.db, &mut self.infer_cache.borrow_mut(), node, level)
            }
            NodeOrToken::Token(token) => {
                infer_token_semantic_decl(self.db, &mut self.infer_cache.borrow_mut(), token, level)
            }
        }
    }

    pub fn is_reference_to(
        &self,
        node: LuaSyntaxNode,
        semantic_decl_id: LuaSemanticDeclId,
        level: SemanticDeclLevel,
    ) -> bool {
        is_reference_to(
            self.db,
            &mut self.infer_cache.borrow_mut(),
            node,
            semantic_decl_id,
            level,
        )
        .unwrap_or(false)
    }

    pub fn is_semantic_visible(
        &self,
        token: LuaSyntaxToken,
        property_owner: LuaSemanticDeclId,
    ) -> bool {
        check_visibility(
            self.db,
            self.file_id,
            &self.emmyrc,
            &mut self.infer_cache.borrow_mut(),
            token,
            property_owner,
        )
        .unwrap_or(true)
    }

    pub fn is_sub_type_of(
        &mut self,
        sub_type_ref_id: &LuaTypeDeclId,
        super_type_ref_id: &LuaTypeDeclId,
    ) -> bool {
        is_sub_type_of(self.db, sub_type_ref_id, super_type_ref_id)
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

    pub fn get_config(&self) -> &RefCell<LuaInferCache> {
        &self.infer_cache
    }
}

/// Guard to prevent infinite recursion
/// Some type may reference itself, so we need to check if we have already inferred this type
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

    pub fn check(&mut self, type_id: &LuaTypeDeclId) -> Result<(), InferFailReason> {
        if self.guard.contains(type_id) {
            return Err(InferFailReason::RecursiveInfer);
        }
        self.guard.insert(type_id.clone());
        Ok(())
    }
}
