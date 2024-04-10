fn wizytowka(imie: &str, nazwisko: &str) -> String {
    let pierwsza_litera: String  = imie.chars().take(1).collect();
    let pierwsza_litera: String = pierwsza_litera.to_ascii_uppercase();
    let pierwsza_nazwisko: String = nazwisko.chars().take(1).collect();
    let pierwsza_nazwisko: String = pierwsza_nazwisko.to_ascii_uppercase();
    let reszta_nazwiska: String = nazwisko.chars().skip(1).collect();
    pierwsza_litera + ". " + &pierwsza_nazwisko + &reszta_nazwiska

}


fn main() {
println!("{}",wizytowka("Krystian", "Wasil"));


}