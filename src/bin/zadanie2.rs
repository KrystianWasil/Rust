fn main() {
    let month = 9;
    let year = 2023;

    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => println!("{} days", 31),
        4 | 6 | 9 | 11 => println!("{} days", 30),
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                println!("{} days", 29);
            } else {
                println!("{} days", 28);
            }
        }
        _ => println!("Invalid month"),
    }
}
