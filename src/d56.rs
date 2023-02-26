use std::ops::Add;

use ux::i48;

// A 7-byte version of DEC64:
// https://www.crockford.com/dec64.html
struct d56 {
    coefficient: i48,
    exponent: i8
}

impl Add<d56> for d56 {
    type Output = d56;

    fn add(self, rhs: d56) -> d56 {
        if (self.exponent == rhs.exponent) {
            return d56 { coefficient: self.coefficient + rhs.coefficient, exponent: self.exponent }
        } else {
            // ...
        }
    }
}
