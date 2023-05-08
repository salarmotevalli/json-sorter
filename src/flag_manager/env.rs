use super::parser;

// pub fn registered_flags_count() -> u8 {
//     unsafe { parser::FLAGS.len() as u8 }
// }

pub fn passed_flags_count() -> u8 {
      parser::get_args().len() as u8        
}