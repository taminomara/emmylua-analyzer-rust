mod cmd_args;
mod init;
mod output;

use cmd_args::CmdArgs;
use emmylua_code_analysis::{DbIndex, FileId};
use output::output_result;
use std::{error::Error, path::PathBuf, sync::Arc};
use structopt::StructOpt;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let cmd_args = CmdArgs::from_args();
    let mut workspace = cmd_args.workspace;
    if !workspace.is_absolute() {
        workspace = std::env::current_dir()?.join(workspace);
    }

    let analysis = match init::load_workspace(workspace.clone(), cmd_args.config, cmd_args.ignore) {
        Some(analysis) => analysis,
        None => {
            eprintln!("Failed to load workspace");
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
            let diagnostics = analysis.diagnose_file(file_id, cancel_token).await;
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

fn get_need_check_ids(db: &DbIndex, files: Vec<FileId>, workspace: &PathBuf) -> Vec<FileId> {
    let mut need_check_files = Vec::new();
    for file_id in files {
        let file_path = db.get_vfs().get_file_path(&file_id).unwrap();
        if file_path.starts_with(workspace) {
            need_check_files.push(file_id);
        }
    }

    need_check_files
}
