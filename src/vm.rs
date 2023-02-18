use crate::instruction::RegisterIndex;

use super::value::{Value, VALUE_BYTES};
use super::instruction::{Instruction, INSTRUCTION_BYTES};

struct VM {
    registers: Vec<Value>,
    instructions: Vec<Instruction>,
}

impl VM {
    fn run(mut self) -> Value {
        let mut instruction_iter = self.instructions.iter();

        loop {
            match instruction_iter.next() {
                Some(Instruction::And(r1, r2, r3)) => {
                    println!("AND {} {} -> {}", r1, r2, r3);
                    let b1 = self.get_bool(*r1);
                    let b2 = self.get_bool(*r2);
                    self.registers[*r3 as usize] = Value::Bool(b1 && b2);
                },
                Some(Instruction::Or(r1, r2, r3)) => {
                    println!("OR {} {} -> {}", r1, r2, r3);
                    let b1 = self.get_bool(*r1);
                    let b2 = self.get_bool(*r2);
                    self.registers[*r3 as usize] = Value::Bool(b1 || b2);
                },
                Some(Instruction::Return(r1)) => {
                    println!("RETURN {}", r1);
                    break self.registers[(*r1) as usize]
                },
                None => panic!("No more instructions!")
            }
        }
    }

    fn get_bool(&self, index: RegisterIndex) -> bool {
        match self.registers[index as usize] {
            Value::Bool(b) => b,
            _ => panic!("Not a bool!")
        }
    }
}

pub fn exec(bytecode: &[u8]) -> Value {
    let mut bytecode_iterator = bytecode.iter();

    let num_registers = *bytecode_iterator.next().unwrap();
    println!("{} registers:", num_registers);
    let mut registers: Vec<Value> = Vec::with_capacity(num_registers.into());
    for r in 0..num_registers {
        registers.push(Value::from_bytes(bytecode_iterator.as_slice()));
        println!("  r{} = {}", r, registers[r as usize]);
        
        // TODO: advance_by?
        for _ in 0..VALUE_BYTES {
            bytecode_iterator.next();
        }
    }

    let num_instructions = *bytecode_iterator.next().unwrap();
    println!("{} instructions:", num_instructions);
    let mut instructions: Vec<Instruction> = Vec::with_capacity(num_instructions.into());
    for i in 0..num_instructions {
        instructions.push(Instruction::from_bytes(bytecode_iterator.as_slice()));
        println!("  {}", instructions[i as usize]);

        // TODO: advance_by?
        for _ in 0..INSTRUCTION_BYTES {
            bytecode_iterator.next();
        }
    }

    // Make sure we've used all the bytes.
    if let Some(_) = bytecode_iterator.next() {
        panic!("Too many bytes!")
    };

    let vm = VM { registers, instructions };
    return vm.run();
}
