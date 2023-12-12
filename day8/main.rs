use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].as_str()
    } else {
        "input"
    };

    let file_contents = match read_file(file_path) {
        Ok(contents) => contents,
        Err(_) => std::process::exit(1),
    };

    println!("{}", part1(&file_contents));
    println!("{}", part2(&file_contents));
}

fn part2(file_contents: &str) -> u64 {
    return 0;
}

fn part1(file_contents: &str) -> usize {
    // read into map
    let mut node_map: HashMap<String, Node> = HashMap::new();

    let instructions = file_contents.lines().nth(0).unwrap().trim();
    let start = "AAA";
    let end = "ZZZ";

    for line in file_contents.lines().skip(2) {
        let parts: Vec<&str> = line
            .split([' ', '=', '(', ')', ','].as_ref())
            .filter(|s| !s.is_empty())
            .collect();

        if parts.len() >= 3 {
            let value = parts[0].to_string();
            let left = parts[1].to_string();
            let right = parts[2].to_string();
            
            let node = Node {value: value.clone(), left, right};
            node_map.insert(value.clone(), node);
        }
    }

    let mut current_node = node_map.get(start).unwrap();
    let mut steps = 0;
    while current_node.value != end {
        let c = instructions.chars().nth(steps % instructions.len()).unwrap();
        steps += 1;
        if c == 'L' {
            current_node = node_map.get(&current_node.left).unwrap();
        } else if c == 'R' {
            current_node = node_map.get(&current_node.right).unwrap();
        }
    }
    return steps;
}

#[derive(Debug)]
struct Node {
    value: String,
    left: String,
    right: String,
}
