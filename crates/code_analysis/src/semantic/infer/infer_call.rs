use emmylua_parser::LuaCallExpr;

use crate::db_index::{DbIndex, LuaMultiReturn, LuaType};

use super::{infer_expr, LuaInferConfig};

pub fn infer_call_expr(
    db: &DbIndex,
    config: &mut LuaInferConfig,
    call_expr: LuaCallExpr,
) -> Option<LuaType> {
    let prefix_expr = call_expr.get_prefix_expr()?;
    let prefix_type = infer_expr(db, config, prefix_expr)?;

    infer_call_result(db, config, prefix_type, call_expr)
}

fn infer_call_result(db: &DbIndex, config: &mut LuaInferConfig, prefix_type: LuaType, call_expr: LuaCallExpr) -> Option<LuaType> {
    let return_type = match prefix_type {
        LuaType::DocFunction(func) =>{
            let rets = func.get_ret();
            let is_generic_rets = rets.iter().any(|ret| ret.is_tpl());
            if is_generic_rets {
                // instantiate_doc_function(db, config, prefix_type);
                todo!()
            } else {
                match rets.len() {
                    0 => LuaType::Nil,
                    1 => rets[0].clone(),
                    _ => LuaType::MuliReturn(LuaMultiReturn::Multi(rets.to_vec()).into()),
                }
            }
        },
        LuaType::Signature(signature_id) => {
            todo!()
        },
        _ => return None,
    };

    unwrapp_return_type(db, config, return_type, call_expr)
}

fn unwrapp_return_type(db: &DbIndex, config: &mut LuaInferConfig, return_type: LuaType, call_expr: LuaCallExpr) -> Option<LuaType> {
    todo!()
}

#[allow(unused)]
fn instantiate_doc_function(db: &DbIndex, config: &mut LuaInferConfig, func: LuaType) -> Option<LuaType> {
    todo!()
}