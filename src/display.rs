use text_colorizer::Colorize;

use crate::flag_manager::parser;

#[allow(dead_code)]
fn dot(flag_len: u8) -> String {
    // return dots beetween flag and its description

    let dots_len: u8 = 37;
    let difference: u8 = dots_len - flag_len;

    let mut dots: String = String::new();

    for _ in 0..difference {
        dots.push('.');
    }

    dots
}

#[allow(dead_code)]

pub fn hello() {
    println!("{}\n", "Sort every thing you need ;)".purple());
}

#[allow(dead_code)]
pub fn usage() {
    println!("usage");
}

#[allow(dead_code)]
pub fn flags() {
    unsafe {
        // Loop on all flags and display them
        for f in &parser::FLAGS {
            let flag = f.clone();
            println!(
                "{} {} {}",
                f.identifier.bright_yellow(),
                dot(flag.identifier.len() as u8),
                flag.description.unwrap().bright_blue()
            )
        }
    }
}

#[allow(dead_code)]
pub fn err(message: &str, description: Option<&str>) {
    println!("{} {}", "Error:".red().bold(), message);

    match description {
        None => {}
        Some(v) => {
            println!("{} {}", "description:".black(), v.bright_black())
        }
    };

    println!();
}
