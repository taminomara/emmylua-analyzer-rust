use std::sync::Arc;

use emmylua_parser::{BinaryOperator, LuaAst, LuaBinaryExpr, LuaExpr};

use crate::TypeAssertion;

#[derive(Debug, Clone)]
pub struct VarTraceInfo {
    pub type_assertion: TypeAssertion,
    pub node: LuaAst,
}

impl VarTraceInfo {
    pub fn new(type_assertion: TypeAssertion, node: LuaAst) -> Self {
        Self {
            type_assertion,
            node,
        }
    }

    pub fn with_type_assertion(&self, type_assertion: TypeAssertion) -> Arc<VarTraceInfo> {
        Arc::new(VarTraceInfo {
            type_assertion,
            node: self.node.clone(),
        })
    }

    pub fn check_cover_all_branch(&self) -> bool {
        match &self.node {
            LuaAst::LuaBinaryExpr(binary_expr) => {
                if let Some(op) = binary_expr.get_op_token() {
                    match op.get_op() {
                        BinaryOperator::OpAnd => {
                            let count = count_binary_all_branch(binary_expr);
                            if let TypeAssertion::And(a) = &self.type_assertion {
                                return count == a.len();
                            } else {
                                return count == 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        true
    }
}

fn count_binary_all_branch(binary_expr: &LuaBinaryExpr) -> usize {
    let mut count = 0;
    if let Some(op) = binary_expr.get_op_token() {
        match op.get_op() {
            BinaryOperator::OpAnd => {
                let exprs = binary_expr.get_exprs();
                if let Some(exprs) = exprs {
                    count += count_expr_all_branch(&exprs.0);
                    count += count_expr_all_branch(&exprs.1);
                }

                return count;
            }
            _ => return 1,
        }
    }

    0
}

fn count_expr_all_branch(expr: &LuaExpr) -> usize {
    match expr {
        LuaExpr::BinaryExpr(binary_expr) => count_binary_all_branch(binary_expr),
        LuaExpr::CallExpr(call_expr) => {
            if call_expr.is_error() {
                return 0;
            } else {
                return 1;
            }
        }
        _ => 1,
    }
}
