use crate::FileId;

#[allow(unused)]
pub trait LuaIndex {
    type Key;
    type Data;

    fn remove(&mut self, file_id: FileId);
    fn update(&mut self, file_id: FileId, key: Self::Key, data: Self::Data);
    fn get(&self, key: &Self::Key) -> Option<&Self::Data>;
}
