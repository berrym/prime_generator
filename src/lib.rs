pub mod eratosthenes {
    /// Mark all non prime indices as false in a boolean vevtor.
    fn mark_false_if_not_prime(limit: usize, sieve: &mut Vec<bool>) {
        sieve[0] = false;

        if limit >= 1 {
            sieve[1] = false;
        }

        for x in 2..limit + 1 {
            if sieve[x] {
                let mut multiple = x * x;
                while multiple <= limit {
                    sieve[multiple] = false;
                    multiple += x;
                }
            }
        }
    }

    /// Calculate primes up to limit using the Sieve of Eratosthenes.
    #[allow(dead_code)]
    pub fn sieve(limit: usize) -> Vec<usize> {
        // Create a mutable boolean vector to represent the sieve
        let mut primes = vec![true; limit + 1];

        // Mark all non prime numbers as false
        mark_false_if_not_prime(limit, &mut primes);

        // Enumerate and filter_map primes for true values and return them
        primes.iter().enumerate()
            .filter_map(|(n, &is_prime)| if is_prime {Some(n)} else {None})
            .collect()
    }
}
