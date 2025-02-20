use std::{path::PathBuf, str::FromStr};

use emmylua_code_analysis::{load_workspace_files, EmmyLuaAnalysis};

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let _profiler = dhat::Profiler::new_heap();
    let workspace = std::env::args().nth(1).unwrap();
    let include_pattern = vec!["*.lua".to_string()];
    let encoding = "utf-8";
    let workspace_path = PathBuf::from_str(&workspace).unwrap();

    let files = load_workspace_files(
        &workspace_path,
        &include_pattern,
        &vec![],
        &vec![],
        Some(encoding),
    )
    .unwrap()
    .into_iter()
    .map(|it| it.into_tuple())
    .collect();

    let mut analysis = EmmyLuaAnalysis::new();
    analysis.init_std_lib(false);
    analysis.update_files_by_path(files);
}
