use emmylua_code_analysis::{
    LuaMember, LuaMemberOwner, LuaPropertyOwnerId, LuaType, SemanticModel,
};
use emmylua_parser::{LuaAstNode, LuaCallExpr, LuaSyntaxKind, LuaSyntaxToken};
use lsp_types::{Hover, HoverContents, MarkedString, MarkupContent};

use super::build_hover::{add_signature_param_description, add_signature_ret_description};

#[derive(Debug)]
pub struct HoverBuilder<'a> {
    /// 类型描述, 不包含 overload
    pub type_description: MarkedString,
    /// 类的全路径
    pub location_path: Option<MarkedString>,
    /// 函数重载签名, 第一个是重载签名
    pub signature_overload: Option<Vec<MarkedString>>,
    /// 注释描述, 包含函数参数与返回值描述
    pub annotation_description: Vec<MarkedString>,

    pub is_completion: bool,
    trigger_token: Option<LuaSyntaxToken>,
    pub semantic_model: &'a SemanticModel<'a>,
}

impl<'a> HoverBuilder<'a> {
    pub fn new(
        semantic_model: &'a SemanticModel,
        token: Option<LuaSyntaxToken>,
        is_completion: bool,
    ) -> Self {
        Self {
            semantic_model,
            type_description: MarkedString::String("".to_string()),
            location_path: None,
            signature_overload: None,
            annotation_description: Vec::new(),
            is_completion,
            trigger_token: token,
        }
    }

    pub fn set_type_description(&mut self, type_description: String) {
        self.type_description =
            MarkedString::from_language_code("lua".to_string(), type_description);
    }

    pub fn set_location_path(&mut self, owner_member: Option<&LuaMember>) {
        if let Some(owner_member) = owner_member {
            if let LuaMemberOwner::Type(ty) = &owner_member.get_owner() {
                self.location_path = Some(MarkedString::from_markdown(format!(
                    "{}{} `{}`",
                    "&nbsp;&nbsp;",
                    "in class",
                    ty.get_name()
                )));
            }
        }
    }

    pub fn add_signature_overload(&mut self, signature_overload: String) {
        if self.signature_overload.is_none() {
            self.signature_overload = Some(Vec::new());
        }
        self.signature_overload
            .as_mut()
            .unwrap()
            .push(MarkedString::from_language_code(
                "lua".to_string(),
                signature_overload,
            ));
    }

    pub fn add_annotation_description(&mut self, annotation_description: String) {
        self.annotation_description
            .push(MarkedString::from_markdown(annotation_description));
    }

    pub fn add_description(&mut self, property_owner: LuaPropertyOwnerId) -> Option<()> {
        if let Some(property) = self
            .semantic_model
            .get_db()
            .get_property_index()
            .get_property(property_owner.clone())
        {
            if let Some(detail) = &property.description {
                self.add_annotation_description(detail.to_string());
                return Some(());
            }
        }
        None
    }

    pub fn add_signature_params_rets_description(&mut self, typ: LuaType) {
        if let LuaType::Signature(signature_id) = typ {
            add_signature_param_description(
                &self.semantic_model.get_db(),
                &mut self.annotation_description,
                signature_id,
            );
            if self.is_completion {
                add_signature_ret_description(
                    &self.semantic_model.get_db(),
                    &mut self.annotation_description,
                    signature_id,
                );
            }
        }
    }

    /// 尝试设置完全匹配的签名
    pub fn try_set_full_match_signature(&mut self) -> Option<()> {
        if !self.is_completion || self.signature_overload.is_none() {
            return None;
        }
        // 根据当前输入的参数, 匹配完全匹配的签名
        if let Some(token) = self.trigger_token.clone() {
            if let Some(call_expr) = token.parent()?.parent() {
                match call_expr.kind().into() {
                    LuaSyntaxKind::CallExpr => {
                        let call_expr = LuaCallExpr::cast(call_expr)?;
                        let func = self.semantic_model.infer_call_expr_func(call_expr, None);
                        if let Some(func) = func {
                            dbg!(&func);
                        }
                    }
                    _ => {}
                }
            }
        }

        Some(())
    }

    pub fn build_hover_result(&self, range: Option<lsp_types::Range>) -> Option<Hover> {
        let mut result = String::new();
        match &self.type_description {
            MarkedString::String(s) => {
                result.push_str(&format!("\n{}\n", s));
            }
            MarkedString::LanguageString(s) => {
                result.push_str(&format!("\n```{}\n{}\n```\n", s.language, s.value));
            }
        }
        if let Some(location_path) = &self.location_path {
            match location_path {
                MarkedString::String(s) => {
                    result.push_str(&format!("\n{}\n", s));
                }
                _ => {}
            }
        }

        for marked_string in &self.annotation_description {
            match marked_string {
                MarkedString::String(s) => {
                    result.push_str(&format!("\n{}\n", s));
                }
                MarkedString::LanguageString(s) => {
                    result.push_str(&format!("\n```{}\n{}\n```\n", s.language, s.value));
                }
            }
        }

        if let Some(signature_overload) = &self.signature_overload {
            result.push_str("\n---\n");
            for signature in signature_overload {
                match signature {
                    MarkedString::String(s) => {
                        result.push_str(&format!("\n{}\n", s));
                    }
                    MarkedString::LanguageString(s) => {
                        result.push_str(&format!("\n```{}\n{}\n```\n", s.language, s.value));
                    }
                }
            }
        }

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: lsp_types::MarkupKind::Markdown,
                value: result,
            }),
            range,
        })
    }
}
