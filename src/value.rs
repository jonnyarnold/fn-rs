use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
pub enum Value {
    Bool(bool)
}

const TYPE_BYTE_BOOL: u8 = 0x00;

pub const VALUE_BYTES: usize = 2;

impl Value {
    // https://stackoverflow.com/a/74237171
    pub fn into_bytes(self) -> [u8; VALUE_BYTES] {
        match self {
            Value::Bool(b) => [TYPE_BYTE_BOOL, if b { 1 } else { 0 }]
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Value {
        match bytes[0] {
            TYPE_BYTE_BOOL => Value::Bool(bytes[1] == 1),
            _ => panic!("Unexpected Type Byte")
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b)
        }
    }
}
