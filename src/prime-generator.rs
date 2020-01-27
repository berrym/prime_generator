use std::{error, io, io::Write};

use primes::eratosthenes;

#[macro_use]
extern crate clap;

fn main() -> Result<(), Box<dyn error::Error>> {
    // Parse command line
    let matches = clap_app!(prime_generator =>
        (version: "0.1.1")
        (author: "Michael Berry <trismegustis@gmail.com>")
        (about: "Generate prime numbers")
        (@arg N: -p --primes +takes_value "Generate primes up to N")
        (@arg INTERACTIVE: -i --interactive "Interacive program")
    )
    .get_matches();

    // Run a one time command
    if let Some(n) = matches.value_of("N") {
        match n.parse() {
            Ok(n) => println!(
                "List of prime numbers up to {}\n{:#?}",
                n,
                eratosthenes::sieve(n)
            ),
            Err(e) => println!("Error: {}", e),
        }
        return Ok(());
    };

    // Run the interactive loop
    if matches.is_present("INTERACTIVE") {
        interactive()?;
    }
    Ok(())
}

fn interactive() -> Result<(), Box<dyn error::Error>> {
    println!("Prime Number Generator\n");
    println!("Calculate prime numbers up to N");
    println!("Type \"quit\" to end the program");

    loop {
        let mut n = String::new();

        // Print directly to stdout without a newline, flush stdout
        print!("\nEnter a positive integer: ");
        io::stdout().flush().unwrap();

        // Read a line of user input
        io::stdin().read_line(&mut n).expect("Failed to read line");

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
            io::stdout().flush().unwrap()
        }
        println!();
    }
    Ok(())
}
