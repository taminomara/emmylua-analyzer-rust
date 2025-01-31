use log::info;
use lsp_types::InitializeParams;

pub fn set_ls_locale(params: &InitializeParams) -> Option<()> {
    let mut locale: String = params.locale.clone()?;

    // 如果传递的`locale`包含`-`, 则转换为`_`且后面的字母大写
    if locale.contains("-") {
        let parts = locale.split("-").collect::<Vec<&str>>();
        if parts.len() == 2 {
            locale = format!("{}_{}", parts[0], parts[1].to_uppercase());
        }
    }

    info!("set locale: {}", locale);
    emmylua_parser::set_locale(&locale);
    code_analysis::set_locale(&locale);
    meta_text::set_locale(&locale);
    Some(())
}
