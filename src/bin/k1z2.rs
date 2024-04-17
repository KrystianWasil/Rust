fn dopelnienie_rna(dna: &str) -> String {
    let mut result = String::new();
  for i in dna.chars() {
    if i == 'A' {
        result.push('U');
    }
    else if i == 'C' {
        result.push('G');
    }
    else if i == 'G' {
        result.push('C');
    }
    else {
        result.push('A');
    }
  }
  result
}

fn main() {
    let dna = "AAGGTTAC";
    println!("{}",dopelnienie_rna(dna));

}