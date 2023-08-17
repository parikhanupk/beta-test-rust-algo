use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};



struct Prng {
    seed: u32,
}

impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }
}



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



// Perform fast exponentiation in a modulus.
fn fast_exp_mod(mut num: i64, mut pow: i64, modulus: i64) -> i64 {
    let mut result = 1;
    while pow > 0 {
        if pow & 1 == 1 { result = (result * num) % modulus; }
        pow /= 2;
        num = (num * num) % modulus;
    }
    return result;
}



fn is_probably_prime(p: i32, num_tests: i64, prng: &mut Prng) -> bool {
    let mut n: i32;
    let mut np: i64;
    for _ in 0..num_tests {
        n = prng.next_i32(2, p);
        np = fast_exp_mod(n as i64, (p - 1) as i64, p as i64);
        if np != 1 { return false; }
    }
    return true
}



fn find_prime(prng: &mut Prng, min: i32, max: i32, num_tests: i64) -> i64 {
    let mut p: i32;
    loop {
        p = prng.next_i32(min, max);
        p |= 1; //increments p by 1 if p is even
        if is_probably_prime(p, num_tests, prng) {
            return p as i64;
        }
    }
}



const NUM_TESTS: i64 = 20;

fn main() {
    // Prepare a Prng.
    let mut prng = Prng::new();

    // Display the probability that a number is prime
    // if it passes all NUM_TESTS tests.
    let probability = (1.0 - (0.5 as f64).powf(NUM_TESTS as f64)) * 100.0;
    println!("Probability: {}%\n", probability);

    // Generate random primes.
    loop {
        // Get the number of digits.
        let num_digits = get_i64("# Digits (max 9): ");
        if num_digits < 1 { break; }

        // Calculate minimum and maximum values.
        let min = 10i64.pow((num_digits - 1) as u32);
        let max = 10 * min;
        if min == 1 { min = 2; } // 1 is not prime.

        // Find a prime.
        println!("Prime: {}", find_prime(&mut prng, min as i32, max as i32, NUM_TESTS));
    }
}
