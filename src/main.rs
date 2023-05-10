use std::fs::metadata;
use std::os::unix::{io::AsRawFd, prelude::MetadataExt};
use std::{fs, io, process};
use serde_json::{Result, Value};

mod flag_manager;
mod display;

fn main() {
    // Parse flags
    let input = flag_manager::parser::new("-i", None, Some("Define input file"));
    let _ = flag_manager::parser::new("-o", None, Some("Define output file"));
    let _ = flag_manager::parser::new("-h", None, Some("Help!"));

    // check flag and stdin to is there any input data
    if input == None && !is_data_piped() {
        display::err(
            "No data passed to the app",
            Some("pleas pass data to app with '-i' flag or pipe in stdin".to_string())
        );

        process::exit(1);
    }

    // Store entry data
    let mut entry_data: String = String::new();

    if input != None {
        // Check the error
        match fs::read_to_string(input.unwrap()) {
            Err(e) => {
                display::err("Couldn't open target file", Some(e.to_string()));
                process::exit(1);
            },
            Ok(f) => entry_data.push_str(&f),
        };
    } else {
        match stdin_data() {
            Ok(stdin) => entry_data.push_str(&stdin),
            Err(e) => {
                display::err(e.as_str(), None);
                process::exit(1);
            },
        };
    }

    match valid_json(entry_data) {
        Err(e) => display::err("message", Some(e.to_string())),
        Ok(v) => println!("{:?}", v),
    }
    
    // validate
    // decode
    // sort
    // encode
    // put out

}


fn stdin_data() -> std::result::Result<String, String> {
    let mut lines = io::stdin().lines();
    let mut buffer = String::new();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        // Stop reading
        if last_input.len() == 0 {
            break;
        }

        // Add a new line once user_input starts storing user input
        if buffer.len() > 0 {
            buffer.push_str("\n");
        }

        // Store input
        buffer.push_str(&last_input);
    }

    Ok(buffer)
}

fn is_data_piped() -> bool {
    let fd = io::stdin().as_raw_fd();
    let meta = metadata("/dev/fd/".to_owned() + &fd.to_string());

    match meta {
        Ok(meta) => return meta.mode() == 4480, // Return is data piped
        Err(_) => false,
    }
}

fn valid_json(parsed_data: String) -> Result<Value> {
    let valid_json: Value = serde_json::from_str(&parsed_data)?; 
    Ok(valid_json)
}