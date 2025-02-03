use rowan::{TextRange, TextSize};

use crate::db_index::{LuaDeclId, TypeAssertion};

#[derive(Debug)]
pub struct LuaFlowChain {
    decl_id: LuaDeclId,
    type_asserts: Vec<(TypeAssertion, TextRange)>,
}

impl LuaFlowChain {
    pub fn new(decl_id: LuaDeclId) -> Self {
        Self {
            decl_id,
            type_asserts: Vec::new(),
        }
    }

    pub fn get_decl_id(&self) -> LuaDeclId {
        self.decl_id
    }

    pub fn add_type_assert(&mut self, type_assert: TypeAssertion, range: TextRange) {
        self.type_asserts.push((type_assert, range));
    }

    pub fn get_type_asserts(&self, position: TextSize) -> impl Iterator<Item = &TypeAssertion> {
        self.type_asserts
            .iter()
            .filter_map(move |(type_assert, range)| {
                if range.contains_inclusive(position) {
                    Some(type_assert)
                } else {
                    None
                }
            })
    }
}
