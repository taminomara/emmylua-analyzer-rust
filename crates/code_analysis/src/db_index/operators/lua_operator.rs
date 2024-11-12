use rowan::{TextRange, TextSize};

use crate::{db_index::{LuaType, LuaTypeDeclId}, FileId};

use super::lua_operator_meta_method::LuaOperatorMetaMethod;


#[derive(Debug)]
pub struct LuaOperator {
    owner: LuaTypeDeclId,
    op: LuaOperatorMetaMethod,
    operands: Vec<LuaType>,
    result: LuaType,
    file_id: FileId,
    range: TextRange,
}

impl LuaOperator {
    pub fn new(
        owner: LuaTypeDeclId,
        op: LuaOperatorMetaMethod,
        operands: Vec<LuaType>,
        result: LuaType,
        file_id: FileId,
        range: TextRange,
    ) -> Self {
        Self {
            owner,
            op,
            operands,
            result,
            file_id,
            range
        }
    }

    pub fn get_owner(&self) -> &LuaTypeDeclId {
        &self.owner
    }

    pub fn get_op(&self) -> LuaOperatorMetaMethod {
        self.op
    }

    pub fn get_operands(&self) -> &[LuaType] {
        &self.operands
    }

    pub fn get_result(&self) -> &LuaType {
        &self.result
    }

    pub fn get_file_id(&self) -> FileId {
        self.file_id
    }

    pub fn get_id(&self) -> LuaOperatorId {
        LuaOperatorId {
            file_id: self.file_id,
            position: self.range.start(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LuaOperatorId {
    pub file_id: FileId,
    pub position: TextSize,
}

impl LuaOperatorId {
    pub fn new(position: TextSize, file_id: FileId) -> Self {
        Self { position, file_id }
    }
}