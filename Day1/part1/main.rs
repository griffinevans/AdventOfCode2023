use std::fs::File;
use std::io::{self, Read};

fn main() {

    let file_contents = match read_file("input") {
        Ok(contents) => contents,
        Err(_) => std::process::exit(1),
    };


    let mut sum = 0;
    for line in file_contents.lines() {
        let calibration_value = first_digit(line) * 10 + last_digit(line); 
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
    input_string
        .chars()
        .find(|c| c.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn last_digit(input_string: &str) -> u32 {
    input_string
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap()
}
