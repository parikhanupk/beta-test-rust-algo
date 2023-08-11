use std::env;

static mut DEBUG_CALLS_GCD: u64 = 0;



fn gcd(a: i64, b: i64, debug: bool) -> i64 {
    if debug {
        unsafe {
            DEBUG_CALLS_GCD += 1;
        }
        println!("gcd({a}, {b})");
    }
    if b == 0 {
        return match a < 0 {
            true => -a,
            false => a,
        };
    } else {
        return gcd(b, a % b, debug);
    }
}



fn lcm(a: i64, b: i64, gcdv: i64) -> i64 {
    let res = (a / gcdv) * b;
    return match res < 0 {
        true => -res,
        false => res,
    };
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: rsa_gcd_lcm <a> <b>");
        return;
    }
    let (a, b, debug) = (args[1].parse().unwrap(), args[2].parse().unwrap(), false);
    let gcdv = gcd(a, b, debug);
    println!("GCD({}, {}) = {}", a, b, gcdv);
    println!("LCM({}, {}) = {}", a, b, lcm(a, b, gcdv));
}
