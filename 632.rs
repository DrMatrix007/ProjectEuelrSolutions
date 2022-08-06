fn is_prime(i: u128) -> bool {
    for x in 2..((i as f64).sqrt() as u128) {
        if i % x == 0 {
            return false;
        }
    }
    true
}

fn count_sqrs(val: u128) -> u128 {
    let mut ans = 0;
    for i in 2..=(val as f64).sqrt() as u128 {
        if val % (i * i) == 0 && is_prime(i) {
            ans += 1;
        }
    }
    ans
}

fn C(n: u128, k: u128) -> u128 {
    let mut ans = 0;

    for i in 1..=n {
        ans += if count_sqrs(i) == k { 1 } else { 0 }
    }
    ans
}

fn main() {
    // not working!
    println!("{}", 10000000000000000_u128%1000000007);

}
