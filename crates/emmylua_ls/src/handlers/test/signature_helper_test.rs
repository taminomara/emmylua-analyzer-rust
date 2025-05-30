#[cfg(test)]
mod tests {

    use crate::handlers::test_lib::{ProviderVirtualWorkspace, VirtualSignatureHelp};
    #[test]
    fn test_1() {
        let mut ws = ProviderVirtualWorkspace::new();
        assert!(ws.check_signature_helper(
            r#"
                ---@class Action
                ---@field id fun(self:Action, itemId:integer, ...:integer?):boolean
                ---@overload fun():Action
                Action = {}

                Action:id(1, <??>)
            "#,
            VirtualSignatureHelp {
                target_label: "Action:id(itemId: integer, ...: integer?): boolean".to_string(),
                active_signature: 0,
                active_parameter: 1,
            },
        ));
    }

    #[test]
    fn test_2() {
        let mut ws = ProviderVirtualWorkspace::new_with_init_std_lib();
        assert!(ws.check_signature_helper(
            r#"
                ---@param path string
                local function readFile(path)
                end

                pcall(readFile, <??>)
            "#,
            VirtualSignatureHelp {
                target_label: "pcall(f: fun(path: string), path: string): boolean".to_string(),
                active_signature: 0,
                active_parameter: 1,
            },
        ));
    }
}
