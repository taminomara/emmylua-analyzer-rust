#[cfg(test)]
mod test {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_generic() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        ws.check_code_for(
            DiagnosticCode::TypeNotFound,
            r#"
        --- @generic T
        --- @param ... T
        --- @return T
        return function (...) end
        "#,
        );
    }
}
