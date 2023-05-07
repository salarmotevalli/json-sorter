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

mod flag_manager;

fn main() {
    let input = flag_manager::parser::new("i", None);
    match input {
        Some(inner) => {println!("Fuck {}", inner)},
        None => {println!("Salar")},
    }
    // flags::parser::parse();
}

