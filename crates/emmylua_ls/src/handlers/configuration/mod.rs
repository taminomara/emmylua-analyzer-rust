use lsp_types::{
    ClientCapabilities, DidChangeConfigurationParams, ServerCapabilities
};

use crate::context::ServerContextSnapshot;

pub async fn on_did_change_configuration(
    context: ServerContextSnapshot,
    params: DidChangeConfigurationParams,
) -> Option<()> {
    let pretty_json = serde_json::to_string_pretty(&params).ok()?;
    log::info!("on_did_change_configuration: {}", pretty_json);

    let config = context.config_manager.read().await;
    if config.client_config.client_id.is_vscode() {
        return Some(());
    }

    // I don't know what to do here
    Some(())
}

pub fn register_capabilities(_: &mut ServerCapabilities, _: &ClientCapabilities) -> Option<()> {
    Some(())
}
