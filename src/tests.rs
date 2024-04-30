// You can also organize tests into modules
mod tests {
  // Import the test module to access the #[test] attribute
  use crate::libs::stream_handler::read_until_crlf;

  // Write more tests
  #[test]
  fn test_read_until_crlf() {
    let expected = "2";
    let input = String::from("*2\r\n$4\r\necho\r\n$3\r\nhey\r\n");

    let output = read_until_crlf(input);

    let output_value = output.unwrap();

    println!("output: {:?}", output_value);

    assert_eq!(output_value, expected);
  }
}