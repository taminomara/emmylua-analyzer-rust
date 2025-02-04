use cmd_args::CmdArgs;
use structopt::StructOpt;

mod cmd_args;
mod init;
mod markdown_generator;

fn main() {
    let args = CmdArgs::from_args();
    let mut input = args.input;
    if input.is_relative() {
        input = std::env::current_dir().ok().unwrap().join(&input);
    }

    let analysis = init::load_workspace(vec![input.to_str().unwrap()]);
    if let Some(mut analysis) = analysis {
        markdown_generator::generate_markdown(&mut analysis, &input, &args.output);
    }
}
