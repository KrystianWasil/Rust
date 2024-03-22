fn main() {
    let mut n = 1234;
    while n > 0 {
        print!("{} ", n % 10);
        n = n / 10;
    }
}
