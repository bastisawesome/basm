// TODO: Remove this line
#![allow(warnings, unused)]

// use std::error::Error;
// use std::ptr::null_mut;
// use std::io::Bytes;
use lazy_static::lazy_static;
use regex::Regex;
use std::vec;
use std::fmt;
use std::boxed::Box;

mod tokens;

#[derive(Debug)]
pub enum ParserError {
    NotImplementedError,
    MissingLabelError,
    MismatchedQuotesError,
    SymbolWhitespaceError,
    InvalidTokenError(String)
}

impl std::error::Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::NotImplementedError => write!(f, "Sorry, that's not available at this time. :("),
            ParserError::MissingLabelError => write!(f, "Expected label name."),
            ParserError::MismatchedQuotesError =>
                write!(f, "Mismatched quotes."),
            ParserError::SymbolWhitespaceError =>
                write!(f, "Unexpected whitespace in symbol name."),
            ParserError::InvalidTokenError(token) =>
                write!(f, "Invalid token: {}", token),
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
        let mut is_in_quotes = false;
        let mut opcode = String::from("");
        let mut symbol = String::from("");
        let mut quoted = String::from("");
        let mut bytecode = Vec::<u8>::new();
        self.char_num = 0;

        for current_char in self.current_line.chars() {
            self.char_num += 1;

            // if Parser::COMMENT_CHARS.contains(&current_char) && !is_in_quotes {
            //     if !opcode.is_empty() {
            //         symbol = opcode;
            //         opcode = "";
            //         continue;
            //     } else {
            //         return Err(ParserError::MissingLabelError);
            //     }
            // }
            if Parser::COMMENT_CHARS.contains(&current_char) && !is_in_quotes {
                break;
            }

            if current_char == ':' && !is_in_quotes {
                if !opcode.is_empty() {
                    symbol = opcode;
                    opcode = String::from("");
                    continue;
                } else {
                    return Err(ParserError::MissingLabelError);
                }
            }

            if current_char == '"' {
                if !is_in_quotes {
                    is_in_quotes = true;
                    quoted.push(current_char);
                } else if is_in_quotes {
                    is_in_quotes = false;
                    quoted.push(current_char);
                }
            } else if is_in_quotes {
                quoted.push(current_char);
            } else {
                opcode.push(current_char);
            }
        }

        let symbol = symbol.trim();
        let opcode = opcode.trim();

        if is_in_quotes {
            return Err(ParserError::MismatchedQuotesError);
        }

        if symbol.split_ascii_whitespace().count() > 1 {
            return Err(ParserError::SymbolWhitespaceError);
        }

        if !opcode.is_empty() {
            bytecode = self.assemble_instruction(opcode)?;
        }

        self.char_num = 0;

        Ok(bytecode)

        // Err(ParserError::NotImplementedError)
    }

    fn assemble_instruction(&self, opcode: &str) -> Result<Vec<u8>, ParserError> {
        lazy_static! {
            static ref re: Regex = Regex::new(r"^(\w+)(.*)").unwrap();
        }
        let mut instr_regex = re.captures(opcode).unwrap();
        let mut instruction = instr_regex.get(1).map(|x| x.as_str()).unwrap();
        let mut args = instr_regex.get(2).map_or("", |x| x.as_str());

        println!("Instruction: {}", instruction);
        println!("Args: {}", args);

        Err(ParserError::NotImplementedError)
    }

    fn error(&self, error: ParserError) -> ! {
        println!("Error on line {}: {}", self.line_num, error);
        println!("{}", self.current_line);
        println!("{}", format!("{:^1$}", '^', self.char_num));

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
