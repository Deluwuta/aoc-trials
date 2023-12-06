use std::fs;

fn main() {
    println!("Hola mama!");
    let input = fs::read_to_string("1input.txt")
                .expect("Deberia haber podido leer el fichero owo");
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: String) -> () {
    let mut aux_val = 0;
    let mut a = 0;
    let mut b = 0;

    for word in input.lines() {
        for char in word.chars() {
            if char.is_numeric() {
                a = char.to_digit(10).unwrap();
                break;
            }
        }
        for char in word.chars().rev() {
            if char.is_numeric() {
                b = char.to_digit(10).unwrap();
                break;
            }
        }
        aux_val += a * 10 + b;
    }
    println!("La suma es {aux_val}");
}

fn part2(input: String) -> () { // Now you should be able to read spelled numbers
    let nums = [
        "one", "two", "three",
        "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let mut aux_val = 0;
    let mut a = 0; 
    let mut b = 0;

    for mut word in input.lines() {
        // Reduction
        while !nums.iter().any(|x| word.starts_with(x)) &&
            !word.chars().next().unwrap().is_numeric() {
                word = &word[1..];
            }
        while !nums.iter().any(|x| word.ends_with(x)) &&
            !word.chars().last().unwrap().is_numeric() {
                word = &word[.. word.len() - 1];
            }

        // Getting the number spelled
        for (index, num) in nums.iter().enumerate() {
            if word.starts_with(num) {
                a = index + 1;
            }
            if word.ends_with(num) {
                b = index + 1;
            }
        }

        // Getting the number as numeric
        let mut c = word.chars().next().unwrap();
        if c.is_numeric() {
            a = c.to_digit(10).unwrap() as usize;
        }
        c = word.chars().last().unwrap();
        if c.is_numeric() {
            b = c.to_digit(10).unwrap() as usize;
        }
        aux_val += a * 10 + b;
    }
    println!("La suma es: {aux_val}");
}
