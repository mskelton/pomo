use std::path::{Path, PathBuf};
use std::{fs, process};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StatusType {
    Idle,
    Focus,
    Break,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    #[serde(rename = "type")]
    pub status_type: StatusType,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub last_notified: Option<DateTime<Utc>>,
}

pub fn get_status_file() -> PathBuf {
    Path::new(&crate::config::get_config_dir()).join("status.json")
}

pub fn read_status() -> Option<Status> {
    let contents = fs::read_to_string(get_status_file());
    if contents.is_err() {
        println!("Error when reading status file");
        process::exit(1);
    }

    return match serde_json::from_str::<Status>(&contents.unwrap()) {
        Ok(s) => Some(s),
        Err(_) => None,
    };
}

pub fn write_status(status: Status) {
    let serialized = match serde_json::to_string(&status) {
        Ok(s) => s,
        Err(_) => {
            println!("Error saving status");
            process::exit(1);
        }
    };

    if fs::write(get_status_file(), serialized).is_err() {
        println!("Error saving status");
        process::exit(1);
    }
}

pub fn clear_status() {
    write_status(Status {
        status_type: StatusType::Idle,
        start: Utc::now(),
        end: Utc::now(),
        last_notified: None,
    });
}
