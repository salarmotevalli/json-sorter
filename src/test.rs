#[test]
fn test_put_out_result() {
    // Create a mock writer that writes to a Vec<u8>
    let mut mock_writer: Vec<u8> = Vec::new();

    // Call the put_out_result function with a test vector of bytes
    super::put_out_result(&mut mock_writer, String::from("salar")).expect("fuckkkk");

    // Check that the bytes were correctly written to the mock writer
    assert_eq!(mock_writer, Vec::<u8>::from("salar"));
}
