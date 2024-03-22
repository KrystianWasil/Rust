fn main() {
    let a = 12;
    let b = 16;
    let c = 20;

    for i in 1..a {
        for j in 1..b {
            for k in 1..c {
                if i * i == j * j + k * k {
                    println!("{},{},{}", i, j, k);
                    continue;
                }
            }
        }
    }
}
