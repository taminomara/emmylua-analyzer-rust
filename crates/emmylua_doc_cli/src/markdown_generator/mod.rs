mod index_gen;
mod mod_gen;
mod render;
mod typ_gen;

use std::path::PathBuf;

use code_analysis::EmmyLuaAnalysis;
use serde::{Deserialize, Serialize};
use tera::Tera;

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

    let mut mkdocs_index = MkdocsIndex::default();
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
        typ_gen::generate_type_markdown(db, &tl, type_decl, &input, &types_out, &mut mkdocs_index);
    }

    let module_index = db.get_module_index();
    let modules = module_index.get_module_infos();
    for module in modules {
        mod_gen::generate_module_markdown(db, &tl, module, &input, &module_out, &mut mkdocs_index);
    }

    index_gen::generate_index(&tl, &mkdocs_index, output);
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
            // Windows 无效文件名字符
            if "<>:\"/\\|?*".contains(c) {
                '_'
            } else {
                c
            }
        })
        .collect()
}
