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

    #[test]
    fn test_factorize_prime() {
        let n = 5;
        let factors = factorize::factorize(n);
        assert_eq!(
            factors,
            vec![(5, 1)].into_iter().collect::<HashMap<usize, usize>>()
        );
    }

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

    #[test]
    fn test_factorize() {
        let n = 1922375;
        let factors = factorize::factorize(n);
        assert_eq!(
            factors,
            vec![(5, 3), (7, 1), (13, 3),]
                .into_iter()
                .collect::<HashMap<usize, usize>>()
        );
    }
}
