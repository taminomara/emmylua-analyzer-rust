#[cfg(test)]
mod test {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_issue_223() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        ws.check_code_for(
            DiagnosticCode::ReturnTypeMismatch,
            r#"
        --- @return integer
        function foo()
            local a
            return a --[[@as integer]]
        end
        "#,
        );
    }
}
