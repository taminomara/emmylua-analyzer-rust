#[cfg(test)]
mod test {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_issue_221() {
        let mut ws = VirtualWorkspace::new();

        assert!(ws.check_code_for(
            DiagnosticCode::NeedCheckNil,
            r#"
        --- @param opts table|nil
        --- @return any
        function foo(opts)
        opts = opts or {}
        return opts.field
        end

        --- @param opts table?
        --- @return any
        function bar(opts)
        opts = opts or {}
        return opts.field
        end
            "#,
        ));
    }

    #[test]
    fn test_issue_222() {
        let mut ws = VirtualWorkspace::new();

        assert!(ws.check_code_for(
            DiagnosticCode::AssignTypeMismatch,
            r#"
            local a --- @type boolean
            local b --- @type true?
            a = b or false
            "#,
        ));
    }

    #[test]
    fn test_issue_230() {
        let mut ws = VirtualWorkspace::new();

        ws.def(
            r#"
            local b = true ---@type boolean
            a = b and 2 or nil
            "#,
        );

        let a_ty = ws.expr_ty("a");
        assert_eq!(
            format!("{:?}", a_ty).to_string(),
            "Union(LuaUnionType { types: [Nil, IntegerConst(2)] })"
        );
    }
}
