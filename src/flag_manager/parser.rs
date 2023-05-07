use std::{env};

pub struct Flag {
    // Define the name of flag
    identifire: String,

    // Defalut value of the flag
    value: Option<String>,
}

static mut FLAGS: Vec<Flag> = vec![];

fn get_args() -> Option<Vec<String>> {
    // Get args
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        return None;
    }

    Some(args)
}

fn get_arg(none_value: &String) -> Option<String> {
    // TODO
    let args = get_args()?;
    let arg = args.into_iter().filter(|a| a.contains(none_value)).collect();
    Some(arg)
}

fn falgs_value(flag: &String) -> Option<String> {
    let equal_mark: String = String::from("=");

    // Concatenate flag and "="
    let none_value: String = format!("{}{}", flag, equal_mark);

    // find arg by none_value content
    let whole_arg: String = get_arg(&none_value)?;
    
    // Remove "=" and flag's identifire from argument
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

pub fn new(identifire: &str, default: Option<&str>) -> Option<String> {
    // Make new flag
    let new_flag: Flag = Flag{
        identifire: identifire.to_string(),

        // Check default value if defined, set it as String type 
        value: match default {
            Some(v) => {
                let value = v.to_string();
                Some(value)
            }
            None => None,
        },
    };

    // TODO: refactor
    unsafe {
        // Push to flags
        push_to_flags(new_flag);
    }

    match falgs_value(&identifire.to_string()) {
        None => None,
        Some(v) => Some(v)
    }
}

