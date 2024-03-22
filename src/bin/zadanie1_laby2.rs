fn f(x: f64) -> f64 {
    x.powf(2.0) - 1.0
}

fn fp(x: f64) -> f64 {
    2.0 * x
}

fn met_newt_loop(x0: f64, eps: f64, n: u128) -> f64 {
    let mut xk = x0;
    let mut i: u128 = 0;
    loop {
        if f(xk).abs() < eps {
            return xk;
        }
        if i > n {
            break;
        }
        i += 1;
        xk = xk - f(xk) / fp(xk);
    }
    xk
}

fn met_newt_while(x0: f64, eps: f64, n: u128) -> f64 {
    let mut xk = x0;
    let mut i: u128 = 0;
    while i < n && f(xk).abs() >= eps {
        xk = xk - f(xk) / fp(xk);
        i += 1;
    }
    xk
}

fn met_newt_rec(x0: f64, eps: f64, n: u128, mut xk: f64) -> f64 {
    if n <= 0 {
        return xk;
    }
    if f(xk).abs() < eps {
        return xk;
    }
    xk = xk - f(xk) / fp(xk);
    met_newt_rec(x0, eps, n - 1, xk)
}
fn met_newt_for(x0: f64, eps: f64, n: u128) -> f64 {
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
    println!("{} -for loop", met_newt_loop(12.0, 0.000001, 20));
    println!("{} -for for", met_newt_for(12.0, 0.000001, 20));
    println!("{} - for while", met_newt_while(12.0, 0.000001, 20));
    println!("{} -for recursion", met_newt_rec(12.0, 0.000001, 20, 12.0));
}
