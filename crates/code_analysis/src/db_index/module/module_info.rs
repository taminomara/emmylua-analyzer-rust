use crate::{db_index::LuaType, FileId};

use super::module_node::ModuleNodeId;

#[derive(Debug)]
pub struct ModuleInfo {
    pub file_id: FileId,
    pub full_module_name: String,
    pub name: String,
    pub module_id: ModuleNodeId,
    pub visible: bool,
    pub export_type: Option<LuaType>,
}