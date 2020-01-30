pub mod write_colors {
    use std::io::{self, Write};
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    pub fn write_green(text: &str) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        write!(&mut stdout, "{}", text)
    }

    pub fn write_red(text: &str) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        write!(&mut stdout, "{}", text)
    }

    pub fn write_white(text: &str) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        write!(&mut stdout, "{}", text)
    }
}

pub mod eratosthenes {
    use super::write_colors;
    use std::io;

    /// Mark all non prime indices as false in a boolean vevtor.
    fn mark_false_if_not_prime(limit: usize, sieve: &mut Vec<bool>) -> io::Result<()> {
        sieve[0] = false;

        if limit >= 1 {
            sieve[1] = false;
        }

        // Perform a trial division
        write_colors::write_white("")?;
        println!("Sieve of Eratosthenes");
        println!("performing a trial division...\n");

        for n in 2..=limit {
            if sieve[n] {
                let mut x = n * n;
                write_colors::write_green(format!("{}", n).as_str())?;
                write_colors::write_white(format!(" is prime\n").as_str())?;
                while x <= limit {
                    write_colors::write_green(format!("{} ", n).as_str())?;
                    write_colors::write_white(format!("+ ").as_str())?;
                    write_colors::write_red(format!("{} ", x).as_str())?;
                    write_colors::write_white(format!("= ").as_str())?;
                    write_colors::write_red(format!("{}\n", n + x).as_str())?;
                    sieve[x] = false;
                    x += n;
                }
                write_colors::write_white("\n")?;
            }
        }
        Ok(())
    }

    /// Calculate primes up to limit using the Sieve of Eratosthenes.
    pub fn sieve_to_n(limit: usize) -> Vec<usize> {
        // Create a mutable boolean vector to represent the sieve
        let mut primes = vec![true; limit + 1];

        // Mark all non prime numbers as false
        mark_false_if_not_prime(limit, &mut primes).unwrap();

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
        mark_false_if_not_prime((nth * nth) - (nth * 2), &mut sieve).unwrap();

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
