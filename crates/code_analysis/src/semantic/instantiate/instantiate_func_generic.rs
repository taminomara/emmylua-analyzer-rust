use std::collections::HashMap;

use emmylua_parser::{LuaAstNode, LuaCallExpr, LuaSyntaxNode};

use crate::{
    db_index::{DbIndex, LuaType},
    semantic::{infer_expr, LuaInferConfig},
};

use super::{instantiate_type, tpl_pattern::tpl_pattern_match};

pub fn instantiate_func(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    colon_define: bool,
    call_expr: LuaCallExpr,
    func_param_types: &mut Vec<LuaType>,
    func_return_types: &mut Vec<LuaType>,
) -> Option<()> {
    let arg_list = call_expr.get_args_list()?;
    let mut arg_types = Vec::new();
    for arg in arg_list.get_args() {
        let arg_type = infer_expr(db, infer_config, arg.clone())?;
        arg_types.push(arg_type);
    }

    let colon_call = call_expr.is_colon_call();

    match (colon_define, colon_call) {
        (true, true) | (false, false) => {}
        (true, false) => {
            if !arg_types.is_empty() {
                arg_types.remove(0);
            }
        }
        (false, true) => {
            arg_types.insert(0, LuaType::Any);
        }
    }

    instantiate_func_by_args(
        db,
        infer_config,
        func_param_types,
        func_return_types,
        &arg_types,
        &call_expr.get_root(),
    );
    // instantiate_func_by_return(
    //     db,
    //     infer_config,
    //     file_id,
    //     func_param_types,
    //     func_return_types,
    // );

    Some(())
}

fn instantiate_func_by_args(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    func_param_types: &mut Vec<LuaType>,
    func_return_types: &mut Vec<LuaType>,
    arg_types: &Vec<LuaType>,
    root: &LuaSyntaxNode,
) -> Option<()> {
    let mut result = HashMap::new();
    for i in 0..func_param_types.len() {
        let func_param_type = &mut func_param_types[i];
        let arg_type = if i < arg_types.len() {
            &arg_types[i]
        } else {
            continue;
        };

        tpl_pattern_match(
            db,
            infer_config,
            root,
            func_param_type,
            arg_type,
            &mut result,
        );
    }

    let max = *result.keys().max()?;
    let mut generic_params = Vec::new();
    for i in 0..max {
        if let Some(generic_param) = result.get(&i) {
            generic_params.push(generic_param.clone());
        } else {
            generic_params.push(LuaType::Unknown);
        }
    }

    for i in 0..func_param_types.len() {
        let func_param_type = &mut func_param_types[i];
        let new_func_param_type = instantiate_type(&func_param_type, &generic_params);
        *func_param_type = new_func_param_type;
    }

    for i in 0..func_return_types.len() {
        let func_return_type = &mut func_return_types[i];
        let new_func_return_type = instantiate_type(&func_return_type, &generic_params);
        *func_return_type = new_func_return_type;
    }

    Some(())
}

// fn instantiate_func_by_return(
//     db: &mut DbIndex,
//     infer_config: &mut LuaInferConfig,
//     file_id: FileId,
//     func_param_types: &mut Vec<LuaType>,
//     func_return_types: &mut Vec<LuaType>,
//     return_types: Vec<LuaType>,
// ) -> Option<()> {
//     todo!()
// }
