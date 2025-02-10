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
