mod index_gen;
mod init_tl;
mod mod_gen;
mod render;
mod typ_gen;

use std::path::PathBuf;

use emmylua_code_analysis::EmmyLuaAnalysis;
use serde::{Deserialize, Serialize};

#[allow(unused)]
pub fn generate_markdown(
    analysis: &mut EmmyLuaAnalysis,
    input: &PathBuf,
    output: &PathBuf,
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

    let tl = init_tl::init_tl()?;
    let mut mkdocs_index = MkdocsIndex::default();
    let db = analysis.compilation.get_db();
    let type_index = db.get_type_index();
    let types = type_index.get_all_types();
    for type_decl in types {
        typ_gen::generate_type_markdown(db, &tl, type_decl, &input, &types_out, &mut mkdocs_index);
    }

    let module_index = db.get_module_index();
    let modules = module_index.get_module_infos();
    for module in modules {
        mod_gen::generate_module_markdown(db, &tl, module, &input, &module_out, &mut mkdocs_index);
    }

    index_gen::generate_index(&tl, &mut mkdocs_index, output);
    Some(())
}

#[derive(Debug, Serialize, Deserialize)]
struct MemberDisplay {
    pub name: String,
    pub display: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct MkdocsIndex {
    pub types: Vec<IndexStruct>,
    pub modules: Vec<IndexStruct>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexStruct {
    pub name: String,
    pub file: String,
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
