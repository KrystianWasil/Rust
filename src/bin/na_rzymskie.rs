fn na_rzymskie(mut liczba: u32) -> String {
    let mut rzymskie: String = String::new();
    let wartosci = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    for &(wartosc, rzymska) in wartosci.iter() {
        while liczba >= wartosc {
            rzymskie.push_str(rzymska);
            liczba -= wartosc;
        }
    }

    rzymskie
}

fn main() {
    println!("{}", na_rzymskie(3));    // Output: III
    println!("{}", na_rzymskie(9));    // Output: IX
    println!("{}", na_rzymskie(19));   // Output: XIX
    println!("{}", na_rzymskie(1999)); // Output: MCMXCIX
}
