// use log::{error, info};
// use rust_i18n::Backend;
// use std::{collections::HashMap, env, fs, path::PathBuf};
// use glob::glob;

// pub struct I18nBackend {
//     trs: HashMap<String, HashMap<String, String>>,
// }

// impl I18nBackend {
//     pub fn new() -> Self {
//         let mut backend = Self {
//             trs: HashMap::new(),
//         };
//         backend.load_metas();
//         backend
//     }

//     fn load_metas(&mut self) -> Option<()> {
//         let meta_dir = self.get_meta_dir()?;
//         info!("load meta from dir: {:?}", meta_dir);
//         let pattern = format!("{}/**/*.yaml", meta_dir.display());
//         for entry in glob(&pattern).expect("Failed to read glob pattern") {
//             match entry {
//                 Ok(path) => {
//                     let locale = path.file_stem()?.to_str()?.to_lowercase();
//                     let content = fs::read_to_string(&path).ok()?;
//                     let yaml: HashMap<String, String> = serde_yml::from_str(&content).ok()?;

//                     self.trs
//                         .entry(locale.to_string())
//                         .or_insert_with(HashMap::new)
//                         .extend(yaml);
//                 }
//                 Err(e) => {
//                     error!("Error reading path: {:?}", e);
//                 }
//             }
//         }

//         Some(())
//     }

//     fn get_meta_dir(&self) -> Option<PathBuf> {
//         let exe_path = env::current_exe().ok()?;
//         let mut current_dir = exe_path.parent()?.to_path_buf();

//         loop {
//             let potential = current_dir.join("resources/meta");
//             info!("try location meta dir: {:?} ...", potential);
//             if potential.is_dir() {
//                 return Some(potential);
//             }

//             match current_dir.parent() {
//                 Some(parent) => current_dir = parent.to_path_buf(),
//                 None => break,
//             }
//         }

//         None
//     }
// }

// impl Backend for I18nBackend {
//     fn available_locales(&self) -> Vec<&str> {
//         return self.trs.keys().map(|k| k.as_str()).collect();
//     }

//     fn translate(&self, locale: &str, key: &str) -> Option<&str> {
//         let locale = locale.to_lowercase();
//         // Write your own lookup logic here.
//         // For example load from database
//         return self.trs.get(&locale)?.get(key).map(|k| k.as_str());
//     }
// }

pub fn meta_keyword(key: &str) -> String {
    t!(format!("keywords.{}", key)).to_string()
}

#[allow(unused)]
pub fn meta_builtin_std(key: &str) -> String {
    t!(format!("builtin_std.{}", key)).to_string()
}

pub fn meta_doc_tag(key: &str) -> String {
    t!(format!("tags.{}", key)).to_string()
}
