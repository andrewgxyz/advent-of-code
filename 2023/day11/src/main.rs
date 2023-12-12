use std::fs;

#[derive(Debug, Clone)]
struct Node {
    value: char,
    x: usize,
    y: usize
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.x == other.x && self.y == other.y
    }

}

#[derive(Debug, Clone)]
struct Distance <T> {
    value: T,
    a_point: Node,
    b_point: Node
}

fn part1 (str: String) -> i32 {
    let mut data: Vec<Vec<char>> = str.split("\n").map(|a| a.chars().collect()).collect();
    let mut grid: Vec<Node> = vec![];
    let mut distances: Vec<Distance<i32>>  = vec![];
    let mut missing_x: Vec<usize> = (0..data.len()).collect();
    let mut missing_y: Vec<usize> = (0..data[0].len()).collect();

    for (y, row) in data.clone().iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if data[y][x] == '#' {
                grid.push(Node {
                    value: data[y][x],
                    x,
                    y
                });
            }
        }
    }

    for point in grid.iter() {
        if missing_x.contains(&point.x) {
            let index = missing_x.iter().position(|a| a == &point.x).unwrap();
            missing_x.remove(index);
        }

        if missing_y.contains(&point.y) {
            let index = missing_y.iter().position(|a| a == &point.y).unwrap();
            missing_y.remove(index);
        }
    }

    println!("{:?} {:?}", missing_x, missing_y);
    // println!("{:?} {}x{}", data, data[0].len(), data.len());

    let mut new_index = 0;
    for x in missing_x.clone()  {
        for i in 0..data.len() - 1 {
            let idx = x + new_index;
            println!("{}, {}", new_index, x);
            data[i].insert(idx, '.');
        }

        new_index += 1;
    }

    for y in missing_y.clone()  {
        let mut empty_row: Vec<char> = vec![];
        for i in 0..data[0].len() {
            empty_row.push('.');
        }
        data.insert(y, empty_row);
    }

    grid = vec![];

    for (y, row) in data.clone().iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if data[y][x] == '#' {
                grid.push(Node {
                    value: data[y][x],
                    x,
                    y
                });
            }
        }
    }

    println!("{:?} {}x{}", data, data[0].len(), data.len());
    println!("{:?}", missing_x);
    println!("{:?}", missing_y);

    for a_point in grid.iter()  {
        for b_point in grid.iter() {
            if a_point.x == b_point.x && a_point.y == b_point.y {
                continue;
            }

            if distances.iter().find(|a| {
                a.a_point == *a_point && a.b_point == *b_point || a.a_point == *b_point && a.b_point == *a_point 
            }).is_some() {
                continue;
            }

            let mut distance_x = (b_point.x as i32 - a_point.x as i32) as i32;
            let mut distance_y = (b_point.y as i32 - a_point.y as i32) as i32;

            if distance_x < 0 {
                distance_x *= -1;
            }

            if distance_y < 0 {
                distance_y *= -1;
            }

            // println!("point A: {:?}, point B: {:?} : {}", a_point, b_point, distance_x + distance_y);
            distances.push(Distance {
                value: distance_x + distance_y,
                a_point: a_point.clone(),
                b_point: b_point.clone()
            });

        }
}

    let mut sum = 0;

    for distance in distances.iter() {
        sum += distance.value;
    }

    println!("{} of {}", sum, distances.len());

    sum
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
    static TEST_DATA: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_DATA.to_string()), 374)
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("5 10 13 16 21 30 45".to_string()), 5)
    // }
}
