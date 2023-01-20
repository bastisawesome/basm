use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
enum OpCode {
    Nop,
    Adc,
    Add,
    And,
    Bit,
    Call,
    Ccf,
    Cp,
    Cpd,
    Cpdr,
    Cpi,
    Cpir,
    Cpl,
    Daa,
    Dec,
    Di,
    Djnz,
    Ei,
    Ex,
    Exx,
    Halt,
    Im,
    In,
    Inc,
    Ind,
    Indr,
    Ini,
    Inir,
    Jp,
    Jr,
    Ld,
    Ldd,
    Lddr,
    Ldi,
    Ldir,
    Neg,
    Or,
    Otdr,
    Otir,
    Out,
    Outd,
    Outi,
    Pop,
    Push,
    Res,
    Ret,
    Reti,
    Retn,
    Rl,
    Rla,
    Rlc,
    Rlca,
    Rld,
    Rr,
    Rra,
    Rrc,
    Rrca,
    Rrd,
    Rst,
    Sbc,
    Scf,
    Set,
    Sla,
    Sll,
    Sra,
    Srl,
    Sub,
    Xor,
}

impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum Token {
    None,
    Invalid,
    OpCode(OpCode),
}

impl Token {
    fn from_str(token: String) -> Token {
        if let Ok(op) = OpCode::from_str(&token) {
            return Token::OpCode(op);
        }

        Token::Invalid
    }
}
