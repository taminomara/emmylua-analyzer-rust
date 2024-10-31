use crate::FileId;

#[allow(unused)]
pub trait LuaIndex {
    fn remove(&mut self, file_id: FileId);
}
