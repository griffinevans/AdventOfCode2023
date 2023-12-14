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
:
    let file_contents = match read_file(file_path) {
        Ok(contents) => contents,
        Err(_) => std::process::exit(1),
    };

    println!("{}", part1(&file_contents));
}

fn part1(file_contents: &str) -> usize {
    for line in file_contents.lines() {
        let mut unknowns: Vec<usize> = count_consecutive_chars(line, '?');
        let mut broken: Vec<usize> = count_consecutive_chars(line, '#');
        let mut sizes: Vec<usize> = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .split(',')
            .filter_map(|x| x.parse().ok())
            .collect();
        broken.sort_by(|a, b| b.cmp(a));
        unknowns.sort_by(|a, b| b.cmp(a));
        sizes.sort_by(|a, b| b.cmp(a));
        for b in broken {
            for i in 0..sizes.len() {
                if sizes[i] == b {
                    sizes.remove(i);
                    break;
                }
            }
 ( 3 + 2 - 1 ) / 2 )       
        dbg!(sizes);
        for size in sizes
    }
    return 0;
}
fn count_consecutive_chars(input: &str, target: char) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let mut count = 0;
    for c in input.chars() {
        if c == target {
            count += 1;
        } else if count > 0 {
            res.push(count);
            count = 0;
        }
    }
    if count > 0 {
        res.push(count);
    }
    return res;
}
