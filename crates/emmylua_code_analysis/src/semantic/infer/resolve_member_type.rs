use crate::{LuaMemberInfo, LuaType, TypeOps};

pub fn resolve_member_type(member_infos: &Vec<LuaMemberInfo>) -> Option<LuaType> {
    if member_infos.len() == 1 {
        return Some(member_infos[0].typ.clone());
    }

    let mut resolve_state = MemberResolveState::All;
    for member_info in member_infos {
        match member_info.feature {
            Some(feature) => {
                if feature.is_meta_decl() {
                    resolve_state = MemberResolveState::Meta;
                    break;
                } else if feature.is_file_decl() {
                    resolve_state = MemberResolveState::FileDecl;
                }
            }
            None => {}
        }
    }

    match resolve_state {
        MemberResolveState::All => {
            let mut typ = LuaType::Unknown;
            for member_info in member_infos {
                typ = TypeOps::Union.apply(&typ, &member_info.typ);
            }
            Some(typ)
        }
        MemberResolveState::Meta => {
            let mut typ = LuaType::Unknown;
            for member_info in member_infos {
                if let Some(feature) = member_info.feature {
                    if feature.is_meta_decl() {
                        typ = TypeOps::Union.apply(&typ, &member_info.typ);
                    }
                }
            }
            Some(typ)
        }
        MemberResolveState::FileDecl => {
            let mut typ = LuaType::Unknown;
            for member_info in member_infos {
                if let Some(feature) = member_info.feature {
                    if feature.is_file_decl() {
                        typ = TypeOps::Union.apply(&typ, &member_info.typ);
                    }
                }
            }
            Some(typ)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemberResolveState {
    All,
    Meta,
    FileDecl,
}
