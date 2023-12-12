use std::fs::{File, self};
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Card {
    id: i32,
    winning_num: Vec<usize>,
    num: Vec<usize>,
    matched: usize,
}

fn loop_through_winners (cards: &Vec<Card>, start: usize, end: usize, mut points: i32, nest: i32) -> i32 {
    let mut i = start;

    while i < end {
        if cards[i].matched > 0 {
            println!("loop {}: start {} end {} points {}", nest, start, end, points);
            let mut new_end = (i + 1) + cards[i].matched;
            if new_end > cards.len() {
                new_end = cards.len() - 1;
            }

            points = loop_through_winners(&cards, i+1, new_end, points + 1, nest + 1);
            points += 1;
        }
        i += 1;
    }

    points
}

fn part2 (str: String) -> i32 {
    // 0: ID 1: rounds
    let cards: Vec<Card> = str.lines().map(|e| {
        let card_id_split: Vec<&str> = e.split(": ").collect();
        let id: Vec<&str> = card_id_split[0].split_whitespace().collect();
        let rounds: Vec<&str> = card_id_split[1].split(" | ").collect();
        let winning_num: Vec<usize> = rounds[0].split_whitespace().map(|c| c.parse::<usize>().unwrap()).collect();
        let num: Vec<usize> = rounds[1].split_whitespace().map(|c| c.parse::<usize>().unwrap()).collect();

        let id_int = id[1].parse::<i32>().unwrap();
        let mut matched_num = 0;

        for winner in &winning_num {
            for card in &num {
                if card == winner {
                    matched_num += 1;
                }
            }
        }

        Card {
            id: id_int,
            winning_num,
            matched: matched_num,
            num
        }
    }).collect();
    // println!("{:#?}", cards);
    // return 0;

    println!("loop root: start {} end {} points {}", 0, cards.len(), 0);

    loop_through_winners(&cards, 0, cards.len(), 0, 1)
}

fn part1 (str: String, index: usize) -> i32 {
    // 0: ID 1: rounds
    let split_game_rounds: Vec<&str> = str.split(": ").collect();
    let mut points: i32 = 0;
    let mut wins = 0;
    let mut tally = 0;

    let rounds: Vec<&str> = split_game_rounds[1].split(" | ").collect();
    let winning_num: Vec<&str> = rounds[0].split_whitespace().collect();
    let card_num: Vec<&str> = rounds[1].split_whitespace().collect();

    // println!("{:?}\n{:?}", winning_num, card_num);
    let mut old_wins = wins;
    let mut double = false;
    let mut idx = 0;

    for winner in &winning_num {
        old_wins = wins;
        for card in &card_num {
            if card == winner {
                // print!("card {}, winner {} card {} wins {}\n", index + 1, winner, card, wins);
                wins += 1;
            }
        }

        idx += 1;
    }

    if wins > 0 {
        points = i32::pow(2, wins - 1)
    }

    print!("card {}, points {}\n", index + 1, points);

    points
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut total = 0;
    if let Ok(lines) = fs::read_to_string("./input2") {
        total = part2(lines);
    }

    println!("{}", total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let mut total = 0;
        let mut index = 0;

        for line in test_data.lines() {
            let points = part1(line.to_string(), index);

            total += points;
            index += 1;
        }

        assert_eq!(total, 13)
    }

    #[test]
    fn test_part2() {
        let test_data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part2(test_data.to_string()), 30)
    }
}
