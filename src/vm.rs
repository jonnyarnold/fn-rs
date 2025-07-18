use crate::chunk::{Chunk, Op};
use crate::value::Value;

pub enum InterpretResult {
    OK,
    // CompileError,
    // RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    op_pointer: usize,
    stack: Vec<Value>,
    stack_top: usize,
}

const DEBUG_EXECUTION: bool = true;

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            op_pointer: 0,
            stack: vec![],
            stack_top: 0,
        }
    }

    pub fn run(mut self) -> InterpretResult {
        loop {
            if DEBUG_EXECUTION {
                print!("≡");
                for value in self.stack.iter() {
                    print!(" {}", value);
                }
                println!("\n▶ {}", self.chunk.ops[self.op_pointer]);
            }

            match self.chunk.ops[self.op_pointer] {
                Op::Constant(i) => {
                    self.push_value(self.chunk.get_constant(i));
                }
                Op::Negate => {
                    let value = -1.0 * self.pop_value();
                    self.push_value(value);
                }
                Op::Add => {
                    let right = self.pop_value();
                    let left = self.pop_value();
                    let result = left + right;
                    self.push_value(result);
                }
                Op::Subtract => {
                    let right = self.pop_value();
                    let left = self.pop_value();
                    let result = left - right;
                    self.push_value(result);
                }
                Op::Multiply => {
                    let right = self.pop_value();
                    let left = self.pop_value();
                    let result = left * right;
                    self.push_value(result);
                }
                Op::Divide => {
                    let right = self.pop_value();
                    let left = self.pop_value();
                    let result = left / right;
                    self.push_value(result);
                }
                Op::Return => {
                    println!("{}", self.pop_value());
                    return InterpretResult::OK;
                }
            }

            self.op_pointer += 1;
        }
    }

    fn push_value(&mut self, value: Value) -> () {
        self.stack.push(value);
        self.stack_top += 1;
    }

    fn pop_value(&mut self) -> Value {
        self.stack_top -= 1;
        return self
            .stack
            .pop()
            .expect("pop_value() called on empty stack!");
    }
}
