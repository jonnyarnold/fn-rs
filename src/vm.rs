use crate::instruction::RegisterIndex;

use super::instruction::{Instruction, INSTRUCTION_BYTES};
use super::value::{Value, VALUE_BYTES};

struct VM {
    registers: Vec<Value>,
    instructions: Vec<Instruction>,
}

impl VM {
    fn run(mut self) -> Value {
        let mut instruction_iter = self.instructions.iter();

        loop {
            match instruction_iter.next() {
                Some(Instruction::Not(r1, r2)) => {
                    let b1 = self.get_bool(*r1);
                    let b2 = Value::Bool(!b1);
                    self.registers[*r2 as usize] = b2;
                    println!("NOT({} <- r{}) = {} -> r{}", b1, *r1, b2, *r2);
                }
                Some(Instruction::And(r1, r2, r3)) => {
                    let b1 = self.get_bool(*r1);
                    let b2 = self.get_bool(*r2);
                    let b3 = Value::Bool(b1 && b2);
                    self.registers[*r3 as usize] = b3;
                    println!(
                        "AND({} <- r{}, {} <- r{}) = {} -> r{}",
                        b1, *r1, b2, *r2, b3, *r3
                    );
                }
                Some(Instruction::Or(r1, r2, r3)) => {
                    let b1 = self.get_bool(*r1);
                    let b2 = self.get_bool(*r2);
                    let b3 = Value::Bool(b1 || b2);
                    self.registers[*r3 as usize] = b3;
                    println!(
                        "OR({} <- r{}, {} <- r{}) = {} -> r{}",
                        b1, *r1, b2, *r2, b3, *r3
                    );
                }
                Some(Instruction::Return(r1)) => {
                    let v1 = self.registers[(*r1) as usize];
                    println!("RETURN({} <- r{})", v1, *r1);
                    break v1;
                }
                None => panic!("No more instructions!"),
            }
        }
    }

    fn get_bool(&self, index: RegisterIndex) -> bool {
        match self.registers[index as usize] {
            Value::Bool(b) => b,
            _ => panic!("Not a bool!"),
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

    let vm = VM {
        registers,
        instructions,
    };
    return vm.run();
}
