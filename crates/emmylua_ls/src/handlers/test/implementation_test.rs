#[cfg(test)]
mod tests {

    use crate::handlers::test_lib::ProviderVirtualWorkspace;

    #[test]
    fn test_1() {
        let mut ws = ProviderVirtualWorkspace::new();
        ws.def_file(
            "2.lua",
            r#"
               delete = require("virtual_0").delete
               delete()
            "#,
        );
        ws.def_file(
            "3.lua",
            r#"
               delete = require("virtual_0").delete
               delete()
            "#,
        );
        assert!(ws.check_implementation(
            r#"
                local M = {}
                function M.de<??>lete(a)
                end
                return M
            "#,
            1,
        ));
    }

    #[test]
    fn test_2() {
        let mut ws = ProviderVirtualWorkspace::new();
        ws.def_file(
            "1.lua",
            r#"
                ---@class (partial) Test
                test = {}

                test.a = 1
            "#,
        );
        ws.def_file(
            "2.lua",
            r#"
                ---@class (partial) Test
                test = {}
                test.a = 1
            "#,
        );
        ws.def_file(
            "3.lua",
            r#"
                local a = test.a
            "#,
        );
        assert!(ws.check_implementation(
            r#"
                t<??>est
            "#,
            3,
        ));
    }

    #[test]
    fn test_3() {
        let mut ws = ProviderVirtualWorkspace::new();
        ws.def_file(
            "1.lua",
            r#"
                ---@class YYY
                ---@field a number
                yyy = {}

                if false then
                    yyy.a = 1
                    if yyy.a then
                    end
                end

            "#,
        );
        assert!(ws.check_implementation(
            r#"
                yyy.<??>a = 2
            "#,
            3,
        ));
    }

    #[test]
    fn test_table_field_definition_1() {
        let mut ws = ProviderVirtualWorkspace::new();
        assert!(ws.check_implementation(
            r#"
                ---@class T
                ---@field func fun(self: T) 注释注释

                ---@type T
                local t = {
                    func = function(self)
                    end,
                }

                t:fun<??>c()
            "#,
            2,
        ));
    }

    #[test]
    fn test_table_field_definition_2() {
        let mut ws = ProviderVirtualWorkspace::new();
        assert!(ws.check_implementation(
            r#"
                ---@class T
                ---@field func fun(self: T) 注释注释

                ---@type T
                local t = {
                    f<??>unc = function(self)
                    end,
                }
            "#,
            2,
        ));
    }
}
