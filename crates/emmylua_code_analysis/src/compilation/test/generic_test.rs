#[cfg(test)]
mod test {
    use crate::VirtualWorkspace;

    #[test]
    fn test_issue_586() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();
        ws.def(
            r#"
            --- @generic T
            --- @param cb fun(...: T...)
            --- @param ... T...
            function invoke1(cb, ...)
                cb(...)
            end

            invoke1(
                function(a, b, c)
                    _a = a 
                    _b = b 
                    _c = c 
                end,
                1, "2", "3"
            )
            "#,
        );

        let a_ty = ws.expr_ty("_a");
        let b_ty = ws.expr_ty("_b");
        let c_ty = ws.expr_ty("_c");

        assert_eq!(a_ty, ws.ty("integer"));
        assert_eq!(b_ty, ws.ty("string"));
        assert_eq!(c_ty, ws.ty("string"));
    }

    #[test]
    fn test_issue_658() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();
        ws.def(
            r#"
            --- @generic T1, T2, R
            --- @param fn fun(_:T1..., _:T2...): R...
            --- @param ... T1...
            --- @return fun(_:T2...): R...
            local function curry(fn, ...)
            local nargs, args = select('#', ...), { ... }
            return function(...)
                local nargs2 = select('#', ...)
                for i = 1, nargs2 do
                args[nargs + i] = select(i, ...)
                end
                return fn(unpack(args, 1, nargs + nargs2))
            end
            end

            --- @param a string
            --- @param b string
            --- @param c table
            local function foo(a, b, c) end

            bar = curry(foo, 'a')
            "#,
        );

        let bar_ty = ws.expr_ty("bar");
        let expected = ws.ty("fun(b:string, c:table)");
        assert_eq!(bar_ty, expected);
    }
}
