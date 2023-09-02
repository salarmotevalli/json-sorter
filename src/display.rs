use text_colorizer::Colorize;

#[allow(dead_code)]
pub fn hello() {
    println!("{}\n", "Sort every thing you need ;)".purple());
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
