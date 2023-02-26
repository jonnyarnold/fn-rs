use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
pub enum Value {
    Bool(bool),
    Integer(i32), // TODO: General number
}

const TYPE_BYTE_BOOL: u8 = 0x01;
const TYPE_BYTE_INTEGER: u8 = 0x02;

pub const VALUE_BYTES: usize = 8;

impl Value {
    // https://stackoverflow.com/a/74237171
    pub fn into_bytes(self) -> [u8; VALUE_BYTES] {
        match self {
            Value::Bool(b) => [
                TYPE_BYTE_BOOL,
                if b { 1 } else { 0 },
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
            ],
            Value::Integer(i) => {
                // [TYPE_BYTE_INTEGER, ...i.to_be_bytes(), 0x00, 0x00, 0x00]
                let mut v = Vec::with_capacity(8);
                v.push(TYPE_BYTE_INTEGER);
                v.extend(i.to_be_bytes());
                v.extend([0x00, 0x00, 0x00]);
                return v.try_into().unwrap();
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Value {
        match bytes[0] {
            TYPE_BYTE_BOOL => Value::Bool(bytes[1] == 1),
            TYPE_BYTE_INTEGER => Value::Integer(i32::from_be_bytes(bytes[1..5].try_into().unwrap())),
            _ => panic!("Unexpected Type Byte"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
        }
    }
}
