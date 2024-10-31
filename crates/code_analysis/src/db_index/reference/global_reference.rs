use std::collections::HashMap;

use rowan::TextRange;

use crate::{
    db_index::{traits::LuaIndex, LuaDeclId},
    FileId, InFiled,
};

#[derive(Debug)]
pub struct GlobalReference {
    global_decl: HashMap<String, Vec<LuaDeclId>>,
    global_reference: HashMap<String, Vec<InFiled<TextRange>>>,
}

impl GlobalReference {
    pub fn new() -> Self {
        Self {
            global_decl: HashMap::new(),
            global_reference: HashMap::new(),
        }
    }

    pub fn add_global_decl(&mut self, name: String, decl_id: LuaDeclId) {
        self.global_decl
            .entry(name)
            .or_insert_with(Vec::new)
            .push(decl_id);
    }

    pub fn add_global_reference(&mut self, name: String, range: TextRange, file_id: FileId) {
        self.global_reference
            .entry(name)
            .or_insert_with(Vec::new)
            .push(InFiled::new(file_id, range));
    }
}

impl LuaIndex for GlobalReference {
    fn remove(&mut self, file_id: FileId) {
        self.global_decl.retain(|_, decls| {
            decls.retain(|decl| decl.file_id != file_id);
            !decls.is_empty()
        });

        self.global_reference.retain(|_, refs| {
            refs.retain(|ref_| ref_.file_id != file_id);
            !refs.is_empty()
        });
    }
}
