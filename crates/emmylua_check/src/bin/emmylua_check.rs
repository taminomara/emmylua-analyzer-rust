use clap::Parser;
use emmylua_check::{cmd_args::CmdArgs, run_check};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let cmd_args = CmdArgs::parse();
    run_check(cmd_args).await
}
