mod gen;
mod init_tl;
mod markdown_types;
mod mixin_copy;
mod render;

use std::path::PathBuf;

use emmylua_code_analysis::EmmyLuaAnalysis;
use gen::{
    generate_global_markdown, generate_index, generate_module_markdown, generate_type_markdown,
};
use markdown_types::MkdocsIndex;

#[allow(unused)]
pub fn generate_markdown(
    analysis: &mut EmmyLuaAnalysis,
    input: PathBuf,
    output: PathBuf,
    override_template: Option<PathBuf>,
    mixin: Option<PathBuf>,
) -> Option<()> {
    let docs_dir = output.join("docs");
    let types_out = docs_dir.join("types");
    if !types_out.exists() {
        println!("Creating types directory: {:?}", types_out);
        std::fs::create_dir_all(&types_out).ok()?;
    } else {
        println!("Clearing types directory: {:?}", types_out);
        std::fs::remove_dir_all(&types_out).ok()?;
        std::fs::create_dir_all(&types_out).ok()?;
    }

    let module_out = docs_dir.join("modules");
    if !module_out.exists() {
        println!("Creating modules directory: {:?}", module_out);
        std::fs::create_dir_all(&module_out).ok()?;
    } else {
        println!("Clearing modules directory: {:?}", module_out);
        std::fs::remove_dir_all(&module_out).ok()?;
        std::fs::create_dir_all(&module_out).ok()?;
    }

    let global_out = docs_dir.join("globals");
    if !global_out.exists() {
        println!("Creating globals directory: {:?}", global_out);
        std::fs::create_dir_all(&global_out).ok()?;
    } else {
        println!("Clearing globals directory: {:?}", global_out);
        std::fs::remove_dir_all(&global_out).ok()?;
        std::fs::create_dir_all(&global_out).ok()?;
    }

    let tl = init_tl::init_tl(override_template)?;
    let mut mkdocs_index = MkdocsIndex::default();
    let db = analysis.compilation.get_db();
    let type_index = db.get_type_index();
    let types = type_index.get_all_types();
    for type_decl in types {
        generate_type_markdown(db, &tl, type_decl, &types_out, &mut mkdocs_index);
    }

    let module_index = db.get_module_index();
    let modules = module_index.get_module_infos();
    for module in modules {
        generate_module_markdown(db, &tl, module, &module_out, &mut mkdocs_index);
    }

    let decl_index = db.get_decl_index();
    let globals = decl_index.get_global_decls();
    for global_decl_id in globals {
        generate_global_markdown(db, &tl, &global_decl_id, &global_out, &mut mkdocs_index);
    }

    generate_index(&tl, &mut mkdocs_index, &output);

    if let Some(mixin) = mixin {
        mixin_copy::mixin_copy(&output, mixin);
    }

    Some(())
}

fn escape_type_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            // Windows Invalid Characters
            if "<>:\"/\\|?*".contains(c) {
                '_'
            } else {
                c
            }
        })
        .collect()
}
