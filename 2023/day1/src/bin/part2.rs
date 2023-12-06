use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn find_spelled_number (str: &str, num: &str) -> bool {
    if str.len() < num.len() {
        return false
    }

    let new_str = &str[..num.len()];

    if new_str == num {
        return true;
    }

    false
}

fn get_number (line: String) -> String {
    let mut nums: Vec<char> = vec![];
    let mut index: usize = 0;
    let mut test_str: &str = &"";

    for cha in line.chars() {
        if cha.is_digit(10) {
            nums.push(cha);
            index += 1;
            continue;
        }

        if cha == 'o' || cha == 't' || cha == 'f' || cha == 's' || cha == 'e' || cha == 'n' {
            test_str = &line[index..];
        }

        if cha == 'o' {
            if find_spelled_number(test_str, "one") {
                nums.push('1');
            }
        } else if cha == 't' {
            if find_spelled_number(test_str, "two") {
                nums.push('2');
            } else if find_spelled_number(test_str, "three") {
                nums.push('3');
            }
        } else if cha == 'f' {
            if find_spelled_number(test_str, "four") {
                nums.push('4');
            } else if find_spelled_number(test_str, "five") {
                nums.push('5');
            }
        } else if cha == 's' {
            if find_spelled_number(test_str, "six") {
                nums.push('6');
            } else if find_spelled_number(test_str, "seven") {
                nums.push('7');
            }
        } else if cha == 'e' {
            if find_spelled_number(test_str, "eight") {
                nums.push('8');
            }
        } else if cha == 'n' {
            if find_spelled_number(test_str, "nine") {
                nums.push('9');
            }
        }

        index += 1;
    }


    let mut num = String::from(nums[0]);
    num.push(nums[nums.len() - 1]);

    println!("{}: {:?} {:?}", line, nums, num);

    return num
}

fn main() {
    let mut total = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            total += get_number(line.unwrap()).to_string().parse::<i32>().unwrap()
        }
    }

    println!("{}", total.to_string())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
