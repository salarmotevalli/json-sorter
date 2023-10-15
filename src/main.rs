use serde_json::{from_reader, to_writer_pretty, Value};
use std::collections::{BTreeMap, HashMap};
use std::fs::{metadata, File};
use std::io::{self, Read, Write};
use std::os::unix::{io::AsRawFd, prelude::MetadataExt};

mod error;

#[cfg(test)]
mod test;

use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None, arg_required_else_help = !is_data_piped())]
struct Cli {
    #[arg(short = 'r')]
    reverse: bool,

    #[arg(short = 'i')]
    input: Option<String>,

    #[arg(short = 'o')]
    output: Option<String>,
}

fn main() -> error::Result<()> {
    // Parse flags
    let matches = Cli::parse();

    // Store entry data
    let entry_buf: Box<dyn Read> = define_reader(matches.input);

    // validate and decode json
    let data_map: BTreeMap<String, Value> = from_reader(entry_buf)?;

    // sort
    let result = sort(data_map, matches.reverse);

    let writer = define_writer(matches.output);

    to_writer_pretty(writer, &result)?;

    Ok(())
}

fn is_data_piped() -> bool {
    let fd = io::stdin().as_raw_fd();
    let meta = metadata("/dev/fd/".to_owned() + &fd.to_string());

    match meta {
        Ok(meta) => meta.mode() == 4480, // Return is data piped
        Err(_) => false,
    }
}

fn define_reader(std_in: Option<String>) -> Box<dyn Read> {
    if let Some(input) = std_in {
        let file = File::open(input).expect("cannot open file");
        Box::new(file)
    } else {
        Box::new(io::stdin())
    }
}

fn define_writer(std_out: Option<String>) -> Box<dyn Write> {
    if let Some(output_file) = std_out {
        Box::new(File::create(output_file).expect("cannot create file"))
    } else {
        Box::new(io::stdout())
    }
}

fn sort(data_map: BTreeMap<String, Value>, reverse: bool) -> HashMap<String, Value> {
    if reverse {
        return data_map.into_iter().rev().collect();
    };
    data_map.into_iter().collect()
}
