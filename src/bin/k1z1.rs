fn suma_wielokr(n: i32, a: i32, b: i32) -> i32 {
    let mut suma = 0;
    for i in a..b {
       suma += if i % n == 0 {i} else {0};
    }
    suma
}

fn main() {
    //tesy rozne bloki bo mi sie nazw nie chce zmieniac {}
    {
    let n = 3;
    let a = 7;
    let b = 15;
    println!("{}",suma_wielokr(n,a,b)); 
    }
    {
    let n = 2;
    let a = 1;
    let b = 10;
    println!("{}",suma_wielokr(n,a,b)); 
    }
}
