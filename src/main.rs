mod chunk;
mod value;

use chunk::{Chunk, Op};

fn main() {
    let mut chunk = Chunk::new();
    let c1 = chunk.add_constant(1.2);
    chunk.add_op(Op::Constant(c1), 1);
    chunk.add_op(Op::Return.into(), 2);
    println!("{}", chunk);
}
