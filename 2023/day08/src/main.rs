use std::{fs, collections::HashMap};

fn get_number_steps (hash_map: HashMap<String, (String, String)>, directions: Vec<char>, start: &str, end: &str) -> i32 {
    let length = directions.len();
    let mut index = 0;
    let mut steps = 0;
    let mut curr_location = start.to_string().clone();

    while curr_location != end {
        curr_location = switch_position(&hash_map, &curr_location, directions[index]);

        if index == length - 1 {
            index = 0;
        } else {
            index += 1;
        }
    }

    steps
}

fn create_directions_map (mut str: String) -> (HashMap<String, (String, String)>, Vec<char>) {
    str = str.replace("(", "");
    str = str.replace(")", "");
    let split_items: Vec<&str> = str.split("\n\n").collect();
    let directions: Vec<char> = split_items[0].chars().collect();
    let mut hash_locations: HashMap<String, (String, String)> = HashMap::new();
    
    for line in split_items[1].lines() {
        let new_split: Vec<&str> = line.split(" = ").collect();

        new_split[1].to_string();
        let new_split_locations: Vec<&str> = new_split[1].split(", ").collect();

        let left = new_split_locations[0].to_string().clone();
        let right = new_split_locations[1].to_string().clone();
        let destinations = (left, right);

        hash_locations.insert(new_split[0].to_string(), destinations);
    }

    (hash_locations, directions)
}

fn are_all_z (positions: &Vec<String>) -> i32 {
    let mut all_zee = 0;

    for position in positions {
        if are_a_letter(position, 'Z') {
            all_zee += 1;
        }
    }

    all_zee
}

fn are_a_letter (pos: &String, letter: char) -> bool {
    let key_char: Vec<char> = pos.chars().collect();

    key_char[key_char.len() - 1] == letter
}

fn switch_position (hash_map: &HashMap<String, (String, String)>, key: &str, dir: char) -> String {
    match hash_map.get(key) {
        Some((left, right)) => {
            if dir == 'L' {
                return left.to_string();
            } else {
                return right.to_string();
            }
        },
        _ => "".to_string()
    }
}

fn part1 (str: String) -> i32 {
    let (hash_map, directions) = create_directions_map(str);

    get_number_steps(hash_map, directions, "AAA", "ZZZ")
}

fn part2 (str: String) -> i32 {
    let (hash_map, directions) = create_directions_map(str);

    let mut positions_a: Vec<String> = vec![];
    for key in hash_map.keys() {
        if are_a_letter(&key.to_string(), 'A') {
            positions_a.push(key.clone());
        }
    }

    let mut index = 0;
    let mut steps = 0;
    let d_length = directions.len() - 1;
    let mut pos_all: Vec<i64> = vec![];

    for pos in positions_a.into_iter()  {
        let mut current = pos;
        index = 0;
        steps = 0;

        while !current.ends_with('Z') {
            current = switch_position(&hash_map, &current, directions[index]);
            index = (index + 1) % directions.len();
            steps += 1
        }

        let mut first = true;

        while first || !current.ends_with('Z') {
            current = switch_position(&hash_map, &current, directions[index]);
            index = (index + 1) % directions.len();
            steps += 1;
            first = false;
        }

        println!("{:?}, {}", current, steps);
        
        pos_all.push(steps);
        println!("{:?}", pos_all);
    }

    println!("{:?}", pos_all);

    0
}

fn main() {
    let mut total = 0;
    if let Ok(lines) = fs::read_to_string("./input") {
        total = part2(lines);
    }

    println!("{}", total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_DATA: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_DATA.to_string()), 6)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)".to_string()), 6)
    }
}
