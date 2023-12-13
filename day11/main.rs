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

fn part1(file_contents: &str) -> usize {
    let mut universe: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    grow_universe(&mut universe);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    let mut gap_rows: Vec<usize> = Vec::new();
    let mut gap_cols: Vec<usize> = Vec::new();

    for i in 0..universe.len() {
        if universe[i][0] == '!' {
            gap_rows.push(i);
        }
    }
    for i in 0..universe[0].len() {
        if universe[0][i] == '!' {
            gap_cols.push(i);
        }
    }

    for i in 0..universe.len() {
        for j in 0..universe[0].len() {
            if universe[i][j] == '#' {
                galaxies.push((i,j));
            }
        }
    }
    
    let mut sum = 0;
    for i in 0..galaxies.len() {
        let outer = galaxies[i];
        for j in i+1..galaxies.len() {
            let inner = galaxies[j];

            let path_rows = if inner.0 < outer.0 { inner.0..outer.0 } else { outer.0..inner.0 };
            let path_cols = if inner.1 < outer.1 { inner.1..outer.1 } else { outer.1..inner.1 };

            let mut distance = 0;
            for loc in path_rows.into_iter() {
                if gap_cols.contains(&loc) {
                    distance += 2;
                } else {
                    distance += 1;
                }
            }
            for loc in path_cols.into_iter() {
                if gap_rows.contains(&loc) {
                    distance += 2;
                } else {
                    distance += 1;
                }
            }
            sum += distance;
        }
    }

    return sum;
}

fn part2(file_contents: &str) -> usize {
    let mut universe: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    grow_universe(&mut universe);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for i in 0..universe.len() {
        for j in 0..universe[0].len() {
            if universe[i][j] == '#' {
                galaxies.push((i,j));
            }
        }
    }

    let gap_value = 1_000_000;
    
    let mut sum = 0;
    for i in 0..galaxies.len() {
        let outer = galaxies[i];
        for j in i+1..galaxies.len() {
            let inner = galaxies[j];

            let path_rows = if inner.0 < outer.0 { inner.0..outer.0 } else { outer.0..inner.0 };
            let path_cols = if inner.1 < outer.1 { inner.1..outer.1 } else { outer.1..inner.1 };

            let mut distance = 0;
            for loc in path_rows.into_iter() {
                if universe[loc][0] == '!' {
                    distance += gap_value;
                } else {
                    distance += 1;
                }
            }
            for loc in path_cols.into_iter() {
                if universe[0][loc] == '!' {
                    distance += gap_value;
                } else {
                    distance += 1;
                }
            }
            sum += distance;
        }
    }

    return sum;
}

fn grow_universe(universe: &mut Vec<Vec<char>>) {

    for i in 0..universe.len() {
        if !universe[i].contains(&'#') {
            universe[i].iter_mut().for_each(|c| *c = '!');
        }
    }

    for i in 0..universe[0].len() {
        let column: Vec<char> = universe.iter().map(|row| row[i]).collect();
        if !column.contains(&'#') {
            for row in universe.iter_mut() {
                row[i] = '!';
            }
        }
    }
}
