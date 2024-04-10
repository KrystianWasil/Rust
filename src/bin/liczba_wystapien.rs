fn liczba_wystapien(napis: &str, znak: char) -> usize {
    let mut counter = 0;
    for i in napis.chars() {
        if i == znak {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let napis = "Ala ma kota hehehe".to_string();
    let znak = 'a';
    println!("{}", liczba_wystapien(&napis, znak));
}
