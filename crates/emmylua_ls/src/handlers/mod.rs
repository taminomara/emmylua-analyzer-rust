mod document_symbol;
mod fold_range;
mod hover;
mod initialized;
mod notification_handler;
mod request_handler;
mod response_handler;
mod text_document;

pub use initialized::initialized_handler;
pub use initialized::{init_analysis, ClientConfig};
use lsp_server::Message;
use lsp_types::{
    DocumentSymbolOptions, FoldingRangeProviderCapability, HoverProviderCapability, OneOf,
    SaveOptions, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncSaveOptions,
};
pub use notification_handler::on_notification_handler;
pub use request_handler::on_req_handler;
pub use response_handler::on_response_handler;

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            lsp_types::TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                will_save: None,
                will_save_wait_until: None,
                save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                    include_text: Some(false),
                })),
            },
        )),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        document_symbol_provider: Some(OneOf::Right(DocumentSymbolOptions {
            label: Some("EmmyLua".into()),
            work_done_progress_options: Default::default(),
        })),
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        ..Default::default()
    }
}

