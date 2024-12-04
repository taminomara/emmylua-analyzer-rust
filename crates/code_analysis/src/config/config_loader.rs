use std::{collections::HashSet, path::PathBuf, sync::Arc};

use serde_json::Value;

use super::Emmyrc;

pub fn load_configs(config_files: Vec<PathBuf>) -> Emmyrc {
    let mut config_jsons = Vec::new();
    for config_file in config_files {
        let config_json_str = match std::fs::read_to_string(&config_file) {
            Ok(json_str) => json_str,
            Err(e) => {
                log::error!(
                    "Failed to read config file: {:?}, error: {:?}",
                    config_file,
                    e
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

    if config_jsons.is_empty() {
        log::info!("No valid config file found.");
        Emmyrc::default()
    } else if config_jsons.len() == 1 {
        let first_config = config_jsons.into_iter().next().unwrap();
        let emmyrc: Emmyrc = serde_json::from_value(first_config).ok().unwrap();
        emmyrc
    } else {
        let merge_config =
            config_jsons
                .into_iter()
                .fold(Value::Object(Default::default()), |mut acc, item| {
                    merge_values(&mut acc, item);
                    acc
                });
        let emmyrc: Emmyrc = serde_json::from_value(merge_config).ok().unwrap();
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
