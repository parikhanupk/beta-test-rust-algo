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



fn fibonacci(n: i64) -> i64 {
    return if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    };
}



fn main() {
    println!("Enter -1 to exit\n");
    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        // If n < 0, break out of the loop.
        if n < 0 {
            break;
        }

        // Calculate the Fibonacci number.
        println!("fibonacci({}) = {}\n", n, fibonacci(n));
    }
}
