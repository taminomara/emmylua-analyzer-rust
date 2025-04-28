use std::sync::Arc;

use rowan::{TextRange, TextSize};

use crate::{
    db_index::{LuaType, LuaTypeDeclId},
    DbIndex, FileId, InFiled, InferFailReason, LuaFunctionType, LuaSignatureId,
    SignatureReturnStatus,
};

use super::lua_operator_meta_method::LuaOperatorMetaMethod;

#[derive(Debug)]
pub struct LuaOperator {
    owner: LuaOperatorOwner,
    op: LuaOperatorMetaMethod,
    file_id: FileId,
    range: TextRange,
    func: OperatorFunction,
}

#[derive(Debug, Clone)]
pub enum OperatorFunction {
    Func(Arc<LuaFunctionType>),
    Signature(LuaSignatureId),
}

impl LuaOperator {
    pub fn new(
        owner: LuaOperatorOwner,
        op: LuaOperatorMetaMethod,
        file_id: FileId,
        range: TextRange,
        func: OperatorFunction,
    ) -> Self {
        Self {
            owner,
            op,
            file_id,
            range,
            func,
        }
    }

    pub fn get_owner(&self) -> &LuaOperatorOwner {
        &self.owner
    }

    pub fn get_op(&self) -> LuaOperatorMetaMethod {
        self.op
    }

    pub fn get_operand(&self, db: &DbIndex) -> LuaType {
        match &self.func {
            OperatorFunction::Func(func) => {
                let params = func.get_params();
                if params.len() >= 2 {
                    return params[1].1.clone().unwrap_or(LuaType::Any);
                }

                LuaType::Any
            }
            OperatorFunction::Signature(signature) => {
                let signature = db.get_signature_index().get(signature);
                if let Some(signature) = signature {
                    let param = signature.get_param_info_by_id(1);
                    if let Some(param) = param {
                        return param.type_ref.clone();
                    }
                }

                LuaType::Any
            }
        }
    }

    pub fn get_result(&self, db: &DbIndex) -> Result<LuaType, InferFailReason> {
        match &self.func {
            OperatorFunction::Func(func) => Ok(func.get_ret().clone()),
            OperatorFunction::Signature(signature_id) => {
                let signature = db.get_signature_index().get(signature_id);
                if let Some(signature) = signature {
                    if signature.resolve_return == SignatureReturnStatus::UnResolve {
                        return Err(InferFailReason::UnResolveSignatureReturn(
                            signature_id.clone(),
                        ));
                    }

                    let return_type = signature.return_docs.get(0);
                    if let Some(return_type) = return_type {
                        return Ok(return_type.type_ref.clone());
                    }
                }

                Ok(LuaType::Any)
            }
        }
    }

    pub fn get_operator_func(&self) -> LuaType {
        match &self.func {
            OperatorFunction::Func(func) => LuaType::DocFunction(func.clone()),
            OperatorFunction::Signature(signature) => LuaType::Signature(*signature),
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaOperatorOwner {
    Table(InFiled<TextRange>),
    Type(LuaTypeDeclId),
}

impl From<LuaTypeDeclId> for LuaOperatorOwner {
    fn from(id: LuaTypeDeclId) -> Self {
        LuaOperatorOwner::Type(id)
    }
}

impl From<InFiled<TextRange>> for LuaOperatorOwner {
    fn from(id: InFiled<TextRange>) -> Self {
        LuaOperatorOwner::Table(id)
    }
}
