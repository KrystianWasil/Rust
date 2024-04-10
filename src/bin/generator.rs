fn rand(seed: &mut i128, min_rand: i128, max_rand: i128) -> i128 {
    *seed = *seed % (max_rand - min_rand + 1) + min_rand;
    *seed
}

fn main() {
    let mut seed = 5;
    let a = 12;
    let b = 125;
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
    println!("random = {}", rand(&mut seed, a, b));
}
