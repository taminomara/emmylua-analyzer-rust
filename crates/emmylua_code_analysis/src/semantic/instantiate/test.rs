#[cfg(test)]
mod test {
    #[test]
    fn test_variadic_func() {
        let mut ws = crate::VirtualWorkspace::new();
        ws.def(r#"
        ---@generic T, R
        ---@param call async fun(...: T...): R...
        ---@return async fun(...: T...): R...
        function async_create(call)

        end


        ---@param a number
        ---@param b string
        ---@param c boolean
        ---@return number
        function locaf(a, b, c)
            
        end
        "#);

        let ty = ws.expr_ty("async_create(locaf)");
        let expected = ws.ty("async fun(a: number, b: string, c:boolean): number");
        assert_eq!(ty, expected);
    }
}
