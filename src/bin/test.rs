fn powiekszona_o_1(mut x: i32) -> i32 {
    x += 1;
    x
}

fn powieksz_o_1(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut a = 12;

    let b = powiekszona_o_1(a);
    println!("{}", b == 13);
    println!("{}", a == 12);

    powieksz_o_1(&mut a);
    println!("{}", a == 13);
    powieksz_o_1(&mut a);
    println!("{}", a == 14);
}
