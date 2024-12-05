mod text_document_handler;
mod watched_file_handler;

pub use text_document_handler::{
    on_did_change_text_document, on_did_open_text_document, on_did_save_text_document,
};
pub use watched_file_handler::on_did_change_watched_files;