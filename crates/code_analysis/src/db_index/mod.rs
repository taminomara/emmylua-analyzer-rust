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


use crate::FileId;
pub use declaration::*;
pub use description::{LuaDescriptionIndex, LuaDescription, LuaDescriptionId, LuaDescriptionOwnerId};
pub use member::{LuaMemberIndex, LuaMember, LuaMemberOwner, LuaMemberId};
use meta::MetaFile;
use module::LuaModuleIndex;
use reference::LuaReferenceIndex;
pub use r#type::*;
use traits::LuaIndex;

#[derive(Debug)]
pub struct DbIndex {
    decl_index: LuaDeclIndex,
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
            decl_index: LuaDeclIndex::new(),
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
            self.remove(file_id);
        }
    }

    pub fn get_decl_index(&mut self) -> &mut LuaDeclIndex {
        &mut self.decl_index
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
        self.decl_index.remove(file_id);
        self.references_index.remove(file_id);
        self.types_index.remove(file_id);
        self.modules_index.remove(file_id);
        self.meta_files_index.remove(file_id);
        self.members_index.remove(file_id);
        self.descriptions_index.remove(file_id);
    }
}
