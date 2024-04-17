fn fun(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x >= 0.0 && x < 2.0 {
        x
    } else if x >= 2.0 && x <= 4.0 {
        1.0 + ((x-1.0) * (x-1.0))
    } else {
        x + 6.0
    }
}


fn main() {
    let x = 7;
    let y = -1;
    let z = 0.3;
    let q = 3.0;
    println!("{}",fun(x as f64));
    println!("{}",fun(y as f64));
    println!("{}",fun(z as f64));
    println!("{}",fun(q));
}