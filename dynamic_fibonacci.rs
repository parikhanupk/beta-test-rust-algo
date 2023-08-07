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



fn fibonacci_on_the_fly(values: &mut Vec<i64>, n: i64) -> i64 {
    if (n as usize) < values.len() {
        return values[n as usize];
    } else {
        let fibn = fibonacci_on_the_fly(values, n - 1) + fibonacci_on_the_fly(values, n - 2);
        values.push(fibn);
        return fibn;
    }
}



fn main() {
    // Initialize the prefilled vector.
    //let prefilled_values = prefill_vector();

    // Create a vector for fill-on-the-fly.
    let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];

    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");

        if n < 0 {
            break;
        }

        // Calculate the Fibonacci number.
        //println!("Prefilled:  {}", prefilled_values[n as usize]);
        println!("On the fly: {}", fibonacci_on_the_fly(&mut fill_on_the_fly_values, n));
        //println!("Bottom up:  {}", fibonacci_bottom_up(n));
        println!();
    }
}
