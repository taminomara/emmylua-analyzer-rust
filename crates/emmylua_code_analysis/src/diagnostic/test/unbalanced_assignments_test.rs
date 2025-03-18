#[cfg(test)]
mod tests {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test() {
        let mut ws = VirtualWorkspace::new();

        assert!(!ws.check_code_for(
            DiagnosticCode::UnbalancedAssignments,
            r#"
            local x, y, z = print()
        "#
        ));

        assert!(!ws.check_code_for(
            DiagnosticCode::UnbalancedAssignments,
            r#"
            local x, y, z
            x, y, z = 1
        "#
        ));

        assert!(!ws.check_code_for(
            DiagnosticCode::UnbalancedAssignments,
            r#"
            local x, y, z = 1
        "#
        ));

        assert!(ws.check_code_for(
            DiagnosticCode::UnbalancedAssignments,
            r#"
            local x, y, z
        "#
        ));

        assert!(!ws.check_code_for(
            DiagnosticCode::UnbalancedAssignments,
            r#"
                local x, y, z
                x, y, z = 1
        "#
        ));

        assert!(!ws.check_code_for(
            DiagnosticCode::UnbalancedAssignments,
            r#"
                X, Y, Z = 1
        "#
        ));

        assert!(!ws.check_code_for(
            DiagnosticCode::UnbalancedAssignments,
            r#"
            T = {}
            T.x, T.y, T.z = 1
        "#
        ));

        assert!(!ws.check_code_for(
            DiagnosticCode::UnbalancedAssignments,
            r#"
            T = {}
            T['x'], T['y'], T['z'] = 1
        "#
        ));
    }
}
