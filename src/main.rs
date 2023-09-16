use clap::{Arg, ArgMatches, Command};
use serde_json::Value;
use std::fs::{metadata, File};
use std::io::{self, Read, Write};
use std::os::unix::{io::AsRawFd, prelude::MetadataExt};

mod error;

#[cfg(test)]
mod test;

fn main() -> error::Result<()> {
    // Parse flags
    let matches: ArgMatches = matches();

    // Store entry data
    let entry_buf: Box<dyn Read> = if matches.get_one::<String>("input").is_some() {
        let file = File::open(matches.get_one::<String>("input").unwrap())?;
        Box::new(file)
    } else {
        Box::new(io::stdin())
    };

    // validate and decode json
    // let mut data_map: Value = serde_json::from_str(entry_buf.as_str())?;
    let d_na: Value = serde_json::from_reader(entry_buf)?;

    // sort
    // serde decode json to BTreeMap type so
    // the data map is sorted now

    let result = serde_json::to_string_pretty(&d_na)?;

    let mut writer = define_writer(matches.get_one::<String>("output"));
    // put out
    put_out_result(&mut writer, result)?;

    Ok(())
}

fn matches() -> ArgMatches {
    Command::new("seyn")
        .arg_required_else_help(!is_data_piped())
        .arg(Arg::new("input").short('i').long("input"))
        .arg(Arg::new("output").short('o').long("output"))
        .arg(Arg::new("wrap").short('w').long("wrap"))
        .get_matches()
}

fn is_data_piped() -> bool {
    let fd = io::stdin().as_raw_fd();
    let meta = metadata("/dev/fd/".to_owned() + &fd.to_string());

    match meta {
        Ok(meta) => meta.mode() == 4480, // Return is data piped
        Err(_) => false,
    }
}

fn define_writer(std_out: Option<&String>) -> Box<dyn Write> {
    if let Some(output_file) = std_out {
        Box::new(File::create(output_file).expect("cannot create file"))
    } else {
        Box::new(io::stdout())
    }
}

fn put_out_result(writer: &mut impl Write, result: String) -> error::Result<()> {
    write!(writer, "{}", result)?;

    Ok(())
}
