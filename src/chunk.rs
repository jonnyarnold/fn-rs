use std::fmt::{Display, Formatter};

pub enum Op {
    Return,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Return => write!(f, "RETURN"),
        }
    }
}

pub struct Chunk {
    ops: Vec<Op>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { ops: vec![] }
    }

    pub fn write(&mut self, op: Op) -> () {
        self.ops.push(op);
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut offset = 0;

        for op in self.ops.iter() {
            write!(f, "{:04}: {}\n", offset, op)?;
            offset += 1;
        }

        Ok(())
    }
}
