mod non_literal_expressions_in_assert;

use crate::SemanticModel;

use super::DiagnosticContext;

pub fn check_file_code_style(
    context: &mut DiagnosticContext,
    semantic_model: &SemanticModel,
) -> Option<()> {
    macro_rules! check_code_style {
        ($module:ident) => {
            if $module::CODES
                .iter()
                .any(|code| context.is_checker_enable_by_code(code))
            {
                $module::check(context, semantic_model);
            }
        };
    }

    check_code_style!(non_literal_expressions_in_assert);

    Some(())
}
