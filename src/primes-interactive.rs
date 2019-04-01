extern crate primes;
use primes::eratosthenes;
use std::io;
use std::io::Write;

fn main() {
    println!("Prime Number Generator\n");
    println!("Calculate prime numbers up to N");
    println!("Type \"quit\" to end the program");

    loop {
        let mut n = String::new();

        // Print directly to stdout without a newline, flush stdout
        print!("\nEnter a positive integer: ");
        io::stdout().flush()
            .unwrap();

        // Read a line of user input
        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        // If user input is quit, then break program loop
        if n.trim() == "quit" {
            break;
        }

        // Parse input into a number
        let n: usize = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Generate the first n prime numbers
        let primes = eratosthenes::sieve(n);
        for p in &primes {
            print!("{} ", p);
            io::stdout().flush()
                .unwrap()
        }
    }
}
