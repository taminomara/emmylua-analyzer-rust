use emmylua_code_analysis::{EmmyLuaAnalysis, FileId, VirtualUrlGenerator};
use lsp_types::{Hover, HoverContents, MarkupContent, Position};

mod hover_function_test;
mod hover_test;
use super::hover;

/// A virtual workspace for testing.
#[allow(unused)]
#[derive(Debug)]
struct HoverVirtualWorkspace {
    pub virtual_url_generator: VirtualUrlGenerator,
    pub analysis: EmmyLuaAnalysis,
    id_counter: u32,
}

#[derive(Debug)]
struct VirtualHoverResult {
    pub value: String,
}

#[allow(unused)]
impl HoverVirtualWorkspace {
    pub fn new() -> Self {
        let gen = VirtualUrlGenerator::new();
        let mut analysis = EmmyLuaAnalysis::new();
        let base = &gen.base;
        analysis.add_main_workspace(base.clone());
        HoverVirtualWorkspace {
            virtual_url_generator: gen,
            analysis,
            id_counter: 0,
        }
    }

    pub fn new_with_init_std_lib() -> Self {
        let gen = VirtualUrlGenerator::new();
        let mut analysis = EmmyLuaAnalysis::new();
        analysis.init_std_lib(None);
        let base = &gen.base;
        analysis.add_main_workspace(base.clone());
        HoverVirtualWorkspace {
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

    /// 处理文件内容
    fn handle_file_content(content: &str) -> Option<(String, Position)> {
        let content = content.to_string();
        let cursor_byte_pos = content.find("<??>")?;
        if content.matches("<??>").count() > 1 {
            return None;
        }

        let mut line = 0;
        let mut column = 0;

        for (byte_pos, c) in content.char_indices() {
            if byte_pos >= cursor_byte_pos {
                break;
            }
            if c == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }

        let new_content = content.replace("<??>", "");
        Some((new_content, Position::new(line as u32, column as u32)))
    }

    pub fn check_hover(&mut self, block_str: &str, expect: VirtualHoverResult) -> bool {
        let content = Self::handle_file_content(block_str);
        let Some((content, position)) = content else {
            return false;
        };
        let file_id = self.def(&content);
        let result = hover(&self.analysis, file_id, position);
        let Some(result) = result else {
            return false;
        };
        let Hover { contents, range } = result;
        let HoverContents::Markup(MarkupContent { kind, value }) = contents else {
            return false;
        };
        if value != expect.value {
            return false;
        }

        true
    }
}
