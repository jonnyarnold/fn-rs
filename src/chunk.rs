use crate::value::Value;
use std::fmt::{Display, Formatter};

pub enum Op {
    Constant(usize),
    Return,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(i) => write!(f, "CONST {}", i),
            Self::Return => write!(f, "RETURN"),
        }
    }
}

pub struct Chunk {
    ops: Vec<Op>,
    constants: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            ops: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn add_op(&mut self, op: Op, line: usize) -> () {
        self.ops.push(op);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        let new_constant_index = self.constants.len();
        self.constants.push(value);
        return new_constant_index;
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut op_index = 0;

        write!(f, "LN#  OP#  OP\n")?;

        for op in self.ops.iter() {
            if op_index > 0 && self.lines[op_index] == self.lines[op_index - 1] {
                write!(f, "   | ")?;
            } else {
                write!(f, "{:04} ", self.lines[op_index])?;
            }

            write!(f, "{:04} {}", op_index, op)?;

            match op {
                Op::Constant(i) => write!(f, " [{}]", self.constants[*i])?,
                _ => {}
            };
            write!(f, "\n")?;

            op_index += 1;
        }

        Ok(())
    }
}
