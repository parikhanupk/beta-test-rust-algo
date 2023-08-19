struct Prng {
    seed: u32,
}

impl Prng {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    fn next_i64(&mut self, min: i64, max: i64) -> i64 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i64;
    }
}



// Make some random items.
fn make_items(
    prng: &mut Prng,
    num_items: i32,
    min_value: i32,
    max_value: i32,
    min_weight: i32,
    max_weight: i32,
) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::with_capacity(num_items as usize);
    for i in 0..num_items {
        let item = Item {
            id: i,
            value: prng.next_i32(min_value, max_value),
            weight: prng.next_i32(min_weight, max_weight),
            is_selected: false,
            blocked_by: -1,
            block_list: Vec::new(),
        };
        items.push(item);
    }
    return items;
}



// Return a copy of the items.
fn copy_items(items: &mut Vec<Item>) -> Vec<Item> {
    let mut new_items: Vec<Item> = Vec::with_capacity(items.len());
    for item in items {
        let new_item = Item {
            id: item.id,
            value: item.value,
            weight: item.weight,
            is_selected: item.is_selected,
            blocked_by: item.blocked_by,
            block_list: Vec::new(), //okay as it isn't used in printing solutions (the same goes for blocked_by and id)
        };
        new_items.push(new_item);
    }
    return new_items;
}



// Return the total value of the items.
// If add_all is true, add up all items.
// If add_all is false, only add up the selected items.
fn sum_values(items: &Vec<Item>, add_all: bool) -> i32 {
    if add_all {
        return items.iter().map(|item| item.value).sum();
    } else {
        return items
            .iter()
            .filter(|item| item.is_selected)
            .map(|item| item.value)
            .sum();
    }
}



// Return the total weight of the items.
// If add_all is false, only add up the selected items.
// If add_all is true, add up all items.
fn sum_weights(items: &Vec<Item>, add_all: bool) -> i32 {
    if add_all {
        return items.iter().map(|item| item.weight).sum();
    } else {
        return items
            .iter()
            .filter(|item| item.is_selected)
            .map(|item| item.weight)
            .sum();
    }
}



// Return the value of this solution.
// If the solution is too heavy, return -1 so we prefer an empty solution.
#[allow(dead_code)]
fn solution_value(items: &Vec<Item>, allowed_weight: i32) -> i32 {
    // If the solution's total weight > allowed_weight,
    // return -1 so even an empty solution is better.
    if sum_weights(items, false) > allowed_weight {
        return -1;
    }

    // Return the sum of the selected values.
    return sum_values(items, false);
}



fn print_items(items: &Vec<Item>, all: bool) {
    let mut num_printed = 0;
    for i in 0..items.len() {
        if all || items[i].is_selected {
            print!("{}({}, {}) ", i, items[i].value, items[i].weight)
        }
        num_printed += 1;
        if num_printed > 100 {
            println!("...");
            return;
        }
    }
    println!();
}



// Run the algorithm. Display the elapsed time and solution.
fn run_algorithm(
    alg: &dyn Fn(&mut Vec<Item>, i32) -> (Vec<Item>, i32, i64),
    items: &mut Vec<Item>,
    allowed_weight: i32,
) {
    // Copy the items so the run isn't influenced by a previous run.
    let mut test_items = copy_items(items);

    let start = Instant::now();

    // Run the algorithm.
    let solution: Vec<Item>;
    let total_value: i32;
    let function_calls: i64;
    (solution, total_value, function_calls) = alg(&mut test_items, allowed_weight);

    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);

    print_items(&solution, false);
    println!(
        "Value: {}, Weight: {}, Calls: {}",
        total_value,
        sum_weights(&solution, false),
        function_calls
    );
    println!();
}



// Recursively assign values in or out of the solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made.
fn exhaustive_search(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i64) {
    return do_exhaustive_search(items, allowed_weight, 0);
}



fn do_exhaustive_search(
    items: &mut Vec<Item>,
    allowed_weight: i32,
    next_index: i32,
) -> (Vec<Item>, i32, i64) {
    if (next_index as usize) >= items.len() {
        return (copy_items(items), solution_value(items, allowed_weight), 1);
    } else {
        items[next_index as usize].is_selected = true;
        let (included_solution, included_value, included_calls) =
            do_exhaustive_search(items, allowed_weight, next_index + 1);
        items[next_index as usize].is_selected = false;
        let (excluded_solution, excluded_value, excluded_calls) =
            do_exhaustive_search(items, allowed_weight, next_index + 1);
        if included_value >= excluded_value {
            return (
                included_solution,
                included_value,
                included_calls + excluded_calls + 1,
            );
        } else {
            return (
                excluded_solution,
                excluded_value,
                excluded_calls + included_calls + 1,
            );
        }
    }
}



fn branch_and_bound(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i64) {
    return do_branch_and_bound(items, allowed_weight, 0, 0, 0, sum_values(items, true), 0);
}



fn do_branch_and_bound(
    items: &mut Vec<Item>,
    allowed_weight: i32,
    mut best_value: i32,
    current_value: i32,
    current_weight: i32,
    remaining_value: i32,
    next_index: i32,
) -> (Vec<Item>, i32, i64) {
    if (next_index as usize) >= items.len() {
        return (copy_items(items), current_value, 1);
    } else {
        if current_value + remaining_value <= best_value {
            return (vec![], current_value, 1);
        }
        let (mut included_solution, mut included_value, mut included_calls) = (vec![], 0, 1);
        if current_weight + items[next_index as usize].weight <= allowed_weight {
            items[next_index as usize].is_selected = true;
            (included_solution, included_value, included_calls) = do_branch_and_bound(
                items,
                allowed_weight,
                best_value,
                current_value + items[next_index as usize].value,
                current_weight + items[next_index as usize].weight,
                remaining_value - items[next_index as usize].value,
                next_index + 1,
            );
            if included_value > best_value {
                best_value = included_value;
            }
        }
        items[next_index as usize].is_selected = false;
        let (excluded_solution, excluded_value, excluded_calls) = do_branch_and_bound(
            items,
            allowed_weight,
            best_value,
            current_value,
            current_weight,
            remaining_value - items[next_index as usize].value,
            next_index + 1,
        );
        if included_value >= excluded_value {
            return (
                included_solution,
                included_value,
                included_calls + excluded_calls + 1,
            );
        } else {
            return (
                excluded_solution,
                excluded_value,
                excluded_calls + included_calls + 1,
            );
        }
    }
}



// Build the items' block lists.
fn make_block_lists(items: &mut Vec<Item>) {
    let mut id;
    for i in 0..items.len() {
        items[i].block_list = Vec::new();
        for j in 0..items.len() {
            if i != j {
                if items[i].value >= items[j].value && items[i].weight <= items[j].weight {
                    id = items[j].id;
                    items[i].block_list.push(id);
                }
            }
        }
    }
}



fn rods_technique(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i64) {
    make_block_lists(items);
    return do_rods_technique(items, allowed_weight, 0, 0, 0, sum_values(items, true), 0);
}



fn do_rods_technique(
    items: &mut Vec<Item>,
    allowed_weight: i32,
    mut best_value: i32,
    current_value: i32,
    current_weight: i32,
    remaining_value: i32,
    next_index: i32,
) -> (Vec<Item>, i32, i64) {
    if (next_index as usize) >= items.len() {
        return (copy_items(items), current_value, 1);
    } else {
        if current_value + remaining_value <= best_value {
            return (vec![], current_value, 1);
        }
        let (mut included_solution, mut included_value, mut included_calls) = (vec![], 0, 1);
        if items[next_index as usize].blocked_by == -1 {
            if current_weight + items[next_index as usize].weight <= allowed_weight {
                items[next_index as usize].is_selected = true;
                (included_solution, included_value, included_calls) = do_rods_technique(
                    items,
                    allowed_weight,
                    best_value,
                    current_value + items[next_index as usize].value,
                    current_weight + items[next_index as usize].weight,
                    remaining_value - items[next_index as usize].value,
                    next_index + 1,
                );
                if included_value > best_value {
                    best_value = included_value;
                }
            }
        }
        let mut target: usize;
        for i in 0..items[next_index as usize].block_list.len() {
            target = items[next_index as usize].block_list[i] as usize;
            if items[target].blocked_by == -1 {
                items[target].blocked_by = items[next_index as usize].id;
            }
        }
        items[next_index as usize].is_selected = false;
        let (excluded_solution, excluded_value, excluded_calls) = do_rods_technique(
            items,
            allowed_weight,
            best_value,
            current_value,
            current_weight,
            remaining_value - items[next_index as usize].value,
            next_index + 1,
        );
        for i in 0..items[next_index as usize].block_list.len() {
            target = items[next_index as usize].block_list[i] as usize;
            if items[target].blocked_by == items[next_index as usize].id {
                items[target].blocked_by = -1;
            }
        }
        if included_value >= excluded_value {
            return (
                included_solution,
                included_value,
                included_calls + excluded_calls + 1,
            );
        } else {
            return (
                excluded_solution,
                excluded_value,
                excluded_calls + included_calls + 1,
            );
        }
    }
}



fn rods_technique_sorted(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i64) {
    make_block_lists(items);

    // Sort so items with longer blocked lists come first.
    items.sort_by(|a, b| b.block_list.len().cmp(&a.block_list.len()));

    // Reset the items' IDs.
    for i in 0..items.len() {
        items[i].id = i as i32;
    }

    // Rebuild the blocked lists with the new indices.
    make_block_lists(items);

    return do_rods_technique(items, allowed_weight, 0, 0, 0, sum_values(items, true), 0);
}



// Use dynamic programming to find a solution.
// Return the best assignment, value of that assignment,
// and the number of function calls we made.
fn dynamic_programming(items: &mut Vec<Item>, allowed_weight: i32) -> (Vec<Item>, i32, i64) {
    let num_items = items.len();
    if num_items == 0 {
        return (copy_items(items), 0, 1);
    }

    let mut solution_value: Vec<Vec<i32>> = Vec::with_capacity(num_items);
    let mut prev_weight: Vec<Vec<i32>> = Vec::with_capacity(num_items);
    for _ in 0..num_items {
        solution_value.push(vec![0; (allowed_weight + 1) as usize]);
        prev_weight.push(vec![0; (allowed_weight + 1) as usize]);
    }

    for w in 0..=allowed_weight {
        if items[0].weight <= w {
            solution_value[0][w as usize] = items[0].value;
            prev_weight[0][w as usize] = -1;
        } else {
            solution_value[0][w as usize] = 0;
            prev_weight[0][w as usize] = w;
        }
    }

    // Fill in the remaining table rows.
    for i in 1..num_items {
        for w in 0..=allowed_weight {
            let value_without_i = solution_value[i - 1][w as usize];
            let mut value_with_i = 0;
            if items[i].weight <= w {
                value_with_i = solution_value[i - 1][(w - items[i].weight) as usize] + items[i].value;
            }
            if value_without_i >= value_with_i {
                solution_value[i][w as usize] = value_without_i;
                prev_weight[i][w as usize] = w;
            } else {
                solution_value[i][w as usize] = value_with_i;
                prev_weight[i][w as usize] = w - items[i].weight;
            }
        }
    }

    let mut back_i = (num_items as i32) - 1;
    let mut back_w = allowed_weight;

    while back_i >= 0 {
        if back_w == prev_weight[back_i as usize][back_w as usize] {
            items[back_i as usize].is_selected = false;
        } else {
            items[back_i as usize].is_selected = true;
            back_w = prev_weight[back_i as usize][back_w as usize];
        }
        back_i -= 1;
    }

    return (copy_items(items), solution_value[num_items - 1][allowed_weight as usize], 1);
}



use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};

const NUM_ITEMS: i32 = 1000;

const MIN_VALUE: i32 = 1;
const MAX_VALUE: i32 = 10;
const MIN_WEIGHT: i32 = 4;
const MAX_WEIGHT: i32 = 10;

struct Item {
    id: i32,
    value: i32,
    weight: i32,
    is_selected: bool,
    blocked_by: i32,
    block_list: Vec<i32>,
}

fn main() {
    // Prepare a Prng using the same seed each time.
    let mut prng = Prng { seed: 1337 };
    prng.randomize();

    // Make some random items.
    let mut items = make_items(
        &mut prng, NUM_ITEMS, MIN_VALUE, MAX_VALUE, MIN_WEIGHT, MAX_WEIGHT,
    );
    let allowed_weight = sum_weights(&items, true) / 2;

    // Display basic parameters.
    println!("*** Parameters ***");
    println!("# items:        {}", NUM_ITEMS);
    println!("Total value:    {}", sum_values(&items, true));
    println!("Total weight:   {}", sum_weights(&items, true));
    println!("Allowed weight: {}", allowed_weight);
    print_items(&items, true);
    println!();

    // Exhaustive search
    if NUM_ITEMS > 23 {
        // Only run exhaustive search if num_items is small enough.
        println!("Too many items for exhaustive search\n");
    } else {
        println!("*** Exhaustive Search ***");
        run_algorithm(&exhaustive_search, &mut items, allowed_weight);
    }

    if NUM_ITEMS > 40 {
        // Only run branch and bound search if num_items is small enough.
        println!("Too many items for branch and bound search\n");
    } else {
        println!("*** Branch and Bound Search ***");
        run_algorithm(&branch_and_bound, &mut items, allowed_weight);
    }

    // Rod's technique
    if NUM_ITEMS > 60 {
        // Only run Rod's technique if num_items is small enough.
        println!("Too many items for Rod's technique\n");
    } else {
        println!("*** Rod's Technique ***");
        run_algorithm(&rods_technique, &mut items, allowed_weight);
    }

    // Rod's technique sorted
    if NUM_ITEMS > 200 {
        // Only run Rod's technique sorted if num_items is small enough.
        println!("Too many items for Rod's technique sorted\n");
    } else {
        println!("*** Rod's Technique Sorted***");
        run_algorithm(&rods_technique_sorted, &mut items, allowed_weight);
    }

    // Dynamic programming
    println!("*** Dynamic programming ***");
    run_algorithm(&dynamic_programming, &mut items, allowed_weight);
}
