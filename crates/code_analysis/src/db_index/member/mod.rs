mod lua_member;

use std::collections::HashMap;

use crate::FileId;
pub use lua_member::{LuaMember, LuaMemberId, LuaMemberOwner};

use super::traits::LuaIndex;

#[derive(Debug)]
pub struct LuaMemberIndex {
    members: HashMap<LuaMemberId, LuaMember>,
    id_counter: usize,
    in_field_members: HashMap<FileId, Vec<LuaMemberId>>,
    owner_members: HashMap<LuaMemberOwner, HashMap<String, LuaMemberId>>,
}

impl LuaMemberIndex {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
            id_counter: 0,
            in_field_members: HashMap::new(),
            owner_members: HashMap::new(),
        }
    }

    pub fn add_member(&mut self, member: LuaMember) -> LuaMemberId {
        let id = LuaMemberId::new(self.id_counter);
        self.id_counter += 1;
        let owner = member.get_owner();
        let name = member.get_name().to_string();
        self.owner_members
            .entry(owner)
            .or_insert_with(HashMap::new)
            .insert(name, id);
        let file_id = member.get_file_id();
        self.in_field_members
            .entry(file_id)
            .or_insert_with(Vec::new)
            .push(id);
        self.members.insert(id, member);
        id
    }

    pub fn get_member(&self, id: &LuaMemberId) -> Option<&LuaMember> {
        self.members.get(id)
    }

    pub fn get_mut_member(&mut self, id: &LuaMemberId) -> Option<&mut LuaMember> {
        self.members.get_mut(id)
    }

    pub fn get_member_map(&self, owner: LuaMemberOwner) -> Option<&HashMap<String, LuaMemberId>> {
        self.owner_members.get(&owner)
    }
}

impl LuaIndex for LuaMemberIndex {
    fn remove(&mut self, file_id: FileId) {
        if let Some(member_ids) = self.in_field_members.remove(&file_id) {
            for member_id in member_ids {
                if let Some(member) = self.members.remove(&member_id) {
                    let owner = member.get_owner();
                    let name = member.get_name().to_string();
                    if let Some(owner_members) = self.owner_members.get_mut(&owner) {
                        owner_members.remove(&name);
                        if owner_members.is_empty() {
                            self.owner_members.remove(&owner);
                        }
                    }
                }
            }
        }
    }
}
