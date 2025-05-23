#[cfg(test)]
mod test {
    use crate::{LuaType, VirtualWorkspace};

    #[test]
    fn test_issue_376() {
        let mut ws = VirtualWorkspace::new();

        ws.def(
            r#"
        ---@return any
        function get() end

        local sub = get()

        if sub and type(sub) == 'table' then
            -- sub is nil - wrong
            a = sub
        end
        "#,
        );

        let a_ty = ws.expr_ty("a");
        let expected = LuaType::Table;
        assert_eq!(a_ty, expected);
    }

    #[test]
    fn test_issue_476() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        assert!(ws.check_code_for(
            crate::DiagnosticCode::ParamTypeNotMatch,
            r#"
        ---Converts hex to char
        ---@param hex string
        ---@return string
        function hex_to_char2(hex)
            return string.char(assert(tonumber(hex, 16)))
        end
        "#,
        ));
    }
}
