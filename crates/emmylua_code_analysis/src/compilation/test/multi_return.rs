#[cfg(test)]
mod test {
    use crate::VirtualWorkspace;

    #[test]
    fn test_pcall_return() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();

        ws.def(r#"
        a, b = pcall(string.rep, "a", 1000000000)
        "#);

        let ty = ws.expr_ty("b");
        let expected = ws.ty("string");
        assert_eq!(ty, expected);
    }
}
