use std::fs::File;
use std::io::{self, Read};
use regex::Regex;
use std::collections::HashMap;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {

    let file_contents = match read_file("input") {
        Ok(contents) => contents,
        Err(_) => std::process::exit(1),
    };

    let player_regex = Regex::new(r":(.*?)(?:\|)").unwrap();
    let winning_regex = Regex::new(r"\|(.+)").unwrap();

    let mut sum = 0;
    let mut copies = HashMap::new();

    for (index, line) in file_contents.lines().enumerate() {
        let card_num = index + 1;
        let p_str = player_regex.captures(line).unwrap().get(1).unwrap().as_str();
        let w_str = winning_regex.captures(line).unwrap().get(1).unwrap().as_str();

        let player_nums: Vec<i32> = p_str.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
        let winning_nums: Vec<i32> = w_str.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();

        let mut score = 0;
        for num in player_nums {
            if winning_nums.contains(&num) {
                score += 1;
            }
        }

        let num_duplicates = *copies.get(&card_num).unwrap_or(&0);

        for i in 1..=score {
            let next = copies.entry(card_num + i).or_insert(0);
            *next += 1 + num_duplicates;
        }

        //println!("card: {} score: {} duplicates: {}", card_num, score, num_duplicates);
        sum += 1 + num_duplicates;
    }
    println!("{}", sum);
}
