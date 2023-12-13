use std::env;
use std::fs::File;
use std::io::{self, Read};

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = if args.len() > 1 { args[1].as_str() } else { "input" };

    let file_contents = match read_file(file_path) {
        Ok(contents) => contents,
        Err(_) => std::process::exit(1),
    };

    println!("{}", part1(&file_contents));
    println!("{}", part2(&file_contents));
}

fn part1(file_contents: &str) -> i64 {
    let mut sum = 0;
    let mut diffs: Vec<i64> = Vec::new();

    for line in file_contents.lines() {
        let mut nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        for i in nums.windows(2) {
            diffs.push(i[1] - i[0]);
        }

        while !diffs.iter().all(|&x| x == 0) {
            diffs.clear();
            for i in nums.windows(2) {
                diffs.push(i[1] - i[0]);
            }
            sum += nums.last().unwrap_or(&0);
            nums = diffs.clone();
        }
    }
    return sum;
}

fn part2(file_contents: &str) -> i64 {
    let mut sum = 0;
    let mut diffs: Vec<i64> = Vec::new();

    for line in file_contents.lines() {
        let mut nums: Vec<i64> = line
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        for i in nums.windows(2) {
            diffs.push(i[1] - i[0]);
        }

        while !diffs.iter().all(|&x| x == 0) {
            diffs.clear();
            for i in nums.windows(2) {
                diffs.push(i[0] - i[1]);
            }
            sum += nums.first().unwrap_or(&0);
            nums = diffs.clone();
        }
    }
    return sum;
}
