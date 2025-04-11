mod broadcast_down;
mod broadcast_outside;
mod broadcast_up;

use broadcast_down::broadcast_down;
use broadcast_up::broadcast_up;
use emmylua_parser::{
    BinaryOperator, LuaAssignStat, LuaAst, LuaAstNode, LuaBinaryExpr, LuaBlock, LuaCallArgList,
    LuaCallExpr, LuaCallExprStat, LuaCommentOwner, LuaDocTag, LuaExpr, LuaLiteralToken, LuaStat,
    LuaVarExpr,
};

use crate::{
    db_index::{LuaType, TypeAssertion},
    DbIndex, FileId, LuaDeclId, LuaFlowChain, LuaMemberId, LuaTypeDeclId, LuaTypeOwner, VarRefId,
};

pub fn analyze_ref_expr(
    db: &mut DbIndex,
    flow_chain: &mut LuaFlowChain,
    var_expr: &LuaVarExpr,
    var_ref_id: &VarRefId,
) -> Option<()> {
    let parent = var_expr.get_parent::<LuaAst>()?;
    broadcast_up(
        db,
        flow_chain,
        &var_ref_id,
        parent,
        LuaAst::cast(var_expr.syntax().clone())?,
        TypeAssertion::Exist,
    );

    Some(())
}

pub fn analyze_ref_assign(
    db: &mut DbIndex,
    flow_chain: &mut LuaFlowChain,
    var_expr: &LuaVarExpr,
    var_ref_id: &VarRefId,
    file_id: FileId,
) -> Option<()> {
    let assign_stat = var_expr.get_parent::<LuaAssignStat>()?;
    if is_decl_assign_stat(assign_stat.clone()).unwrap_or(false) {
        let type_owner = match var_expr {
            LuaVarExpr::IndexExpr(index_expr) => {
                let member_id = LuaMemberId::new(index_expr.get_syntax_id(), file_id);
                LuaTypeOwner::Member(member_id)
            }
            LuaVarExpr::NameExpr(name_expr) => {
                let decl_id = LuaDeclId::new(file_id, name_expr.get_position());
                LuaTypeOwner::Decl(decl_id)
            }
        };
        if let Some(type_cache) = db.get_type_index().get_type_cache(&type_owner) {
            let type_assert = TypeAssertion::Narrow(type_cache.as_type().clone());
            broadcast_down(
                db,
                flow_chain,
                var_ref_id,
                LuaAst::LuaAssignStat(assign_stat),
                type_assert,
                true,
            );
        }

        return None;
    }

    let (var_exprs, value_exprs) = assign_stat.get_var_and_expr_list();
    let var_index = var_exprs
        .iter()
        .position(|it| it.get_position() == var_expr.get_position())?;

    if value_exprs.len() == 0 {
        return None;
    }

    let (value_expr, idx) = if let Some(expr) = value_exprs.get(var_index) {
        (expr.clone(), 0)
    } else {
        (
            value_exprs.last()?.clone(),
            (var_index - (value_exprs.len() - 1)) as i32,
        )
    };

    let type_assert = TypeAssertion::Reassign((value_expr.get_syntax_id(), idx));
    broadcast_down(
        db,
        flow_chain,
        var_ref_id,
        LuaAst::LuaAssignStat(assign_stat),
        type_assert,
        true,
    );

    Some(())
}

fn is_decl_assign_stat(assign_stat: LuaAssignStat) -> Option<bool> {
    for comment in assign_stat.get_comments() {
        for tag in comment.get_doc_tags() {
            match tag {
                LuaDocTag::Type(_)
                | LuaDocTag::Class(_)
                | LuaDocTag::Module(_)
                | LuaDocTag::Enum(_) => {
                    return Some(true);
                }
                _ => {}
            }
        }
    }
    Some(false)
}

fn infer_call_arg_list(
    db: &mut DbIndex,
    flow_chain: &mut LuaFlowChain,
    type_assert: TypeAssertion,
    var_ref_id: &VarRefId,
    call_arg: LuaCallArgList,
) -> Option<()> {
    let parent = call_arg.get_parent::<LuaAst>()?;
    match parent {
        LuaAst::LuaCallExpr(call_expr) => {
            if call_expr.is_type() {
                infer_lua_type_assert(db, flow_chain, var_ref_id, call_expr);
            } else if call_expr.is_assert() {
                infer_lua_assert(db, flow_chain, type_assert, var_ref_id, call_expr);
            }
        }
        _ => {}
    }

    Some(())
}

fn infer_lua_type_assert(
    db: &mut DbIndex,
    flow_chain: &mut LuaFlowChain,
    var_ref_id: &VarRefId,
    call_expr: LuaCallExpr,
) -> Option<()> {
    let binary_expr = call_expr.get_parent::<LuaBinaryExpr>()?;
    let op = binary_expr.get_op_token()?;
    let mut is_eq = true;
    match op.get_op() {
        BinaryOperator::OpEq => {}
        BinaryOperator::OpNe => {
            is_eq = false;
        }
        _ => return None,
    };

    let operands = binary_expr.get_exprs()?;
    let literal_expr = if let LuaExpr::LiteralExpr(literal) = operands.0 {
        literal
    } else if let LuaExpr::LiteralExpr(literal) = operands.1 {
        literal
    } else {
        return None;
    };

    let type_literal = match literal_expr.get_literal()? {
        LuaLiteralToken::String(string) => string.get_value(),
        _ => return None,
    };

    let mut type_assert = match type_literal.as_str() {
        "number" => TypeAssertion::Narrow(LuaType::Number),
        "string" => TypeAssertion::Narrow(LuaType::String),
        "boolean" => TypeAssertion::Narrow(LuaType::Boolean),
        "table" => TypeAssertion::Narrow(LuaType::Table),
        "function" => TypeAssertion::Narrow(LuaType::Function),
        "thread" => TypeAssertion::Narrow(LuaType::Thread),
        "userdata" => TypeAssertion::Narrow(LuaType::Userdata),
        "nil" => TypeAssertion::Narrow(LuaType::Nil),
        // extend usage
        str => TypeAssertion::Narrow(LuaType::Ref(LuaTypeDeclId::new(str))),
    };

    if !is_eq {
        type_assert = type_assert.get_negation()?;
    }

    broadcast_up(
        db,
        flow_chain,
        var_ref_id,
        binary_expr.get_parent::<LuaAst>()?,
        LuaAst::LuaBinaryExpr(binary_expr),
        type_assert,
    );

    Some(())
}

fn is_block_has_return(block: Option<LuaBlock>) -> Option<bool> {
    if let Some(block) = block {
        for stat in block.get_stats() {
            if is_stat_change_flow(stat.clone()).unwrap_or(false) {
                return Some(true);
            }
        }
    }

    Some(false)
}

fn is_stat_change_flow(stat: LuaStat) -> Option<bool> {
    match stat {
        LuaStat::CallExprStat(call_stat) => {
            let call_expr = call_stat.get_call_expr()?;
            if call_expr.is_error() {
                return Some(true);
            }
            Some(false)
        }
        LuaStat::ReturnStat(_) => Some(true),
        LuaStat::DoStat(do_stat) => Some(is_block_has_return(do_stat.get_block()).unwrap_or(false)),
        LuaStat::BreakStat(_) => Some(true),
        _ => Some(false),
    }
}

fn infer_lua_assert(
    db: &mut DbIndex,
    flow_chain: &mut LuaFlowChain,
    type_assert: TypeAssertion,
    var_ref_id: &VarRefId,
    call_expr: LuaCallExpr,
) -> Option<()> {
    broadcast_down(
        db,
        flow_chain,
        var_ref_id,
        LuaAst::LuaCallExprStat(call_expr.get_parent::<LuaCallExprStat>()?),
        type_assert.clone(),
        true,
    );
    Some(())
}
