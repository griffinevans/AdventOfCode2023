use std::fs::File;
use std::io::{self, Read};

fn main() {

    let file_contents = match read_file("input") {
        Ok(contents) => contents,
        Err(_) => std::process::exit(1),
    };


    let mut sum = 0;
    for line in file_contents.lines() {
        let first = first_digit(line);
        let last = last_digit(line);
        let calibration_value = first * 10 + last; 
        sum += calibration_value;
    }

    println!("{}", sum);
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn first_digit(input_string: &str) -> u32 {
    for start in 0..input_string.len() {

        if input_string.chars().nth(start).unwrap().is_numeric() {
            return input_string.chars().nth(start).unwrap().to_digit(10).unwrap();
        }

        for end in start + 1..=input_string.len() {
            let substr = &input_string[start..end];

            if word_to_num(substr) != 0 {
                return word_to_num(substr);
            }
        }
    }

    return 0;
}

fn last_digit(input_string: &str) -> u32 {
    for start in (0..input_string.len()).rev() {

        if input_string.chars().nth(start).unwrap().is_numeric() {
            return input_string.chars().nth(start).unwrap().to_digit(10).unwrap();
        }

        for end in start + 1..=input_string.len() {
            let substr = &input_string[start..end];

            if word_to_num(substr) != 0 {
                return word_to_num(substr);
            }
        }
    }

    return 0;
}

fn word_to_num(word: &str) -> u32 {
    match word.to_lowercase().as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}
