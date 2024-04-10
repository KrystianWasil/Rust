fn wyswietl(t: &[i128]) {
    println!("{:?}", t);
}

fn rand(seed: &mut i128, min_rand: i128, max_rand: i128) -> i128 {
    *seed = *seed % (max_rand - min_rand + 1) + min_rand;
    *seed
}

fn rand_perm(arr: &mut [i128], seed: &mut i128) {
    let mut j;
    for i in 0..arr.len() {
        j = rand(seed, 0, arr.len() as i128);
        let temp = arr[i];
        arr[i] = arr[j as usize];
        arr[j as usize] = temp;
    }
}

fn main() {
    let mut array: [i128; 6] = [1, 2, 3, 4, 5, 6];
    let mut seed = 3;
    wyswietl(&array);
    rand_perm(&mut array, &mut seed);
    wyswietl(&array);
    rand_perm(&mut array, &mut seed);
    wyswietl(&array);
    rand_perm(&mut array, &mut seed);
    wyswietl(&array);
    rand_perm(&mut array, &mut seed);
    wyswietl(&array);
}
