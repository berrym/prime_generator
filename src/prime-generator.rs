use std::{
    error,
    io::{self, Write},
    process,
};

use primes::eratosthenes;

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn error::Error>> {
    // Parse command line
    let cli = App::new("Prime Generator")
        .version("0.1.2")
        .author("Michael Berry <trismegustis@gmail.com>")
        .about("Generate prime numbers")
        .arg(
            Arg::with_name("primes")
                .short("p")
                .long("primes-to")
                .help("Generate primes up to N")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("nth_prime")
                .short("n")
                .long("nth-prime")
                .help("Generate primes up to N")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("Interactive program"),
        )
        .get_matches();

    if cli.is_present("primes") {
        if let Some(n) = cli.value_of("primes") {
            match n.parse() {
                Ok(n) => println!(
                    "List of prime numbers up to {}\n{:#?}",
                    n,
                    eratosthenes::sieve_to_n(n)
                ),
                Err(e) => println!("Error: {}", e),
            }
            return Ok(());
        };
    } else if cli.is_present("nth_prime") {
        if let Some(n) = cli.value_of("nth_prime") {
            match n.parse() {
                Ok(n) => match eratosthenes::sieve_nth(n) {
                    Some(p) => println!("Prime number {} is {:?}", n, p),
                    None => println!("Error: unknown"),
                },
                Err(e) => println!("Error: {}", e),
            }
            return Ok(());
        };
    } else if cli.is_present("interactive") {
        interactive()?;
    } else {
        println!("{}\n\nTry passing --help for more information", cli.usage());
    }
    Ok(())
}

// Interactive program
fn interactive() -> Result<(), Box<dyn error::Error>> {
    println!("Prime Number Generator\n");
    println!("Calculate prime numbers up to N");
    println!("Type \"quit\" to end the program");

    let mut n = String::new();

    // Print directly to stdout without a newline, flush stdout
    print!("\nEnter a positive integer: ");
    io::stdout().flush().unwrap();

    // Read a line of user input
    io::stdin().read_line(&mut n).expect("Failed to read line");

    // If user input is quit, then break program loop
    if n.trim() == "quit" {
        process::exit(0);
    }

    // Parse input into a number
    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => process::exit(1),
    };

    // Generate the first n prime numbers
    let primes = eratosthenes::sieve_to_n(n);
    for p in &primes {
        print!("{} ", p);
        io::stdout().flush().unwrap()
    }
    println!();
    Ok(())
}
