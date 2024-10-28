use crate::FileId;

pub struct SemanticModel {
    file_id: FileId,
}

impl SemanticModel {
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id
        }
    }
}