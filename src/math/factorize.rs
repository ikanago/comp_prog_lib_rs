use std::collections::HashMap;

/// 整数 `n` の素因数分解をする. `O(sqrt(n))`.
pub fn factorize(mut n: usize) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            *result.entry(i).or_insert(0) += 1;
        }
        i += 1;
    }
    if n > 1 {
        *result.entry(n).or_insert(0) += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::math::factorize;
    use std::collections::HashMap;

    #[test]
    fn test_factorize_square_number() {
        let n = 36;
        let factors = factorize::factorize(n);
        assert_eq!(
            factors,
            vec![(2, 2), (3, 2)]
                .into_iter()
                .collect::<HashMap<usize, usize>>()
        );
    }

    fn is_prime(n: &usize) -> bool {
        let mut i = 2;
        while i * i <= *n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
        return true;
    }

    proptest::proptest! {
        #[test]
        fn test_factorize_random(expected in 1usize..100000usize) {
            let factors = factorize::factorize(expected);
            assert!(factors.keys().all(is_prime));

            let actual = factors.iter().fold(1usize, |acc, (factor, count)| {
                acc * factor.pow(*count as u32)
            });
            assert_eq!(expected, actual);
        }
    }
}
