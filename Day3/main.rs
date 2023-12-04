use std::fs::File;
use std::io::{self, Read};

fn main() {

    let file_contents = match read_file("input") {
        Ok(contents) => contents,
        Err(_) => std::process::exit(1),
    };

    let lines: Vec<&str> = file_contents.lines().collect();

    let mut prev_line: &str = "";
    let mut cur_line: &str = "";
    let mut next_line: &str = "";

    let mut gear_ratios = 0;
    for i in 0..lines.len() {
        let prev_line = if i > 0 {
            lines[i - 1]
        } else { "" };
        let cur_line = lines[i];
        let next_line =  if i < lines.len() - 1 {
            lines[i + 1]
        } else { "" };

        for (index, c) in cur_line.chars().enumerate() {
            // is symbol
            if c.is_ascii_punctuation() && c != '.' {
                let mut count_adjacent = 0;
                let mut sum = 1;
                // check prev line
                if !prev_line.is_empty() {
                    let left = prev_line.chars().nth(index.wrapping_sub(1));
                    let mid = prev_line.chars().nth(index).unwrap();
                    let right = prev_line.chars().nth(index + 1);

                    // if mid is digit, left and right are digit if included
                    if mid.is_digit(10) {
                        let part_num = is_num(prev_line, index);
                        count_adjacent += 1;
                        sum *= part_num;
                    } else {
                        if left.map_or(false, |c| c.is_digit(10)) {
                            let part_num = is_num(prev_line, index - 1);
                            count_adjacent += 1;
                            sum *= part_num;
                        }

                        if right.map_or(false, |c| c.is_digit(10)) {
                            let part_num = is_num(prev_line, index + 1);
                            count_adjacent += 1;
                            sum *= part_num;
                        }

                    }                    
                }
                // check next line
                if !next_line.is_empty() {
                    let left = next_line.chars().nth(index.wrapping_sub(1));
                    let mid = next_line.chars().nth(index).unwrap();
                    let right = next_line.chars().nth(index + 1);

                    // if mid is digit, left and right are digit if included
                    if mid.is_digit(10) {
                        let part_num = is_num(next_line, index);
                        count_adjacent += 1;
                        sum *= part_num;
                    } else {
                        if left.map_or(false, |c| c.is_digit(10)) {
                            let part_num = is_num(next_line, index - 1);
                            count_adjacent += 1;
                            sum *= part_num;
                        }

                        if right.map_or(false, |c| c.is_digit(10)) {
                            let part_num = is_num(next_line, index + 1);
                            count_adjacent += 1;
                            sum *= part_num;
                        }

                    }                    
                }
                // check previous chars
                if index > 0 {
                    if cur_line.chars().nth(index - 1).unwrap().is_digit(10) {
                        let part_num = is_num(cur_line, index - 1);
                        count_adjacent += 1;
                        sum *= part_num;
                    }
                }
                // check next chars
                if index < cur_line.len() - 1 {
                    if cur_line.chars().nth(index + 1).unwrap().is_digit(10) {
                        let part_num = is_num(cur_line, index + 1);
                        count_adjacent += 1;
                        sum *= part_num;
                    }
                }

                if count_adjacent == 2 {
                    gear_ratios += sum;
                }
            }
        }

    }
    println!("{}", gear_ratios);
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn is_num(s: &str, index: usize) -> usize {
    let mut start = index;
    let mut end = index;

    while start > 0 && s.chars().nth(start - 1).map_or(false, |c| c.is_digit(10)) {
        start -= 1;
    }

    while end < s.len() && s.chars().nth(end).map_or(false, |c| c.is_digit(10)) {
        end += 1;
    }

    end -= 1;

    let num = &s[start..=end];
    return num.parse().unwrap_or(0);
}
