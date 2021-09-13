// use std::error::Error;
// use std::ptr::null_mut;
// use std::io::Bytes;
use std::vec;
use std::fmt;
use std::boxed::Box;

#[derive(Debug)]
pub enum ParserError {
    NotImplementedError
}

impl std::error::Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::NotImplementedError => write!(f, "Sorry, that's not available at this time. :(")
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    symbols_table: Vec<String>,
    lines: Vec<String>,
    line_num: usize,
    char_num: usize,
    current_line: Box<String>
}

impl Parser {
    const COMMENT_CHARS: &'static [char] = &[';'];
    const RESERVED: &'static [&'static str] = &[
        // Instructions:
        "NOP", "LD", "INC", "DEC", "RLCA", "EX", "ADD", "RRCA", "DJNZ", "RLA",
        "JR", "RRA", "DAA", "CPL", "SCF", "CCF", "HALT", "ADC", "SUB", "SBC",
        "AND", "XOR", "OR", "CP", "RET", "POP", "CALL", "PUSH", "RST", "OUT",
        "DI", "EI", "NEG", "RETN", "IM", "IN", "RETI", "RRD", "RLD", "LDI",
        "CPI", "INI", "OUTI", "LDD", "CPD", "IND", "OUTD", "LDIR", "CPIR",
        "INIR", "CPIR", "INIR", "OTIR", "LDDR", "CPDR", "INDR", "OTDR", "RLC",
        "RRC", "RL", "RR", "SLA", "SRA", "SLL", "SRL", "BIT", "RES", "SET",
        "EXX",

        // Jump modifiers:
        "NZ", "Z", "NC", "C", "PE", "M", "PO", "P",

        // Registers:
        "A", "B", "C", "D", "H", "E", "L", "F", "IX", "IY", "I", "R", "SP",
        "PC", "AF", "BC", "DE", "AF'"
    ];

    pub fn new() -> Self {
        Parser {
            symbols_table: Vec::new(),
            lines: Vec::new(),
            line_num: 0,
            char_num: 0,
            current_line: Box::from(String::from(""))
        }
    }

    /// Parses the full contents of the passed string.
    pub fn parse_full(&mut self, asm_text: &str) -> Vec<u8> {
        // self.lines (asm_text.split_whitespace());
        self.lines.extend(
            asm_text.lines()
            .map(|s| String::from(s))
            );

        // self.collect_symbols();

        let asm_data = match self.assemble_full() {
            Ok(asm_data) => { return asm_data; },
            Err(err) => { self.error(err); }
        };
    }

    fn collect_symbols(&mut self) -> Result<(), ParserError> {
        todo!();
    }

    fn assemble_full(&mut self) -> Result<Vec<u8>, ParserError> {
        let mut asm_data: Vec<u8> = Vec::new();
        while self.line_num < self.lines.len() {
            self.current_line = Box::from(String::from(self.lines.get(self.line_num).expect("Unexpected EOF")));

            let data = self.parse_line()?;
            asm_data.extend(data);

            self.line_num += 1;
        }

        Ok(asm_data)
    }

    fn parse_line(&mut self) -> Result<Vec<u8>, ParserError> {
        Err(ParserError::NotImplementedError)
    }

    fn error(&self, error: ParserError) -> ! {
        println!("Error on line {}: {}", self.line_num, error);
        println!("{}", self.current_line);
        println!("{}", format!("{:-^1$}", '^', self.char_num));

        panic!();
    }
}

// Helper functions
fn encode_double_register(reg: &str, features_sp: bool) -> Result<(u8, &[u8]), ParserError> {
    todo!();
}

fn encode_single_register(reg: &str) -> Result<(u8, &[u8]), ParserError> {
    todo!();
}
