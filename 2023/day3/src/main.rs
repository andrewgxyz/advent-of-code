use std::fs;

#[derive(Debug)]
struct Node {
    value: char,
    x: usize,
    y: usize
}

fn is_valid_pos(curr_x: i32, curr_y: i32, max_x: i32, max_y: i32) -> bool {
    if curr_x < 0 || curr_y < 0 || curr_x > (max_x - 1) || curr_y > (max_y - 1) {
        return false;
    }

    true
}

fn get_adjacent (arr: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<Node> {
    let mut adjacent: Vec<Node> = vec![];
    let max_x = arr[0].len() as i32;
    let max_y = arr.len() as i32;
    let curr_x = x as i32;
    let curr_y = y as i32;

    if is_valid_pos(curr_x - 1, curr_y - 1, max_x, max_y) {
        adjacent.push(Node {
            value: arr[y - 1][x - 1],
            x: x - 1,
            y: y - 1
        })
    }
    if is_valid_pos(curr_x, curr_y - 1, max_x, max_y) {
        adjacent.push(Node {
            value: arr[y - 1][x],
            x,
            y: y - 1
        })
    }
    if is_valid_pos(curr_x + 1, curr_y - 1, max_x, max_y) {
        adjacent.push(Node {
            value: arr[y - 1][x + 1],
            x: x + 1,
            y: y - 1
        })
    }
    if is_valid_pos(curr_x + 1, curr_y, max_x, max_y) {
        adjacent.push(Node {
            value: arr[y][x + 1],
            x: x + 1,
            y
        })
    }
    if is_valid_pos(curr_x + 1, curr_y + 1, max_x, max_y) {
        adjacent.push(Node {
            value: arr[y + 1][x + 1],
            x: x + 1,
            y: y + 1
        })
    }
    if is_valid_pos(curr_x, curr_y + 1, max_x, max_y) {
        adjacent.push(Node {
            value: arr[y + 1][x],
            x,
            y: y + 1
        })
    }
    if is_valid_pos(curr_x - 1, curr_y + 1, max_x, max_y) {
        adjacent.push(Node {
            value: arr[y + 1][x - 1],
            x: x - 1,
            y: y + 1
        })
    }
    if is_valid_pos(curr_x - 1, curr_y, max_x, max_y) {
        adjacent.push(Node {
            value: arr[y][x - 1],
            x: x - 1,
            y
        })
    }

    adjacent
}

fn find_number(arr: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut num_vec: Vec<String> = vec![arr[y][x].to_string()];
    let max_x = arr[0].len() as i32;
    let max_y = arr.len() as i32;
    let mut curr_x = x;

    while arr[y][curr_x].is_numeric() {
        if is_valid_pos((curr_x - 1) as i32, y as i32, max_x, max_y) {
            if arr[y][curr_x - 1].is_numeric() {
                num_vec.insert(0, arr[y][curr_x - 1].to_string());
            } else {
                break;
            }

            curr_x -= 1;
        }
    }

    curr_x = x;

    while arr[y][curr_x].is_numeric() {
        if is_valid_pos((curr_x + 1) as i32, y as i32, max_x, max_y) {
            if arr[y][curr_x + 1].is_numeric() {
                num_vec.insert(num_vec.len(), arr[y][curr_x + 1].to_string());
            } else {
                break;
            }

            curr_x += 1;
        }
    }

    let num = num_vec.join("");
    num.parse::<i32>().unwrap_or(1)
}

fn part2 (line: &str) -> i32 {
    let data: Vec<Vec<char>> = line.split("\n").map(|e| e.split("").map(|a| a.chars().next().unwrap_or('.')).collect()).collect();
    let mut total = 0;

    let mut gears: Vec<Node> = vec![];

    for (y, row) in data.iter().enumerate() {
        for (x, col) in data[y].iter().enumerate() {
            if col == &'*' {
                gears.push(Node { value: col.clone(), x, y });
            }
        }
    }


    for gear in gears {
        let adj_arr = get_adjacent(&data, gear.x, gear.y);
        let mut part_nums: Vec<i32> = vec![];

        for adj in adj_arr {
            if adj.value.is_numeric() {
                part_nums.push(find_number(&data, adj.x, adj.y));
            }
        }

        part_nums.dedup();
        if part_nums.len() > 1 {
            total += part_nums[0] * part_nums[1]
        }
    }

    total
}

fn part1 (line: &str) -> i32 {
    // 0: ID 1: rounds
    let data: Vec<Vec<char>> = line.split("\n").map(|e| e.split("").map(|a| a.chars().next().unwrap_or('.')).collect()).collect();
    let mut total = 0;

    let mut has_symbol = false;
    let mut new_number: Vec<String> = vec![];
    // print!("{:?}", data);

    for (y, row) in data.iter().enumerate() {
        new_number = vec![];
        has_symbol = false;

        for (x, col) in data[y].iter().enumerate() {
            if col.is_numeric() {
                let adj_arr = get_adjacent(&data, x, y);
                for adj in adj_arr {
                    if has_a_symbol(adj.value) {
                        has_symbol = true;
                    }
                }
                new_number.push(col.to_string());
            } else {
                // println!("x: {} y: {} {:?} {}\n", x, y, new_number, has_symbol);
                if new_number.len() > 0 && has_symbol == true {
                    total += new_number.join("").parse::<i32>().unwrap();
                }

                new_number = vec![];
                has_symbol = false;
            }
        }
    }

    total
}

fn has_a_symbol(ch: char) -> bool {
    !ch.is_numeric() && !ch.eq(&'.')
}

fn main() {
    let mut total = 0;
    if let Ok(lines) = fs::read_to_string("./input") {
        total = part2(&lines)
    }

    println!("{}", total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_symbol() {
        let mut total: Vec<bool> = vec![];
        let test_data = ".*$/&%()-_=@^!";

        for chars in test_data.chars() {
            total.push(has_a_symbol(chars));
        }

        assert_eq!(total, [false, true, true, true, true, true, true, true, true, true, true, true, true, true])
    }

    #[test]
    fn part1_test() {
        let mut total = 0;
        let test_data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        total = part1(test_data);
        println!("\n{}", total);

        assert_eq!(total, 4361)
    }

    #[test]
    fn part2_test() {
        let mut total = 0;
        let test_data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        total = part2(test_data);
        println!("\n{}", total);

        assert_eq!(total, 467835)
    }
}
