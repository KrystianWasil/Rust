fn swap(x: &mut i32, y: &mut i32) {
    let temp;
    temp = *x;
    *x = *y;
    *y = temp; //
}

fn main() {
    let mut x = 123;
    let mut y = 2;
    swap(&mut x, &mut y);
    println!("{},{}", x, y);
}
