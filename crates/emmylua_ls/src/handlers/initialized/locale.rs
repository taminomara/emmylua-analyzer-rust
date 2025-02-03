use log::info;
use lsp_types::InitializeParams;

pub fn set_ls_locale(params: &InitializeParams) -> Option<()> {
    let mut locale: String = params.locale.clone()?;

    // If the passed `locale` contains '-', convert '-' to '_' and convert the following letters to uppercase
    if locale.contains("-") {
        let parts = locale.split("-").collect::<Vec<&str>>();
        if parts.len() == 2 {
            locale = format!("{}_{}", parts[0], parts[1].to_uppercase());
        }
    }

    info!("set locale: {}", locale);
    emmylua_parser::set_locale(&locale);
    emmylua_code_analysis::set_locale(&locale);
    rust_i18n::set_locale(&locale);
    Some(())
}
