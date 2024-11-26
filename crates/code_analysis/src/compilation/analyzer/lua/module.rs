use emmylua_parser::LuaChunk;

use crate::compilation::analyzer::unresolve::UnResolveModule;

use super::{func_body::analyze_func_body_returns, LuaAnalyzer, LuaReturnPoint};

pub fn analyze_chunk_return(analyzer: &mut LuaAnalyzer, chunk: LuaChunk) -> Option<()> {
    let block = chunk.get_block()?;
    let return_exprs = analyze_func_body_returns(analyzer, block);
    for point in return_exprs {
        match point {
            LuaReturnPoint::Expr(expr) => {
                let expr_type = analyzer.infer_expr(&expr);
                let expr_type = match expr_type {
                    Some(expr_type) => expr_type,
                    None => {
                        let unresolve = UnResolveModule {
                            file_id: analyzer.file_id,
                            expr,
                        };
                        analyzer.add_unresolved(unresolve.into());
                        return None;
                    }
                };

                let module_info = analyzer.db.get_module_index_mut().get_module_mut(analyzer.file_id)?;
                module_info.export_type = Some(expr_type);
                break;
            }
            // Other cases are stupid code
            _ => {}
        }
    }

    Some(())
}
