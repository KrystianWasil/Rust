fn armstrong_number(arg: i128) -> bool {
    let mut n = 0;
    let mut temp = arg;
    let mut number;
    let mut sum = 0;

    while temp > 0 {
        temp /= 10;
        n += 1;
    }

    temp = arg;
    while temp > 0 {
        number = temp % 10;
        sum += number.pow(n);
        temp /= 10;
    }
    sum == arg
}

fn main() {
    println!("{}", armstrong_number(54748));
    // armstrong_number(54748);
}
