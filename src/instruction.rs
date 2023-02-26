use std::fmt::{Display, Formatter, Result};

pub type RegisterIndex = u8;

#[derive(Clone, Copy)]
pub enum Instruction {
    // Boolean
    Not(RegisterIndex, RegisterIndex),
    And(RegisterIndex, RegisterIndex, RegisterIndex),
    Or(RegisterIndex, RegisterIndex, RegisterIndex),
    // Number
    Negate(RegisterIndex, RegisterIndex),
    // Add(RegisterIndex, RegisterIndex, RegisterIndex),
    // Subtract(RegisterIndex, RegisterIndex, RegisterIndex),
    // Multiply(RegisterIndex, RegisterIndex, RegisterIndex),
    // Divide(RegisterIndex, RegisterIndex, RegisterIndex),
    // IntegerDivide(RegisterIndex, RegisterIndex, RegisterIndex, RegisterIndex),
    // Function
    Return(RegisterIndex),
}

const OPCODE_NOT: u8 = 0x10;
const OPCODE_AND: u8 = 0x11;
const OPCODE_OR: u8 = 0x12;

const OPCODE_NEGATE: u8 = 0x20;
const OPCODE_ADD: u8 = 0x21;
const OPCODE_SUBTRACT: u8 = 0x22;
const OPCODE_MULTIPLY: u8 = 0x23;
const OPCODE_DIVIDE: u8 = 0x24;
const OPCODE_INTEGER_DIVIDE: u8 = 0x25;

const OPCODE_RETURN: u8 = 0x30;

pub const INSTRUCTION_BYTES: usize = 4;

impl Instruction {
    // https://stackoverflow.com/a/74237171
    pub fn into_bytes(self) -> [u8; INSTRUCTION_BYTES] {
        match self {
            Instruction::Not(r1, r2) => [OPCODE_NOT, r1, r2, 0x00],
            Instruction::And(r1, r2, r3) => [OPCODE_AND, r1, r2, r3],
            Instruction::Or(r1, r2, r3) => [OPCODE_OR, r1, r2, r3],
            Instruction::Negate(r1, r2) => [OPCODE_NEGATE, r1, r2, 0x00],
            Instruction::Return(r1) => [OPCODE_RETURN, r1, 0x00, 0x00],
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Instruction {
        match bytes[0] {
            OPCODE_NOT => Instruction::Not(bytes[1], bytes[2]),
            OPCODE_AND => Instruction::And(bytes[1], bytes[2], bytes[3]),
            OPCODE_OR => Instruction::Or(bytes[1], bytes[2], bytes[3]),
            OPCODE_NEGATE => Instruction::Negate(bytes[1], bytes[2]),
            OPCODE_RETURN => Instruction::Return(bytes[1]),
            _ => panic!("Unexpected OpCode"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Instruction::Not(r1, r2) => write!(f, "NOT r{} -> r{}", r1, r2),
            Instruction::And(r1, r2, r3) => write!(f, "AND r{} r{} -> r{}", r1, r2, r3),
            Instruction::Or(r1, r2, r3) => write!(f, "OR r{} r{} -> r{}", r1, r2, r3),
            Instruction::Negate(r1, r2) => write!(f, "NEGATE r{} -> r{}", r1, r2),
            Instruction::Return(r1) => write!(f, "RETURN r{}", r1),
        }
    }
}
