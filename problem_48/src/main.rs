#![allow(dead_code)]

const VAL: u128 = 1e10 as u128;

fn pow(a: u128, modulo: u128) -> u128 {
    let mut ans = 1;
    for _ in 0..a {
        ans *= a;
        ans %= modulo;
    }
    ans
}

fn calc_pow(val: u128) -> u128 {
    (1..=val).map(|x| pow(x, VAL)).sum::<u128>() % VAL
}

fn main() {
    println!("{}",calc_pow(1000));
}
