mod notification_handler;
mod request_handler;
mod response_handler;
mod initialized;
mod text_document;


use lsp_types::{
    HoverProviderCapability, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};
pub use notification_handler::on_notification_handler;
pub use request_handler::on_req_handler;
pub use response_handler::on_response_handler;
pub use initialized::initialized_handler;

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            lsp_types::TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::FULL),
                will_save: None,
                will_save_wait_until: None,
                save: None,
            },
        )),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    }
}
