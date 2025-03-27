#[cfg(test)]
mod test {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_1() {
        let mut ws = VirtualWorkspace::new();
        assert!(ws.check_code_for(
            DiagnosticCode::UndefinedField,
            r#"
                ---@alias std.NotNull<T> T - ?

                ---@generic V
                ---@param t {[any]: V}
                ---@return fun(tbl: any):int, std.NotNull<V>
                function ipairs(t) end

                ---@type {[integer]: string|table}
                local a = {}

                for i, extendsName in ipairs(a) do
                    print(extendsName.a)
                end 
            "#
        ));
    }

    #[test]
    fn test() {
        let mut ws = VirtualWorkspace::new();
        assert!(!ws.check_code_for(
            DiagnosticCode::UndefinedField,
            r#"
                ---@class diagnostic.test3
                ---@field private a number

                ---@type diagnostic.test3
                local test = {}

                local b = test.b
            "#
        ));

        assert!(!ws.check_code_for(
            DiagnosticCode::UndefinedField,
            r#"
                ---@class diagnostic.test3
                ---@field private a number
                local Test3 = {}

                local b = Test3.b
            "#
        ));
    }

    #[test]
    fn test_enum() {
        let mut ws = VirtualWorkspace::new();
        assert!(!ws.check_code_for(
            DiagnosticCode::UndefinedField,
            r#"
                ---@enum diagnostic.enum
                local Enum = {
                    A = 1,
                }

                local enum_b = Enum["B"]
            "#
        ));
    }
    #[test]
    fn test_issue_194() {
        let mut ws = VirtualWorkspace::new_with_init_std_lib();
        assert!(ws.check_code_for(
            DiagnosticCode::UndefinedField,
            r#"
            local a ---@type 'A'
            local _ = a:lower()
            "#
        ));
    }
}
