mod lua_member;

use std::collections::{HashMap, HashSet};

use crate::FileId;
pub use lua_member::{LuaMember, LuaMemberFeature, LuaMemberId, LuaMemberKey, LuaMemberOwner};

use super::traits::LuaIndex;

#[derive(Debug)]
pub struct LuaMemberIndex {
    members: HashMap<LuaMemberId, LuaMember>,
    in_filed: HashMap<FileId, HashSet<MemberOrOwner>>,
    owner_members: HashMap<LuaMemberOwner, Vec<LuaMemberId>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MemberOrOwner {
    Member(LuaMemberId),
    Owner(LuaMemberOwner),
}

impl LuaMemberIndex {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
            in_filed: HashMap::new(),
            owner_members: HashMap::new(),
        }
    }

    pub fn add_member(&mut self, member: LuaMember) -> LuaMemberId {
        let id = member.get_id();
        let owner = member.get_owner();
        let file_id = member.get_file_id();
        self.add_in_file_object(file_id, MemberOrOwner::Member(id));
        self.members.insert(id, member);
        if !owner.is_none() {
            self.add_in_file_object(file_id, MemberOrOwner::Owner(owner.clone()));
            self.add_member_to_owner(owner, id);
        }
        id
    }

    fn add_in_file_object(&mut self, file_id: FileId, member_or_owner: MemberOrOwner) {
        self.in_filed
            .entry(file_id)
            .or_insert_with(HashSet::new)
            .insert(member_or_owner);
    }

    pub fn add_member_to_owner(&mut self, owner: LuaMemberOwner, id: LuaMemberId) -> Option<()> {
        let member = self.members.get(&id)?;
        let key = member.get_key().clone();
        let member_vec = self.owner_members.entry(owner).or_insert_with(Vec::new);
        if member.get_feature().is_decl() {
            member_vec.push(id);
        } else {
            // check exist
            for member_id in member_vec.iter() {
                let old_member = self.members.get(member_id)?;
                if old_member.get_key() == &key && old_member.get_file_id() == member.get_file_id()
                {
                    return None;
                }
            }

            member_vec.push(id);
        }
        Some(())
    }

    pub fn set_member_owner(
        &mut self,
        owner: LuaMemberOwner,
        file_id: FileId,
        id: LuaMemberId,
    ) -> Option<()> {
        let member = self.members.get_mut(&id)?;
        member.set_owner(owner.clone());
        self.add_in_file_object(file_id, MemberOrOwner::Owner(owner));

        Some(())
    }

    pub fn get_member(&self, id: &LuaMemberId) -> Option<&LuaMember> {
        self.members.get(id)
    }

    pub fn get_member_mut(&mut self, id: &LuaMemberId) -> Option<&mut LuaMember> {
        self.members.get_mut(id)
    }

    pub fn get_members(&self, owner: &LuaMemberOwner) -> Option<Vec<&LuaMember>> {
        let member_ids = self.owner_members.get(owner)?;
        Some(
            member_ids
                .iter()
                .filter_map(|id| self.members.get(id))
                .collect(),
        )
    }
}

impl LuaIndex for LuaMemberIndex {
    fn remove(&mut self, file_id: FileId) {
        if let Some(member_ids) = self.in_filed.remove(&file_id) {
            let mut owners = HashSet::new();
            for member_id_or_owner in member_ids {
                match member_id_or_owner {
                    MemberOrOwner::Member(member_id) => {
                        if let Some(member) = self.members.remove(&member_id) {
                            let owner = member.get_owner();
                            owners.insert(owner);
                        }
                    }
                    MemberOrOwner::Owner(owner) => {
                        owners.insert(owner);
                    }
                }
            }

            let mut need_removed_owner = Vec::new();
            for owner in owners {
                if let Some(member_ids) = self.owner_members.get_mut(&owner) {
                    member_ids.retain(|it| it.file_id != file_id);
                    if member_ids.is_empty() {
                        need_removed_owner.push(owner);
                    }
                }
            }

            for owner in need_removed_owner {
                self.owner_members.remove(&owner);
            }
        }
    }

    fn clear(&mut self) {
        self.members.clear();
        self.in_filed.clear();
        self.owner_members.clear();
    }
}
