use std::collections::{HashMap, HashSet};

use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaComment, LuaDocFieldKey, LuaDocTagClass, LuaDocTagField,
    LuaDocType, LuaNameToken, LuaSyntaxId, LuaTokenKind,
};
use rowan::TextRange;

use crate::{DiagnosticCode, LuaPropertyOwnerId, SemanticModel};

use super::DiagnosticContext;

pub const CODES: &[DiagnosticCode] = &[DiagnosticCode::DuplicateDocField];

pub fn check(context: &mut DiagnosticContext, semantic_model: &SemanticModel) -> Option<()> {
    let root = semantic_model.get_root().clone();
    let mut duplicated_name = HashSet::new();
    for class_tag in root.descendants::<LuaDocTagClass>() {
        let name = class_tag.get_name_token()?.get_name_text().to_string();
        if duplicated_name.contains(&name) {
            continue;
        }
        check_class_field(context, semantic_model, &class_tag);
        duplicated_name.insert(name);
    }
    Some(())
}

fn check_class_field(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
    class_tag: &LuaDocTagClass,
) -> Option<()> {
    let property_owner = semantic_model.get_property_owner_id(
        class_tag
            .get_syntax_id()
            .to_node_from_root(semantic_model.get_root().syntax())
            .unwrap()
            .into(),
    )?;

    if let LuaPropertyOwnerId::TypeDecl(decl_id) = property_owner {
        let decl = semantic_model
            .get_db()
            .get_type_index()
            .get_type_decl(&decl_id)?;

        let mut field_keys: HashMap<String, (TextRange, bool)> = HashMap::new();
        let current_file_id = semantic_model.get_file_id();

        // 遍历所有声明位置
        for location in decl.get_locations() {
            let root = semantic_model.get_root_by_file_id(location.file_id)?;
            let syntax_id = LuaSyntaxId::new(LuaTokenKind::TkName.into(), location.range);
            let token = LuaNameToken::cast(syntax_id.to_token_from_root(root.syntax())?)?;
            let comment = token.ancestors::<LuaComment>().next()?;

            for field in comment.children::<LuaDocTagField>() {
                if let Some(typ) = field.get_type() {
                    // 允许 Func 重载
                    if matches!(typ, LuaDocType::Func(_)) {
                        continue;
                    }
                }

                let field_key_name = match field.get_field_key()? {
                    LuaDocFieldKey::Name(name_token) => name_token.get_name_text().to_string(),
                    LuaDocFieldKey::String(string_token) => string_token.get_value(),
                    LuaDocFieldKey::Integer(integer_token) => {
                        format!("[{}]", integer_token.get_int_value())
                    }
                    _ => continue,
                };

                let is_current_file = location.file_id == current_file_id;
                let range = field.get_field_key_range()?;

                if let Some(&(prev_range, prev_is_current)) = field_keys.get(&field_key_name) {
                    if is_current_file {
                        context.add_diagnostic(
                            DiagnosticCode::DuplicateDocField,
                            range,
                            format!("Duplicate field '{}'", field_key_name),
                            None,
                        );
                    }
                    if prev_is_current {
                        context.add_diagnostic(
                            DiagnosticCode::DuplicateDocField,
                            prev_range,
                            format!("Duplicate field '{}'", field_key_name),
                            None,
                        );
                    }
                } else {
                    field_keys.insert(field_key_name, (range, is_current_file));
                }
            }
        }
    }

    Some(())
}
