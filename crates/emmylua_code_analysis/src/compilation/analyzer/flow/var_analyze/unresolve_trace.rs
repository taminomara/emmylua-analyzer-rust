use std::sync::Arc;

use emmylua_parser::{LuaExpr, LuaIfStat};

use super::var_trace_info::VarTraceInfo;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum UnResolveTraceId {
    Expr(LuaExpr),
    If(LuaIfStat),
}

#[derive(Debug, Clone)]
pub enum UnResolveTraceInfo {
    Trace(Arc<VarTraceInfo>),
    MultipleTraces(Vec<Arc<VarTraceInfo>>),
}

#[allow(unused)]
impl UnResolveTraceInfo {
    pub fn get_trace_info(&self) -> Option<Arc<VarTraceInfo>> {
        match self {
            UnResolveTraceInfo::Trace(assertion) => Some(assertion.clone()),
            UnResolveTraceInfo::MultipleTraces(assertions) => assertions.get(0).cloned(),
        }
    }

    pub fn get_trace_infos(&self) -> Option<Vec<Arc<VarTraceInfo>>> {
        match self {
            UnResolveTraceInfo::Trace(assertion) => Some(vec![assertion.clone()]),
            UnResolveTraceInfo::MultipleTraces(assertions) => Some(assertions.clone()),
        }
    }

    pub fn add_trace_info(&mut self, trace_info: Arc<VarTraceInfo>) {
        match self {
            UnResolveTraceInfo::Trace(existing_assertion) => {
                *self = UnResolveTraceInfo::MultipleTraces(vec![
                    existing_assertion.clone(),
                    trace_info,
                ]);
            }
            UnResolveTraceInfo::MultipleTraces(assertions) => {
                assertions.push(trace_info);
            }
        }
    }
}
