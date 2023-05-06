use std::{env, process};

pub struct Flag {
    // Define the name of flag
    identifire: String,

    // Defalut value of the flag
    value: Option<String>,
}

static mut FLAGS: Vec<Flag> = vec![];
static ENTRY: Vec<String> = get_args();
    
unsafe fn push_to_flags(flag: Flag) {
    FLAGS.push(flag)
}

pub fn new(identifire: &str, default: Option<&str>) {
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

    unsafe {
        // Push to flags
        push_to_flags(new_flag);
    } 
}

// fn display_flags() {
    // println!("These are flags")
// }

fn get_args() -> Vec<String> {
    // Get args
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        eprint!("fuck");
        process::exit(1);
    }

    args
}

