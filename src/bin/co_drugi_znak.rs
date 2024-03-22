fn co_drugi_znak(napis: &str) -> String {
    //     let mut string = "".to_string();
    //     for i in napis.chars().step_by(2) {
    //         string.push(i);
    //     }
    //    string
    napis.chars().step_by(2).collect()
}
fn main() {
    let s = "abcdefghijk";
    println!("{:?}", co_drugi_znak(s));
}
