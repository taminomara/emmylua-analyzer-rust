mod loader;
mod in_filed;
mod test;
mod document;


use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use document::LuaDocument;
use emmylua_parser::LineIndex;
use url::Url;
use lsp_types::Uri;

pub use loader::load_workspace_files;
pub use in_filed::InFiled;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub struct FileId {
    pub id: u32
}

impl FileId {
    pub fn new() -> Self {
        FileId {
            id: 0
        }
    }
}

#[derive(Debug)]
pub struct Vfs {
    file_id_map: HashMap<Uri, u32>,
    file_uri_map: HashMap<u32, Uri>,
    file_data: Vec<Option<String>>,
    line_index_map: HashMap<FileId, LineIndex>,
}

impl Vfs {
    pub fn new() -> Self {
        Vfs {
            file_id_map: HashMap::new(),
            file_uri_map: HashMap::new(),
            file_data: Vec::new(),
            line_index_map: HashMap::new()
        }
    }

    pub fn file_id(&mut self, uri: &Uri) -> FileId {
        if let Some(&id) = self.file_id_map.get(uri) {
            FileId { id }
        } else {
            let id = self.file_data.len() as u32;
            self.file_id_map.insert(uri.clone(), id);
            self.file_uri_map.insert(id, uri.clone());
            self.file_data.push(None);
            FileId { id }
        }
    }

    pub fn get_file_id(&self, uri: &Uri) -> Option<FileId> {
        self.file_id_map.get(uri).map(|&id| FileId { id })
    }

    pub fn set_file_content(&mut self, uri: &Uri, data: Option<String>) -> bool {
        let fid = self.file_id(uri);
        if let Some(data) = &data {
            let line_index = LineIndex::parse(&data);
            self.line_index_map.insert(fid, line_index);
        } else {
            self.line_index_map.remove(&fid);
        }
        self.file_data[fid.id as usize] = data;
        true
    }

    pub fn get_file_content(&self, id: &FileId) -> Option<&String> {
        let opt = &self.file_data[id.id as usize];
        if let Some(s) = opt {
            Some(s)
        } else{
            None
        }
    }

    pub fn get_document(&self, id: &FileId) -> Option<LuaDocument> {
        let uri = self.file_uri_map.get(&id.id)?;
        let text = self.get_file_content(id)?;
        let line_index = self.line_index_map.get(id)?;
        Some(LuaDocument::new(*id, uri, text, line_index))
    }
}

pub fn file_path_to_uri(path: &PathBuf) -> Option<Uri> {
    match Url::from_file_path(path) {
        Ok(url) => Some(Uri::from_str(url.as_str()).unwrap()),
        Err(_) => {
            None
        },
    }
}

pub fn uri_to_file_path(uri: &Uri) -> Option<PathBuf> {
    if uri.scheme().unwrap().as_str() != "file" {
        return None;
    }

    let url = Url::from_str(uri.as_str()).unwrap();
    Some(url.to_file_path().unwrap())
}