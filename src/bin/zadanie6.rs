fn main() {
    let n = 7;
    let mut iter = 1;
    let mut result = 1;
    while iter <= n {
        result *= iter;
        iter += 1;
    }
    println!("{}", result);
}
