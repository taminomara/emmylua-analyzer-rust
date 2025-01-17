mod mod_gen;
mod render;
mod typ_gen;

use std::path::PathBuf;

use code_analysis::EmmyLuaAnalysis;
use tera::Tera;

#[allow(unused)]
pub fn generate_markdown(analysis: &mut EmmyLuaAnalysis, output: &PathBuf) -> Option<()> {
    let types_out = output.join("types");
    let module_out = output.join("modules");
    let resources = analysis.get_resource_dir()?;
    let tl_template = resources.join("template/**/*.tl");
    let tl = match Tera::new(tl_template.to_string_lossy().as_ref()) {
        Ok(tl) => tl,
        Err(e) => {
            eprintln!("Failed to load template: {}", e);
            return None;
        }
    };
    let db = analysis.compilation.get_db();
    let type_index = db.get_type_index();
    let types = type_index.get_all_types();
    for type_decl in types {
        typ_gen::generate_type_markdown(db, &tl, type_decl, &types_out);
    }

    Some(())
}
