use basm::Parser;
use std::fs;

fn main() {
    let mut parser = Parser::new();
    let asm_text = fs::read_to_string("test.z80")
        .expect("Could not read the file, make sure it exists.");
    println!("{:?}", parser.parse_full(&asm_text));
}
