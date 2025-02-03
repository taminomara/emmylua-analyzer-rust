use include_dir::{include_dir, Dir};
use tera::Tera;

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/template");

pub fn init_tl() -> Option<Tera> {

    let mut tera = Tera::default();
    tera.add_raw_templates(TEMPLATE_DIR.files().map(|file| {
        let path = file.path();
        let content = file.contents_utf8().unwrap();
        (path.to_str().unwrap(), content)
    })).ok()?;

    Some(tera)
}