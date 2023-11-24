#[allow(dead_code)]

/// 組み合わせ,順列を求める.  
/// `n (n < Mod)`を対象の集合の大きさとして前処理 `O(n)`，クエリ `O(1)`.
pub struct Combination {
    fac: Vec<usize>,
    inv: Vec<usize>,
    finv: Vec<usize>,
}

impl Combination {
    const MOD: usize = 1_000_000_007;

    /// 前処理をする. `O(n)`.
    pub fn new(size: usize) -> Self {
        let mut fac = vec![0; size + 1];
        let mut inv = vec![0; size + 1];
        let mut finv = vec![0; size + 1];
        fac[0] = 1;
        fac[1] = 1;
        inv[1] = 1;
        finv[0] = 1;
        finv[1] = 1;

        for i in 2..=size {
            fac[i] = fac[i - 1] * i % Self::MOD;
            inv[i] = Self::MOD - inv[Self::MOD % i] * (Self::MOD / i) % Self::MOD;
            finv[i] = finv[i - 1] * inv[i] % Self::MOD;
        }

        Self { fac, inv, finv }
    }

    /// `nCr` を求める. `O(1)`.
    pub fn combination(&self, n: usize, r: usize) -> usize {
        if n < r {
            0
        } else {
            self.fac[n] * (self.finv[r] * self.finv[n - r] % Self::MOD) % Self::MOD
        }
    }

    /// `nPr` を求める. `O(1)`.
    pub fn permutation(&self, n: usize, r: usize) -> usize {
        if n < r {
            0
        } else {
            self.fac[n] * self.finv[n - r] % Self::MOD
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::combination::Combination;
    use crate::math::modint::ModInt;
    use proptest::strategy::{Just, Strategy};

    fn factorial(size: usize) -> Vec<ModInt> {
        let mut result = vec![ModInt::new(0); size + 1];
        result[0] = ModInt::new(1);
        result[1] = ModInt::new(1);
        for i in 2..=size {
            result[i] = result[i - 1] * ModInt::new(i as isize);
        }
        result
    }

    fn inverse_factorial(size: usize) -> Vec<ModInt> {
        let mut result = vec![ModInt::new(0); size + 1];
        result[0] = ModInt::new(1);
        result[1] = ModInt::new(1);
        for i in 2..=size {
            result[i] = result[i - 1] * ModInt::new(i as isize).inv();
        }
        result
    }

    proptest::proptest! {
        #[test]
        fn test_rand_combination((n, r) in (1..100usize).prop_flat_map(|n| (Just(n), 0..n))) {
            let size = 100;
            let comb = Combination::new(size);
            let c = comb.combination(n, r);
            let factorial_table = factorial(size);
            let inv_factorial_table = inverse_factorial(size);
            assert_eq!(
                c,
                (factorial_table[n] * inv_factorial_table[r] * inv_factorial_table[n - r]).value
                    as usize
            );
        }
    }

    proptest::proptest! {
        #[test]
        fn test_rand_permutation((n, r) in (1..100usize).prop_flat_map(|n| (Just(n), 0..n))) {
            let size = 100;
            let comb = Combination::new(size);
            let c = comb.permutation(n, r);
            let factorial_table = factorial(size);
            let inv_factorial_table = inverse_factorial(size);
            assert_eq!(
                c,
                (factorial_table[n] * inv_factorial_table[n - r]).value as usize
            );
        }
    }
}
