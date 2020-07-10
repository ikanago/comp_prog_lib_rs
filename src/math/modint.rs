#[derive(Clone, Copy, Debug)]
pub struct ModInt {
    value: isize,
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
    use rand::Rng;
    const MOD: isize = 1_000_000_007;
    const LOOP_COUNT: usize = 10000;

    #[test]
    fn test_random_add() {
        let mut rng = rand::thread_rng();
        for _ in 0..LOOP_COUNT {
            let ra = rng.gen_range(1, MOD);
            let rb = rng.gen_range(1, MOD);
            let ma = ModInt::new(ra);
            let mb = ModInt::new(rb);
            assert_eq!((ma + mb).value, (ra + rb) % MOD);
        }
    }

    #[test]
    fn test_random_add_assign() {
        let mut rng = rand::thread_rng();
        let mut m = ModInt::new(0);
        let mut a = 0;
        for _ in 0..LOOP_COUNT {
            let r = rng.gen_range(1, MOD);
            m += ModInt::new(r);
            a += r;
            a %= MOD;
            assert_eq!(m.value, a);
        }
    }

    #[test]
    fn test_random_sub() {
        let mut rng = rand::thread_rng();
        for _ in 0..LOOP_COUNT {
            let ra = rng.gen_range(1, MOD);
            let rb = rng.gen_range(1, MOD);
            let ma = ModInt::new(ra);
            let mb = ModInt::new(rb);
            assert_eq!((ma - mb).value, ((ra - rb) % MOD + MOD) % MOD);
        }
    }

    #[test]
    fn test_random_sub_assign() {
        let mut rng = rand::thread_rng();
        let mut m = ModInt::new(0);
        let mut a = 0;
        for _ in 0..LOOP_COUNT {
            let r = rng.gen_range(1, MOD);
            m -= ModInt::new(r);
            a -= r;
            a = ((a % MOD) + MOD) % MOD;
            assert_eq!(m.value, a);
        }
    }

    #[test]
    fn test_random_mul() {
        let mut rng = rand::thread_rng();
        for _ in 0..LOOP_COUNT {
            let ra = rng.gen_range(1, 10000);
            let rb = rng.gen_range(1, 10000);
            let ma = ModInt::new(ra);
            let mb = ModInt::new(rb);
            assert_eq!((ma * mb).value, (ra * rb) % MOD);
        }
    }

    #[test]
    fn test_random_mul_assign() {
        let mut rng = rand::thread_rng();
        let mut m = ModInt::new(0);
        let mut a = 0;
        for _ in 0..LOOP_COUNT {
            let r = rng.gen_range(1, 10000);
            m *= ModInt::new(r);
            a *= r;
            a %= MOD;
            assert_eq!(m.value, a);
        }
    }

    #[test]
    fn test_random_inv() {
        let mut rng = rand::thread_rng();
        for _ in 0..LOOP_COUNT {
            let r = rng.gen_range(1, 1000);
            let m = ModInt::new(r);
            assert_eq!(r * m.inv().value % MOD, 1);
        }
    }

    #[test]
    fn test_random_div() {
        let mut rng = rand::thread_rng();
        for _ in 0..LOOP_COUNT {
            let ra = rng.gen_range(1, MOD);
            let rb = rng.gen_range(1, MOD);
            let ma = ModInt::new(ra);
            let mb = ModInt::new(rb);
            let m = ma / mb;
            assert_eq!(ma, m * mb);
        }
    }

    #[test]
    fn test_random_div_assign() {
        let mut rng = rand::thread_rng();
        let mut m = ModInt::new(0);
        let mut a = 0;
        for _ in 0..LOOP_COUNT {
            let r = rng.gen_range(1, MOD);
            let inv = m.inv();
            m /= ModInt::new(r);
            a *= inv.value;
            a %= MOD;
            assert_eq!(m.value, a);
        }
    }

    #[test]
    fn test_random_ord() {
        let mut rng = rand::thread_rng();
        for _ in 0..LOOP_COUNT {
            let ra = rng.gen_range(1, MOD);
            let rb = rng.gen_range(1, MOD);
            let ma = ModInt::new(ra);
            let mb = ModInt::new(rb);
            assert_eq!(ma < mb, ra < rb);
        }
    }
}
