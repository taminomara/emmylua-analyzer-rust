#[cfg(test)]
mod test {
    use crate::{DiagnosticCode, VirtualWorkspace};

    #[test]
    fn test_issue_245() {
        let mut ws = VirtualWorkspace::new();

        assert!(ws.check_code_for(
            DiagnosticCode::NeedCheckNil,
            r#"
        local a --- @type table?
        local _ = (a and a.type == 'change') and a.field
        "#
        ));
    }

    #[test]
    fn test_issue_405() {
        let mut ws = VirtualWorkspace::new();
        let mut emmyrc = ws.analysis.emmyrc.as_ref().clone();
        emmyrc.strict.array_index = false;
        ws.analysis.update_config(emmyrc.into());
        assert!(ws.check_code_for(
            DiagnosticCode::NeedCheckNil,
            r#"
                ---@type false|fun(...)[]?
                local calls

                for i = 1, #calls do
                    calls[i](...)
                end
        "#
        ));
    }

    #[test]
    fn test_issue_402() {
        let mut ws = VirtualWorkspace::new();
        assert!(ws.check_code_for(
            DiagnosticCode::NeedCheckNil,
            r#"
            ---@class A
            local a = {}

            ---@param self table?
            function a.new(self)
                if self then
                    self.a = 1
                end
            end
        "#
        ));
    }
}
