mod chunk;

use chunk::{Chunk, Op};

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(Op::Return.into());
    println!("{}", chunk);
}
