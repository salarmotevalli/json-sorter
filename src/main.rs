use std::fs::metadata;
use std::os::unix::io::AsRawFd;
// use std::io;
// fn main() -> io::Result<()> {
//     let stdin = io::stdin();
//     let fd = stdin.as_raw_fd();
//     let meta = metadata("/dev/fd/".to_owned() + &fd.to_string())?;
//     println!("File type: {:?}", meta.file_type());
//     Ok(())
// }

// use crate::flags;

// mod flags;

use std::os::unix::prelude::MetadataExt;
use std::{fs, io, process};

mod flag_manager;
mod display;

fn main() {
    // Parse flags
    let input = flag_manager::parser::new("-i", None, Some("Define input file"));
    let _ = flag_manager::parser::new("-o", None, Some("Define output file"));
    let _ = flag_manager::parser::new("-h", None, Some("Help!"));
    
    // Validate entry  
    if flag_manager::env::passed_flags_count() == 0 {
        display::hello();
        display::usage();
        display::flags();
    }

    // check flags
    if input != None {
        let file = fs::read_to_string(input.unwrap());
        
        // Check the error
        match file
        {
            Err(e) => display::err("Couldn't open target file", Some(e.to_string())),
            Ok(f) => println!("{}", f),     
        };
    }

    match ttest() {
        Ok(stdin) => println!("{}", stdin),
        Err(_) => process::exit(1),
    };

    // check stdin
    // get data
    // validate
    // decode
    // sort
    // encode
    // put out

}


fn ttest () -> Result<String, String> {
    let stdin: io::Stdin = io::stdin();
    let mut lines = stdin.lines();
    let fd = io::stdin().as_raw_fd();
    let meta = metadata("/dev/fd/".to_owned() + &fd.to_string());

    match meta {
        Ok(meta) => {
            if meta.mode() != 4480 {
                return Err(String::from("No data piped to stdin"));
            }

            let mut buffer = String::new();
            while let Some(line) = lines.next() {
                let last_input = line.unwrap();
        
                // stop reading
                if last_input.len() == 0 {
                    break;
                }
        
                // add a new line once user_input starts storing user input
                if buffer.len() > 0 {
                    buffer.push_str("\n");
                }
        
                // store user input
                buffer.push_str(&last_input);
            }            return Ok(buffer);
        },
        Err(e) => {return Err(e.to_string())},
    };
}