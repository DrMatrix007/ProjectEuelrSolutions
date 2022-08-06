fn is_prime(i: u128) -> bool {
    for x in 2..((i as f64).sqrt() as u128) {
        if i % x == 0 {
            return false;
        }
    }
    true
}

fn max_prime(val: u128) -> Option<u128> {
    let mut i = 2;
    while val / i != 0 {
        if val % (val / i) == 0 && is_prime(val / i) {
            return Some(val / i);
        }

        i += 1;
    }
    None
}

fn main() {
    println!("{:?}", max_prime(600851475143));
}
