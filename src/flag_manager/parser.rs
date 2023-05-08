use std::env;

#[derive(Clone)]
#[derive(Debug)]
pub struct Flag {
    // Define the name of flag
    pub identifier: String,

    // Defalut value of the flag
    value: Option<String>,

    // Flag's description
    pub description: Option<String>,
}

pub static mut FLAGS: Vec<Flag> = vec![];

pub fn get_args() -> Vec<String> {
    // Return args
    env::args().skip(1).collect()
}

fn get_arg(none_value: &String) -> Option<String> {
    // -------------------------------------------
    // This function return the whole argument:
    // ./app -i=input.json
    // the function returns "-i=input.json"
    // -------------------------------------------

    let args = get_args();
    // Get the element that contains flag identifier
    let arg = args
        .into_iter()
        .filter(|a| a.contains(none_value))
        .collect();
    Some(arg)
}

fn falgs_value(flag: String) -> Option<String> {
    // -------------------------------------------
    // This function remove the identifier and
    // equal mark from argument and returns it as
    // the value of flag
    // -------------------------------------------

    let equal_mark: String = String::from("=");

    // Concatenate flag and "="
    let none_value: String = format!("{}{}", flag, equal_mark);

    // find arg by none_value content
    let whole_arg: String = get_arg(&none_value)?;

    // Remove "=" and flag's identifier from argument
    let value: String = whole_arg.replace(&none_value, "");

    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}

unsafe fn push_to_flags(flag: Flag) {
    FLAGS.push(flag)
}

fn new_falg(identifier: &str, default: Option<&str>, description: Option<&str>) -> Flag {
    Flag {
        identifier: identifier.to_string(),

        // Check default value if defined, set it as String type
        value: match default {
            Some(v) => Some(v.to_string()),
            None => None,
        },

        description: match description {
            Some(v) => Some(v.to_string()),
            None => None,
        },
    }
}

pub fn new(identifier: &str, default: Option<&str>, description: Option<&str>) -> Option<String> {
    // Make new flag
    let new_flag: Flag = new_falg(identifier, default, description);

    unsafe {
        // Push to flags
        push_to_flags(new_flag.clone());
    }

    match falgs_value(new_flag.identifier) {
        None => new_flag.value,
        Some(v) => Some(v),
    }
}
