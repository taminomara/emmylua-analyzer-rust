use std::{env, fs};

use chrono::Local;
use fern::Dispatch;
use log::{info, LevelFilter};

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn init_logger(root: Option<&str>) {
    let level = if cfg!(debug_assertions) {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    if root.is_none() {
        init_stderr_logger(level);
        return;
    }
    let root = root.unwrap();

    let filename = if root.is_empty() || root == "/" {
        "root".to_string()
    } else {
        root.trim_start_matches('/')
            .split(|c| c == '/' || c == '\\' || c == ':')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("_")
    };

    let exe_path = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let log_dir = exe_dir.join("logs");
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir).unwrap();
    }

    let log_file_path = log_dir.join(format!("{}.log", filename));

    if let Err(e) = fs::create_dir_all(log_dir) {
        eprintln!("Failed to create log directory: {:?}", e);
        return;
    }

    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&log_file_path)
        .unwrap();

    let logger = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
                record.level(),
                record.target(),
                message
            ))
        })
        // set level
        .level(level)
        // set output
        .chain(log_file);

    if let Err(e) = logger.apply() {
        eprintln!("Failed to apply logger: {:?}", e);
        return;
    }

    eprintln!("init logger success with file: {:?}", log_file_path);
    info!("{} v{}", CRATE_NAME, CRATE_VERSION);
}

fn init_stderr_logger(level: LevelFilter) {
    let logger = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
                record.level(),
                record.target(),
                message
            ))
        })
        // set level
        .level(level)
        // set output
        .chain(std::io::stderr());

    if let Err(e) = logger.apply() {
        eprintln!("Failed to apply logger: {:?}", e);
        return;
    }

    info!("{} v{}", CRATE_NAME, CRATE_VERSION);
}
