use crate::{DbIndex, LuaSemanticDeclId, LuaType, TypeOps};

use super::{LuaMemberId, LuaMemberIndex};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaMemberIndexItem {
    One(LuaMemberId),
    Many(Vec<LuaMemberId>),
}

impl LuaMemberIndexItem {
    pub fn resolve_type(&self, db: &DbIndex) -> Option<LuaType> {
        resolve_member_type(db, &self)
    }

    pub fn resolve_semantic_decl(&self, db: &DbIndex) -> Option<LuaSemanticDeclId> {
        resolve_member_property(db, &self)
    }

    pub(super) fn resolve_member_id(&self, member_index: &LuaMemberIndex) -> Option<LuaMemberId> {
        resolve_member_id(member_index, &self)
    }

    pub fn get_member_ids(&self) -> Vec<LuaMemberId> {
        match self {
            LuaMemberIndexItem::One(member_id) => vec![*member_id],
            LuaMemberIndexItem::Many(member_ids) => member_ids.clone(),
        }
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

fn resolve_member_id(
    member_index: &LuaMemberIndex,
    member_item: &LuaMemberIndexItem,
) -> Option<LuaMemberId> {
    match member_item {
        LuaMemberIndexItem::One(member_id) => Some(*member_id),
        LuaMemberIndexItem::Many(member_ids) => {
            let mut resolve_state = MemberTypeResolveState::All;
            let members = member_ids
                .iter()
                .map(|id| member_index.get_member(id))
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
                    for member in members {
                        if member.get_decl_type().is_member_owner() {
                            return Some(member.get_id());
                        }
                    }

                    None
                }
                MemberTypeResolveState::Meta => {
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_meta_decl() {
                            return Some(member.get_id());
                        }
                    }

                    None
                }
                MemberTypeResolveState::FileDecl => {
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_file_decl() {
                            return Some(member.get_id());
                        }
                    }

                    None
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
) -> Option<LuaSemanticDeclId> {
    match member_item {
        LuaMemberIndexItem::One(member_id) => Some(LuaSemanticDeclId::Member(*member_id)),
        LuaMemberIndexItem::Many(member_ids) => {
            let mut resolve_state = MemberSemanticDeclResolveState::MetaOrNone;
            let members = member_ids
                .iter()
                .map(|id| db.get_member_index().get_member(id))
                .collect::<Option<Vec<_>>>()?;
            for member in &members {
                let feature = member.get_feature();
                if feature.is_file_define() {
                    resolve_state = MemberSemanticDeclResolveState::FirstDefine;
                } else if feature.is_file_decl() {
                    resolve_state = MemberSemanticDeclResolveState::FileDecl;
                    break;
                }
            }

            match resolve_state {
                MemberSemanticDeclResolveState::MetaOrNone => {
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_meta_decl() {
                            return Some(LuaSemanticDeclId::Member(member.get_id()));
                        }
                    }

                    Some(LuaSemanticDeclId::Member(members.first()?.get_id()))
                }
                MemberSemanticDeclResolveState::FirstDefine => {
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_file_define() {
                            return Some(LuaSemanticDeclId::Member(member.get_id()));
                        }
                    }

                    None
                }
                MemberSemanticDeclResolveState::FileDecl => {
                    for member in &members {
                        let feature = member.get_feature();
                        if feature.is_file_decl() {
                            return Some(LuaSemanticDeclId::Member(member.get_id()));
                        }
                    }

                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MemberSemanticDeclResolveState {
    MetaOrNone,
    FirstDefine,
    FileDecl,
}
