use std::{collections::HashSet, path::PathBuf};

use serde_json::Value;

use crate::read_file_with_encoding;

use super::{flatten_config::FlattenConfigObject, Emmyrc};

pub fn load_configs(config_files: Vec<PathBuf>, partial_emmyrcs: Option<Vec<Value>>) -> Emmyrc {
    let mut config_jsons = Vec::new();

    for config_file in config_files {
        log::info!("Loading config file: {:?}", config_file);
        let config_json_str = match read_file_with_encoding(&config_file, "utf-8") {
            Some(json_str) => json_str,
            None => {
                log::error!(
                    "Failed to read config file: {:?}, error: File not found or unreadable",
                    config_file
                );
                continue;
            }
        };

        let config_json: Value = match serde_json::from_str(&config_json_str) {
            Ok(json) => json,
            Err(e) => {
                log::error!(
                    "Failed to parse config file: {:?}, error: {:?}",
                    &config_file,
                    e
                );
                continue;
            }
        };

        config_jsons.push(config_json);
    }

    if let Some(partial_emmyrcs) = partial_emmyrcs {
        for partial_emmyrc in partial_emmyrcs {
            config_jsons.push(partial_emmyrc);
        }
    }

    if config_jsons.is_empty() {
        log::info!("No valid config file found.");
        Emmyrc::default()
    } else if config_jsons.len() == 1 {
        let first_config = match config_jsons.into_iter().next() {
            Some(config) => config,
            None => {
                log::error!("No valid config file found.");
                return Emmyrc::default();
            }
        };

        let flatten_config = FlattenConfigObject::parse(first_config);
        let emmyrc_json_value = flatten_config.to_emmyrc();
        let emmyrc: Emmyrc = match serde_json::from_value(emmyrc_json_value) {
            Ok(config) => config,
            Err(err) => {
                log::error!("Failed to parse config, error: {:?}", err);
                Emmyrc::default()
            }
        };
        emmyrc
    } else {
        let merge_config =
            config_jsons
                .into_iter()
                .fold(Value::Object(Default::default()), |mut acc, item| {
                    merge_values(&mut acc, item);
                    acc
                });
        let flatten_config = FlattenConfigObject::parse(merge_config.clone());
        let emmyrc_json_value = flatten_config.to_emmyrc();
        let emmyrc: Emmyrc = match serde_json::from_value(emmyrc_json_value) {
            Ok(config) => config,
            Err(err) => {
                log::error!("Failed to parse config: error: {:?}", err);
                Emmyrc::default()
            }
        };
        emmyrc
    }
}

fn merge_values(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            for (key, overlay_value) in overlay_map {
                match base_map.get_mut(&key) {
                    Some(base_value) => {
                        merge_values(base_value, overlay_value);
                    }
                    None => {
                        base_map.insert(key, overlay_value);
                    }
                }
            }
        }
        (Value::Array(base_array), Value::Array(overlay_array)) => {
            let mut seen = HashSet::new();
            base_array.extend(
                overlay_array
                    .into_iter()
                    .filter(|item| seen.insert(item.clone())),
            );
        }
        (base_slot, overlay_value) => {
            *base_slot = overlay_value;
        }
    }
}
