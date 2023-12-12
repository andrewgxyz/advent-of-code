use std::{fs, collections::HashMap};

fn all_zeroes (nums: &Vec<i32>) -> bool {
    let mut zeroes = 0;

    for num in nums.clone() {
        if num == 0 {
            zeroes += 1;
        }
    }

    zeroes == nums.len()
}

fn part1 (str: String) -> i32 {
    let mut count = 0;
    for line in str.lines() {
        let mut differences: Vec<Vec<i32>> = vec![];
        let mut nums: Vec<i32> = line.split(' ').map(|a| a.parse::<i32>().unwrap()).collect();
        differences.push(nums);


        while !all_zeroes(&differences[differences.len() - 1]) {
            let mut new_diff: Vec<i32> = vec![];
            for (i, num) in differences[differences.len() - 1].iter().enumerate() {
                if i > 0 {
                    new_diff.push(num - differences[differences.len() - 1][i - 1]);
                }
            }
            differences.push(new_diff);
        }

        differences.reverse();

        for (i, diff) in differences.clone().iter().enumerate() {
            let last = diff.last().unwrap();
            let mut num = 0;
            if i > 0 {
                num = *differences[i - 1].last().unwrap();

            }
            differences[i].push(*last + num)

        }

        differences.reverse();

        count += differences[0].last().unwrap();

        println!("{:?}", differences);
    }

    count
}

fn part2 (str: String) -> i32 {
    let mut count = 0;
    for line in str.lines() {
        let mut differences: Vec<Vec<i32>> = vec![];
        let mut nums: Vec<i32> = line.split(' ').map(|a| a.parse::<i32>().unwrap()).collect();
        differences.push(nums);


        while !all_zeroes(&differences[differences.len() - 1]) {
            let mut new_diff: Vec<i32> = vec![];
            for (i, num) in differences[differences.len() - 1].iter().enumerate() {
                if i > 0 {
                    new_diff.push(num - differences[differences.len() - 1][i - 1]);
                }
            }
            differences.push(new_diff);
        }

        differences.reverse();

        for (i, diff) in differences.clone().iter().enumerate() {
            let first = diff.first().unwrap();
            let mut num = 0;
            if i > 0 {
                num = *differences[i - 1].first().unwrap();

            }
            differences[i].insert(0, *first - num )

        }

        differences.reverse();

        count += differences[0].first().unwrap();

        println!("{:#?}", differences);
    }

    count
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
    static TEST_DATA: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_all_zero() {
        assert_eq!(all_zeroes(&vec![0, 0, 0]), true)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_DATA.to_string()), 114)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("5 10 13 16 21 30 45".to_string()), 5)
    }
}
