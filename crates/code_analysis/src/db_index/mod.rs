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
mod diagnostic;

use crate::FileId;
pub use declaration::*;
#[allow(unused_imports)]
pub use diagnostic::{DiagnosticIndex, AnalyzeError, DiagnosticAction};
#[allow(unused_imports)]
pub use member::{LuaMember, LuaMemberId, LuaMemberIndex, LuaMemberOwner, LuaMemberKey};
use meta::MetaFile;
use module::LuaModuleIndex;
#[allow(unused_imports)]
pub use property::{
    LuaPropertyId, LuaPropertyIndex, LuaPropertyOwnerId, LuaVersionCond, LuaVersionCondOp,
};
#[allow(unused_imports)]
pub use operators::{LuaOperatorIndex, LuaOperatorMetaMethod, LuaOperatorId, LuaOperator};
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
    diagnostic_index: DiagnosticIndex,
    operator_index: LuaOperatorIndex,
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
            diagnostic_index: DiagnosticIndex::new(),
            operator_index: LuaOperatorIndex::new(),
        }
    }

    pub fn remove_index(&mut self, file_ids: Vec<FileId>) {
        for file_id in file_ids {
            self.remove(file_id);
        }
    }

    pub fn get_decl_index_mut(&mut self) -> &mut LuaDeclIndex {
        &mut self.decl_index
    }

    pub fn get_reference_index_mut(&mut self) -> &mut LuaReferenceIndex {
        &mut self.references_index
    }

    pub fn get_type_index_mut(&mut self) -> &mut LuaTypeIndex {
        &mut self.types_index
    }

    pub fn get_module_index_mut(&mut self) -> &mut LuaModuleIndex {
        &mut self.modules_index
    }

    pub fn get_meta_file_mut(&mut self) -> &mut MetaFile {
        &mut self.meta_files_index
    }

    pub fn get_member_index_mut(&mut self) -> &mut LuaMemberIndex {
        &mut self.members_index
    }

    pub fn get_property_index_mut(&mut self) -> &mut LuaPropertyIndex {
        &mut self.property_index
    }

    pub fn get_signature_index_mut(&mut self) -> &mut LuaSignatureIndex {
        &mut self.signature_index
    }

    pub fn get_diagnostic_index_mut(&mut self) -> &mut DiagnosticIndex {
        &mut self.diagnostic_index
    }

    pub fn get_operator_index_mut(&mut self) -> &mut LuaOperatorIndex {
        &mut self.operator_index
    }

    pub fn get_decl_index(&self) -> &LuaDeclIndex {
        &self.decl_index
    }

    pub fn get_reference_index(&self) -> &LuaReferenceIndex {
        &self.references_index
    }

    pub fn get_type_index(&self) -> &LuaTypeIndex {
        &self.types_index
    }

    pub fn get_module_index(&self) -> &LuaModuleIndex {
        &self.modules_index
    }

    pub fn get_meta_file(&self) -> &MetaFile {
        &self.meta_files_index
    }

    pub fn get_member_index(&self) -> &LuaMemberIndex {
        &self.members_index
    }

    pub fn get_property_index(&self) -> &LuaPropertyIndex {
        &self.property_index
    }

    pub fn get_signature_index(&self) -> &LuaSignatureIndex {
        &self.signature_index
    }

    pub fn get_diagnostic_index(&self) -> &DiagnosticIndex {
        &self.diagnostic_index
    }

    pub fn get_operator_index(&self) -> &LuaOperatorIndex {
        &self.operator_index
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
        self.diagnostic_index.remove(file_id);
        self.operator_index.remove(file_id);
    }
}
