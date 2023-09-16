use super::*;
use std::io::Write;
use std::path::Path;

#[test]
fn test_define_writer_with_file() {
    // Define the expected output file name
    let output_file_name = "output.txt";
    let test_message = "This is a test message";

    // Call the define_writer function
    let mut writer = define_writer(Some(output_file_name.into()));
    write!(writer, "{}", test_message).expect("Failed to write to file");

    // Ensure that the output file was created and contains the test message
    let output_file_path = Path::new(output_file_name);
    assert!(output_file_path.exists());
    let output_file_contents = std::fs::read_to_string(output_file_path).unwrap();
    assert_eq!(output_file_contents, test_message);

    // Clean up the test environment by deleting the output file
    std::fs::remove_file(output_file_path).unwrap();
}

#[test]
fn test_define_reader_with_file_path() {
    let file_path = "test.txt";

    let mut file = File::create(file_path).unwrap();
    file.write_all(b"Hello, world!").unwrap();
    let mut reader = define_reader(Some(file_path.to_string()));
    let mut buf = String::new();
    reader.read_to_string(&mut buf).unwrap();

    assert_eq!(buf, "Hello, world!");

    // Clean up the test environment by deleting the output file
    std::fs::remove_file(file_path).unwrap();
}
