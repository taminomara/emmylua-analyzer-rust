mod access_invisible;
mod analyze_error;
mod assign_type_mismatch;
mod await_in_sync;
mod check_field;
mod check_return_count;
mod circle_doc_class;
mod code_style;
mod code_style_check;
mod deprecated;
mod discard_returns;
mod duplicate_require;
mod duplicate_type;
mod incomplete_signature_doc;
mod local_const_reassign;
mod missing_fields;
mod missing_parameter;
mod need_check_nil;
mod param_type_check;
mod redefined_local;
mod redundant_parameter;
mod return_type_mismatch;
mod syntax_error;
mod undefined_doc_param;
mod undefined_global;
mod unused;

use code_style::check_file_code_style;
use emmylua_parser::{
    LuaAstNode, LuaClosureExpr, LuaComment, LuaReturnStat, LuaStat, LuaSyntaxKind,
};
use lsp_types::{Diagnostic, DiagnosticSeverity, DiagnosticTag, NumberOrString};
use rowan::TextRange;
use std::sync::Arc;

use crate::{
    db_index::DbIndex, humanize_type, semantic::SemanticModel, FileId, LuaType, RenderLevel,
};

use super::{
    lua_diagnostic_code::{get_default_severity, is_code_default_enable},
    lua_diagnostic_config::LuaDiagnosticConfig,
    DiagnosticCode,
};

pub fn check_file(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    macro_rules! check {
        ($module:ident) => {
            if $module::CODES
                .iter()
                .any(|code| context.is_checker_enable_by_code(code))
            {
                $module::check(context, semantic_model);
            }
        };
    }

    check!(syntax_error);
    check!(analyze_error);
    check!(unused);
    check!(deprecated);
    check!(undefined_global);
    check!(access_invisible);
    check!(missing_parameter);
    check!(redundant_parameter);
    check!(local_const_reassign);
    check!(discard_returns);
    check!(await_in_sync);
    check!(param_type_check);
    check!(need_check_nil);
    check!(code_style_check);
    check!(return_type_mismatch);
    check!(undefined_doc_param);
    check!(redefined_local);
    check!(missing_fields);
    check!(check_field);
    check!(circle_doc_class);
    check!(incomplete_signature_doc);
    check!(assign_type_mismatch);
    check!(duplicate_require);
    check!(check_return_count);

    check_file_code_style(context, semantic_model);
    Some(())
}

pub struct DiagnosticContext<'a> {
    file_id: FileId,
    db: &'a DbIndex,
    diagnostics: Vec<Diagnostic>,
    pub config: Arc<LuaDiagnosticConfig>,
}

impl<'a> DiagnosticContext<'a> {
    pub fn new(file_id: FileId, db: &'a DbIndex, config: Arc<LuaDiagnosticConfig>) -> Self {
        Self {
            file_id,
            db,
            diagnostics: Vec::new(),
            config,
        }
    }

    pub fn get_db(&self) -> &DbIndex {
        &self.db
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn add_diagnostic(
        &mut self,
        code: DiagnosticCode,
        range: TextRange,
        message: String,
        data: Option<serde_json::Value>,
    ) {
        if !self.is_checker_enable_by_code(&code) {
            return;
        }

        if !self.should_report_diagnostic(&code, &range) {
            return;
        }

        let diagnostic = Diagnostic {
            message,
            range: self.translate_range(range).unwrap_or(lsp_types::Range {
                start: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
            }),
            severity: self.get_severity(code),
            code: Some(NumberOrString::String(code.get_name().to_string())),
            source: Some("EmmyLua".into()),
            tags: self.get_tags(code),
            data,
            ..Default::default()
        };

        self.diagnostics.push(diagnostic);
    }

    fn should_report_diagnostic(&self, code: &DiagnosticCode, range: &TextRange) -> bool {
        let diagnostic_index = self.get_db().get_diagnostic_index();

        !diagnostic_index.is_file_diagnostic_code_disabled(&self.get_file_id(), code, range)
    }

    fn get_severity(&self, code: DiagnosticCode) -> Option<DiagnosticSeverity> {
        if let Some(severity) = self.config.severity.get(&code) {
            return Some(severity.clone());
        }

        Some(get_default_severity(code))
    }

    fn get_tags(&self, code: DiagnosticCode) -> Option<Vec<DiagnosticTag>> {
        match code {
            DiagnosticCode::Unused | DiagnosticCode::UnreachableCode => {
                Some(vec![DiagnosticTag::UNNECESSARY])
            }
            DiagnosticCode::Deprecated => Some(vec![DiagnosticTag::DEPRECATED]),
            _ => None,
        }
    }

    fn translate_range(&self, range: TextRange) -> Option<lsp_types::Range> {
        let document = self.db.get_vfs().get_document(&self.file_id)?;
        let (start_line, start_character) = document.get_line_col(range.start())?;
        let (end_line, end_character) = document.get_line_col(range.end())?;

        Some(lsp_types::Range {
            start: lsp_types::Position {
                line: start_line as u32,
                character: start_character as u32,
            },
            end: lsp_types::Position {
                line: end_line as u32,
                character: end_character as u32,
            },
        })
    }

    pub fn get_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    pub fn is_checker_enable_by_code(&self, code: &DiagnosticCode) -> bool {
        let file_id = self.get_file_id();
        let db = self.get_db();
        let diagnostic_index = db.get_diagnostic_index();
        // force enable
        if diagnostic_index.is_file_enabled(&file_id, &code) {
            return true;
        }

        // workspace force disabled
        if self.config.workspace_disabled.contains(&code) {
            return false;
        }

        let module_index = db.get_module_index();
        // ignore meta file diagnostic
        if module_index.is_meta_file(&file_id) {
            return false;
        }

        // is file disabled this code
        if diagnostic_index.is_file_disabled(&file_id, &code) {
            return false;
        }

        // workspace force enabled
        if self.config.workspace_enabled.contains(&code) {
            return true;
        }

        // default setting
        is_code_default_enable(&code)
    }
}

pub fn get_closure_expr_comment(closure_expr: &LuaClosureExpr) -> Option<LuaComment> {
    let comment = closure_expr
        .ancestors::<LuaStat>()
        .next()?
        .syntax()
        .prev_sibling()?;
    match comment.kind().into() {
        LuaSyntaxKind::Comment => {
            let comment = LuaComment::cast(comment)?;
            Some(comment)
        }
        _ => None,
    }
}

/// 获取属于自身的返回语句
pub fn get_own_return_stats(
    closure_expr: &LuaClosureExpr,
) -> impl Iterator<Item = LuaReturnStat> + '_ {
    closure_expr
        .descendants::<LuaReturnStat>()
        .filter(move |stat| {
            stat.ancestors::<LuaClosureExpr>()
                .next()
                .map_or(false, |expr| &expr == closure_expr)
        })
}

pub fn humanize_lint_type(db: &DbIndex, typ: &LuaType) -> String {
    match typ {
        LuaType::Ref(type_decl_id) => type_decl_id.get_simple_name().to_string(),
        LuaType::Generic(generic_type) => generic_type
            .get_base_type_id()
            .get_simple_name()
            .to_string(),
        LuaType::IntegerConst(_) => "integer".to_string(),
        LuaType::FloatConst(_) => "number".to_string(),
        LuaType::BooleanConst(_) => "boolean".to_string(),
        LuaType::StringConst(_) => "string".to_string(),
        LuaType::DocStringConst(_) => "string".to_string(),
        LuaType::DocIntegerConst(_) => "integer".to_string(),
        LuaType::DocBooleanConst(_) => "boolean".to_string(),
        _ => humanize_type(db, typ, RenderLevel::Simple),
    }
}
