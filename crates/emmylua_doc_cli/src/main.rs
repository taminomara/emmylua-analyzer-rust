use crate::cmd_args::Format;
use clap::Parser;
use cmd_args::CmdArgs;
use std::process::exit;

mod cmd_args;
mod common;
mod init;
mod json_generator;
mod markdown_generator;

fn main() {
    let args = CmdArgs::parse();
    let mut input = args.input;
    if input.is_relative() {
        input = std::env::current_dir().ok().unwrap().join(&input);
    }

    let analysis = init::load_workspace(vec![input.to_str().unwrap()]);
    if let Some(mut analysis) = analysis {
        let res = match args.format {
            Format::Markdown => markdown_generator::generate_markdown(
                &mut analysis,
                args.output,
                args.override_template,
                args.mixin,
            ),
            Format::Json => json_generator::generate_json(&mut analysis, args.output),
        };

        if let Err(err) = res {
            eprintln!("Error: {}", err);
            exit(1);
        }
    } else {
        eprintln!("Analysis failed.");
        exit(1);
    }
}
