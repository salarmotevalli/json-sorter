use super::parser;

pub fn registered_flags_count() -> u8 {
    unsafe { parser::FLAGS.len() as u8 }
}

pub fn passed_flags_count() -> u8 {
     match parser::get_args() {
        Some(v) => v.len() as u8,
        None => 0,
     }
}