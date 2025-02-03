use emmylua_parser::LuaSyntaxNode;

use crate::{DbIndex, LuaMemberId, LuaPropertyOwnerId};

use super::{semantic_info::infer_node_property_owner, LuaInferConfig};

pub fn is_reference_to(
    db: &DbIndex,
    infer_config: &mut LuaInferConfig,
    node: LuaSyntaxNode,
    property_owner: LuaPropertyOwnerId,
) -> Option<bool> {
    let node_property_owner_id = infer_node_property_owner(db, infer_config, node)?;
    if node_property_owner_id == property_owner {
        return Some(true);
    }

    match (node_property_owner_id, property_owner) {
        (LuaPropertyOwnerId::Member(node_member_id), LuaPropertyOwnerId::Member(member_id)) => {
            is_member_reference_to(db, node_member_id, member_id)
        }
        _ => Some(false),
    }
}

fn is_member_reference_to(
    db: &DbIndex,
    node_member_id: LuaMemberId,
    member_id: LuaMemberId,
) -> Option<bool> {
    let node_owner = db
        .get_member_index()
        .get_member(&node_member_id)?
        .get_owner();
    let owner = db.get_member_index().get_member(&member_id)?.get_owner();

    Some(node_owner == owner)
}
