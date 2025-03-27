#[cfg(test)]
mod tests {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_issue_231() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();
        assert!(ws.check_code_for(
            DiagnosticCode::AssignTypeMismatch,
            r#"

            --- @type [boolean, string]
            local ret = { coroutine.resume(coroutine.create(function () end), ...) }
            "#
        ));
    }
}
