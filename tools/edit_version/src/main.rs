use std::path::Path;
use std::fs;
use toml_edit::{value,DocumentMut};

const CARGOS: &[&str] = &[
    "crates/emmylua_code_analysis/Cargo.toml",
    "crates/emmylua_doc_cli/Cargo.toml",
    "crates/emmylua_ls/Cargo.toml",
];



fn main() {
    let version = std::env::args().nth(1).expect("Please provide a version");
    for cargo in CARGOS {
        for cargo in CARGOS {
            let path = Path::new(cargo);
            let content = fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("Unable to read {}", cargo));
    
            let mut doc = content.parse::<DocumentMut>()
                .unwrap_or_else(|_| panic!("Failed to parse {}", cargo));
    

            doc["package"]["version"] = value(version.clone());
    
            fs::write(path, doc.to_string())
                .unwrap_or_else(|_| panic!("Unable to write to {}", cargo));
        }
    }

    let workspacec_cargo = Path::new("Cargo.toml");
    let content = fs::read_to_string(workspacec_cargo)
        .unwrap_or_else(|_| panic!("Unable to read {:?}", workspacec_cargo));
    let mut doc = content.parse::<DocumentMut>()
        .unwrap_or_else(|_| panic!("Failed to parse {:?}", workspacec_cargo));

    let dependencies = doc["workspace"]["dependencies"].as_table_mut().unwrap();
    if let Some(dep) = dependencies.get_mut("emmylua_code_analysis") {
        dep["version"] = value(version.clone());
    }

    fs::write(workspacec_cargo, doc.to_string())
        .unwrap_or_else(|_| panic!("Unable to write to {:?}", workspacec_cargo));
}
