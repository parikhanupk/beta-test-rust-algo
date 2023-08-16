use std::io;
use std::io::Write;



// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i64>().expect("Error parsing integer");
}



// Build a sieve of Eratosthenes.
fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut is_prime: Vec<bool> = Vec::with_capacity(max + 1);
    is_prime.resize(max + 1, false);
    if max >= 2 {
        is_prime[2] = true;
        for i in (3..=max).step_by(2) {
            is_prime[i] = true;
        }
        for i in (3..(max as f64).sqrt().ceil() as usize).step_by(2) {
            if is_prime[i] == true {
                for j in ((i * i)..=max).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
    }
    return is_prime;
}



// Print out the primes in the sieve.
fn print_sieve(sieve: &Vec<bool>) {
    if sieve.len() > 2 {
        print!("2");
        for i in (3..sieve.len()).step_by(2) {
            if sieve[i] {
                print!(" {}", i);
            }
        }
        println!();
    }
}



// Convert the sieve into a vector holding prime numbers.
fn sieve_to_primes(sieve: &Vec<bool>) -> Vec<i64> {
    let mut primes: Vec<i64> = vec![];
    if sieve.len() > 2 {
        primes.push(2);
        for i in (3..sieve.len()).step_by(2) {
            if sieve[i] {
                primes.push(i as i64);
            }
        }
    }
    return primes;
}



// Print the vector of numbers.
fn print_numbers(primes: &Vec<i64>) {
    for prime in primes {
        print!("{} ", prime);
    }
    println!();
}



fn main() {
    let max = get_i64("Max: ");
    let sieve = sieve_of_eratosthenes(max as usize);
    if max < 1000 {
        print_sieve(&sieve);
    }

    let primes = sieve_to_primes(&sieve);
    if max < 1000 {
        print_numbers(&primes);
    }
}
