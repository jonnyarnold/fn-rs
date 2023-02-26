extern crate ux;

mod bytecode;
mod d56;
mod instruction;
mod value;
mod vm;

use bytecode::BytecodeBuilder;
use instruction::Instruction;
use value::Value;
use vm::exec;

fn main() {
    let mut builder = BytecodeBuilder::new();
    let r1 = builder.add_register(Value::Integer(1));

    builder.add_instruction(Instruction::Negate(r1, r1));
    builder.add_instruction(Instruction::Return(r1));

    let bytecode = builder.into_bytes();

    let value = exec(&bytecode);
    println!("{}", value);
}
