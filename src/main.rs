mod scanner;
mod token;

use crate::scanner::Scanner;

fn main() {
    for token in Scanner::new("1") {
        println!("{:?}", token);
    }
}
