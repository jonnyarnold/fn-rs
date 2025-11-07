use crate::token::Token;

pub struct Scanner<'s> {
    code: &'s str,
    head: CodePoint,
}

impl<'s> Scanner<'s> {
    pub fn new(code: &str) -> Scanner {
        Scanner {
            code,
            head: CodePoint { line: 1, char: 1 },
        }
    }
}

impl<'s> Iterator for Scanner<'s> {
    type Item = Token;

    // Returns the next token from the code.
    fn next(&mut self) -> Option<Token> {
        if self.head.char > 1 {
            None
        } else {
            self.head.char += 1;
            Some(Token::Number(1))
        }
    }
}

struct CodePoint {
    pub line: usize,
    pub char: usize,
}
