#[cfg(test)]
mod tests {

    use lsp_types::CompletionItemKind;

    use crate::handlers::completion::test::{CompletionVirtualWorkspace, VirtualCompletionItem};

    #[test]
    fn test_basic() {
        let mut ws = CompletionVirtualWorkspace::new();

        assert!(ws.check_completion(
            r#"
            local zabcde
            za<??>
        "#,
            vec![VirtualCompletionItem {
                label: "zabcde".to_string(),
                kind: CompletionItemKind::VARIABLE,
            }],
        ));
    }
}
