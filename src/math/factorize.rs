use std::collections::HashMap;

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
    use rand::Rng;

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

    #[test]
    fn test_factorize_random() {
        let range = 100000;
        let loop_count = 100;
        let mut rng = rand::thread_rng();
        for _ in 0..loop_count {
            let expected = rng.gen_range(1, range);
            let factors = factorize::factorize(expected);
            // Check if all factors are prime.
            factors.keys().all(is_prime);
            let actual = factors.iter().fold(1usize ,|acc, (factor, count)| {
                acc * factor.pow(*count as u32)
            });
            assert_eq!(expected, actual);
        }
    }
}
