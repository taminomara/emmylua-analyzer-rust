use std::collections::HashMap;

use crate::{DbIndex, LuaMemberKey, LuaType};

use super::{
    infer_members::{self},
    LuaMemberInfo,
};

pub fn infer_member_map(
    db: &DbIndex,
    prefix_type: &LuaType,
) -> Option<HashMap<LuaMemberKey, Vec<LuaMemberInfo>>> {
    let members = infer_members::infer_members(db, prefix_type)?;

    let mut member_map = HashMap::new();
    for member in members {
        let key = member.key.clone();
        member_map.entry(key).or_insert_with(Vec::new).push(member);
    }

    Some(member_map)
}
