fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    if z.is_empty() { return None; }
    
    let mut result = String::new();
    
    for i in z.chars().enumerate() {
        if i.0 < 4 {
            match i.1 {
                '0' => result.push_str(""),
                '1' => result.push_str("1"),
                '2' => result.push_str("10"),
                '3' => result.push_str("11"),
                '4' => result.push_str("100"),
                '5' => result.push_str("101"),
                '6' => result.push_str("110"),
                '7' => result.push_str("111"),
                _ => return None, 
            }
        } else {
            match i.1 {
                '0' => result.push_str("000"),
                '1' => result.push_str("001"),
                '2' => result.push_str("010"),
                '3' => result.push_str("110"),
                '4' => result.push_str("100"),
                '5' => result.push_str("101"),
                '6' => result.push_str("110"),
                '7' => result.push_str("111"),
                _ => return None, 
            }
        }
        
    }
    
    Some(result)
}

fn main() {
    let octal_number = "36";
    if let Some(binary) = zamien_syst8_na_syst2(octal_number) {
        println!("{} w systemie dwójkowym: {}", octal_number, binary);
    } else {
        println!("Nieprawidłowy numer w systemie ósemkowym");
    }
}
