fn dodaj_pisemnie(a: &str, b: &str) -> String {
    let mut a1: String = a.chars().rev().collect();
    let mut b1: String = b.chars().rev().collect();
    // dodaje 0 zeby moc dodawac cos
    while a1.len() != b1.len() {
        if a1.len() < b1.len() {
            a1.push('0');
        } else {
            b1.push('0');
        }
    }

    let mut result = String::new();
    let mut carry = 0; 

    for (char_a, char_b) in a1.chars().zip(b1.chars()) {
        let digit_a = char_a.to_digit(10).unwrap();
        let digit_b = char_b.to_digit(10).unwrap();
        let sum = digit_a + digit_b + carry;

        result.push_str(&(sum % 10).to_string()); // czesc jednosci dodana do wyniku
        carry = sum / 10; //przeniezienie dzesiateek (o ile istnieje ale to adlej)
    }

    // dodanei tego prseniesienia
    if carry > 0 {
        result.push_str(&carry.to_string());
    }
    result.chars().rev().collect()
}

fn main() {
    let string1 = "1239".to_string();
    let string2 = "9954".to_string();
    println!("{}", dodaj_pisemnie(&string1, &string2)); 
}
