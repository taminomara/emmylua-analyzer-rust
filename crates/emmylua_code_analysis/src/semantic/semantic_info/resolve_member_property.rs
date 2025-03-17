use crate::{DbIndex, LuaMemberIndexItem, LuaPropertyOwnerId};

// need support multi
pub fn resolve_member_property(
    db: &DbIndex,
    member_item: &LuaMemberIndexItem,
) -> Option<LuaPropertyOwnerId> {
    match member_item {
        LuaMemberIndexItem::One(member_id) => Some(LuaPropertyOwnerId::Member(*member_id)),
        LuaMemberIndexItem::Many(member_ids) => {
            let mut resolve_state = MemberResolveState::MetaOrNone;
            let members = member_ids
                .iter()
                .map(|id| db.get_member_index().get_member(id))
                .collect::<Option<Vec<_>>>()?;
            for member in &members {
                let feature = member.get_feature();
                if feature.is_first_define() {
                    resolve_state = MemberResolveState::FirstDefine;
                } else if feature.is_file_decl() {
                    resolve_state = MemberResolveState::FileDecl;
                    break;
                }
            }

            match resolve_state {
                MemberResolveState::MetaOrNone => {
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_meta_decl() {
                            return Some(LuaPropertyOwnerId::Member(member.get_id()));
                        }
                    }

                    Some(LuaPropertyOwnerId::Member(members.first()?.get_id()))
                }
                MemberResolveState::FirstDefine => {
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_first_define() {
                            return Some(LuaPropertyOwnerId::Member(member.get_id()));
                        }
                    }

                    None
                }
                MemberResolveState::FileDecl => {
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_file_decl() {
                            return Some(LuaPropertyOwnerId::Member(member.get_id()));
                        }
                    }

                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemberResolveState {
    MetaOrNone,
    FirstDefine,
    FileDecl,
}
