mod declaration;
mod module;
mod symbol;
mod traits;
mod r#type;
mod reference;
mod meta;
mod member;
mod operators;
mod description;

use std::collections::HashMap;

use crate::FileId;
pub use declaration::*;
pub use description::{LuaDescriptionIndex, LuaDescription, LuaDescriptionId, LuaDescriptionOwnerId};
pub use member::{LuaMemberIndex, LuaMember, LuaMemberOwner, LuaMemberId};
use meta::MetaFile;
use module::LuaModuleIndex;
use reference::LuaReferenceIndex;
use traits::LuaIndex;
pub use r#type::*;

#[derive(Debug)]
pub struct DbIndex {
    decl_trees: HashMap<FileId, LuaDeclarationTree>,
    references_index: LuaReferenceIndex,
    types_index: LuaTypeIndex,
    modules_index: LuaModuleIndex,
    meta_files_index: MetaFile,
    members_index: LuaMemberIndex,
    descriptions_index: LuaDescriptionIndex,
}

impl DbIndex {
    pub fn new() -> Self {
        Self {
            decl_trees: HashMap::new(),
            references_index: LuaReferenceIndex::new(),
            types_index: LuaTypeIndex::new(),
            modules_index: LuaModuleIndex::new(),
            meta_files_index: MetaFile::new(),
            members_index: LuaMemberIndex::new(),
            descriptions_index: LuaDescriptionIndex::new(),
        }
    }

    pub fn remove_index(&mut self, file_ids: Vec<FileId>) {
        for file_id in file_ids {
            self.decl_trees.remove(&file_id);
        }
    }

    pub fn add_decl_tree(&mut self, tree: LuaDeclarationTree) {
        self.decl_trees.insert(tree.file_id(), tree);
    }

    #[allow(unused)]
    pub fn get_decl_tree(&self, file_id: &FileId) -> Option<&LuaDeclarationTree> {
        self.decl_trees.get(file_id)
    }

    pub fn get_reference_index(&mut self) -> &mut LuaReferenceIndex {
        &mut self.references_index
    }

    pub fn get_type_index(&mut self) -> &mut LuaTypeIndex {
        &mut self.types_index
    }

    pub fn get_module_index(&mut self) -> &mut LuaModuleIndex {
        &mut self.modules_index
    }

    pub fn get_meta_file(&mut self) -> &mut MetaFile {
        &mut self.meta_files_index
    }

    pub fn get_member_index(&mut self) -> &mut LuaMemberIndex {
        &mut self.members_index
    }

    pub fn get_description_index(&mut self) -> &mut LuaDescriptionIndex {
        &mut self.descriptions_index
    }
}

impl LuaIndex for DbIndex {
    fn remove(&mut self, file_id: FileId) {
        self.decl_trees.remove(&file_id);
        self.references_index.remove(file_id);
        self.types_index.remove(file_id);
        self.modules_index.remove(file_id);
        self.meta_files_index.remove(file_id);
        self.members_index.remove(file_id);
        self.descriptions_index.remove(file_id);
    }
}
