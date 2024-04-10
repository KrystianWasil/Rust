fn wyswietl(t: &[i32]) {
    println!("{:?}", t);
}

fn swap(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn swap_arr(arr: &mut [i32], i: usize, j: usize) {
    let (left, right) = arr.split_at_mut(j);
    let x = &mut left[i];
    let y = &mut right[0];
    swap(x, y);
}

fn main() {
    let mut array = [1, 2, 3, 4, 5];
    swap_arr(&mut array, 2, 3);
    wyswietl(&array);
} //s
