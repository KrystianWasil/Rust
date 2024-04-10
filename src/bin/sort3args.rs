fn swap(x: &mut i32, y: &mut i32) {
    let temp;
    temp = *x;
    *x = *y;
    *y = temp; //
}

fn sort(a: &mut i32, b: &mut i32, c: &mut i32) {
    if a >= b {
        swap(a, b);
    } else if b >= c {
        swap(b, c);
    } else if a >= c {
        swap(a, c);
    }
}

fn main() {
    let mut a = 10;
    let mut b = 2;
    let mut c = 12;
    println!("{},{}.{}", a, b, c);
    sort(&mut a, &mut b, &mut c);
    println!("{},{},{}", a, b, c);
}
