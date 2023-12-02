use std::fs::File;
use std::io::{self, Read};
use std::str::FromStr;

fn main() {

    let file_contents = match read_file("input") {
        Ok(contents) => contents,
        Err(_) => std::process::exit(1),
    };

    let mut games: Vec<Game> = Vec::new();
    
    // game # is line #, starting at 1
    for (i, line) in file_contents.lines().enumerate() {
        let mut game_pulls: Vec<CubeSet> = Vec::new();
        let index = line.find(':').unwrap();
        let game_str = &line[index + 2..].trim();
        for pull in game_str.split(';') {
            game_pulls.push(pull.parse().unwrap());
        }
        let game = Game {
            id: i+1,
            pulls: game_pulls,
        };
        games.push(game);
    }

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum = 0;

    for game in games {
        let mut valid = game.id;
        for set in game.pulls {
            println!("id: {} red: {} green: {} blue: {}", game.id, set.red, set.blue, set.green);
            if set.red > max_red || set.green > max_green || set.blue > max_blue {
                println!("game {} is not valid", game.id);
                valid = 0;
                break;
            }
        }
        sum += valid;
    }

    println!("{}", sum);
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

struct Game {
    id: usize,
    pulls: Vec<CubeSet>,
}

struct CubeSet {
    red:    usize,
    blue:   usize,
    green:  usize,
}

impl FromStr for CubeSet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        
        // "3 blue, 1 red, 2 green"
        for pair in s.split(',') {
            let tokens: Vec<&str> = pair.trim().split_whitespace().collect();
            let count: usize = tokens[0].parse().unwrap();
            match tokens[1] {
                "red" => red += count,
                "blue" => blue+= count,
                "green" => green+= count,
                _ => return Err("Invalid color {}"),
            }
        }
        Ok(CubeSet {red, blue, green})
    }
}

