// use std::fs::metadata;
// use std::os::unix::io::AsRawFd;
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

use std::process;

mod flag_manager;

fn main() {
    // Parse flags
    parse_flags();
    
    // Validate entry  
    if flag_manager::env::passed_flags_count() == 0 {
        flag_manager::display::hello();
        flag_manager::display::usage();
        flag_manager::display::flags();
        process::exit(1);
    }


}

fn parse_flags() {
    let _ = flag_manager::parser::new("-i", None, Some("Define input file"));
    let _ = flag_manager::parser::new("-o", None, Some("Define output file"));
    let _ = flag_manager::parser::new("-h", None, Some("Help!"));
}