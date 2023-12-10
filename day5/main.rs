use std::fs::File;
use std::io::{self, Read};
use std::ops::Range;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {

    let file_contents = match read_file("input2") {
        Ok(contents) => contents,
        Err(_) => std::process::exit(1),
    };

    println!("{}", part1(&file_contents));
    println!("{}", part2(&file_contents));
}

fn part2(file_contents: &str) -> i64 {
    let seeds: Vec<i64> = file_contents
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut seed_ranges: Vec<Range<i64>> = Vec::new();
    let mut start: i64 = 0;
    for (idx, val) in seeds.iter().enumerate() {
        if idx % 2  == 0 {
            start = *val;
        } else {
            seed_ranges.push(start..*val);
        }
    }

    let mut maps: Vec<Vec<[i64; 3]>> = get_maps(&file_contents);

    let best_range: Range<i64>;
    for seed_range in seed_ranges {
        if let res = apply_range_map(seed_range, map).start < best_range.start {
            best_range = res;
        }
    }

    return 0;
}

fn apply_range_map(seed_range: Range<i64>, maps: &Vec<Vec<[i64; 3]>>) -> i64 {
    let mut res = seed_range;

    for current_map in maps {
        for mapping in current_map {
            let src_min = mapping[1];
            let src_max = mapping[1] + mapping[2] - 1;
            if src_min <= target && target <= src_max {
                target = mapping[0] + (target - mapping[1]);
                break;
            }

        }
    }
    return target;
}

fn part1(file_contents: &str) -> i64 {
    let seeds: Vec<i64> = file_contents
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let maps: Vec<Vec<[i64; 3]>> = get_maps(&file_contents);

    let mut lowest: i64 = -1;
    for seed in seeds {
        let target = apply_maps(seed, &maps);
        if lowest == -1 || target < lowest {
            lowest = target;
        }
    }

    return lowest;
}

fn apply_maps(seed: i64, maps: &Vec<Vec<[i64; 3]>>) -> i64 {
    let mut target = seed;
    for current_map in maps {
        for mapping in current_map {

            let src_min = mapping[1];
            let src_max = mapping[1] + mapping[2] - 1;
            if src_min <= target && target <= src_max {
                target = mapping[0] + (target - mapping[1]);
                break;
            }

        }
    }
    return target;
}

fn get_maps(input: &str) -> Vec<Vec<[i64; 3]>> {

    // all maps of form vector of array of 3 ints
    let mut maps: Vec<Vec<[i64; 3]>> = Vec::new();

    let mut current_map: Vec<[i64; 3]> = Vec::new();

    // store all maps skip empty lines
    for line in input.lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        // new mappings
        if line.contains("map") {
            if current_map.len() > 0 {
                maps.push(current_map);
            }
            current_map = Vec::new();
        } else {
            let nums: Vec<i64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            current_map.push([nums[0], nums[1], nums[2]]);
        }
    }
    maps.push(current_map);

    return maps;
}
