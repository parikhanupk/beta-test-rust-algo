use std::io;
use std::io::Write;



// Perform fast exponentiation.
fn fast_exp(mut num: i64, mut pow: i64) -> i64 {
    let mut result = 1;
    while pow > 0 {
        if pow & 1 == 1 { result *= num; }
        pow /= 2;
        num *= num;
    }
    return result;
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



// Prompt the user for an i64.
fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i64>()
        .expect("Error parsing integer");
}



fn main() {
    loop {
        let num = get_i64("Num: ");
        let pow = get_i64("Pow: ");
        let modulus = get_i64("Mod: ");

        // fast_exp
        println!("    fast_exp     = {}", fast_exp(num, pow));
        println!("    Num ^ Pow    = {}", num.pow(pow as u32));
        println!();

        // fast_exp_mod
        println!("    fast_exp_mod = {}", fast_exp_mod(num, pow, modulus));
        println!("    Num ^ Pow    = {}", num.pow(pow as u32) % modulus);
        println!("----------");
    }
}
