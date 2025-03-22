use crate::LuaType;

use super::TypeOps;

pub fn narrow_false_or_nil(t: LuaType) -> LuaType {
    if t.is_boolean() {
        return LuaType::BooleanConst(false);
    }

    return TypeOps::Narrow.apply(&t, &LuaType::Nil);
}
