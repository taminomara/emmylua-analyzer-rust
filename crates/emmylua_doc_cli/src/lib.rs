use std::path::PathBuf;

pub use crate::cmd_args::Format;
pub use clap::Parser;
pub use cmd_args::*;

mod cmd_args;
mod common;
mod init;
mod json_generator;
mod markdown_generator;

#[allow(unused)]
pub fn run_doc_cli(cmd_args: CmdArgs) -> Result<(), Box<dyn std::error::Error>> {
    let current_path = std::env::current_dir()?;
    let input = cmd_args.input;
    let mut files: Vec<PathBuf> = Vec::new();
    for path in input {
        if path.is_relative() {
            files.push(current_path.join(path));
        } else {
            files.push(path);
        }
    }

    let mut analysis = match init::load_workspace(files) {
        Some(a) => a,
        None => return Err("failed to load workspace".into()),
    };

    match cmd_args.format {
        Format::Markdown => markdown_generator::generate_markdown(
            &mut analysis,
            cmd_args.output,
            cmd_args.override_template,
            cmd_args.site_name,
            cmd_args.mixin,
        ),
        Format::Json => json_generator::generate_json(&mut analysis, cmd_args.output),
    }
}
