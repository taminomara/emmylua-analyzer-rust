#[cfg(test)]
mod test {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_closure_return() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        ws.def(
            r#"
        --- @generic T, U
        --- @param arr T[]
        --- @param op fun(item: T, index: integer): U
        --- @return U[]
        function map(arr, op)
        end
        "#,
        );

        let ty = ws.expr_ty(
            r#"
        map({ 1, 2, 3 }, function(item, i)
            return tostring(item)
        end)
        "#,
        );
        let expected = ws.ty("string[]");
        assert_eq!(ty, expected);
    }

    #[test]
    fn test_issue_140_1() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        ws.def(
            r#"
        ---@class Object
        
        ---@class T
        local inject2class ---@type (Object| T)?
        if jsonClass then
            if inject2class then
                A = inject2class
            end
        end
        "#,
        );

        let ty = ws.expr_ty("A");
        let expected = ws.ty("Object | T");
        assert_eq!(ty, expected);
    }

    #[test]
    fn test_issue_140_2() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        assert!(ws.check_code_for(
            DiagnosticCode::NeedCheckNil,
            r#"
        local msgBody ---@type { _hgQuiteMsg : 1 }?
        if not msgBody or not msgBody._hgQuiteMsg then
        end
        "#
        ));
    }
}
