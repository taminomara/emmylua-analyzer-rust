use emmylua_parser::{LuaExpr, LuaIfStat};

use crate::TypeAssertion;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum UnResolveTraceId {
    Expr(LuaExpr),
    If(LuaIfStat),
}

#[derive(Debug, Clone)]
pub enum UnResolveTraceInfo {
    Assertion(TypeAssertion),
    MultipleAssertion(Vec<TypeAssertion>),
}

#[allow(unused)]
impl UnResolveTraceInfo {
    pub fn get_assertion(&self) -> Option<TypeAssertion> {
        match self {
            UnResolveTraceInfo::Assertion(assertion) => Some(assertion.clone()),
            UnResolveTraceInfo::MultipleAssertion(assertions) => assertions.get(0).cloned(),
        }
    }

    pub fn get_assertions(&self) -> Option<Vec<TypeAssertion>> {
        match self {
            UnResolveTraceInfo::Assertion(assertion) => Some(vec![assertion.clone()]),
            UnResolveTraceInfo::MultipleAssertion(assertions) => Some(assertions.clone()),
        }
    }

    pub fn add_assertion(&mut self, assertion: TypeAssertion) {
        match self {
            UnResolveTraceInfo::Assertion(existing_assertion) => {
                *self = UnResolveTraceInfo::MultipleAssertion(vec![
                    existing_assertion.clone(),
                    assertion,
                ]);
            }
            UnResolveTraceInfo::MultipleAssertion(assertions) => {
                assertions.push(assertion);
            }
        }
    }
}
