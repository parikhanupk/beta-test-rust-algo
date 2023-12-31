use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};



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
fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    return vec;
}



// Print at most num_items items.
fn print_vec(vec: &Vec<i32>, num_items: i32) {
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



fn partition(vec: &mut [i32]) -> i32 {
    let (lo, hi) = (0, vec.len() - 1);
    let pivot = vec[hi];
    let mut i = (lo as i32) - 1;
    for j in lo..hi {
        if vec[j] <= pivot {
            i += 1;
            vec.swap(i as usize, j);
        }
    }
    i += 1;
    vec.swap(i as usize, hi);
    return i;
}

fn quicksort(vec: &mut [i32]) {
    if vec.len() < 2 {
        return;
    }
    let p = partition(&mut vec[0..]) as usize;
    quicksort(&mut vec[..p]);
    quicksort(&mut vec[p + 1..]);
}



// Perform binary search.
// Return the target's location in the vector and the number of tests.
// If the item is not found, return -1 and the number of tests.
fn binary_search(vec: &Vec<i32>, target: i32) -> (i32, i32) {
    let (mut lo, mut hi) = (0, vec.len() - 1);
    let mut index: usize;
    let mut num_tests = 0;
    while lo <= hi {
        num_tests += 1;
        index = (lo + hi) / 2;
        if vec[index] == target {
            return (index as i32, num_tests);
        } else {
            if vec[index] < target {
                lo = index + 1;
            } else {
                hi = index - 1;
            }
        }
    }
    return (-1, num_tests);
}



fn main() {
    let num_items = get_i32("# Items: ");
    let max_value = get_i32("Max: ");
    let mut vec = make_random_vec(num_items, max_value);
    quicksort(&mut vec);
    print_vec(&vec, 40);
    println!();

    loop {
        let target = get_i32("Target (-1 to quit): ");
        if target == -1 {
            break;
        }

        let (index, num_tests) = binary_search(&vec, target);
        if index < 0 || index >= vec.len() as i32 {
            println!("Target {} not found, {} tests", target, num_tests);
        } else {
            println!(
                "numbers[{}] = {}, {} tests",
                index, vec[index as usize], num_tests
            );
        }
    }
}
