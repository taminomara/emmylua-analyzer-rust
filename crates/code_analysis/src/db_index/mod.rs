mod declaration;
mod member;
mod meta;
mod module;
mod operators;
mod property;
mod reference;
mod signature;
mod symbol;
mod traits;
mod r#type;

use crate::FileId;
pub use declaration::*;
pub use member::{LuaMember, LuaMemberId, LuaMemberIndex, LuaMemberOwner};
use meta::MetaFile;
use module::LuaModuleIndex;
pub use property::{LuaPropertyId, LuaPropertyIndex, LuaPropertyOwnerId};
pub use r#type::*;
use reference::LuaReferenceIndex;
pub use signature::*;
use traits::LuaIndex;

#[derive(Debug)]
pub struct DbIndex {
    decl_index: LuaDeclIndex,
    references_index: LuaReferenceIndex,
    types_index: LuaTypeIndex,
    modules_index: LuaModuleIndex,
    meta_files_index: MetaFile,
    members_index: LuaMemberIndex,
    property_index: LuaPropertyIndex,
    signature_index: LuaSignatureIndex,
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
            property_index: LuaPropertyIndex::new(),
            signature_index: LuaSignatureIndex::new(),
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

    pub fn get_property_index(&mut self) -> &mut LuaPropertyIndex {
        &mut self.property_index
    }

    pub fn get_signature_index(&mut self) -> &mut LuaSignatureIndex {
        &mut self.signature_index
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
        self.property_index.remove(file_id);
        self.signature_index.remove(file_id);
    }
}
