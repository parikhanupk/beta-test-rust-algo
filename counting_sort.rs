use std::fmt;
use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};



struct Customer {
    id: String,
    num_purchases: i32,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}



// Prompt the user for an i32.
fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i32>().expect("Error parsing integer");
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
}



// Make a vector of random i32 values in the range [0 and max).
fn make_random_vec(num_items: i32, max: i32) -> Vec<Customer> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<Customer> = Vec::with_capacity(num_items as usize);
    for i in 0..num_items {
        let customer = Customer {
            id: format!("C{i}"),
            num_purchases: prng.next_i32(0, max),
        };
        vec.push(customer);
    }
    return vec;
}



// Print at most num_items items.
fn print_vec(vec: &Vec<Customer>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}



// Verify that the Vec is sorted.
fn check_sorted(vec: &Vec<Customer>) {
    for i in 1usize..vec.len() {
        if vec[i - 1].num_purchases > vec[i].num_purchases {
            println!("The array is NOT sorted!");
            return;
        }
    }
    println!("The array is sorted");
}



fn counting_sort(vec: &Vec<Customer>, max: i32) -> Vec<Customer> {
    let mut counts = vec![0; max as usize]; //num_purchases in [0, max)
    for i in 0usize..vec.len() {
        counts[vec[i].num_purchases as usize] += 1;
    }
    for i in 1usize..max as usize {
        counts[i] += counts[i - 1];
    }
    let mut rvec: Vec<Customer> = Vec::with_capacity(vec.len());
    for _ in 0usize..vec.len() {
        rvec.push(Customer {
            id: String::from(""),
            num_purchases: 0,
        });
    }
    for i in (0usize..vec.len()).rev() {
        counts[vec[i].num_purchases as usize] -= 1;
        rvec[counts[vec[i].num_purchases as usize]] = Customer {
            id: String::from(&vec[i].id),
            num_purchases: vec[i].num_purchases,
        };
    }
    return rvec;
}



fn main() {
    let num_items = get_i32("Enter number of items:");
    let max_value = get_i32("Enter non-inclusive maximum item value:");
    let mut vec = make_random_vec(num_items, max_value);
    print_vec(&vec, 20);
    let rvec = counting_sort(&mut vec, max_value);
    print_vec(&rvec, 20);
    check_sorted(&rvec);
}
