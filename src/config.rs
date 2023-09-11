use std::path::{Path, PathBuf};
use std::{fs, process};

use home;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Durations {
    #[serde(default = "default_break_duration", rename = "break")]
    pub break_duration: String,

    #[serde(default = "default_focus_duration", rename = "focus")]
    pub focus_duration: String,
}

impl Default for Durations {
    fn default() -> Self {
        Durations {
            break_duration: default_break_duration(),
            focus_duration: default_focus_duration(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Emojis {
    #[serde(default = "default_break_emoji", rename = "break")]
    pub break_emoji: String,

    #[serde(default = "default_focus_emoji", rename = "focus")]
    pub focus_emoji: String,

    #[serde(default = "default_warn_emoji", rename = "warn")]
    pub warn_emoji: Vec<String>,
}

impl Default for Emojis {
    fn default() -> Self {
        Emojis {
            break_emoji: default_break_emoji(),
            focus_emoji: default_focus_emoji(),
            warn_emoji: default_warn_emoji(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sounds {
    #[serde(default = "default_sound", rename = "start")]
    pub start: String,

    #[serde(default = "default_sound", rename = "end")]
    pub end: String,
}

impl Default for Sounds {
    fn default() -> Self {
        Sounds { start: default_sound(), end: default_sound() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingHours {
    start: Option<String>,
    end: Option<String>,
}

impl Default for WorkingHours {
    fn default() -> Self {
        WorkingHours { start: None, end: None }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub durations: Durations,

    #[serde(default)]
    pub emojis: Emojis,

    #[serde(default)]
    pub sound: Sounds,

    #[serde(default)]
    pub working_hours: WorkingHours,
}

fn default_break_duration() -> String {
    String::from("5m")
}

fn default_focus_duration() -> String {
    String::from("30m")
}

fn default_break_emoji() -> String {
    String::from("🥂")
}

fn default_focus_emoji() -> String {
    String::from("🍅")
}

fn default_warn_emoji() -> Vec<String> {
    vec![String::from("🔴"), String::from("⭕")]
}

fn default_sound() -> String {
    String::from("default")
}

pub fn get_config_dir() -> PathBuf {
    let home_dir = home::home_dir();

    if home_dir.is_none() {
        println!("Could not find home directory");
        process::exit(1);
    }

    return Path::new(&home_dir.unwrap()).join(".config").join("pomo");
}

pub fn get_config_file() -> PathBuf {
    Path::new(&get_config_dir()).join("config.json")
}

pub fn read_config() -> Config {
    let contents = fs::read_to_string(get_config_file());
    if contents.is_err() {
        println!("Error when reading config file");
        process::exit(1);
    }

    let deserialized = serde_json::from_str::<Config>(&contents.unwrap());
    if deserialized.is_err() {
        println!("Error when reading config file");
        process::exit(1);
    }

    return deserialized.unwrap();
}
