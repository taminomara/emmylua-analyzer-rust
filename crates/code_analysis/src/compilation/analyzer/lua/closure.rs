use emmylua_parser::LuaClosureExpr;

use crate::{compilation::analyzer::unresolve::UnResolveReturn, db_index::{LuaDocReturnInfo, LuaSignatureId}};

use super::{func_body::analyze_func_body_returns, LuaAnalyzer, LuaReturnPoint};

pub fn analyze_closure(analyzer: &mut LuaAnalyzer, closure: LuaClosureExpr) -> Option<()> {
    let signature_id = LuaSignatureId::new(analyzer.file_id, &closure);

    analyze_params(analyzer, &signature_id, &closure);
    analyze_return(analyzer, &signature_id, &closure);
    Some(())
}

fn analyze_params(
    analyzer: &mut LuaAnalyzer,
    signature_id: &LuaSignatureId,
    closure: &LuaClosureExpr,
) -> Option<()> {
    let signature = analyzer
        .db
        .get_signature_index_mut()
        .get_or_create(signature_id.clone());
    let params = closure.get_params_list()?.get_params();
    for param in params {
        let name = if let Some(name_token) = param.get_name_token() {
            name_token.get_name_text().to_string()
        } else if param.is_dots() {
            "...".to_string()
        } else {
            return None;
        };

        signature.params.push(name);
    }

    Some(())
}

fn analyze_return(
    analyzer: &mut LuaAnalyzer,
    signature_id: &LuaSignatureId,
    closure: &LuaClosureExpr,
) -> Option<()> {
    let signature = analyzer.db.get_signature_index().get(&signature_id)?;
    let ret = &signature.return_docs;
    if !ret.is_empty() {
        return None;
    }

    let block = closure.get_block()?;
    let return_points = analyze_func_body_returns(block);
    let returns = match analyze_return_point(analyzer, &return_points) {
        Some(returns) => returns,
        None => {
            let unresolve = UnResolveReturn {
                signature_id: signature_id.clone(),
                return_points
            };

            analyzer.add_unresolved(unresolve.into());
            return None
        },
    };
    let signature = analyzer
        .db
        .get_signature_index_mut()
        .get_or_create(signature_id.clone());
    signature.return_docs = returns;
    signature.resolve_return = true;
    Some(())
}

fn analyze_return_point(
    analyzer: &mut LuaAnalyzer,
    return_points: &Vec<LuaReturnPoint>,
) -> Option<Vec<LuaDocReturnInfo>> {
    let mut return_infos = Vec::new();
    for point in return_points {
        match point {
            LuaReturnPoint::Expr(expr) => {
                let expr_type = analyzer.infer_expr(&expr)?;
                if return_infos.is_empty() {
                    return_infos.push(LuaDocReturnInfo {
                        name: None,
                        type_ref: expr_type,
                        description: None,
                    });
                } else {
                    // todo merge two type
                    // let has_return = return_infos[0].type_ref.clone();
                    return_infos[0].type_ref = expr_type;
                }
            }
            LuaReturnPoint::MuliExpr(exprs) => {
                // todo merge type
                if return_infos.is_empty() {
                    for expr in exprs {
                        let expr_type = analyzer.infer_expr(&expr)?;
                        return_infos.push(LuaDocReturnInfo {
                            name: None,
                            type_ref: expr_type,
                            description: None,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    Some(return_infos)
}
