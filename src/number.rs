use std::collections::VecDeque;
use std::str::FromStr;
use std::fmt;
use std::ops;
use std::cmp::{self, Ordering};

#[cfg(not(feature = "base10"))]
pub const BASE: i8 = 9;

#[cfg(feature = "base10")]
pub const BASE: i8 = 10;

mod addition;
mod subtraction;
mod multiplication;

#[derive(Debug, Clone)]
pub struct Number {
    digits: VecDeque<i8>,
    comma_index: usize,
    is_positive: bool,
}

impl Number {
    pub fn zero() -> Self {
        Self {
            digits: VecDeque::new(),
            comma_index: 0,
            is_positive: true,
        }
    }

    pub fn one() -> Self {
        Self {
            digits: vec![1].into(),
            comma_index: 1,
            is_positive: true,
        }
    }

    fn equalize_with(&mut self, other: &mut Self) {
        self.pad_int_digits_to(other.num_int_digits());
        other.pad_int_digits_to(self.num_int_digits());
        
        self.pad_fract_digits_to(other.num_fract_digits());
        other.pad_fract_digits_to(self.num_fract_digits());

        assert_eq!(self.num_int_digits(), other.num_int_digits());
        assert_eq!(self.num_fract_digits(), other.num_fract_digits());
    }

    fn normalize(&mut self) {
        while self.first_int_digit() == Some(0) {
            self.digits.pop_front();
            self.comma_index -= 1;
        }

        while self.last_fract_digit() == Some(0) {
            self.digits.pop_back();
        }
    }

    fn pad_int_digits_to(&mut self, num: usize) {
        while self.num_int_digits() < num {
            self.prepend_int(0);
        }
    }

    fn pad_fract_digits_to(&mut self, num: usize) {
        while self.num_fract_digits() < num {
            self.append_fract(0);
        }
    }

    fn prepend_int(&mut self, digit: i8) {
        self.digits.push_front(digit);
        self.comma_index += 1;
    }

    fn append_int(&mut self, digit: i8) {
        self.digits.insert(self.comma_index, digit);
        self.comma_index += 1;
    }

    fn append_fract(&mut self, digit: i8) {
        self.digits.push_back(digit);
    }

    fn num_int_digits(&self) -> usize {
        self.comma_index
    }

    fn num_fract_digits(&self) -> usize {
        self.digits.len() - self.num_int_digits()
    }

    fn int_digits(&self) -> impl Iterator<Item = i8> + '_ {
        self.digits.iter().copied().take(self.comma_index)
    }

    fn fract_digits(&self) -> impl Iterator<Item = i8> + '_ {
        self.digits.iter().copied().skip(self.comma_index)
    }

    fn first_int_digit(&self) -> Option<i8> {
        if self.num_int_digits() > 0 {
            Some(self.digits[0])
        } else {
            None
        }
    }

    fn last_fract_digit(&self) -> Option<i8> {
        if self.num_fract_digits() > 0 {
            Some(self.digits[self.digits.len() - 1])
        } else {
            None
        }
    }

    fn negate(&mut self) {
        self.is_positive = !self.is_positive;
    }

    pub fn abs_smaller_than(&self, other: &Self) -> bool {
        match self.num_int_digits().cmp(&other.num_int_digits()) {
            Ordering::Less => true,
            Ordering::Greater => false,
            Ordering::Equal => {
                let digits_a = self.digits.iter().take(self.num_int_digits());
                let digits_b = other.digits.iter().take(other.num_int_digits());

                for (a, b) in digits_a.zip(digits_b) {
                    if a < b {
                        return true;
                    }

                    if a > b {
                        return false;
                    }
                }

                self.num_fract_digits() < other.num_fract_digits()
            },
        }
    }

    fn shift_comma_left(&mut self) {
        if self.comma_index > 0 {
            self.comma_index -= 1;
            return;
        }

        self.digits.push_front(0);
    }
}

impl ops::Neg for Number {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.is_positive = !self.is_positive;
        self
    }
}

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut chars = s.chars();
        let mut digits = VecDeque::new();
        let mut is_positive = true;
        let mut comma_index = None;

        match chars.next().ok_or("Empty number")? {
            digit @ '0' ..= '9' => {
                let value = digit as i8 - b'0' as i8;
                digits.push_back(value);
            },
            '-' => is_positive = false,
            invalid => Err(format!("Invalid first character: {}", invalid))?,
        };

        for char in chars {
            match char {
                digit @ '0' ..= '9' => {
                    let value = digit as i8 - b'0' as i8;
                    digits.push_back(value);
                },
                ',' if comma_index.is_none() => comma_index = Some(digits.len()),
                ',' if comma_index.is_some() => Err("Multiple commas")?,
                invalid => Err(format!("Invalid first character: {}", invalid))?,    
            }
        }

        if comma_index == Some(digits.len()) {
            Err("Trailing comma")?
        }

        let mut number = Self {
            comma_index: comma_index.unwrap_or(digits.len()),
            digits,
            is_positive,
        };

        number.normalize();

        Ok(number)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_positive {
            write!(f, "-")?;
        }

        if self.num_int_digits() == 0 {
            write!(f, "0")?;
        } else {
            for digit in self.int_digits() {
                write!(f, "{}", digit)?;
            }
        }

        if self.num_fract_digits() > 0 {
            write!(f, ",")?;

            for digit in self.fract_digits() {
                write!(f, "{}", digit)?;
            }
        }

        Ok(())
    }
}

impl cmp::PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl cmp::PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut num_a = self.clone();
        let mut num_b = other.clone();

        num_a.equalize_with(&mut num_b);

        let a_digits = num_a.digits.iter();
        let b_digits = num_b.digits.iter();

        for (a, b) in a_digits.zip(b_digits) {
            match a.cmp(&b) {
                Ordering::Equal => continue,
                ordering => return Some(ordering),
            }
        }

        Some(Ordering::Equal)
    }
}
