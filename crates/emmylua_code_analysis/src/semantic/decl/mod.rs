use std::collections::HashSet;

use emmylua_parser::{LuaAstNode, LuaIndexExpr};
use rowan::NodeOrToken;

use crate::{
    infer_node_semantic_decl, semantic::semantic_info::infer_token_semantic_decl, DbIndex,
    LuaDeclId, LuaInferCache, LuaSemanticDeclId, LuaType, SemanticDeclLevel,
};

pub fn enum_variable_is_param(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    index_expr: &LuaIndexExpr,
    prefix_typ: &LuaType,
) -> Option<()> {
    let LuaType::Ref(id) = prefix_typ else {
        return None;
    };

    let type_decl = db.get_type_index().get_type_decl(id)?;
    if !type_decl.is_enum() {
        return None;
    }

    let prefix_expr = index_expr.get_prefix_expr()?;
    let prefix_decl = infer_node_semantic_decl(
        db,
        cache,
        prefix_expr.syntax().clone(),
        SemanticDeclLevel::default(),
    )?;

    let LuaSemanticDeclId::LuaDecl(decl_id) = prefix_decl else {
        return None;
    };

    let mut decl_guard = DeclGuard::new();
    let origin_decl_id = find_enum_origin(db, cache, decl_id, &mut decl_guard).unwrap_or(decl_id);
    let decl = db.get_decl_index().get_decl(&origin_decl_id)?;

    if decl.is_param() {
        Some(())
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclGuard {
    decl_set: HashSet<LuaDeclId>,
}

impl DeclGuard {
    pub fn new() -> Self {
        Self {
            decl_set: HashSet::new(),
        }
    }

    pub fn check(&mut self, decl_id: LuaDeclId) -> Option<()> {
        if self.decl_set.contains(&decl_id) {
            None
        } else {
            self.decl_set.insert(decl_id);
            Some(())
        }
    }
}

fn find_enum_origin(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    decl_id: LuaDeclId,
    decl_guard: &mut DeclGuard,
) -> Option<LuaDeclId> {
    decl_guard.check(decl_id)?;
    let syntax_tree = db.get_vfs().get_syntax_tree(&decl_id.file_id)?;
    let root = syntax_tree.get_red_root();

    let node = db
        .get_decl_index()
        .get_decl(&decl_id)?
        .get_value_syntax_id()?
        .to_node_from_root(&root)?;

    let semantic_decl = match node.into() {
        NodeOrToken::Node(node) => {
            infer_node_semantic_decl(db, cache, node, SemanticDeclLevel::NoTrace)
        }
        NodeOrToken::Token(token) => {
            infer_token_semantic_decl(db, cache, token, SemanticDeclLevel::NoTrace)
        }
    };

    match semantic_decl {
        Some(LuaSemanticDeclId::Member(_)) => None,
        Some(LuaSemanticDeclId::LuaDecl(new_decl_id)) => {
            let decl = db.get_decl_index().get_decl(&new_decl_id)?;
            if decl.get_value_syntax_id().is_some() {
                Some(find_enum_origin(db, cache, new_decl_id, decl_guard).unwrap_or(new_decl_id))
            } else {
                Some(new_decl_id)
            }
        }
        _ => None,
    }
}
