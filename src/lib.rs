pub mod eratosthenes {
    /// Mark all non prime indices as false in a boolean vevtor.
    fn mark_false_if_not_prime(limit: usize, sieve: &mut Vec<bool>) {
        sieve[0] = false;

        if limit >= 1 {
            sieve[1] = false;
        }

        // Perform a trial division
        println!("Sieve of Eratosthenes");
        println!("performing a trial division...\n");

        for n in 2..=limit {
            if sieve[n] {
                let mut x = n * n;
                println!("the square root of {} is {} which is not prime", n, x);
                while x <= limit {
                    println!("{} + {} is not prime", x, n);
                    sieve[x] = false;
                    x += n;
                }
                println!();
            }
        }
    }

    /// Calculate primes up to limit using the Sieve of Eratosthenes.
    pub fn sieve_to_n(limit: usize) -> Vec<usize> {
        // Create a mutable boolean vector to represent the sieve
        let mut primes = vec![true; limit + 1];

        // Mark all non prime numbers as false
        mark_false_if_not_prime(limit, &mut primes);

        // Enumerate and filter_map primes for true values and return them
        primes
            .iter()
            .enumerate()
            .filter_map(|(n, &is_prime)| if is_prime { Some(n) } else { None })
            .collect()
    }

    /// Calculate the nth prime using the Sieve of Eratosthenes
    pub fn sieve_nth(nth: usize) -> u128 {
        // Create a mutable boolean vector to represent the sieve
        let mut sieve = vec![true; nth * nth];

        // Mark all non prime numbers as false
        mark_false_if_not_prime(nth + 1, &mut sieve);

        // Enumerate and filter_map primes for true values and return them
        let primes: Vec<usize> = sieve
            .iter()
            .enumerate()
            .filter_map(|(n, &is_prime)| if is_prime { Some(n) } else { None })
            .collect();

        // Return the nth prime
        primes[nth] as u128
    }
}
