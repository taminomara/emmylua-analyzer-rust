use super::FileId;

#[derive(Debug, Clone, Hash)]
pub struct InFiled<N> {
    pub file_id: FileId,
    pub value: N,
}

impl<N> InFiled<N> {
    pub fn new(file_id: FileId, value: N) -> Self {
        InFiled { file_id, value }
    }
}
