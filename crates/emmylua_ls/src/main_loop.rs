use lsp_server::{Connection, Message};
use lsp_types::InitializeParams;
use std::error::Error;

use crate::{
    cmd_args::CmdArgs,
    context,
    handlers::{initialized_handler, on_notification_handler, on_req_handler, on_response_handler},
};

pub async fn main_loop(
    connection: Connection,
    params: InitializeParams,
    cmd_args: CmdArgs,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let mut server_context = context::ServerContext::new(Connection {
        sender: connection.sender.clone(),
        receiver: connection.receiver.clone(),
    });

    let server_context_snapshot = server_context.snapshot();
    tokio::spawn(async move {
        initialized_handler(server_context_snapshot, params, cmd_args).await;
    });

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    server_context.close().await;
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

    server_context.close().await;
    Ok(())
}
