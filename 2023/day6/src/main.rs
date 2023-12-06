use std::fs;

fn parse_races (str: String) -> Vec<(i64, i64)> {
    // 0: ID 1: rounds
    let lines: Vec<Vec<&str>> = str.lines().map(|d| {
        let remove_title: Vec<&str> = d.split(":").collect();

        remove_title[1].split_whitespace().collect()
    }).collect();
    let mut races: Vec<(i64, i64)> = vec![];

    for (i, line) in lines[0].iter().enumerate() {
        races.push((lines[0][i].parse::<i64>().unwrap(), lines[1][i].parse::<i64>().unwrap()));
    }

    races
}

fn merge_races_into_one (races: Vec<(i64, i64)>) -> (i64, i64) {
    let mut time = "".to_string();
    let mut distance = "".to_string();

    for (t, d) in races {
        time.push_str(&t.to_string());
        distance.push_str(&d.to_string());
    }

    let n_time = time.parse::<i64>().unwrap();
    let n_distance = distance.parse::<i64>().unwrap();

    (n_time, n_distance)
}

fn calculate_possible_wins ((time, distance): (i64, i64)) -> i64 {
    let mut MmPMs = 0;
    let mut total_wins = 0;


    for i in 0..time {
        MmPMs = i;
        let current_distance = MmPMs * (time - i);
        println!("remaining time {}, total distance to end {}", (time - i), current_distance);
        // 
        // if MmPMs < distance / time {
        //     break;
        // }

        if current_distance > distance {
            total_wins += 1;
        }
    }

    total_wins
}

fn part2 (str: String) -> i64 {
    let mut total = 0;
    let race = merge_races_into_one(parse_races(str));

    println!("{:?}", race);

    calculate_possible_wins(race)
}

fn part1 (str: String) -> i64 {
    let mut total = 0;
    let races = parse_races(str);
    let mut num_of_wins: Vec<i64> = vec![];

    for race in races {
        num_of_wins.push(calculate_possible_wins(race))
    }

    println!("{:?}", num_of_wins);

    for (i, num) in num_of_wins.iter().enumerate() {
        if i == 0 {
            total = *num;
            continue;
        }

        total *= num;
        println!("{}", total);
    }

    total
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
    static TEST_DATA: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_DATA.to_string()), 288)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_DATA.to_string()), 71503)
    }
}
