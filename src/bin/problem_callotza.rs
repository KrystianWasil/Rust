fn problem_collatza(iteration: i32, c: i32) -> i32 {
    if c == 1 {
        return iteration;
    }
    if c % 2 == 0 {
        problem_collatza(iteration + 1, c / 2)
    } else {
        problem_collatza(iteration + 1, 3 * c + 1)
    }
}

fn main() {
    println!("{}", problem_collatza(0, 12));
}
