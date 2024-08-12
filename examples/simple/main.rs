fn main() {
  let mut parser = es51986::parser::Parser::new();
  let input: Vec<u8> = "00000;<0:\r\n".chars().map(|c| c as u8).collect();
  
  for result in parser.parse(&input) {
    match result {
      Ok(parsed_output) => println!("Parsed output: {:?}", parsed_output),
      Err(err) => println!("Parse error: {:?}", err,)
    }
  }
}
