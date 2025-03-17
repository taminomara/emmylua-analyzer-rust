use crate::{DbIndex, LuaMemberIndexItem, LuaType, TypeOps};

pub fn resolve_member_type(db: &DbIndex, member_item: &LuaMemberIndexItem) -> Option<LuaType> {
    match member_item {
        LuaMemberIndexItem::One(member_id) => {
            let member = db.get_member_index().get_member(&member_id)?;
            let member_type = member.get_decl_type();
            Some(member_type)
        }
        LuaMemberIndexItem::Many(member_ids) => {
            let mut resolve_state = MemberResolveState::All;
            let members = member_ids
                .iter()
                .map(|id| db.get_member_index().get_member(id))
                .collect::<Option<Vec<_>>>()?;
            for member in &members {
                let feature = member.get_feature();
                if feature.is_meta_decl() {
                    resolve_state = MemberResolveState::Meta;
                    break;
                } else if feature.is_file_decl() {
                    resolve_state = MemberResolveState::FileDecl;
                }
            }

            match resolve_state {
                MemberResolveState::All => {
                    let mut typ = LuaType::Unknown;
                    for member in members {
                        typ = TypeOps::Union.apply(&typ, &member.get_decl_type());
                    }
                    Some(typ)
                }
                MemberResolveState::Meta => {
                    let mut typ = LuaType::Unknown;
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_meta_decl() {
                            typ = TypeOps::Union.apply(&typ, &member.get_decl_type());
                        }
                    }
                    Some(typ)
                }
                MemberResolveState::FileDecl => {
                    let mut typ = LuaType::Unknown;
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_file_decl() {
                            typ = TypeOps::Union.apply(&typ, &member.get_decl_type());
                        }
                    }
                    Some(typ)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemberResolveState {
    All,
    Meta,
    FileDecl,
}
