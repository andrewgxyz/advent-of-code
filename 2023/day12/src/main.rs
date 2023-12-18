use std::fs;
use nom::{self, sequence::separated_pair, bytes::complete::is_a, character::complete::space1, multi::separated_list1, complete::{tag, self}, IResult};

fn part1 (str: &str) -> IResult<&str, ()> {
    let mut data: Vec<Vec<&str>> = str.split("\n")
        .map(|a| a.split(" ").collect()).collect();
    let mut total = 0;
    let (input, (line, batches)) = separated_pair(
        is_a("?.#"), 
        space1, 
        separated_list1(tag(","), complete::u32),
    )(input)?;

    println!("{:?}", data);

    for line in data {
        let combination: Vec<usize> = line[1].split(",").map(|n| n.parse::<usize>().unwrap()).collect();
    }

    0
}

fn part2 (str: String) -> i32 {
    0
}

fn main() {
    let mut total = 0;
    if let Ok(lines) = fs::read_to_string("./input") {
        total = part1(lines);
    }

    println!("{}", total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
//     static TEST_DATA: &str = "..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...";
    static TEST_DATA: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_DATA.to_string()), 21)
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("5 10 13 16 21 30 45".to_string()), 5)
    // }
}
