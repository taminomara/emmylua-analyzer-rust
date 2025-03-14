use std::{collections::HashMap, sync::Arc};

use crate::{CacheEntry, CacheKey, DbIndex, LuaInferCache, LuaMemberKey, LuaType};

use super::{
    infer_members::{self},
    LuaMemberInfo,
};

pub fn infer_member_map(
    db: &DbIndex,
    cache: &mut LuaInferCache,
    prefix_type: &LuaType,
) -> Option<Arc<HashMap<LuaMemberKey, Vec<LuaMemberInfo>>>> {
    let key = CacheKey::TypeMemberOwner(prefix_type.clone());
    if let Some(cache) = cache.get(&key) {
        match cache {
            CacheEntry::TypeMemberOwnerCache(result) => return Some(result.clone()),
            _ => return None,
        }
    };

    let members = infer_members::infer_members(db, prefix_type)?;

    let mut member_map = HashMap::new();
    for member in members {
        let key = member.key.clone();
        member_map.entry(key).or_insert_with(Vec::new).push(member);
    }

    let arc_member_map = Arc::new(member_map);

    let entry = CacheEntry::TypeMemberOwnerCache(arc_member_map.clone());
    cache.add_cache(&key, entry);

    Some(arc_member_map)
}
