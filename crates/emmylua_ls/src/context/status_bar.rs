use std::sync::Arc;

use lsp_types::{
    NumberOrString, ProgressParams, ProgressParamsValue, WorkDoneProgress, WorkDoneProgressBegin,
    WorkDoneProgressCreateParams, WorkDoneProgressEnd, WorkDoneProgressReport,
};
use serde::{Deserialize, Serialize};

use super::ClientProxy;

pub struct VsCodeStatusBar {
    client: Arc<ClientProxy>,
}

#[derive(Clone, Copy)]
pub enum Task {
    LoadWorkspace = 0,
    DiagnoseWorkspace = 1,
}

fn get_task_name(task: &Task) -> &'static str {
    match task {
        Task::LoadWorkspace => "Load workspace",
        Task::DiagnoseWorkspace => "Diagnose workspace",
    }
}

impl VsCodeStatusBar {
    pub fn new(client: Arc<ClientProxy>) -> Self {
        Self { client }
    }

    pub fn set_server_status(&self, health: &str, loading: bool, message: &str) {
        self.client.send_notification(
            "emmy/setServerStatus",
            EmmyServerStatus {
                health: health.to_string(),
                loading,
                message: message.to_string(),
            },
        );
    }

    pub fn start_task(&self, task: Task) {
        self.client.send_notification(
            "window/workDoneProgress/create",
            WorkDoneProgressCreateParams {
                token: NumberOrString::Number(task as i32),
            },
        );
        self.client.send_notification(
            "$/progress",
            ProgressParams {
                token: NumberOrString::Number(task as i32),
                value: ProgressParamsValue::WorkDone(WorkDoneProgress::Begin(
                    WorkDoneProgressBegin {
                        title: get_task_name(&task).to_string(),
                        cancellable: Some(false),
                        message: Some(get_task_name(&task).to_string()),
                        percentage: None,
                    },
                )),
            },
        )
    }

    pub fn update_task(&self, task: Task, percentage: Option<u32>, message: Option<String>) {
        self.client.send_notification(
            "$/progress",
            ProgressParams {
                token: NumberOrString::Number(task as i32),
                value: ProgressParamsValue::WorkDone(WorkDoneProgress::Report(
                    WorkDoneProgressReport {
                        percentage,
                        cancellable: Some(false),
                        message,
                    },
                )),
            },
        )
    }

    pub fn finish_task(&self, task: Task, message: Option<String>) {
        self.client.send_notification(
            "$/progress",
            ProgressParams {
                token: NumberOrString::Number(task as i32),
                value: ProgressParamsValue::WorkDone(WorkDoneProgress::End(WorkDoneProgressEnd {
                    message,
                })),
            },
        )
    }

    pub fn report_progress(&self, message: &str, percentage: f64) {
        self.client.send_notification(
            "emmy/progressReport",
            EmmyProgress {
                text: message.to_string(),
                percent: percentage,
            },
        );
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
