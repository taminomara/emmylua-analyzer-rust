mod add_completions;
mod completion_builder;
mod data;
mod providers;
mod resolve_completion;

use add_completions::CompletionData;
use completion_builder::CompletionBuilder;
use emmylua_parser::LuaAstNode;
use log::error;
use lsp_types::{
    ClientCapabilities, CompletionItem, CompletionOptions, CompletionOptionsCompletionItem,
    CompletionParams, CompletionResponse, ServerCapabilities,
};
use providers::add_completions;
use resolve_completion::resolve_completion;
use rowan::TokenAtOffset;
use tokio_util::sync::CancellationToken;

use crate::context::ServerContextSnapshot;

pub async fn on_completion_handler(
    context: ServerContextSnapshot,
    params: CompletionParams,
    cancel_token: CancellationToken,
) -> Option<CompletionResponse> {
    let uri = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;
    let analysis = context.analysis.read().await;
    let file_id = analysis.get_file_id(&uri)?;
    let semantic_model = analysis.compilation.get_semantic_model(file_id)?;
    let root = semantic_model.get_root();
    let position_offset = {
        let document = semantic_model.get_document();
        document.get_offset(position.line as usize, position.character as usize)?
    };

    let token = match root.syntax().token_at_offset(position_offset) {
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(left, _) => left,
        TokenAtOffset::None => {
            return None;
        }
    };

    let mut builder = CompletionBuilder::new(token, semantic_model, cancel_token);
    add_completions(&mut builder);
    Some(CompletionResponse::Array(builder.get_completion_items()))
}

#[allow(unused_variables)]
pub async fn on_completion_resolve_handler(
    context: ServerContextSnapshot,
    params: CompletionItem,
    cancel_token: CancellationToken,
) -> CompletionItem {
    let analysis = context.analysis.read().await;
    let db = analysis.compilation.get_db();
    let mut completion_item = params;
    if let Some(data) = completion_item.data.clone() {
        let completion_data = match serde_json::from_value::<CompletionData>(data.clone()) {
            Ok(data) => data,
            Err(err) => {
                error!("Failed to deserialize completion data: {:?}", err);
                return completion_item;
            }
        };

        resolve_completion(db, &mut completion_item, completion_data);
    }

    completion_item
}

pub fn register_capabilities(
    server_capabilities: &mut ServerCapabilities,
    _: &ClientCapabilities,
) -> Option<()> {
    server_capabilities.completion_provider = Some(CompletionOptions {
        resolve_provider: Some(true),
        trigger_characters: Some(
            vec![".", ":", "(", "[", "\"", "\'", ",", "@", "\\", "/"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        ),
        work_done_progress_options: Default::default(),
        completion_item: Some(CompletionOptionsCompletionItem {
            label_details_support: Some(true),
        }),
        all_commit_characters: Default::default(),
    });

    Some(())
}
