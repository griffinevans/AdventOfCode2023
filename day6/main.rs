use std::fs::File;
use std::io::{self, Read};

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

    println!("{}", part1(&file_contents));
    println!("{}", part2(&file_contents));
}

fn part2(file_contents: &str) -> u64 {
    let lines: Vec<&str> = file_contents.lines().collect();
    let time: u64 = lines[0]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();
    let record: u64 = lines[1]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    // two pointer
    let mut win_start = 0;
    for i in 0..=time {
        win_start = i;
        let distance = i * (time - i);
        if distance > record {
            break;
        }
    }

    let mut win_end = time;
    for i in (0..=time).rev() {
        win_end = i;
        let distance = i * (time - i);
        if distance > record {
            break;
        }
    }

    let total = win_end - win_start + 1;
    return total;
}


fn part1(file_contents: &str) -> u32 {
    let lines: Vec<&str> = file_contents.lines().collect();
    let times: Vec<u32> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let games: Vec<Game> = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &record)| Game {time, record})
        .collect();

    let mut sum = 1;
    for game in games {
        let mut wins = 0;
        for i in 0..=game.time {
            let distance = i * (game.time - i);
            if distance > game.record {
                wins += 1;
            }
        }
        sum *= wins;
    }
    return sum;
}

#[derive(Debug)]
struct Game {
    time: u32,
    record: u32,
}

