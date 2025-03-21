use emmylua_code_analysis::{EmmyLuaAnalysis, FileId, VirtualUrlGenerator};
use emmylua_parser::LuaAstNode;
use lsp_types::{CompletionItemKind, CompletionResponse, CompletionTriggerKind, Position};
use tokio_util::sync::CancellationToken;

mod completion_test;
use super::completion;

/// A virtual workspace for testing.
#[allow(unused)]
#[derive(Debug)]
struct CompletionVirtualWorkspace {
    pub virtual_url_generator: VirtualUrlGenerator,
    pub analysis: EmmyLuaAnalysis,
    id_counter: u32,
}

#[derive(Debug)]
struct VirtualCompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
}

#[allow(unused)]
impl CompletionVirtualWorkspace {
    pub fn new() -> Self {
        let gen = VirtualUrlGenerator::new();
        let mut analysis = EmmyLuaAnalysis::new();
        let base = &gen.base;
        analysis.add_main_workspace(base.clone());
        CompletionVirtualWorkspace {
            virtual_url_generator: gen,
            analysis,
            id_counter: 0,
        }
    }

    pub fn new_with_init_std_lib() -> Self {
        let gen = VirtualUrlGenerator::new();
        let mut analysis = EmmyLuaAnalysis::new();
        analysis.init_std_lib(false);
        let base = &gen.base;
        analysis.add_main_workspace(base.clone());
        CompletionVirtualWorkspace {
            virtual_url_generator: gen,
            analysis,
            id_counter: 0,
        }
    }

    pub fn def(&mut self, content: &str) -> FileId {
        let id = self.id_counter;
        self.id_counter += 1;
        self.def_file(&format!("virtual_{}.lua", id), content)
    }

    pub fn def_file(&mut self, file_name: &str, content: &str) -> FileId {
        let uri = self.virtual_url_generator.new_uri(file_name);
        let file_id = self
            .analysis
            .update_file_by_uri(&uri, Some(content.to_string()))
            .unwrap();
        file_id
    }

    pub fn get_node<Ast: LuaAstNode>(&self, file_id: FileId) -> Ast {
        let tree = self
            .analysis
            .compilation
            .get_db()
            .get_vfs()
            .get_syntax_tree(&file_id)
            .unwrap();
        tree.get_chunk_node().descendants::<Ast>().next().unwrap()
    }

    /// 处理文件内容
    fn handle_file_content(content: &str) -> Option<(String, Position)> {
        let mut content = content.to_string();
        let cursor_pos = content.find("<??>");
        if let Some(cursor_pos) = cursor_pos {
            // 确保只有一个 <??> 标记
            if content.matches("<??>").count() > 1 {
                return None;
            }

            let mut line = 0;
            let mut column = 0;
            for (i, c) in content.chars().take(cursor_pos).enumerate() {
                if c == '\n' {
                    line += 1;
                    column = 0;
                } else {
                    column += 1;
                }
            }

            content = content.replace("<??>", "");

            return Some((content, Position::new(line as u32, column as u32)));
        }
        None
    }

    pub fn check_completion(
        &mut self,
        block_str: &str,
        expect: Vec<VirtualCompletionItem>,
    ) -> bool {
        self.check_completion_with_kind(block_str, expect, CompletionTriggerKind::INVOKED)
    }

    pub fn check_completion_with_kind(
        &mut self,
        block_str: &str,
        expect: Vec<VirtualCompletionItem>,
        trigger_kind: CompletionTriggerKind,
    ) -> bool {
        let content = Self::handle_file_content(block_str);
        let Some((content, position)) = content else {
            return false;
        };
        let file_id = self.def(&content);
        let result = completion(
            &self.analysis,
            file_id,
            position,
            trigger_kind,
            CancellationToken::new(),
        );
        let Some(result) = result else {
            return false;
        };
        // 对比
        let items = match result {
            CompletionResponse::Array(items) => items,
            CompletionResponse::List(list) => list.items,
        };
        if items.len() != expect.len() {
            return false;
        }
        // 需要顺序一致
        for (item, expect) in items.iter().zip(expect.iter()) {
            if item.label != expect.label || item.kind != Some(expect.kind) {
                return false;
            }
        }
        true
    }
}
