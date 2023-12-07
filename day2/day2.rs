use std::fs;

fn main() {
    println!("Hello mum!");
    let input = fs::read_to_string("2input.txt")
                .expect("Deberia haber podido leer el fichero");
    // part1(input.clone());
    // part2(input.clone());
    all_in_one(input.clone());
}

fn all_in_one(input: String) -> () {
    let mut sum_of_ids = 0;
    let mut power_of_sets = 0;

    for line in input.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let (game_id, sets) = line.split_once(": ").unwrap();
        for set in sets.split("; ") {
            for cube in set.split(", ") {
                let (num, color) = cube.split_once(" ").unwrap();
                if color == "red" {
                    red = red.max(num.parse::<u32>().unwrap());
                }
                if color == "green" {
                    green = green.max(num.parse::<u32>().unwrap());
                }
                if color == "blue" {
                    blue = blue.max(num.parse::<u32>().unwrap());
                }
            }
        }
        if red <= 12 && green <= 13 && blue <= 14 {
            sum_of_ids += game_id.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        }
        power_of_sets += red * green * blue;
    }
    println!("Part1 result: {sum_of_ids}");
    println!("Part2 result: {power_of_sets}");
}

fn part1(input: String) -> () {
    let mut sum_of_ids = 0;

    for line in input.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let (game_id, sets) = line.split_once(": ").unwrap();
        for set in sets.split("; ") {
            for cube in set.split(", ") {
                let (num, color) = cube.split_once(" ").unwrap();
                if color == "red" {
                    red = red.max(num.parse::<u32>().unwrap());
                }
                if color == "green" {
                    green = green.max(num.parse::<u32>().unwrap());
                }
                if color == "blue" {
                    blue = blue.max(num.parse::<u32>().unwrap());
                }
            }
        }
        if red <= 12 && green <= 13 && blue <= 14 {
            sum_of_ids += game_id.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        }
    }
    println!("Part1 result: {sum_of_ids}");
}


fn part2(input: String) -> () {
    let mut sum_of_ids = 0;

    for line in input.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let (_game_id, sets) = line.split_once(": ").unwrap();
        for set in sets.split("; ") {
            for cube in set.split(", ") {
                let (num, color) = cube.split_once(" ").unwrap();
                if color == "red" {
                    red = red.max(num.parse::<u32>().unwrap());
                }
                if color == "green" {
                    green = green.max(num.parse::<u32>().unwrap());
                }
                if color == "blue" {
                    blue = blue.max(num.parse::<u32>().unwrap());
                }
            }
        }
        sum_of_ids += red * green * blue;
    }
    println!("Part2 result: {sum_of_ids}");
}
