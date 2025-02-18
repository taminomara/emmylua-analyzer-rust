#[cfg(test)]
mod test {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_issue_82() {
        let mut ws = VirtualWorkspace::new();

        assert!(ws.check_file_for(
            DiagnosticCode::ParamTypeNotMatch,
            r#"
            ---@generic F: function
            ---@param _a F|integer
            ---@param _b? F
            ---@return F
            function foo(_a, _b)
                return _a
            end
            foo(function() end)
        "#
        ));
    }
}
