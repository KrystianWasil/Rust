fn pow_mod(x: u128, n: u128, p: u128) -> u128 {
    if p == 1 {
        return 0; // if p == 1
    }

    let mut result = 1;
    let mut base = x % p;

    let mut power = n;
    while power > 0 {
        if power % 2 == 1 {
            result = (result * base) % p;
        }

        power >>= 1;
        base = (base * base) % p;
    }

    result
}

fn main() {
    let x = 5;
    let n = 3;
    let p = 13;
    let result = pow_mod(x as u128, n as u128, p as u128);
    println!("({}^{}) % {} = {}", x, n, p, result);
}
