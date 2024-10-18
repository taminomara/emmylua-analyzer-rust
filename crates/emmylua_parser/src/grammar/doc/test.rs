#[cfg(test)]
mod tests {
    use crate::{parser::ParserConfig, LuaParser};

    #[test]
    fn test_normal_doc() {
        let lua_code = r#"
        -- comment
        "#;

        let tree = LuaParser::parse(lua_code, ParserConfig::default());
        println!("{:#?}", tree.get_red_root());
    }

    #[test]
    fn test_class_doc() {
        let lua_code = r#"
        ---@class A
        "#;

        let tree = LuaParser::parse(lua_code, ParserConfig::default());
        println!("{:#?}", tree.get_red_root());
    }

    #[test]
    fn test_basic_doc() {
        let lua_code = r#"
        ---@class A
        ---@field a number
        local a = {}

        ---@type A
        local b = {}

        ---@alias A B

        ---@param a number
        ---@param b string
        local function f(a, b)
        end

        ---@return number
        local function f()
        end

        ---@overload fun(a: number, b: string): number
        ---@overload fun(a: string, b: number): string
        local function f(a, b)
        end

        ---@generic T
        ---@param a T
        ---@return T
        local function f(a)
        end
        "#;

        let tree = LuaParser::parse(lua_code, ParserConfig::default());
        println!("{:#?}", tree.get_red_root());
    }
}