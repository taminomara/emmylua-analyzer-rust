use emmylua_code_analysis::EmmyrcExternalTool;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::timeout;

pub async fn external_tool_format(
    emmyrc_external_tool: &EmmyrcExternalTool,
    text: &str,
    file_path: &str,
) -> Option<String> {
    let exe_path = &emmyrc_external_tool.program;
    let args = &emmyrc_external_tool.args;
    let timeout_duration = Duration::from_millis(emmyrc_external_tool.timeout as u64);

    let mut cmd = Command::new(exe_path);

    for arg in args {
        let processed_arg = arg.replace("${file}", file_path);
        cmd.arg(processed_arg);
    }

    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            log::error!("Failed to spawn external formatter process: {}", e);
            return None;
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        if let Err(e) = stdin.write_all(text.as_bytes()).await {
            log::error!("Failed to write to external formatter stdin: {}", e);
            return None;
        }
        if let Err(e) = stdin.shutdown().await {
            log::error!("Failed to close external formatter stdin: {}", e);
            return None;
        }
    }

    let output = match timeout(timeout_duration, child.wait_with_output()).await {
        Ok(Ok(output)) => output,
        Ok(Err(e)) => {
            log::error!("External formatter process error: {}", e);
            return None;
        }
        Err(_) => {
            log::error!(
                "External formatter process timed out after {}ms",
                emmyrc_external_tool.timeout
            );
            return None;
        }
    };

    if !output.status.success() {
        log::error!(
            "External formatter exited with non-zero status: {}. Stderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
        return None;
    }

    match String::from_utf8(output.stdout) {
        Ok(formatted_text) => {
            log::debug!("External formatter completed successfully");
            Some(formatted_text)
        }
        Err(e) => {
            log::error!("External formatter output is not valid UTF-8: {}", e);
            None
        }
    }
}
