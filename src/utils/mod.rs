use crate::structs;
use chrono;
use colorize::*;
use rand::prelude::*;
use serde_json::from_str;
use serde_json::Result;
use std::{fs, io::Write};

const DATA_FILE: &'static str = "src/.todobook/data.json";

pub fn init() {
    // Check if folder exists
    if !fs::metadata("src/.todobook").is_ok() {
        fs::create_dir("src/.todobook").unwrap();

        // Create File
        let mut file = fs::File::create(DATA_FILE).unwrap();
        file.write_all(b"{\"data\":[]}").unwrap();

        println!("{} {}", "Created folder and file".green(), DATA_FILE);
    }
    // Check if file exists
    else if !fs::metadata(DATA_FILE).is_ok() {
        let mut file = fs::File::create(DATA_FILE).unwrap();
        // Write to file
        file.write_all(b"{\"data\":[]}").unwrap();

        println!("{} {}", "Created file".green(), DATA_FILE);
    }
}

pub fn get_args() -> structs::Command {
    let args = std::env::args().collect::<Vec<String>>();
    let command = args.get(1).unwrap_or(&"".to_string()).to_string(); // Get command or set it to an empty string
    let arguments = args.get(2).unwrap_or(&"".to_string()).to_string(); // "" arguments or ""
    structs::Command {
        command,
        arguments
    }
}

pub fn get_timestamp() -> String {
    let now = chrono::Local::now();
    let timestamp = now.format("%m-%d %H:%M").to_string();
    timestamp
}

pub fn get_id() -> u32 {
    // Generate number between 1 and 1000
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(1..1000);
    id + rng.gen_range(1..1000)
}

pub fn get_todos() -> Result<Vec<structs::Todo>> {
    let data = fs::read_to_string(DATA_FILE).unwrap();
    let todos: structs::ConfigFile = from_str(&data)?;
    Ok(todos.data)
}

pub fn save_todos(todos: Vec<structs::Todo>) {
    let config_file = structs::ConfigFile { data : todos};
    let json = serde_json::to_string(&config_file).unwrap();
    let mut file = fs::File::create(DATA_FILE).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}