use std::ops;
use super::{Number, BASE};

impl ops::AddAssign for Number {
    fn add_assign(&mut self, mut other: Self) {
        if self.is_positive != other.is_positive {
            return *self -= -other;
        }

        self.equalize_with(&mut other);
        
        let digits_a = self.digits.iter_mut().rev();
        let digits_b = other.digits.iter_mut().rev();

        let mut carry = 0;

        for (a, b) in digits_a.zip(digits_b) {
            let sum = *a + *b + carry;

            *a = sum.abs() % BASE;
            carry = sum / BASE;
        }

        self.prepend_int(carry.abs());
        self.normalize();
    }
}

impl ops::Add for Number {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}
