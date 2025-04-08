use emmylua_code_analysis::{
    LuaMemberOwner, LuaSemanticDeclId, LuaType, SemanticDeclLevel, SemanticModel,
};
use emmylua_parser::{LuaAstNode, LuaCallExpr, LuaExpr};
use rowan::NodeOrToken;

use crate::handlers::hover::{get_function_member_owner, infer_prefix_global_name};

#[derive(Debug)]
pub struct SignatureHelperBuilder<'a> {
    pub semantic_model: &'a SemanticModel<'a>,
    pub call_expr: LuaCallExpr,
    pub prefix_name: Option<String>,
    pub function_name: String,
    self_type: Option<LuaType>,
}

impl<'a> SignatureHelperBuilder<'a> {
    pub fn new(semantic_model: &'a SemanticModel<'a>, call_expr: LuaCallExpr) -> Self {
        let mut builder = Self {
            semantic_model,
            call_expr,
            prefix_name: None,
            function_name: String::new(),
            self_type: None,
        };
        builder.self_type = builder.infer_self_type();
        builder.build_full_name();
        builder
    }

    fn infer_self_type(&self) -> Option<LuaType> {
        let prefix_expr = self.call_expr.get_prefix_expr();
        if let Some(prefix_expr) = prefix_expr {
            if let LuaExpr::IndexExpr(index) = prefix_expr {
                let self_expr = index.get_prefix_expr();
                if let Some(self_expr) = self_expr {
                    return self.semantic_model.infer_expr(self_expr.into()).ok();
                }
            }
        }
        None
    }

    pub fn get_self_type(&self) -> Option<LuaType> {
        self.self_type.clone()
    }

    pub fn build_full_name(&mut self) -> Option<()> {
        let semantic_model = self.semantic_model;
        let db = semantic_model.get_db();
        let prefix_expr = self.call_expr.get_prefix_expr()?;
        let mut semantic_decl = semantic_model.find_decl(
            NodeOrToken::Node(prefix_expr.syntax().clone().into()),
            SemanticDeclLevel::Trace(50),
        );
        // 推断为来源
        semantic_decl = match semantic_decl {
            Some(LuaSemanticDeclId::Member(member_id)) => {
                get_function_member_owner(semantic_model, member_id).or(semantic_decl)
            }
            Some(LuaSemanticDeclId::LuaDecl(_)) => semantic_decl,
            _ => None,
        };
        let Some(semantic_decl) = semantic_decl else {
            return None;
        };
        dbg!(&semantic_decl);

        match semantic_decl {
            LuaSemanticDeclId::Member(member_id) => {
                let member = db.get_member_index().get_member(&member_id)?;
                let global_name = infer_prefix_global_name(self.semantic_model, member);
                // 处理前缀
                let parent_owner = db.get_member_index().get_current_owner(&member.get_id());
                if let Some(LuaMemberOwner::Type(ty)) = parent_owner {
                    let mut name = String::new();
                    // 如果是全局定义, 则使用定义时的名称
                    if let Some(global_name) = global_name {
                        name.push_str(global_name);
                    } else {
                        name.push_str(ty.get_simple_name());
                    }
                    self.prefix_name = Some(name);
                }
                self.function_name = member.get_key().to_path().to_string();
            }
            LuaSemanticDeclId::LuaDecl(decl_id) => {
                let decl = db.get_decl_index().get_decl(&decl_id)?;
                self.function_name = decl.get_name().to_string();
            }
            _ => {}
        }
        Some(())
    }
}
