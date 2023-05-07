use text_colorizer::Colorize;
use super::parser;


fn dot(flag_len: u8) ->String {
    let dots_len: u8 = 37;
    let difference: u8 = dots_len - flag_len;

    let mut dots: String = String::new();

    for _ in 0..difference {
        dots.push('.');
    }

    dots
}

pub fn hello() {
    println!("sorter");
}

pub fn usage() {
    println!("usage");
}

pub fn flags() {
    unsafe {
        for f in &parser::FLAGS {
            let flag = f.clone();
            println!("{} {} {}", f.identifier.bright_yellow(), dot(flag.identifier.len() as u8), flag.description.unwrap().bright_blue())
        }

    }
}

pub fn err(message: &str, description: Option<&str>) {
    println!("{} {}", "Error:".red().bold() , message);

    match description {
        None => {},
        Some(v) => {
            println!("{} {}", "description:".black(), v.bright_black())
        }
    };

    println!();
}