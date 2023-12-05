use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
                .expect("Deberia haber podido leer el fichero owo");
    let mut aux_val = 0;

    for word in input.split("\n") {
        for char in word.chars() {
            if char.is_numeric() {
                aux_val += char.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for char in word.chars().rev() {
            if char.is_numeric() {
                aux_val += char.to_digit(10).unwrap();
                break;
            }
        }
    }
    println!("The sum is: {aux_val}");
}
