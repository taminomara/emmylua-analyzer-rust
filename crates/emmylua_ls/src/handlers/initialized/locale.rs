use log::info;
use lsp_types::InitializeParams;

pub fn set_ls_locale(params: &InitializeParams) -> Option<()> {
    let locale: String = params.locale.clone()?;

    info!("set locale: {}", locale);
    emmylua_parser::set_locale(&locale);
    code_analysis::set_locale(&locale);
    meta_text::set_locale(&locale);
    Some(())
}
