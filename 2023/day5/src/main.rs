fn part2 (str: String) -> i64 {
    let seeds_to_soil: Vec<&str> = str.split("\n\n").collect();
    let mut no_titles: Vec<&str> = vec![];
    for seeds in seeds_to_soil {
        let remove_title: &str = seeds.split(&[':'][..]).last().unwrap();
        no_titles.push(remove_title.trim());
    }

    let seeds: Vec<i64> = no_titles[0].split(' ').map(|e| e.parse::<i64>().unwrap()).collect();
    let soil: Vec<Vec<i64>> = no_titles[1].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let fertilizer: Vec<Vec<i64>> = no_titles[2].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let water: Vec<Vec<i64>> = no_titles[3].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let light: Vec<Vec<i64>> = no_titles[4].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let temperature: Vec<Vec<i64>> = no_titles[5].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let humidity: Vec<Vec<i64>> = no_titles[6].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let location: Vec<Vec<i64>> = no_titles[7].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let mut destinations: Vec<i64> = vec![];

    let mut n_of_seeds = 0;
    let mut list_seeds: Vec<(i64, i64)> = vec![];

    for (i, seed) in seeds.iter().enumerate() {
        if i % 2 == 0 {
            n_of_seeds += seeds[i + 1];
            list_seeds.push((seeds[i], (seed + seeds[i + 1])));

            // let mut all_seeds: Vec<i64> = (seeds[i]..(seed + seeds[i + 1])).collect();
            // new_seeds.append(&mut all_seeds);
        }
    }

    println!("Concat Mode\n");

    println!("Old Seed: {:?}\n", n_of_seeds);

    // exit(000);

    for seed in list_seeds {
        let mut destination = find_map_location_by_range(&soil, seed, 0);
        destination = find_map_location_by_range(&fertilizer, seed, destination);
        destination = find_map_location_by_range(&water, seed, destination);
        destination = find_map_location_by_range(&light, seed, destination);
        destination = find_map_location_by_range(&temperature, seed, destination);
        destination = find_map_location_by_range(&humidity, seed, destination);
        destination = find_map_location_by_range(&location, seed, destination);

        destinations.push(destination);
    }

    destinations.sort();

    destinations[0]
}

fn find_map_location_by_range(farm_map: &Vec<Vec<i64>>, seed: (i64, i64), destination: i64) -> i64 {
    let (start, end) = seed;
    for row in farm_map {
        let os = start.max(row[1]);
        let oe = end.min(row[1] + row[2]);

        // if seed < row[1] {
        //     continue;
        // }

        // let mut destination_index = row[0];
        // let mut source_index = row[1];
        // let range = std::ops::Range {
        //     start: 0, 
        //     end: row[2]
        // };

        // for i in range {
        //     if seed == (source_index + i) {
        //         print!("destination: {} - source: {}\n", destination_index + i, source_index + i);
        //         return destination_index + i
        //     }
        // }
    }

    destination
}

fn find_map_location(farm_map: &Vec<Vec<i64>>, seed: i64) -> i64 {
    for row in farm_map {
        let mut high = row[1] + row[2];
        let mut low = row[1];
        let idx = 0;

        if seed >= row[1] && seed <= row[1] + row[2] {
            while low < high {
                let mid = low + (high - low) / 2;

                if mid == seed {
                    // print!("source: {}\n", seed + (row[0] - row[1]));
                    return seed + (row[0] - row[1])
                } else if mid > seed {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            } 
        }

        // if seed < row[1] {
        //     continue;
        // }

        // let mut destination_index = row[0];
        // let mut source_index = row[1];
        // let range = std::ops::Range {
        //     start: 0, 
        //     end: row[2]
        // };

        // for i in range {
        //     if seed == (source_index + i) {
        //         print!("destination: {} - source: {}\n", destination_index + i, source_index + i);
        //         return destination_index + i
        //     }
        // }
    }

    seed
}

fn part1 (str: String) -> i64 {
    // 0: ID 1: rounds
    let seeds_to_soil: Vec<&str> = str.split("\n\n").collect();
    let mut no_titles: Vec<&str> = vec![];
    for seeds in seeds_to_soil {
        let remove_title: &str = seeds.split(&[':'][..]).last().unwrap();
        no_titles.push(remove_title.trim());
    }

    let seeds: Vec<i64> = no_titles[0].split(' ').map(|e| e.parse::<i64>().unwrap()).collect();
    let soil: Vec<Vec<i64>> = no_titles[1].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let fertilizer: Vec<Vec<i64>> = no_titles[2].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let water: Vec<Vec<i64>> = no_titles[3].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let light: Vec<Vec<i64>> = no_titles[4].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let temperature: Vec<Vec<i64>> = no_titles[5].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let humidity: Vec<Vec<i64>> = no_titles[6].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let location: Vec<Vec<i64>> = no_titles[7].split('\n').map(|e| e.split(' ').map(|a| a.parse::<i64>().unwrap()).collect()).collect();
    let mut destinations: Vec<i64> = vec![];



    for seed in seeds {
        let mut destination = find_map_location(&soil, seed);
        destination = find_map_location(&fertilizer, destination);
        destination = find_map_location(&water, destination);
        destination = find_map_location(&light, destination);
        destination = find_map_location(&temperature, destination);
        destination = find_map_location(&humidity, destination);
        destination = find_map_location(&location, destination);

        destinations.push(destination);
    }

    destinations.sort();

    destinations[0]
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
    static TEST_DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_DATA.to_string()), 35)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_DATA.to_string()), 46)
    }
}
