mod cmd_args;
mod init;
mod output;
mod terminal_display;

pub use clap::Parser;
pub use cmd_args::CmdArgs;
use fern::Dispatch;
use log::LevelFilter;
use output::output_result;
use std::{error::Error, sync::Arc};
use tokio_util::sync::CancellationToken;

use crate::init::get_need_check_ids;

#[allow(unused)]
pub async fn run_check(cmd_args: CmdArgs) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut workspace = cmd_args.workspace;
    if !workspace.is_absolute() {
        workspace = std::env::current_dir()?.join(workspace);
    }

    let verbose = cmd_args.verbose;
    let logger = Dispatch::new()
        .format(move |out, message, record| {
            let (color, reset) = match record.level() {
                log::Level::Error => ("\x1b[31m", "\x1b[0m"), // Red
                log::Level::Warn => ("\x1b[33m", "\x1b[0m"),  // Yellow
                log::Level::Info | log::Level::Debug | log::Level::Trace => ("", ""),
            };
            out.finish(format_args!(
                "{}{}: {}{}",
                color,
                record.level(),
                if verbose {
                    format!("({}) {}", record.target(), message)
                } else {
                    message.to_string()
                },
                reset
            ))
        })
        .level(if verbose {
            LevelFilter::Info
        } else {
            LevelFilter::Warn
        })
        .chain(std::io::stderr());

    if let Err(e) = logger.apply() {
        eprintln!("Failed to apply logger: {:?}", e);
    }

    let analysis = match init::load_workspace(workspace.clone(), cmd_args.config, cmd_args.ignore) {
        Some(analysis) => analysis,
        None => {
            return Err("Failed to load workspace".into());
        }
    };

    let files = analysis.compilation.get_db().get_vfs().get_all_file_ids();
    let db = analysis.compilation.get_db();
    let need_check_files = get_need_check_ids(db, files, &workspace);

    let (sender, receiver) = tokio::sync::mpsc::channel(100);
    let analysis = Arc::new(analysis);
    let db = analysis.compilation.get_db();
    for file_id in need_check_files.clone() {
        let sender = sender.clone();
        let analysis = analysis.clone();
        tokio::spawn(async move {
            let cancel_token = CancellationToken::new();
            let diagnostics = analysis.diagnose_file(file_id, cancel_token);
            sender.send((file_id, diagnostics)).await.unwrap();
        });
    }

    let exit_code = output_result(
        need_check_files.len(),
        db,
        workspace,
        receiver,
        cmd_args.output_format,
        cmd_args.output,
        cmd_args.warnings_as_errors,
    )
    .await;

    if exit_code != 0 {
        return Err(format!("exit code: {}", exit_code).into());
    }

    eprintln!("Check finished");
    Ok(())
}
