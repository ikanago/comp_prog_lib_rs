#[derive(Clone, Copy, Debug)]
pub struct ModInt {
    pub value: isize,
}

impl ModInt {
    const MOD: isize = 1_000_000_007;

    pub fn new(value: isize) -> Self {
        let value = value % Self::MOD;
        let value = if value < 0 { value + Self::MOD } else { value };
        Self { value }
    }

    pub fn pow(self, mut power: isize) -> Self {
        let mut result = ModInt::new(1);
        let mut accum = ModInt::new(self.value);
        while power > 0 {
            if power % 2 == 1 {
                result *= accum;
            }
            accum *= accum;
            power /= 2;
        }
        result
    }

    pub fn inv(self) -> Self {
        self.pow(Self::MOD - 2)
    }
}

impl std::ops::Add for ModInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = ModInt::new(self.value + rhs.value)
    }
}

impl std::ops::Sub for ModInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = ModInt::new(self.value - rhs.value)
    }
}

impl std::ops::Mul for ModInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        *self = ModInt::new(self.value * rhs.value)
    }
}

impl std::ops::Div for ModInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.inv().value)
    }
}

impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        *self = ModInt::new(self.value * rhs.inv().value)
    }
}

impl std::cmp::PartialEq for ModInt {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl std::cmp::Eq for ModInt {}

use std::cmp::Ordering;
impl std::cmp::PartialOrd for ModInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for ModInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::modint::ModInt;
    const MOD: isize = 1_000_000_007;

    proptest::proptest! {
        #[test]
        fn test_random_add(a in 1..MOD, b in 1..MOD) {
            let ma = ModInt::new(a);
            let mb = ModInt::new(b);
            assert_eq!((ma + mb).value, (a + b) % MOD);
        }
    }

    proptest::proptest! {
        #[test]
        fn test_random_add_assign(a in 1..MOD, b in 1..MOD) {
            let mut m = ModInt::new(a);
            m += ModInt::new(b);
            assert_eq!(m.value, (a + b) % MOD);
        }
    }

    proptest::proptest! {
        #[test]
        fn test_random_sub(a in 1..MOD, b in 1..MOD) {
            let ma = ModInt::new(a);
            let mb = ModInt::new(b);
            assert_eq!((ma - mb).value, ((a - b) % MOD + MOD) % MOD);
        }
    }

    proptest::proptest! {
        #[test]
        fn test_random_sub_assign(a in 1..MOD, b in 1..MOD) {
            let mut m = ModInt::new(a);
            m -= ModInt::new(b);
            assert_eq!(m.value, ((a - b) % MOD + MOD) % MOD);
        }
    }

    proptest::proptest! {
        #[test]
        fn test_random_mul(a in 1..10000isize, b in 1..10000isize) {
            let ma = ModInt::new(a);
            let mb = ModInt::new(b);
            assert_eq!((ma * mb).value, a * b % MOD);
        }
    }

    proptest::proptest! {
        #[test]
        fn test_random_mul_assign(a in 1..10000isize, b in 1..10000isize) {
            let mut m = ModInt::new(a);
            m *= ModInt::new(b);
            assert_eq!(m.value, a * b % MOD);
        }
    }

    proptest::proptest! {
    #[test]
        fn test_random_inv(r in 1..1000isize) {
            let m = ModInt::new(r);
            assert_eq!(r * m.inv().value % MOD, 1);
        }
    }

    proptest::proptest! {
        #[test]
        fn test_random_div(a in 1..MOD, b in 1..MOD) {
            let ma = ModInt::new(a);
            let mb = ModInt::new(b);
            assert_eq!((ma / mb).value, a * mb.inv().value % MOD);
        }
    }

    proptest::proptest! {
        #[test]
        fn test_random_div_assign(a in 1..MOD, b in 1..MOD) {
            let mut m = ModInt::new(a);
            m /= ModInt::new(b);
            assert_eq!(m.value, a * ModInt::new(b).inv().value % MOD);
        }
    }

    proptest::proptest! {
        #[test]
        fn test_random_ord(a in 1..MOD, b in 1..MOD) {
            let ma = ModInt::new(a);
            let mb = ModInt::new(b);
            assert_eq!(ma < mb, a < b);
        }
    }
}
