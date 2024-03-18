fn main() {
    let year = 2001;
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                println!("przestepny");
            } else {
                println!("nie jest przestepny");
            }
        } else {
            println!("przestepny");
        }
    } else {
        println!("nie jest przestepny");
    }
}
