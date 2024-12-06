use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::ClientProxy;


pub struct VsCodeStatusBar {
    client: Arc<ClientProxy>
}

impl VsCodeStatusBar {
    pub fn new(client: Arc<ClientProxy>) -> Self {
        Self {
            client
        }
    }

    pub fn set_server_status(&self, health: &str, loading: bool, message: &str) {
        self.client.send_notification("emmy/setServerStatus", EmmyServerStatus {
            health: health.to_string(),
            loading,
            message: message.to_string(),
        });
    }

    pub fn report_progress(&self, message: &str, percentage: f64) {
        self.client.send_notification("emmy/progressReport", EmmyProgress {
            text: message.to_string(),
            percent: percentage,
        });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmmyServerStatus {
    health: String,
    loading: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmmyProgress {
    text: String,
    percent: f64,
}
