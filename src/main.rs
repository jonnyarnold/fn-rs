mod chunk;
mod value;
mod vm;

use chunk::{Chunk, Op};
use vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    let c1 = chunk.add_constant(1.2);
    chunk.add_op(Op::Constant(c1), 1);
    chunk.add_op(Op::Return.into(), 2);
    println!("{}", chunk);

    let vm = VM::new(chunk);
    vm.run();
}
