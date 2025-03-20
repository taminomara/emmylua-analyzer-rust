use crate::{DbIndex, LuaPropertyOwnerId, LuaType, TypeOps};

use super::LuaMemberId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaMemberIndexItem {
    One(LuaMemberId),
    Many(Vec<LuaMemberId>),
}

impl LuaMemberIndexItem {
    pub fn resolve_type(&self, db: &DbIndex) -> Option<LuaType> {
        resolve_member_type(db, &self)
    }

    pub fn resolve_property_owner(&self, db: &DbIndex) -> Option<LuaPropertyOwnerId> {
        resolve_member_property(db, &self)
    }
}

fn resolve_member_type(db: &DbIndex, member_item: &LuaMemberIndexItem) -> Option<LuaType> {
    match member_item {
        LuaMemberIndexItem::One(member_id) => {
            let member = db.get_member_index().get_member(&member_id)?;
            let member_type = member.get_decl_type();
            Some(member_type)
        }
        LuaMemberIndexItem::Many(member_ids) => {
            let mut resolve_state = MemberTypeResolveState::All;
            let members = member_ids
                .iter()
                .map(|id| db.get_member_index().get_member(id))
                .collect::<Option<Vec<_>>>()?;
            for member in &members {
                let feature = member.get_feature();
                if feature.is_meta_decl() {
                    resolve_state = MemberTypeResolveState::Meta;
                    break;
                } else if feature.is_file_decl() {
                    resolve_state = MemberTypeResolveState::FileDecl;
                }
            }

            match resolve_state {
                MemberTypeResolveState::All => {
                    let mut typ = LuaType::Unknown;
                    for member in members {
                        typ = TypeOps::Union.apply(&typ, &member.get_decl_type());
                    }
                    Some(typ)
                }
                MemberTypeResolveState::Meta => {
                    let mut typ = LuaType::Unknown;
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_meta_decl() {
                            typ = TypeOps::Union.apply(&typ, &member.get_decl_type());
                        }
                    }
                    Some(typ)
                }
                MemberTypeResolveState::FileDecl => {
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
enum MemberTypeResolveState {
    All,
    Meta,
    FileDecl,
}

fn resolve_member_property(
    db: &DbIndex,
    member_item: &LuaMemberIndexItem,
) -> Option<LuaPropertyOwnerId> {
    match member_item {
        LuaMemberIndexItem::One(member_id) => Some(LuaPropertyOwnerId::Member(*member_id)),
        LuaMemberIndexItem::Many(member_ids) => {
            let mut resolve_state = MemberPropertyResolveState::MetaOrNone;
            let members = member_ids
                .iter()
                .map(|id| db.get_member_index().get_member(id))
                .collect::<Option<Vec<_>>>()?;
            for member in &members {
                let feature = member.get_feature();
                if feature.is_first_define() {
                    resolve_state = MemberPropertyResolveState::FirstDefine;
                } else if feature.is_file_decl() {
                    resolve_state = MemberPropertyResolveState::FileDecl;
                    break;
                }
            }

            match resolve_state {
                MemberPropertyResolveState::MetaOrNone => {
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_meta_decl() {
                            return Some(LuaPropertyOwnerId::Member(member.get_id()));
                        }
                    }

                    Some(LuaPropertyOwnerId::Member(members.first()?.get_id()))
                }
                MemberPropertyResolveState::FirstDefine => {
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_first_define() {
                            return Some(LuaPropertyOwnerId::Member(member.get_id()));
                        }
                    }

                    None
                }
                MemberPropertyResolveState::FileDecl => {
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
enum MemberPropertyResolveState {
    MetaOrNone,
    FirstDefine,
    FileDecl,
}
