mod context;
mod handlers;
mod util;
mod logger;

use handlers::{
    initialized_handler, on_notification_handler, on_req_handler, on_response_handler,
    server_capabilities,
};
use lsp_server::{Connection, Message};
use lsp_types::InitializeParams;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let args: Vec<String> = std::env::args().collect();
    let (connection, threads) = if args.len() > 1 {
        let port = args[1].parse::<u16>().unwrap();
        let addr = ("127.0.0.1", port);
        Connection::listen(addr).unwrap()
    } else {
        Connection::stdio()
    };

    let server_capabilities = serde_json::to_value(server_capabilities()).unwrap();

    let initialization_params_json = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(err) => {
            if err.channel_is_disconnected() {
                return Ok(());
            } else {
                return Err(err.into());
            }
        }
    };

    let initialization_params =
        serde_json::from_value::<InitializeParams>(initialization_params_json)?;

    main_loop(&connection, initialization_params).await?;
    threads.join()?;

    eprintln!("Server shutting down.");
    Ok(())
}

async fn main_loop(
    connection: &Connection,
    params: InitializeParams,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut server_context = context::ServerContext::new(Connection {
        sender: connection.sender.clone(),
        receiver: connection.receiver.clone(),
    });

    let server_context_snapshot = server_context.snapshot();
    tokio::spawn(async move {
        initialized_handler(server_context_snapshot, params).await;
    });

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }

                on_req_handler(req, &mut server_context).await?;
            }
            Message::Notification(notify) => {
                on_notification_handler(notify, &mut server_context).await?;
            }
            Message::Response(response) => {
                on_response_handler(response, &mut server_context).await?;
            }
        }
    }

    Ok(())
}
