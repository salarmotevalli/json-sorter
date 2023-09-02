use clap::{Arg, Command};
use std::fs::{metadata, File};
use std::io::{self, BufWriter, Write};
use std::os::unix::{io::AsRawFd, prelude::MetadataExt};
use std::{fs, process};

mod display;

fn main() {
    // Parse flags
    let matches = Command::new("salar")
        .arg_required_else_help(!is_data_piped())
        .arg(Arg::new("input").short('i').long("input"))
        .arg(Arg::new("output").short('o').long("output"))
        .get_matches();

    // Store entry data
    let mut entry_buf: String = String::new();

    if matches.get_one::<String>("input") != None {
        // Check the error
        match fs::read_to_string(matches.get_one::<String>("input").unwrap()) {
            Err(e) => {
                display::err("Couldn't open target file", Some(&e.to_string()));
                process::exit(1);
            }
            Ok(f) => entry_buf.push_str(&f),
        };
    } else {
        match stdin_data() {
            Ok(stdin) => entry_buf.push_str(&stdin),
            Err(e) => {
                display::err(e.as_str(), None);
                process::exit(1);
            }
        };
    }

    // validate and decode json
    let data_map = match serde_json::from_str(&entry_buf) {
        Err(e) => {
            display::err("Unable to decode json", Some(&e.to_string()));
            process::exit(1);
        }
        Ok(v) => v,
    };

    // sort
    // serde decode json to BTreeMap type so
    // the data map is sorted now

    // put out

    let mut buffer: BufWriter<Box<dyn io::Write>> = BufWriter::new(Box::new(io::stdout()));

    if let Some(output_file) = matches.get_one::<String>("input") {
        let file = File::create(&output_file).expect("cannot create file");
        buffer = BufWriter::new(Box::new(file));
    }

    match buffer.write_all(serde_json::to_string_pretty(&data_map).unwrap().as_bytes()) {
        Err(e) => display::err("Unable to write in buffer", Some(&e.to_string())),
        Ok(_) => {}
    }
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
