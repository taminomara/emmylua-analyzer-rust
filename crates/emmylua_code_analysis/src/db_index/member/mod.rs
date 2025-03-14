mod lua_member;

use std::collections::HashMap;

use crate::FileId;
pub use lua_member::{LuaMember, LuaMemberId, LuaMemberKey, LuaMemberOwner};

use super::traits::LuaIndex;

#[derive(Debug)]
pub struct LuaMemberIndex {
    members: HashMap<LuaMemberId, LuaMember>,
    in_field_members: HashMap<FileId, Vec<LuaMemberId>>,
    owner_members: HashMap<LuaMemberOwner, Vec<LuaMemberId>>,
}

impl LuaMemberIndex {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
            in_field_members: HashMap::new(),
            owner_members: HashMap::new(),
        }
    }

    pub fn add_member(&mut self, member: LuaMember) -> LuaMemberId {
        let id = member.get_id();
        let owner = member.get_owner();
        let file_id = member.get_file_id();
        self.in_field_members
            .entry(file_id)
            .or_insert_with(Vec::new)
            .push(id);
        self.members.insert(id, member);

        if !owner.is_none() {
            self.add_member_to_owner(owner, id);
        }
        id
    }

    pub fn add_member_to_owner(&mut self, owner: LuaMemberOwner, id: LuaMemberId) -> Option<()> {
        let member = self.members.get(&id)?;
        let key = member.get_key().clone();
        let member_vec = self.owner_members.entry(owner).or_insert_with(Vec::new);
        if member.is_decl() {
            member_vec.push(id);
        } else {
            // check exist
            for member_id in member_vec.iter() {
                if self.members.get(member_id)?.get_key() == &key {
                    return Some(());
                }
            }

            member_vec.push(id);
        }
        Some(())
    }

    pub fn set_member_owner(&mut self, owner: LuaMemberOwner, id: LuaMemberId) -> Option<()> {
        let member = self.members.get_mut(&id)?;
        member.set_owner(owner);

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
        if let Some(member_ids) = self.in_field_members.remove(&file_id) {
            let mut owners = Vec::new();
            for member_id in member_ids {
                if let Some(member) = self.members.remove(&member_id) {
                    let owner = member.get_owner();
                    owners.push(owner);
                }
            }

            for owner in owners {
                if let Some(member_ids) = self.owner_members.get_mut(&owner) {
                    member_ids.retain(|it| it.file_id != file_id);
                }
            }
        }
    }

    fn clear(&mut self) {
        self.members.clear();
        self.in_field_members.clear();
        self.owner_members.clear();
    }
}
