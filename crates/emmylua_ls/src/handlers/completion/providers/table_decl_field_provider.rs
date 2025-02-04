use std::collections::HashSet;

use emmylua_code_analysis::LuaType;
use emmylua_parser::{
    LuaAst, LuaAstNode, LuaAstToken, LuaCallArgList, LuaCallExpr, LuaNameExpr, LuaNameToken,
    LuaTableExpr, LuaTableField,
};

use crate::handlers::completion::{
    add_completions::{add_member_completion, CompletionTriggerStatus},
    completion_builder::CompletionBuilder,
};

pub fn add_completion(builder: &mut CompletionBuilder) -> Option<()> {
    let table_expr = LuaNameToken::cast(builder.trigger_token.clone())?
        .get_parent::<LuaNameExpr>()?
        .get_parent::<LuaTableField>()?
        .get_parent::<LuaTableExpr>()?;

    // todo non-function completion (e.g. in other tables)
    // todo support parents which types are inferred implicitly
    let table_type = match table_expr.get_parent()? {
        LuaAst::LuaCallArgList(call_arg_list) => {
            get_table_type_by_calleee(builder, call_arg_list, LuaAst::LuaTableExpr(table_expr))
        }
        _ => None,
    }?;

    let mut duplicated_set = HashSet::new();
    let member_infos = builder.semantic_model.infer_member_infos(&table_type)?;
    for member_info in member_infos {
        if duplicated_set.contains(&member_info.key) {
            continue;
        }

        duplicated_set.insert(member_info.key.clone());
        add_member_completion(builder, member_info, CompletionTriggerStatus::InTable);
    }

    Some(())
}

fn get_table_type_by_calleee(
    builder: &mut CompletionBuilder,
    call_arg_list: LuaCallArgList,
    table_expr: LuaAst,
) -> Option<LuaType> {
    let call_expr = call_arg_list.get_parent::<LuaCallExpr>()?;
    let func_type = builder
        .semantic_model
        .infer_call_expr_func(call_expr.clone(), None)?;
    let param_types = func_type.get_params();
    let call_arg_number = call_arg_list
        .children::<LuaAst>()
        .into_iter()
        .enumerate()
        .find(|(_, arg)| *arg == table_expr)?
        .0;
    let table_type = param_types.get(call_arg_number)?.1.clone();

    return table_type;
}
