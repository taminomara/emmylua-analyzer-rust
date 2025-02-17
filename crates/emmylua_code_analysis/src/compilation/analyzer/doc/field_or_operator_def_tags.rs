use emmylua_parser::{
    LuaAstNode, LuaAstToken, LuaDocDescriptionOwner, LuaDocFieldKey, LuaDocTagField,
    LuaDocTagOperator,
};

use crate::db_index::{
    LuaMember, LuaMemberKey, LuaMemberOwner, LuaOperator, LuaOperatorMetaMethod,
    LuaPropertyOwnerId, LuaType,
};

use super::{infer_type::infer_type, DocAnalyzer};

pub fn analyze_field(analyzer: &mut DocAnalyzer, tag: LuaDocTagField) -> Option<()> {
    let current_type_id = analyzer.current_type_id.clone()?;
    let owner = LuaMemberOwner::Type(current_type_id.clone());
    let visibility_kind = if let Some(visibility_token) = tag.get_visibility_token() {
        Some(visibility_token.get_visibility())
    } else {
        None
    };

    let nullable = tag.is_nullable();
    let mut type_ref = if let Some(type_ref) = tag.get_type() {
        infer_type(analyzer, type_ref)
    } else {
        LuaType::Unknown
    };

    if nullable && !type_ref.is_nullable() {
        type_ref = LuaType::Nullable(type_ref.into());
    }

    let description = if let Some(description) = tag.get_description() {
        Some(description.get_description_text().to_string())
    } else {
        None
    };

    let key = tag.get_field_key()?;
    let member_id = match key {
        LuaDocFieldKey::Name(name_token) => {
            let key = LuaMemberKey::Name(name_token.get_name_text().to_string().into());
            let member = LuaMember::new(
                owner,
                key.clone(),
                analyzer.file_id,
                tag.get_syntax_id(),
                Some(type_ref),
            );

            analyzer.db.get_reference_index_mut().add_index_reference(
                key,
                analyzer.file_id,
                tag.get_syntax_id(),
            );
            analyzer.db.get_member_index_mut().add_member(member)
        }
        LuaDocFieldKey::String(string_token) => {
            let key = LuaMemberKey::Name(string_token.get_value().into());
            let member = LuaMember::new(
                owner,
                key.clone(),
                analyzer.file_id,
                tag.get_syntax_id(),
                Some(type_ref),
            );

            analyzer.db.get_reference_index_mut().add_index_reference(
                key,
                analyzer.file_id,
                tag.get_syntax_id(),
            );
            analyzer.db.get_member_index_mut().add_member(member)
        }
        LuaDocFieldKey::Integer(int_token) => {
            let key = LuaMemberKey::Integer(int_token.get_int_value());
            let member = LuaMember::new(
                owner,
                key.clone(),
                analyzer.file_id,
                tag.get_syntax_id(),
                Some(type_ref),
            );

            analyzer.db.get_reference_index_mut().add_index_reference(
                key,
                analyzer.file_id,
                tag.get_syntax_id(),
            );
            analyzer.db.get_member_index_mut().add_member(member)
        }
        LuaDocFieldKey::Type(doc_type) => {
            let range = doc_type.get_range();
            let key_type_ref = infer_type(analyzer, doc_type);
            if key_type_ref.is_unknown() {
                return None;
            }

            let operator = LuaOperator::new(
                current_type_id,
                LuaOperatorMetaMethod::Index,
                vec![key_type_ref],
                type_ref,
                analyzer.file_id,
                range,
            );
            analyzer.db.get_operator_index_mut().add_operator(operator);
            return Some(());
        }
    };

    let property_owner = LuaPropertyOwnerId::Member(member_id);
    if let Some(visibility_kind) = visibility_kind {
        analyzer.db.get_property_index_mut().add_visibility(
            analyzer.file_id,
            property_owner.clone(),
            visibility_kind,
        );
    }

    if let Some(description) = description {
        analyzer.db.get_property_index_mut().add_description(
            analyzer.file_id,
            property_owner.clone(),
            description,
        );
    }

    Some(())
}

pub fn analyze_operator(analyzer: &mut DocAnalyzer, tag: LuaDocTagOperator) -> Option<()> {
    let current_type_id = analyzer.current_type_id.clone()?;
    let name_token = tag.get_name_token()?;
    let op_kind = LuaOperatorMetaMethod::from_str(name_token.get_name_text())?;
    let operands: Vec<LuaType> = tag
        .get_param_list()?
        .get_types()
        .map(|operand| infer_type(analyzer, operand))
        .collect();

    let return_type = if let Some(return_type) = tag.get_return_type() {
        infer_type(analyzer, return_type)
    } else {
        LuaType::Unknown
    };

    let operator = LuaOperator::new(
        current_type_id,
        op_kind,
        operands,
        return_type,
        analyzer.file_id,
        name_token.get_range(),
    );

    analyzer.db.get_operator_index_mut().add_operator(operator);

    Some(())
}
