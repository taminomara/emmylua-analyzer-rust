use super::PriorityTable;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LuaTypeUnaryOperator {
    None,
    Keyof,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LuaTypeBinaryOperator {
    None,
    Union,
    Intersection,
    In,
    Extends,
    // TODO: Add more binary operators
    // As
}

pub const PRIORITY: &[PriorityTable] = &[
    PriorityTable { left: 0, right: 0 }, // None
    PriorityTable { left: 1, right: 1 }, // Union
    PriorityTable { left: 2, right: 2 }, // Intersection
    PriorityTable { left: 0, right: 0 }, // In
    PriorityTable { left: 4, right: 4 }, // Extends
];

impl LuaTypeBinaryOperator {
    pub fn get_priority(&self) -> &PriorityTable {
        &PRIORITY[*self as usize]
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LuaTypeTernaryOperator {
    None,
    Conditional,
}
