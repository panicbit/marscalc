use std::ops;
use std::mem::swap;
use super::Number;

impl ops::SubAssign for Number {
    fn sub_assign(&mut self, mut other: Self) {
        if self.is_positive != other.is_positive {
            return *self += -other;
        }
        
        let swapped = self.abs_smaller_than(&other);

        if swapped {
            swap(self, &mut other);
        }

        self.equalize_with(&mut other);

        let digits_a = self.digits.iter_mut().rev();
        let digits_b = other.digits.iter_mut().rev();

        let mut carry = 0;
        for (a, b) in digits_a.zip(digits_b) {
            let b = *b + carry;

            if *a >= b {
                carry = 0;
            } else {
                carry = 1;
                *a += 10;
            }
            
            *a -= b;
        }

        self.prepend_int(carry);
        self.normalize();

        if swapped {
            self.negate();
        }
    }
}

impl ops::Sub for Number {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        self -= other;
        self
    }
}
