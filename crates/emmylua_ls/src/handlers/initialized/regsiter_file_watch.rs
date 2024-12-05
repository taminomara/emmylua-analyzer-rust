use std::{sync::Arc, vec};

use log::{info, warn};
use lsp_types::{
    ClientCapabilities, DidChangeWatchedFilesRegistrationOptions, FileSystemWatcher, GlobPattern,
    Registration, RegistrationParams, WatchKind,
};
// use notify::{Event, RecommendedWatcher};

use crate::context::ClientProxy;

pub fn register_files_watch(client: Arc<ClientProxy>, client_capabilities: &ClientCapabilities) {
    let lsp_client_can_watch_files = if let Some(workspace) = &client_capabilities.workspace {
        if let Some(did_change_watched_files) = &workspace.did_change_watched_files {
            if let Some(dynamic_registration) = &did_change_watched_files.dynamic_registration {
                dynamic_registration.clone()
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    if lsp_client_can_watch_files {
        register_files_watch_use_lsp_client(client);
    } else {
        info!("use notify to watch files");
        register_files_watch_use_fsnotify(client);
    }
}

fn register_files_watch_use_lsp_client(client: Arc<ClientProxy>) {
    let options = DidChangeWatchedFilesRegistrationOptions {
        watchers: vec![
            FileSystemWatcher {
                glob_pattern: GlobPattern::String("**/*.lua".into()),
                kind: Some(WatchKind::Create | WatchKind::Change | WatchKind::Delete),
            },
            FileSystemWatcher {
                glob_pattern: GlobPattern::String("**/.editorconfig".into()),
                kind: Some(WatchKind::Create | WatchKind::Change | WatchKind::Delete),
            },
            FileSystemWatcher {
                glob_pattern: GlobPattern::String(".luarc.json".into()),
                kind: Some(WatchKind::Create | WatchKind::Change | WatchKind::Delete),
            },
            FileSystemWatcher {
                glob_pattern: GlobPattern::String(".emmyrc.json".into()),
                kind: Some(WatchKind::Create | WatchKind::Change | WatchKind::Delete),
            },
        ],
    };

    let registration = Registration {
        id: "emmylua_watch_files".to_string(),
        method: "workspace/didChangeWatchedFiles".to_string(),
        register_options: Some(serde_json::to_value(options).unwrap()),
    };
    client.dynamic_register_capability(RegistrationParams {
        registrations: vec![registration],
    });
}

fn register_files_watch_use_fsnotify(_: Arc<ClientProxy>) {
    // todo: use notify to watch files
    warn!("use notify to watch files is not implemented");
}