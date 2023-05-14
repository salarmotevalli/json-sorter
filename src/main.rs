use json::Json;
use serde_json::Value;
use std::fs::{metadata, File};
use std::io::{self, BufWriter, Write};
use std::os::unix::{io::AsRawFd, prelude::MetadataExt};
use std::{fs, process};

mod display;
mod flag_manager;
mod json;

fn main() {
    // Parse flags
    let input = flag_manager::parser::new("-i", None, Some("Define input file"));
    let output = flag_manager::parser::new("-o", None, Some("Define output file"));
    let _ = flag_manager::parser::new("-h", None, Some("Help!"));

    // check flag and stdin to is there any input data
    if input == None && !is_data_piped() {
        display::err(
            "No data passed to the app",
            Some("pleas pass data to app with '-i' flag or pipe in stdin"),
        );

        process::exit(1);
    }

    // Store entry data
    let mut entry_data: String = String::new();

    if input != None {
        // Check the error
        match fs::read_to_string(input.unwrap()) {
            Err(e) => {
                display::err("Couldn't open target file", Some(&e.to_string()));
                process::exit(1);
            }
            Ok(f) => entry_data.push_str(&f),
        };
    } else {
        match stdin_data() {
            Ok(stdin) => entry_data.push_str(&stdin),
            Err(e) => {
                display::err(e.as_str(), None);
                process::exit(1);
            }
        };
    }

    let mut data_map = Value::Null;
    // validate and decode json
    match Json::decode(&entry_data) {
        Err(e) => display::err("Unable to decode json", Some(&e.to_string())),
        Ok(v) => data_map = v,
    };

    // sort
    // TODO

    // put out

    let mut buffer: BufWriter<Box<dyn io::Write>> = BufWriter::new(Box::new(io::stdout()));

    if let Some(output_file) = output {
        let file = File::create(&output_file).expect("cannot create file");
        buffer = BufWriter::new(Box::new(file));
    }

    match buffer.write_all(Json::encode_with_indent(&data_map).as_bytes()) {
        Err(e) => display::err("Unable to write in buffer", Some(&e.to_string())),
        Ok(_) => {}
    }
    // println!("{}", );
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
