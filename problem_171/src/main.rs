fn f(mut val: u128) -> u128 {
    let mut ans = 0;
    while val != 0 {
        ans += (val % 10) * (val % 10);
        val /= 10;
    }
    ans
}

fn is_square(f: u128) -> bool {
    (f64::sqrt(f as f64) as u128 as f64) == f64::sqrt(f as f64)
}

fn main() {
    let mut ans = 0;
    let mut val = 0;
    let max = 1e20 as u128;
    for i in 1..10 {
        ans += f64::log10(max as f64) as u128 *max/10 * i*i;
    }


    println!("{}",ans);
    println!("{}", is_square(442));
    println!("{}", is_square(3));
    println!("{}", is_square(25));
    println!("Hello, world!");
}
