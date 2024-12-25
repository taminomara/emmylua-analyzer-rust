use lsp_types::Uri;
use percent_encoding::percent_decode_str;
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

    let mut decoded_path = percent_decode_str(url.path())
        .decode_utf8()
        .ok()?
        .to_string();

    #[cfg(windows)]
    {
        decoded_path = decoded_path.trim_start_matches('/').replace('\\', "/");
        // 解码并处理驱动器字母
        if decoded_path.len() >= 4 && &decoded_path[1..4].to_lowercase() == "%3a" {
            let drive = decoded_path.chars().next()?.to_ascii_uppercase();
            let rest = &decoded_path[4..];
            decoded_path = format!("{}:{}", drive, rest);
        } else if decoded_path.len() >= 2 && decoded_path.chars().nth(1) == Some(':') {
            let drive = decoded_path.chars().next()?.to_ascii_uppercase();
            decoded_path.replace_range(..2, &format!("{}:", drive));
        }
    }

    Some(PathBuf::from(decoded_path))
}
