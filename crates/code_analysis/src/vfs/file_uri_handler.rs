use lsp_types::Uri;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

pub fn file_path_to_uri(path: &PathBuf) -> Option<Uri> {
    Url::from_file_path(path)
        .ok()
        .and_then(|url| Uri::from_str(url.as_str()).ok())
}
pub fn uri_to_file_path(uri: &Uri) -> Option<PathBuf> {
    let url = Url::parse(uri.as_str()).ok()?;
    if url.scheme() != "file" {
        return None;
    }

    let mut path = url.path().to_string();

    #[cfg(windows)]
    {
        path = path.trim_start_matches('/').replace('\\', "/");
        // 解码并处理驱动器字母
        if path.len() >= 4 && &path[1..4].to_lowercase() == "%3a" {
            let drive = path.chars().next()?.to_ascii_uppercase();
            let rest = &path[4..];
            path = format!("{}:{}", drive, rest);
        } else if path.len() >= 2 && path.chars().nth(1) == Some(':') {
            let drive = path.chars().next()?.to_ascii_uppercase();
            path.replace_range(..2, &format!("{}:", drive));
        }
    }

    Some(PathBuf::from(path))
}
