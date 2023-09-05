use clap::{Arg, Command};
use serde_json::Value;
use std::fs;
use std::fs::{metadata, File};
use std::io::{self, BufWriter, Write};
use std::os::unix::{io::AsRawFd, prelude::MetadataExt};

mod error;

fn main() -> error::Result<()> {
    // Parse flags
    let matches = Command::new("salar")
        .arg_required_else_help(!is_data_piped())
        .arg(Arg::new("input").short('i').long("input"))
        .arg(Arg::new("output").short('o').long("output"))
        .get_matches();

    // Store entry data
    let entry_buf: String = if matches.get_one::<String>("input") != None {
        fs::read_to_string(matches.get_one::<String>("input").unwrap())?
    } else {
        stdin_data()
    };

    // validate and decode json
    let data_map: Value = serde_json::from_str(entry_buf.as_str())?;

    // sort
    // serde decode json to BTreeMap type so
    // the data map is sorted now

    // put out
    put_out_result(matches.get_one::<String>("output"), data_map)?;

    Ok(())
}

fn stdin_data() -> String {
    let mut lines = io::stdin().lines();
    let mut buffer = String::new();

    while let Some(line) = lines.next() {
        let last_input = line.expect("fuck your entry");

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

    buffer
}

fn is_data_piped() -> bool {
    let fd = io::stdin().as_raw_fd();
    let meta = metadata("/dev/fd/".to_owned() + &fd.to_string());

    match meta {
        Ok(meta) => return meta.mode() == 4480, // Return is data piped
        Err(_) => false,
    }
}

fn put_out_result(std_out: Option<&String>, data_map: Value) -> error::Result<()> {
    let mut buffer: BufWriter<Box<dyn io::Write>> = BufWriter::new(Box::new(io::stdout()));

    if let Some(output_file) = std_out {
        let file = File::create(&output_file).expect("cannot create file");
        buffer = BufWriter::new(Box::new(file));
    }
    buffer.write_all(serde_json::to_string_pretty(&data_map).unwrap().as_bytes())?;

    Ok(())
}
