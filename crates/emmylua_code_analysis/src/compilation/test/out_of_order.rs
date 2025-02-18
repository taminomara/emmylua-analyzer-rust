#[cfg(test)]
mod test {
    use crate::VirtualWorkspace;

    #[test]
    fn test_unorder_analysis() {
        let mut ws = VirtualWorkspace::new();

        let files = vec![
            (
                "rx.lua",
                r#"
            local subject = require("subject")

            local rx = {
                subject = subject,
            }

            return rx
            "#,
            ),
            (
                "subject.lua",
                r#"
            ---@class Subject
            local subject = {}

            ---@return Subject
            function subject.new()

            end

            return subject
            "#,
            ),
        ];

        ws.def_files(files);

        let ty = ws.expr_ty("require('rx').subject.new()");
        let expected = ws.ty("subject");
        assert_eq!(ty, expected);
    }
}
