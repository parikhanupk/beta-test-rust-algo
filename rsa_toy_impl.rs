use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};



fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return match a < 0 {
            true => -a,
            false => a,
        };
    } else {
        return gcd(b, a % b);
    }
}



fn lcm(a: i64, b: i64) -> i64 {
    let res = (a / gcd(a, b)) * b;
    return match res < 0 {
        true => -res,
        false => res,
    };
}



// Calculate Carmichael's totient function λ(n)
// where n = p * q and p and q are prime.
fn totient(p: i64, q: i64) -> i64 {
    return lcm(p - 1, q - 1);
}



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

    // Return a pseudorandom value in the range [min, max).
    fn next_i64(&mut self, min: i64, max: i64) -> i64 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i64;
    }
}



// Pick a random exponent e in the range [3, λ_n)
// such that gcd(e, λ_n) = 1.
fn random_exponent(prng: &mut Prng, λ_n: i64) -> i64 {
    let mut e: i64;
    loop {
        e = prng.next_i64(3, λ_n);
        if gcd(e, λ_n) == 1 {
            return e;
        }
    }
}



fn inverse_mod(a: i64, n: i64) -> i64 {
    let (mut t, mut newt): (i64, i64) = (0, 1);
    let (mut r, mut newr): (i64, i64) = (n, a);
    let mut quotient: i64;
    while newr != 0 {
        quotient = r / newr;
        (t, newt) = (newt, t - (quotient * newt));
        (r, newr) = (newr, r - (quotient * newr));
    }
    if r > 1 {
        panic!("a is not invertible");
    }
    if t < 0 {
        t = t + n;
    }
    return t;
}



// Perform fast exponentiation in a modulus.
fn fast_exp_mod(mut num: i64, mut pow: i64, modulus: i64) -> i64 {
    let mut result = 1;
    while pow > 0 {
        if pow & 1 == 1 {
            result = (result * num) % modulus;
        }
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
        if np != 1 {
            return false;
        }
    }
    return true;
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



fn main() {
    let mut prng = Prng::new();

    let (p, mut q): (i64, i64) = (
        find_prime(&mut prng, 10000, 50000, 20),
        find_prime(&mut prng, 10000, 50000, 20),
    );
    while p == q {
        q = find_prime(&mut prng, 10000, 50000, 20);
    }
    let n = p * q;
    let λ_n = totient(p, q);
    let e = random_exponent(&mut prng, λ_n);
    let d = inverse_mod(e, λ_n);

    println!("*** Public ***");
    println!("Public key modulus (n):{n}");
    println!("Public key exponent (e):{e}");
    println!();
    println!("*** Private ***");
    println!("Primes (p, q):{p}, {q}");
    println!("λ(n):{λ_n}");
    println!("d:{d}");
    println!();

    let (mut m, mut ciphertext, mut plaintext): (i64, i64, i64);
    loop {
        m = get_i64(&format!("Enter a number in range [2, {} - 2] which will be the message, but experiment with numbers out of that range too:", n));
        if m < 0 {
            break;
        }

        ciphertext = fast_exp_mod(m, e, n);
        plaintext = fast_exp_mod(ciphertext, d, n);
        println!("Ciphertext:{ciphertext}");
        println!("Plaintext:{plaintext}");
    }
}
