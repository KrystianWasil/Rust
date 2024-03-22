fn f(x: f64) -> f64 {
    x.powf(2.0) - 1.0
}

fn fp(x: f64) -> f64 {
    2.0 * x
}

fn met_newt_for(x0: f64, eps: f64, n: u128, f: fn(f64) -> f64, fp: fn(f64) -> f64) -> f64 {
    let mut xk = x0;
    for _i in 0..n {
        if f(xk).abs() < eps {
            break;
        }
        xk = xk - f(xk) / fp(xk);
    }
    xk
}

fn main() {
    println!("result = {} ", met_newt_for(10.0, 0.000001, 100, f, fp));
}
