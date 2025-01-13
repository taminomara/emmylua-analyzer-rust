#[cfg(test)]
mod tests {
    use std::{path::Path, str::FromStr};

    use lsp_types::Uri;

    use crate::{file_path_to_uri, uri_to_file_path, Emmyrc, Vfs};

    fn create_vfs() -> Vfs {
        let mut vfs = Vfs::new();
        vfs.update_config(Emmyrc::default().into());
        vfs
    }

    #[test]
    fn test_basic() {
        let mut vfs = create_vfs();

        let uri = Uri::from_str("file:///C:/Users/username/Documents/test.lua").unwrap();
        let id = vfs.file_id(&uri);
        assert_eq!(id.id, 0);
        let id_another = vfs.get_file_id(&uri).unwrap();
        assert_eq!(id_another, id);
        let uri2 = Uri::from_str("file:///C:/Users/username/Documents/test2.lua").unwrap();
        
        let id2 = vfs.file_id(&uri2);
        assert_eq!(id2.id, 1);
        assert!(id2 != id);

        vfs.set_file_content(&uri, Some("content".to_string()));
        let content = vfs.get_file_content(&id).unwrap();
        assert_eq!(content, "content");

        let content2 = vfs.get_file_content(&id2);
        assert!(content2.is_none());
    }

    #[test]
    fn test_clear_file() {
        let mut vfs = create_vfs();
        let uri = Uri::from_str("file:///C:/Users/username/Documents/test.lua").unwrap();
        let id = vfs.file_id(&uri);
        vfs.set_file_content(&uri, Some("content".to_string()));
        let content = vfs.get_file_content(&id).unwrap();
        assert_eq!(content, "content");

        vfs.set_file_content(&uri, None);
        let content = vfs.get_file_content(&id);
        assert!(content.is_none());
    }

    #[test]
    fn test_file_path_to_uri() {
        let mut vfs = create_vfs();
        if cfg!(windows) {
            let uri = Uri::from_str("file:///C:/Users/username/Documents/test.lua").unwrap();
            let id = vfs.file_id(&uri);
            let path = Path::new("C:/Users/username/Documents/test.lua");
            let uri2 = file_path_to_uri(&path.into()).unwrap();
            assert_eq!(uri2, uri);
            let id2 = vfs.file_id(&uri2);
            assert_eq!(id2, id);
        }
    }

    #[test]
    fn test_uri_to_file_path() {
        if cfg!(windows) {
            let uri = Uri::from_str("file:///C:/Users/username/Documents/test.lua").unwrap();
            let path2 = uri_to_file_path(&uri).unwrap();
            assert_eq!(path2, Path::new("C:/Users/username/Documents/test.lua"));

            let windows_path = Path::new("C:\\Users\\username\\Documents\\test.lua");
            assert_eq!(path2, windows_path);

            let uri = Uri::from_str("file:///c%3A/Users//username/Desktop/learn/test%20main/test.lua").unwrap();
            let path = uri_to_file_path(&uri).unwrap();
            let path2 = Path::new("C:/Users//username/Desktop/learn/test main/test.lua");
            assert_eq!(path, path2);
        }
    }

    #[test]
    fn test_relative_path() {
        let worksapce = Path::new("C:/Users\\username/Documents");
        let uri = Uri::from_str("file:///C:/Users/username/Documents/test.lua").unwrap();
        let file_path = uri_to_file_path(&uri).unwrap();
        let relative_path = file_path.strip_prefix(worksapce).unwrap();
        assert_eq!(relative_path, Path::new("test.lua"));
        let file_path2 = Path::new("C:\\Users\\username/Documents\\test.lua");
        let relative_path2 = file_path2.strip_prefix(worksapce).unwrap();
        assert_eq!(relative_path2, Path::new("test.lua"));
    }
}