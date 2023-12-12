use std::{fs, collections::HashMap};

#[derive(Debug, Clone)]
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

fn get_adjacent (arr: &Vec<Vec<char>>, curr: &Node) -> Vec<Node> {
    let (x, y) = (curr.x, curr.y);
    let mut adjacent: Vec<Node> = vec![];
    let max_x = arr[0].len() as i32;
    let max_y = arr.len() as i32;
    let curr_x = x as i32;
    let curr_y = y as i32;

    if is_valid_pos(curr_x, curr_y - 1, max_x, max_y) {
        if arr[y - 1][x] != '.' {
            adjacent.push(Node {
                value: arr[y - 1][x],
                x,
                y: y - 1
            })
        }
    }
    if is_valid_pos(curr_x + 1, curr_y, max_x, max_y) {
        if arr[y][x + 1] != '.' {
            adjacent.push(Node {
                value: arr[y][x + 1],
                x: x + 1,
                y
            })
        }
    }
    if is_valid_pos(curr_x, curr_y + 1, max_x, max_y) {
        if arr[y + 1][x] != '.' {
            adjacent.push(Node {
                value: arr[y + 1][x],
                x,
                y: y + 1
            })
        }
    }
    if is_valid_pos(curr_x - 1, curr_y, max_x, max_y) {
        if arr[y][x - 1] != '.' {
            adjacent.push(Node {
                value: arr[y][x - 1],
                x: x - 1,
                y
            })
        }
    }

    adjacent
}

#[derive(Debug)]
struct Pipe {
    from: (i32, i32),
    to: (i32, i32),
}

fn walk (data: Vec<Vec<char>>, curr_node: Node, mut seen: Vec<Vec<bool>>, pipes: HashMap<char, Pipe>) -> Vec<Pipe> {
    // Can't go back on previous step
    let mut paths: Vec<Pipe> = vec![];
    let mut curr: Node = curr_node;
    let dirs = Vec::from([
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ]);

    while !seen[curr.y][curr.x] {
        seen[curr.y][curr.x] = true;

        // find which point is the valid point
        if curr.value == 'S' {
            let adj_arr: Vec<Node> = get_adjacent(&data, &curr);

            for adj in adj_arr.iter() {
                if seen[adj.y][adj.x] {
                    println!("{} \nprev: {:?}\n", adj.value, (adj.x, adj.y));
                    println!("{:?}\n", adj_arr);
                    continue;
                }

                if let Some(next_pipe) = pipes.get(&adj.value) {
                    let (mut x, mut y) = next_pipe.from;

                    if curr.value == 'S' {

                        for to in dirs.clone() {
                            if to == next_pipe.from {
                                paths.push(Pipe {
                                    from: (0, 0),
                                    to
                                });
                                curr = adj.clone();
                                break;
                            }
                        }

                        if curr.value == adj.value  {
                            break;
                        }
                    }

                }
            }

        }

        if let Some(curr_pipe) = pipes.get(&curr.value) {
            let (mut c_x, mut c_y) = curr_pipe.to;
            let (mut cf_x, mut cf_y) = curr_pipe.from;
            let (p_x, p_y) = paths[paths.len() - 1].to;

            if p_x + cf_x != 0 && p_y + cf_y != 0 {
                let tmp_x = cf_x;
                cf_x = c_x;
                c_x = tmp_x;

                let tmp_y = cf_y;
                cf_y = c_y;
                c_y = tmp_y;
            }

            paths.push(Pipe {
                from: (cf_x, cf_y), 
                to: (c_x, c_y) 
            });

            curr = Node {
                value: data[(curr.y as i32 + c_y) as usize][(curr.x as i32 + c_x) as usize],
                x: (curr.x as i32 + c_x) as usize,
                y: (curr.y as i32 + c_y) as usize
            };
        }
    }

    let (latest_x, latest_y) = paths[paths.len() - 1].to;
    let (first_x, first_y) = paths[0].from;

    paths.push(Pipe {
        from: (latest_x * -1, latest_y * -1), 
        to: (first_x * -1, first_y * -1)
    });

    paths
}

fn part1 (str: String) -> i32 {
    let data: Vec<Vec<char>> = str.split("\n").map(|a| a.chars().collect()).collect();
    let mut start_point = (0, 0);
    let pipes: HashMap<char, Pipe> = HashMap::from([
        ('|', Pipe {from: (0, -1), to: (0, 1)}),
        ('-', Pipe {from: (-1, 0), to: (1, 0)}),
        ('L', Pipe {from: (0, -1), to: (1, 0)}),
        ('J', Pipe {from: (-1, 0), to: (0, -1)}),
        ('7', Pipe {from: (-1, 0), to: (0, 1)}),
        ('F', Pipe {from: (0, 1), to: (1, 0)}),
    ]);

    // println!("{:?}", data);

    for (y, row) in data.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if data[y][x] == 'S' {
                start_point = (x, y);
                break;
            }
        }

        if start_point != (0, 0) {
            break;
        }
    }

    let seen: Vec<Vec<bool>> = vec![vec![false; data[0].len()]; data.len()];
    let (s_x, s_y) = start_point;
    let steps = walk(data, Node{value: 'S', x: s_x, y: s_y}, seen, pipes);

    println!("{:?}", steps);

    (steps.len() - 1) as i32
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
    static TEST_DATA: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_DATA.to_string()), 4)
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("5 10 13 16 21 30 45".to_string()), 5)
    // }
}
