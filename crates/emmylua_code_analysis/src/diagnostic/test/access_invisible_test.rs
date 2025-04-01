#[cfg(test)]
mod tests {
    use crate::{DiagnosticCode, Emmyrc, EmmyrcLuaVersion, VirtualWorkspace};

    #[test]
    fn test_issue_289() {
        let mut ws = VirtualWorkspace::new();
        let mut config = Emmyrc::default();
        config.runtime.version = EmmyrcLuaVersion::LuaJIT;
        ws.analysis.update_config(config.into());
        assert!(ws.check_code_for_namespace(
            DiagnosticCode::AccessInvisible,
            r#"
            local file = io.open("test.txt", "r")
            if file then
                file:close()
            end
            "#
        ));
    }
}
