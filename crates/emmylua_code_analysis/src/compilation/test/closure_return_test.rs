#[cfg(test)]
mod test {
    use crate::VirtualWorkspace;

    #[test]
    fn test_flow() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        ws.def(
            r#"

        "#,
        );

        ws.def(
            r#"
        --- @return string[] stdout
        --- @return string? stderr
        local function foo() end

        --- @param _a string[]
        local function bar(_a) end

        local a = {}

        a = foo()

        b = a
        "#,
        );
        let ty = ws.expr_ty("b");
        let expected = ws.ty("string[]");
        assert_eq!(ty, expected);
    }
}
