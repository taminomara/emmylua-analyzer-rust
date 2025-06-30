use std::path::PathBuf;

use crate::cmd_args::Format;
use clap::Parser;
use cmd_args::CmdArgs;

mod cmd_args;
mod common;
mod init;
mod json_generator;
mod markdown_generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CmdArgs::parse();
    let current_path = std::env::current_dir()?;
    let input = args.input;
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

    match args.format {
        Format::Markdown => markdown_generator::generate_markdown(
            &mut analysis,
            args.output,
            args.override_template,
            args.mixin,
        ),
        Format::Json => json_generator::generate_json(&mut analysis, args.output),
    }
}
