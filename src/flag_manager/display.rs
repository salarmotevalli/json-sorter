use text_colorizer::Colorize;

pub fn hello() {
    println!("Hello world");
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