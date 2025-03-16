use crate::{LuaMemberInfo, LuaPropertyOwnerId};

// need support multi
pub fn resolve_member_property(member_infos: &Vec<LuaMemberInfo>) -> Option<LuaPropertyOwnerId> {
    if member_infos.len() == 1 {
        return member_infos[0].property_owner_id.clone();
    }

    let mut resolve_state = MemberResolveState::MetaOrNone;
    for member_info in member_infos {
        match member_info.feature {
            Some(feature) => {
                if feature.is_first_define() {
                    resolve_state = MemberResolveState::FirstDefine;
                } else if feature.is_file_decl() {
                    resolve_state = MemberResolveState::FileDecl;
                    break;
                }
            }
            None => {}
        }
    }

    match resolve_state {
        MemberResolveState::MetaOrNone => {
            for member_info in member_infos {
                if member_info.property_owner_id.is_some() {
                    return member_info.property_owner_id.clone();
                }
            }

            None
        }
        MemberResolveState::FirstDefine => {
            for member_info in member_infos {
                if let Some(feature) = member_info.feature {
                    if feature.is_first_define() {
                        return member_info.property_owner_id.clone();
                    }
                }
            }

            None
        }
        MemberResolveState::FileDecl => {
            for member_info in member_infos {
                if let Some(feature) = member_info.feature {
                    if feature.is_file_decl() {
                        return member_info.property_owner_id.clone();
                    }
                }
            }

            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemberResolveState {
    MetaOrNone,
    FirstDefine,
    FileDecl,
}
