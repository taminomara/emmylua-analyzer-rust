#[cfg(test)]
mod test {

    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_issue_421() {
        let mut ws = VirtualWorkspace::new();

        assert!(ws.check_code_for(
            DiagnosticCode::AssignTypeMismatch,
            r#"
        local a         --- @type string?
        local b = { a } --- @type string[] error

        b[2] = nil
        "#,
        ));
    }
}
