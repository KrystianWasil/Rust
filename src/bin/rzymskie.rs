fn rzymskie(napis: &str) -> u128 {
    let mut result = 0;
    let mut prev_value = 0;

    for (index, current_char) in napis.chars().enumerate() {
        let current_value = match current_char {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => {
                println!("Nieznany znak: {}", current_char);
                0
            }
        };

        if current_value > prev_value && index > 0 {
            result += current_value - 2 * prev_value; // Odejmujemy dwukrotność poprzedniej wartości, ponieważ została ona już dodana
        } else {
            result += current_value;
        }

        prev_value = current_value;
    }

    result
}

fn main() {
    let liczba_rzymska = String::from("IX");
    let wynik = rzymskie(&liczba_rzymska);
    println!("Wynik: {}", wynik);
}
