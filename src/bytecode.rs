use super::value::Value;
use super::instruction::{Instruction,RegisterIndex};

pub struct BytecodeBuilder {
    registers: Vec<Value>,
    instructions: Vec<Instruction>,
}

impl BytecodeBuilder {
    pub fn new() -> BytecodeBuilder {
        BytecodeBuilder { registers: Vec::new(), instructions: Vec::new() }
    }

    pub fn add_register(&mut self, initial_value: Value) -> RegisterIndex {
        let register_index = self.registers.len();
        self.registers.push(initial_value);
        return register_index.try_into().unwrap();
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        
        bytes.push(self.registers.len().try_into().unwrap());
        for initial_value in self.registers {
            bytes.append(&mut Vec::from(initial_value.into_bytes()));
        }

        bytes.push(self.instructions.len().try_into().unwrap());
        for instruction in self.instructions {
            bytes.append(&mut Vec::from(instruction.into_bytes()));
        }

        return bytes;
    }
}
