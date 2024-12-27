use lsp_types::{
    ClientCapabilities, ExecuteCommandOptions, ExecuteCommandParams, ServerCapabilities,
};
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

pub const COMMANDS: &[&str] = &["emmy.auto.require", "emmy.fix.format"];

pub async fn on_execute_command_handler(
    context: ServerContextSnapshot,
    params: ExecuteCommandParams,
    _: CancellationToken,
) -> Option<Value> {
    None
}

pub fn register_capabilities(
    server_capabilities: &mut ServerCapabilities,
    _: &ClientCapabilities,
) -> Option<()> {
    server_capabilities.execute_command_provider = Some(ExecuteCommandOptions {
        commands: COMMANDS.iter().map(|s| s.to_string()).collect(),
        ..Default::default()
    });
    Some(())
}
