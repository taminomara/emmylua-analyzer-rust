mod initialized;
mod notification_handler;
mod request_handler;
mod response_handler;
mod text_document;
mod hover;

pub use initialized::initialized_handler;
use lsp_types::{
    HoverProviderCapability, SaveOptions, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextDocumentSyncSaveOptions,
};
pub use notification_handler::on_notification_handler;
pub use request_handler::on_req_handler;
pub use response_handler::on_response_handler;
pub use initialized::{ClientConfig, init_analysis};

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
        ..Default::default()
    }
}
