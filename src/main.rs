mod bytecode;
mod instruction;
mod value;
mod vm;

use bytecode::BytecodeBuilder;
use instruction::Instruction;
use value::Value;
use vm::exec;

fn main() {
    let mut builder = BytecodeBuilder::new();
    let r1 = builder.add_register(Value::Bool(false));
    let r2 = builder.add_register(Value::Bool(true));

    builder.add_instruction(Instruction::Or(r1, r2, r1));
    builder.add_instruction(Instruction::Return(r1));

    let bytecode = builder.into_bytes();
    println!("{:#?}", bytecode);

    let value = exec(&bytecode);
    println!("{}", value);
}
