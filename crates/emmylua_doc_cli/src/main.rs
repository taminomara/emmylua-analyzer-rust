use cmd_args::CmdArgs;
use structopt::StructOpt;

mod cmd_args;
mod init;
mod markdown_generator;

fn main() {
    let args = CmdArgs::from_args();
    let analysis = init::load_workspace(vec![args.input.to_str().unwrap()]);
    if let Some(mut analysis) = analysis {
        markdown_generator::generate_markdown(&mut analysis, &args.input, &args.output);
    }
}
