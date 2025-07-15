mod cmd_args;
mod context;
mod handlers;
mod logger;
mod main_loop;
mod meta_text;
mod util;

pub use clap::Parser;
pub use cmd_args::*;
use handlers::server_capabilities;
use lsp_server::Connection;
use lsp_types::InitializeParams;
use std::{env, error::Error};

#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("./locales", fallback = "en");

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[allow(unused)]
async fn run_ls(cmd_args: CmdArgs) -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, threads) = match cmd_args.communication {
        cmd_args::Communication::Stdio => Connection::stdio(),
        cmd_args::Communication::Tcp => {
            let port = cmd_args.port;
            let ip = cmd_args.ip.clone();
            let addr = (ip.as_str(), port);
            Connection::listen(addr).unwrap()
        }
    };

    let (id, params) = connection.initialize_start()?;
    let initialization_params: InitializeParams = serde_json::from_value(params).unwrap();
    let server_capbilities = server_capabilities(&initialization_params.capabilities);
    let initialize_data = serde_json::json!({
        "capabilities": server_capbilities,
        "serverInfo": {
            "name": CRATE_NAME,
            "version": CRATE_VERSION
        }
    });

    connection.initialize_finish(id, initialize_data)?;

    main_loop::main_loop(connection, initialization_params, cmd_args).await?;
    threads.join()?;

    eprintln!("Server shutting down.");
    Ok(())
}
