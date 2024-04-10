fn perfect(arg: i64) -> bool {
    let mut sum = 0;
    let mut n = 1;
    loop {
        if n >= arg {
            break;
        }
        if arg % n == 0 {
            sum += n;
        }
        n += 1;
    }
    sum == arg
}

fn main() {
    println!("{}", perfect(28));
}
