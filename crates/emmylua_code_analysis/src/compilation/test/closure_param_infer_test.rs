#[cfg(test)]
mod test {
    use crate::VirtualWorkspace;

    #[test]
    fn test_closure_param_infer() {
        let mut ws = VirtualWorkspace::new();

        ws.def(
            r#"

        ---@class EventData
        ---@field name string

        ---@class EventDispatcher
        ---@field pre fun(self:EventDispatcher,callback:fun(context:EventData))
        local EventDispatcher = {}

        EventDispatcher:pre(function(context)
            b = context
        end)
        "#,
        );

        let ty = ws.expr_ty("b");
        let expected = ws.ty("EventData");
        assert_eq!(ty, expected);
    }
}
