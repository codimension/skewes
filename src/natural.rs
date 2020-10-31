use core::cmp::Ordering;
use core::fmt;
use core::convert::TryFrom;
use core::iter::FromIterator;
use core::ops::{Add, Sub, Mul, Div, Rem};
use core::iter::Iterator;
use crate::integer::Sign;
use crate::algorithms::{add, mul, div, cmp_slice, sub_signed};

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Natural {
    pub(crate) digits: Vec<u64>
}


impl From<u64> for Natural {
    fn from(digit: u64) -> Self {
        Self {
            digits: vec!(digit),
        }
    }
}

impl TryFrom<Natural> for u32 {
    type Error = &'static str;

    fn try_from(n: Natural) -> Result<u32, Self::Error> {
        if n.digits.len() == 0 {
            Ok(0u32)
        }
        else if n.digits.len() == 1 {
            Ok(n.digits[0] as u32) 
        }
        else {
            Err("Error converting a Natural to unsigned integer")
        }
    }
}

impl From<Vec<u64>> for Natural {
    fn from(digits: Vec<u64>) -> Self {
        Self {
            digits: digits,
        }
    }
}

impl Ord for Natural {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_slice(&self.digits, &other.digits)
    }
}

impl PartialOrd for Natural {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for &Natural {
    type Output = Natural;

    fn add(self, other: Self) -> Natural {
        self.add(other)
    }
}

impl Sub for &Natural {
    type Output = Natural;

    fn sub(self, other: Self) -> Natural {
        match sub_signed(self, other) {
            (Sign::Positive, result) => result,
            (Sign::Negative, _) => panic!("Tried to subtract larger natural from smaller natural. 
                                                Maybe you meant to use the Integer type?"),
        }
    }
}

impl Mul for &Natural {
    type Output = Natural;

    fn mul(self, other: Self) -> Natural {
        self.mul(other)
    }
}

impl Div for &Natural {
    type Output = Natural;

    fn div(self, other: Self) -> Natural {
        self.div(other).0
    }
}

impl Rem for &Natural {
    type Output = Natural;

    fn rem(self, other: Self) -> Natural {
        self.div(other).1
    }
}

impl Natural {

    pub fn zero() -> Self {
        Self {
            digits: Vec::new(),
        }
    }

    pub fn one() -> Self {
        Self {
            digits: vec!(1),
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let result = add(&self.digits, &other.digits);
        Self::from(result)
    }

    pub fn sub(&self, other: &Self) -> Option<Self> {
        let (sign, value) = sub_signed(&self, &other);
        match sign {
            Sign::Negative => None,
            _ => Some(value)
        }
    }


    pub fn mul(&self, other: &Self) -> Self {
        let result = mul(&self.digits, &other.digits);
        Self::from(result)
    }

    pub fn div(&self, other: &Self) -> (Self, Self) {
        div(&self, &other)
    }

    // TODO: This increments a number in-place. Implement in-place addition for +=
    pub fn inc(&mut self) -> () {
        let mut carry = false;
        for digit in &mut self.digits {
            let (a, b) = digit.overflowing_add(1);
            *digit = a;
            carry = b;
            if !carry {
                break
            }
        }
        if carry {
            self.digits.push(1);
        }
    }

    // TODOs: change to potentially fail, chunk digits
    pub fn from_string<S: Into<String>>(s: S) -> Self {
        let mut n = Self::zero();
        s.into().chars()
            .for_each(|c| {
                let d: u32 = c.to_digit(10).unwrap();
                n = &(&n * &Natural::from(10)) + &Natural::from(d as u64);
            });
        n
    }
}

impl fmt::Display for Natural {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut n = self.clone();
        let mut s = Vec::<char>::new();
        while n != Natural::zero() {
            let (d, r) = div(&n, &Natural::from(10));
            let rem = u32::try_from(r).unwrap(); // Guaranteed to be correct because remainder < 10
            s.push(std::char::from_digit(rem, 10).unwrap());
            n = d;
        }
        write!(f, "{}", String::from_iter(s.iter().rev()))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const NINE: u64 = u64::MAX;
    const EIGHT: u64 = u64::MAX - 1;
    const ONE: u64 = 1;

    macro_rules! assert_plus_identity {
        ($a:expr, $b:expr => $c:expr) => (
            assert_eq!($a.add(&$b), $c);
            assert_eq!($b.add(&$a), $c);
        );
    }

    #[test]
    fn can_create_from_u64() {
        let a = Natural::from(42);
        let mut b = Natural::zero();
        b.digits.push(42);
        assert_eq!(a, b);
    }

    #[test]
    fn can_add_two_one_digit_numbers_without_carry() {
        let a = Natural::from(21);
        let b = Natural::from(42);
        let c = Natural::from(63);
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn can_add_two_one_digit_numbers_with_carry() {
        let a = Natural::from(NINE);
        let b = Natural::from(ONE);
        let c = Natural::from(vec!(0, 1));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn eight_plus_one_is_nine () {
        let a = Natural::from(EIGHT);
        let b = Natural::from(ONE);
        let c = Natural::from(NINE);
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn can_add_numbers_with_same_number_of_digits () {
        let a = Natural::from(vec!(1, 2));
        let b = Natural::from(vec!(2, 4));
        let c = Natural::from(vec!(3, 6));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn can_add_numbers_with_different_number_of_digits_wo_carry() {
        let a = Natural::from(vec!(1, 2));
        let b = Natural::from(vec!(2, 4, 6, 8));
        let c = Natural::from(vec!(3, 6, 6, 8));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn addition_series_of_carries() {
        let a = Natural::from(vec!(1));
        let b = Natural::from(vec!(NINE, NINE, NINE, NINE));
        let c = Natural::from(vec!(0, 0, 0, 0, 1));
        assert_plus_identity!(a, b => c);
    }

    #[test]
    fn test_two_digit_by_two_digit_multiply() {
        let a = Natural::from(vec!(NINE, ONE));
        let b = Natural::from(vec!(2, 1));
        let c = Natural::from(vec!(EIGHT, 2, 2));
        assert_eq!(a.mul(&b), c);
    }
    #[test]
    fn test_sub_more_digits() {
        let a = Natural::from(vec!(1, 2));
        let b = Natural::from(vec!(8, 1, 3));
        assert_eq!(a.sub(&b), None);
    }

    #[test]
    fn test_sub_fewer_digits() {
        let a = Natural::from(vec!(1, 2, 3));
        let b = Natural::from(vec!(NINE, 1));
        let c = Natural::from(vec!(2, 0, 3));
        assert_eq!(a.sub(&b), Some(c));
    }

    #[test]
    fn test_sub_bigger_number() {
        let a = Natural::from(vec!(1, 2));
        let b = Natural::from(vec!(3, 2));
        assert_eq!(a.sub(&b), None);
    }

    #[test]
    fn test_sub_smaller_number() {
        let a = Natural::from(vec!(3, 2));
        let b = Natural::from(vec!(1, 2));
        let c = Natural::from(vec!(2));
        assert_eq!(a.sub(&b), Some(c));
    }

    #[test]
    fn print_numbers () {
        let a = Natural::from(vec!(3));
        let b = Natural::from(vec!(156));
        assert_eq!(a.to_string(), "3");
        assert_eq!(b.to_string(), "156");
    }


    #[test]
    fn can_parse_integers () {
        let n = Natural::from_string("1234");
        let m = Natural::from(1234);
        assert_eq!(n, m);
    }
}


