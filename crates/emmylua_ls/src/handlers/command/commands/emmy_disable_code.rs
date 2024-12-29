use code_analysis::{DiagnosticCode, FileId};
use lsp_types::{Command, Range};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::ServerContextSnapshot;

pub const COMMAND: &str = "emmy.disable.code";
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DisableAction {
    DisableLine,
    DisableFile,
    DisableProject,
}

pub async fn handle(context: ServerContextSnapshot, args: Vec<Value>) -> Option<()> {
    let action: DisableAction = serde_json::from_value(args.get(0)?.clone()).ok()?;
    let file_id: FileId = serde_json::from_value(args.get(1)?.clone()).ok()?;
    let range: Range = serde_json::from_value(args.get(2)?.clone()).ok()?;
    let code: String = serde_json::from_value(args.get(3)?.clone()).ok()?;

    Some(())
}

pub fn make_disable_code_command(
    title: &str,
    action: DisableAction,
    code: DiagnosticCode,
    file_id: FileId,
    range: Range,
) -> Command {
    let args = vec![
        serde_json::to_value(action).unwrap(),
        serde_json::to_value(file_id).unwrap(),
        serde_json::to_value(range).unwrap(),
        serde_json::to_value(code.get_name()).unwrap(),
    ];

    Command {
        title: title.to_string(),
        command: COMMAND.to_string(),
        arguments: Some(args),
    }
}
