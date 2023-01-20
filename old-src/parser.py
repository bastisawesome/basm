import re
import sys
# from typing import Union


# ***************
# ****HELPERS****
# ***************
def create_bytes_object(number: int) -> bytes:
    return bytes(number)


'''
The following two functions (encoding registers/pairs) are made possible by:
http://www.z80.info/decoding.htm
The use of tables and diagrams on this website allowed me to properly encode
instructions in a simplified manner where registers and pairs, and raw data are
used. Without it, there would be a lot more logic in place to account for every
possibility.
Feel free to read the above-linked website to get an understanding of the logic
used.
'''


def encode_double_register(reg: str,
                           features_sp: bool = True) -> tuple[int, bytes]:
    '''Encodes register pairs into basic Z80 logic.

    `encode_double_register` uses the above information to form a basic
    encoding of register pairs to the Z80's hardware logic.

    :param reg: The particular register pair passed to be parsed.
    :type reg: str
    :param features_sp: Determines if the generated table should consist of
        SP or AF. When false, will parse for AF.
    :type features_sp: bool

    :return: Returns the encoded form of the register pair and,
            if applicable, the prefix used by that register pair.
        Can also return None if the passed register is not a pair or is an
            invalid pair.
    :rtype: tuple[bytes, bytes] or None
    '''

    '''
    The double encoding map is formatted as follows:
    {
        Register Pair: [Encoding bytes], [Prefix or None],
        ...
    }
    '''
    double_encoding_map: dict[str, tuple[int, bytes]] = {
        'BC': (0, b''),
        'DE': (1, b''),
        'HL': (2, b''),
        'IX': (2, b'0xDD'),
        'IY': (2, b'0xFD')
    }
    if features_sp:
        double_encoding_map['SP'] = (3, b'')
    else:
        double_encoding_map['AF'] = (3, b'')

    register_pair = double_encoding_map.get(reg.strip().upper(), (-1, b''))

    return register_pair


def encode_single_register(reg: str) -> tuple[int, bytes]:
    '''Encodes a register into Z80 hardware logic.

    Returns the Z80 encoding of the passed register, as long as an optional
    prefix for the register.

    :param reg: Register to encode.
    :type reg: str
    :return: The binary encoding of the register and the prefix, if applicable.
    :rtype: tuple[int, bytes]
    '''
    single_encoding_map: dict[str, tuple[int, bytes]] = {
        'B': (0, b''),
        'C': (1, b''),
        'D': (2, b''),
        'E': (3, b''),
        'H': (4, b''),
        'L': (5, b''),
        '(HL)': (6, b''),
        'A': (7, b''),
        'IXH': (4, b'DD'),
        'IXL': (5, b'DD'),
        'IYH': (4, b'FD'),
        'IYL': (4, b'FD'),
        'I': (0, b'ED'),
        'R': (1, b'ED')
    }

    register = single_encoding_map.get(reg.strip().upper(), (-1, b''))

    return register


class Parser:
    COMMENT_CHARS = ';'
    RESERVED = [
        # Instructions:
        'NOP', 'LD', 'INC', 'DEC', 'RLCA', 'EX', 'ADD', 'RRCA', 'DJNZ', 'RLA',
        'JR', 'RRA', 'DAA', 'CPL', 'SCF', 'CCF', 'HALT', 'ADC', 'SUB', 'SBC',
        'AND', 'XOR', 'OR', 'CP', 'RET', 'POP', 'CALL', 'PUSH', 'RST', 'OUT',
        'DI', 'EI', 'NEG', 'RETN', 'IM', 'IN', 'RETI', 'RRD', 'RLD', 'LDI',
        'CPI', 'INI', 'OUTI', 'LDD', 'CPD', 'IND', 'OUTD', 'LDIR', 'CPIR',
        'INIR', 'CPIR', 'INIR', 'OTIR', 'LDDR', 'CPDR', 'INDR', 'OTDR', 'RLC',
        'RRC', 'RL', 'RR', 'SLA', 'SRA', 'SLL', 'SRL', 'BIT', 'RES', 'SET',
        'EXX'

        # Jump modifiers:
        'NZ', 'Z', 'NC', 'C', 'PE', 'M', 'PO', 'P',

        # Registers:
        'A', 'B', 'C', 'D', 'H', 'E', 'L', 'F', 'IX', 'IY', 'I', 'R', 'SP',
        'PC', 'AF', 'BC', 'DE', 'AF\''
    ]

    def __init__(self):
        # Format for symbols table:
        # (name: str, line_num: uint)
        self.symbols_table = []
        self.lines = []

        # Debug information
        self.filename = ''
        self.line_num = 0
        self.char_num = 0
        self.current_line = ''

    def parse_file(self, filename: str) -> bytearray:
        self.filename = filename
        with open(filename, 'r') as asm_file:
            self.lines = asm_file.readlines()

        self.collect_symbols()
        try:
            asm_data = self.assemble_file()
        except NotImplementedError:
            self.error("Sorry, that's not available at this time. :(")
        except Exception as e:
            self.error(str(e))

        return asm_data

    def collect_symbols(self):
        '''Collects labels and variables from the file.'''
        pass

    def assemble_file(self) -> bytearray:
        '''Assembles the file into bytecode using the symbols table.'''
        bytecode = bytearray()
        while self.line_num < len(self.lines):
            current_line = self.lines[self.line_num]
            self.current_line = current_line

            bytecode += self.parse_line(current_line)

            self.line_num += 1

        print(bytecode)
        return bytecode

    def parse_line(self, line: str) -> bytes:
        is_in_quotes = False
        opcode = ''
        symbol = ''
        quoted = ''
        prev_char = ''
        bytecode = b''
        self.char_num = 0

        for current_char in line:
            self.char_num += 1

            if current_char in self.COMMENT_CHARS and not is_in_quotes:
                break
            if current_char == ':' and not is_in_quotes:
                if opcode:
                    symbol = opcode
                    opcode = ''
                    continue
                else:
                    self.error('Expected label name.')

            if current_char == '"':
                if not is_in_quotes:
                    is_in_quotes = True
                    quoted += current_char
                elif is_in_quotes:
                    is_in_quotes = False
                    quoted += current_char
            elif is_in_quotes:
                quoted += current_char
            else:
                opcode += current_char

            prev_char = current_char

        symbol = symbol.strip()
        opcode = opcode.strip()

        if is_in_quotes:
            self.error("Mismatched quotes.")

        if len(symbol.split()) > 1:
            self.error("Unexcepted whitespace in symbol name.")

        self.char_num = 0

        if opcode:
            bytecode = self.assemble_instruction(opcode)

        return bytecode

    def assemble_instruction(self, opcode: str) -> bytes:
        instr_regex = re.match(r'^(\w+)(.*)', opcode).groups()
        instruction = instr_regex[0].upper()
        args = instr_regex[1].strip() if len(instr_regex) > 1 else ''

        asm_func_name = f'op_{instruction}'
        func = getattr(self, asm_func_name, None)
        if not func:
            self.error(f'Unrecognised or unsupported opcode: {instruction}')

        return func(args)

    def verify_args(self, args: str, expected_num: int) -> bool:
        num_args = len(args.split(',')) if args else 0

        if num_args != expected_num:
            self.error(f'Expected {expected_num} arguments, found {num_args}.')

        return True

    def op_NOP(self, args: str) -> bytes:
        self.verify_args(args, 0)
        return create_bytes_object(0x00)

    def op_LD(self, args: str) -> bytes:
        self.verify_args(args, 2)
        dest, src = args.split(',', 1)

        dest_reg, dest_prefix = encode_double_register(dest)

        if dest_reg == -1:
            # Passed register is not a pair.
            # Check if it's a single register (or memory address)
            dest_reg, dest_prefix = encode_single_register(dest)

        if dest_reg == -1:
            # Not a register, so maybe it's a memory address
            raise NotImplementedError()

        print(dest_reg)
        print(dest_prefix)

        src_reg, src_prefix = encode_double_register(src)

        if src_reg == -1:
            # Pass register is not a pair.
            # Check if it's a single register (or memory address)
            src_reg, src_prefix = encode_single_register(src)

        if src_reg == -1:
            # Not a register, so maybe it's a memory address
            # Or maybe it's an immediate value
            raise NotImplementedError()

        # Error handling for special cases.
        # For example: IX(H/L)/IY(H/L) in one instruction or
        # LD (HL), (HL), which encodes into a HALT

        # LD (HL), (HL)
        if ((dest_reg == 6 and src_reg == 6) or
                # IX/IY with (HL)
                (src_prefix and src_reg == 6) or
                (dest_prefix and src_reg == 6) or
                (src_prefix and dest_reg == 6) or
                (dest_prefix and dest_reg == 6)):
            self.error("Invalid combination of operands.")

        return b''

    def error(self, message: str):
        print(f'Error in file {self.filename} line {self.line_num}: {message}')
        print(f'{self.current_line.strip()}')
        print(' '*(self.char_num-2) + '^')
        sys.exit(1)
