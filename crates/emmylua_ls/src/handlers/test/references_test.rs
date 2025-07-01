#[cfg(test)]
mod tests {

    use crate::handlers::test_lib::ProviderVirtualWorkspace;

    #[test]
    fn test_function_references() {
        let mut ws = ProviderVirtualWorkspace::new();
        ws.def_file(
            "1.lua",
            r#"
                local flush = require("virtual_0").flush
                flush()
            "#,
        );
        let result = ws.check_references(
            r#"
                local export = {}
                local function fl<??>ush()
                end
                export.flush = flush
                return export
            "#,
        );
        assert!(result.is_some());
        let locations = result.unwrap();
        assert!(locations.len() >= 4);
    }
}
