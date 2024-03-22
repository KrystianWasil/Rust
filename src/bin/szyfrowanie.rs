fn szyfruj(napis: &str, klucz: usize) -> String {
    let temp1: String = napis.chars().take(klucz).collect();
    let temp1: String = temp1.chars().rev().collect();
    let temp2: String = napis.chars().skip(klucz).collect();
    let temp2: String = temp2.chars().rev().collect();
    temp1 + &temp2
    //juz czaje czemu nie 
    
}

fn main() {
    let s = "Aladyn".to_string();
    println!("{}",szyfruj(&s, 3));
    println!("============");
    println!("{}",szyfruj(&s, 5));
    println!("============");
    println!("{}",szyfruj(&s, 3));
    println!("============");
    println!("{}",szyfruj(&s, 2));
    println!("============");
    
}
