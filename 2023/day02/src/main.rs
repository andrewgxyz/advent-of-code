use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_id (str: &str) -> i32 {
    let number: Vec<&str> = str.split(" ").collect();

    number[1].parse::<i32>().unwrap()
}

fn is_game_possible (str: String) -> i32 {
    // 0: ID 1: rounds
    let split_game_rounds: Vec<&str> = str.split(": ").collect();
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;

    let rounds: Vec<&str> = split_game_rounds[1].split("; ").collect();

    for round in rounds {
        let split_num: Vec<&str> = round.split(", ").collect();

        for num in split_num {
            let number: Vec<&str> = num.split(" ").collect();
            let parse_num = number[0].parse::<i32>().unwrap();
            if num.contains("red") {
                if parse_num > red {
                    red = parse_num;
                }
            } else if num.contains("green") {
                if parse_num > green {
                    green = parse_num;
                }
            } else if num.contains("blue") {
                if parse_num > blue{
                    blue = parse_num;
                }
            }
        }

    }

    // if red > 12 || green > 13 || blue > 14 {
    //     print!("{} is not possible\n", split_str[0]);
    //     return 0;
    // }

    red * blue * green
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut total = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            total += is_game_possible(line.unwrap())
        }
    }

    println!("{}", total.to_string())
}
