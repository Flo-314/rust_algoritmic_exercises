fn main() {
    let ten_collatz: u128 = collatz(1);
    println!("Hello, world! {}", ten_collatz);
}

pub fn collatz(n: u128) -> u128 {
    let mut n = n;
    let mut collatz_counter = 0;
    while n != 1 {
        collatz_counter += 1;
        if n % 2 == 0 {
            n = n / 2
        } else {
            n = n * 3 + 1
        }
    }
    collatz_counter
}
