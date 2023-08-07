fn factorial(n: i64) -> i64 {
    return if n == 0 { 1 } else { n * factorial(n - 1) };
}



fn main() {
    for n in 0..22 {
        println!("{}! = {}", n, factorial(n));
    }
}
