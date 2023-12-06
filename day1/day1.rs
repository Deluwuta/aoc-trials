use std::fs;

fn main() {
    println!("Hola mama!");
    let input = fs::read_to_string("realInput.txt")
                .expect("Deberia haber podido leer el fichero owo");
    part1(input.clone());
    // part2(input.clone());
}

fn part1(input: String) -> () {
    let mut aux_val = 0;

    for word in input.lines() {
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
    println!("La suma es {aux_val}");
}
