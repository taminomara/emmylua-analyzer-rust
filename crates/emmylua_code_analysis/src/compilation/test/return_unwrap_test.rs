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
}
