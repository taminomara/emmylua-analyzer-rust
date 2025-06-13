#[cfg(test)]
mod tests {

    use crate::handlers::test_lib::{ProviderVirtualWorkspace, VirtualHoverResult};
    #[test]
    fn test_1() {
        let mut ws = ProviderVirtualWorkspace::new();
        assert!(ws.check_hover(
            r#"
                ---@class <??>A
                ---@field a number
                ---@field b string
                ---@field c boolean
            "#,
            VirtualHoverResult {
                value: "\n```lua\n(class) A {\n    a: number,\n    b: string,\n    c: boolean,\n}\n```\n".to_string(),
            },
        ));
    }

    #[test]
    fn test_right_to_left() {
        let mut ws = ProviderVirtualWorkspace::new();
        assert!(ws.check_hover(
            r#"
                ---@class H4
                local m = {
                    x = 1
                }

                ---@type H4
                local m1

                m1.x = {}
                m1.<??>x = {}
            "#,
            VirtualHoverResult {
                value: "\n```lua\n(field) x: integer = 1\n```\n".to_string(),
            },
        ));

        assert!(ws.check_hover(
            r#"
                ---@class Node
                ---@field x number
                ---@field right Node?

                ---@return Node
                local function createRBNode()
                end

                ---@type Node
                local node

                if node.right then
                else
                    node.<??>right = createRBNode()
                end
            "#,
            VirtualHoverResult {
                value: "\n```lua\n(field) right: Node\n```\n".to_string(),
            },
        ));

        assert!(ws.check_hover(
            r#"
                 ---@class Node1
                ---@field x number

                ---@return Node1
                local function createRBNode()
                end

                ---@type Node1?
                local node

                if node then
                else
                    <??>node = createRBNode()
                end
            "#,
            VirtualHoverResult {
                value: "\n```lua\nlocal node: Node1 {\n    x: number,\n}\n```\n".to_string(),
            },
        ));
    }

    #[test]
    fn test_hover_nil() {
        let mut ws = ProviderVirtualWorkspace::new();
        assert!(ws.check_hover(
            r#"
                ---@class A
                ---@field a? number

                ---@type A
                local a

                local d = a.<??>a
            "#,
            VirtualHoverResult {
                value: "\n```lua\n(field) a: number?\n```\n".to_string(),
            },
        ));
    }

    #[test]
    fn test_function_infer_return_val() {
        let mut ws = ProviderVirtualWorkspace::new();
        assert!(ws.check_hover(
            r#"
                local function <??>f(a, b)
                    a = 1
                end
            "#,
            VirtualHoverResult {
                value: "\n```lua\nlocal function f(a, b)\n```\n".to_string(),
            },
        ));
    }

    #[test]
    fn test_decl_desc() {
        let mut ws = ProviderVirtualWorkspace::new();
        assert!(ws.check_hover(
            r#"
                ---@class Buff.AddData
                ---@field pulse? number 心跳周期

                ---@type Buff.AddData
                local data

                data.pu<??>lse
            "#,
            VirtualHoverResult {
                value: "\n```lua\n(field) pulse: number?\n```\n\n&nbsp;&nbsp;in class `Buff.AddData`\n\n---\n\n心跳周期\n".to_string(),
            },
        ));
    }

    #[test]
    fn test_issue_535() {
        let mut ws = ProviderVirtualWorkspace::new();
        assert!(ws.check_hover(
            r#"
                ---@type table<string, number>
                local t

                ---@class T1
                local a

                function a:init(p)
                    self._c<??>fg = t[p]
                end
            "#,
            VirtualHoverResult {
                value: "\n```lua\n(field) _cfg: number\n```\n".to_string(),
            },
        ));

        assert!(ws.check_hover(
            r#"
                ---@type table<string, number>
                local t = {
                }
                ---@class T2
                local a = {}

                function a:init(p)
                    self._cfg = t[p]
                end

                ---@param p T2
                function fun(p)
                    local x = p._c<??>fg
                end
            "#,
            VirtualHoverResult {
                value: "\n```lua\n(field) _cfg: number\n```\n".to_string(),
            },
        ));
    }
}
