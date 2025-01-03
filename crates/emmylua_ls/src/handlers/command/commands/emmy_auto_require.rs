use std::{collections::HashMap, time::Duration};

use code_analysis::FileId;
use emmylua_parser::{LuaAstNode, LuaExpr, LuaStat};
use lsp_types::{ApplyWorkspaceEditParams, Command, Position, TextEdit, WorkspaceEdit};
use serde_json::Value;

use crate::{
    context::ServerContextSnapshot,
    util::{module_name_convert, time_cancel_token},
};

pub const COMMAND: &str = "emmy.auto.require";

pub async fn handle(context: ServerContextSnapshot, args: Vec<Value>) -> Option<()> {
    let add_to: FileId = serde_json::from_value(args.get(0)?.clone()).ok()?;
    let need_require_file_id: FileId = serde_json::from_value(args.get(1)?.clone()).ok()?;
    let position: Position = serde_json::from_value(args.get(2)?.clone()).ok()?;

    let analysis = context.analysis.read().await;
    let semantic_model = analysis.compilation.get_semantic_model(add_to)?;
    let module_info = semantic_model
        .get_db()
        .get_module_index()
        .get_module(need_require_file_id)?;
    let emmyrc = semantic_model.get_emmyrc();
    let require_like_func = &emmyrc.runtime.require_like_function;
    let auto_require_func = emmyrc.completion.auto_require_function.clone();
    let file_convension = emmyrc.completion.auto_require_naming_convention;
    let local_name = module_name_convert(&module_info.name, file_convension);
    let require_str = format!(
        "local {} = {}(\"{}\")",
        local_name, auto_require_func, module_info.full_module_name
    );
    let document = semantic_model.get_document();
    let offset = document.get_offset(position.line as usize, position.character as usize)?;
    let root_block = semantic_model.get_root().get_block()?;
    let mut last_require_stat: Option<LuaStat> = None;
    for stat in root_block.get_stats() {
        if stat.get_position() > offset {
            break;
        }

        if is_require_stat(stat.clone(), &require_like_func).unwrap_or(false) {
            last_require_stat = Some(stat);
        }
    }

    let line = if let Some(last_require_stat) = last_require_stat {
        let last_require_stat_end = last_require_stat.get_range().end();
        document.get_line(last_require_stat_end)? + 1
    } else {
        0
    };

    let text_edit = TextEdit {
        range: lsp_types::Range {
            start: Position {
                line: line as u32,
                character: 0,
            },
            end: Position {
                line: line as u32,
                character: 0,
            },
        },
        new_text: format!("{}\n", require_str),
    };

    let uri = document.get_uri();
    let mut changes = HashMap::new();
    changes.insert(uri.clone(), vec![text_edit.clone()]);

    let client = context.client;
    let cancel_token = time_cancel_token(Duration::from_secs(5));
    let apply_edit_params = ApplyWorkspaceEditParams {
        label: None,
        edit: WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        },
    };

    tokio::spawn(async move {
        let res = client.apply_edit(apply_edit_params, cancel_token).await;
        if let Some(res) = res {
            if !res.applied {
                log::error!("Failed to apply edit: {:?}", res.failure_reason);
            }
        }
    });

    Some(())
}

fn is_require_stat(stat: LuaStat, require_like_func: &Vec<String>) -> Option<bool> {
    match stat {
        LuaStat::LocalStat(local_stat) => {
            let exprs = local_stat.get_value_exprs();
            for expr in exprs {
                if is_require_expr(expr, require_like_func).unwrap_or(false) {
                    return Some(true);
                }
            }
        }
        LuaStat::AssignStat(assign_stat) => {
            let (_, exprs) = assign_stat.get_var_and_expr_list();
            for expr in exprs {
                if is_require_expr(expr, require_like_func).unwrap_or(false) {
                    return Some(true);
                }
            }
        }
        LuaStat::CallExprStat(call_expr_stat) => {
            let expr = call_expr_stat.get_call_expr()?;
            if is_require_expr(expr.into(), require_like_func).unwrap_or(false) {
                return Some(true);
            }
        }
        _ => {}
    }

    Some(false)
}

fn is_require_expr(expr: LuaExpr, require_like_func: &Vec<String>) -> Option<bool> {
    if let LuaExpr::CallExpr(call_expr) = expr {
        let name = call_expr.get_prefix_expr()?;
        if let LuaExpr::NameExpr(name_expr) = name {
            let name = name_expr.get_name_text()?;
            if require_like_func.contains(&name.to_string()) || name == "require" {
                return Some(true);
            }
        }
    }

    Some(false)
}

pub fn make_auto_require(
    title: &str,
    add_to: FileId,
    need_require_file_id: FileId,
    position: Position,
) -> Command {
    let args = vec![
        serde_json::to_value(add_to).unwrap(),
        serde_json::to_value(need_require_file_id).unwrap(),
        serde_json::to_value(position).unwrap(),
    ];

    Command {
        title: title.to_string(),
        command: COMMAND.to_string(),
        arguments: Some(args),
    }
}
