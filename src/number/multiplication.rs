use std::ops;
use super::Number;

impl ops::MulAssign for Number {
    fn mul_assign(&mut self, other: Self) {
        let mut result = Number::zero();

        let digits_a = self.digits.iter().copied().rev().enumerate();
        let digits_b = other.digits.iter().copied().rev().enumerate();

        for (pos_a, a) in digits_a {
            for (pos_b, b) in digits_b.clone() {
                let mut product = Number::zero();

                for _ in 0 .. a * b {
                    product += Number::one();
                }

                for _ in 0 .. pos_a + pos_b {
                    product.append_int(0);
                }

                result += product;
            }
        }

        let num_fracts = self.num_fract_digits() + other.num_fract_digits();

        if result.comma_index >= num_fracts {
            result.comma_index -= num_fracts;
        }

        
        if self.is_positive != other.is_positive {
            result.is_positive = false;
        }

        result.normalize();

        *self = result;
    }
}

impl ops::Mul for Number {
    type Output = Self;

    fn mul(mut self, other: Self) -> Self {
        self *= other;
        self
    }
}
