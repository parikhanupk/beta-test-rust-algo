use std::io;
use std::io::Write;
use std::time::{Instant};



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



fn find_factors(mut num: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = vec![];
    while num % 2 == 0 {
        factors.push(2);
        num /= 2;
    }
    let mut factor: i64 = 3;
    while factor * factor <= num {
        while num % factor == 0 {
            factors.push(factor);
            num /= factor;
        }
        factor += 2;
    }
    if num > 1 {
        factors.push(num);
    }
    return factors;
}



fn multiply_vector(factors: &Vec<i64>) -> i64 {
    let mut product: i64 = 1;
    for num in factors.iter() {
        product *= num;
    }
    return product;
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



/*
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
*/



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



fn find_factors_sieve(primes: &Vec<i64>, mut num: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = vec![];
    let sqrt = (num as f64).sqrt().ceil() as i64;
    for factor in primes.into_iter() {
        while num % factor == 0 {
            factors.push(*factor);
            num /= factor;
        }
        if *factor > sqrt || num < *factor {
            break;
        }
    }
    if num > 1 {
        factors.push(num);
    }
    return factors;
}



fn main() {
    let primes: Vec<i64> = sieve_to_primes(&sieve_of_eratosthenes(1000000000));

    loop {
        let num = get_i64("Num: ");
        if num <= 0 {
            break;
        }

        /*
        let mut factors = find_factors(num);
        print_numbers(&factors);
        println!("Is factorization by find_factors correct: {}", multiply_vector(&factors) == num);

        factors = find_factors_sieve(num, primes);
        print_numbers(&factors);
        println!("Is factorization by find_factors correct: {}", multiply_vector(&factors) == num);
        */

        // Find the factors the slow way.
        let start1 = Instant::now();
        let factors1 = find_factors(num);
        let duration1 = start1.elapsed();
        println!("find_factors: {:?} seconds", duration1);
        print_numbers(&factors1);
        println!("Product: {}", multiply_vector(&factors1));
        println!();

        // Use the Eratosthenes' sieve to find the factors.
        let start2 = Instant::now();
        let factors2 = find_factors_sieve(&primes, num);
        let duration2 = start2.elapsed();
        println!("find_factors_sieve: {:?} seconds", duration2);
        print_numbers(&factors2);
        println!("Product: {}", multiply_vector(&factors2));
        println!();
    }
}
