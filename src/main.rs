mod chunk;
mod value;
mod vm;

use chunk::{Chunk, Op};
use vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    let c1 = chunk.add_constant(1.2);
    chunk.add_op(Op::Constant(c1), 1);

    let c2 = chunk.add_constant(3.4);
    chunk.add_op(Op::Constant(c2), 1);

    chunk.add_op(Op::Add, 1);

    let c3 = chunk.add_constant(5.6);
    chunk.add_op(Op::Constant(c3), 1);

    chunk.add_op(Op::Divide, 1);
    chunk.add_op(Op::Negate, 1);
    chunk.add_op(Op::Return, 1);

    println!("{}", chunk);

    let vm = VM::new(chunk);
    vm.run();
}
