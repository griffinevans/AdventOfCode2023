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

            let x = (inner.0 as isize - outer.0 as isize).abs() as usize;
            let y = (inner.1 as isize - outer.1 as isize).abs() as usize;
            let distance = x + y;
            sum += distance;
        }
    }

    return sum;
}

fn grow_universe(universe: &mut Vec<Vec<char>>) {

    let mut rows: Vec<(usize, Vec<char>)> = Vec::new();

    for i in 0..universe.len() {
        if !universe[i].contains(&'#') {
            rows.push((i, universe[i].clone()));
        }
    }

    let mut off = 0;
    for (i, row) in rows {
        universe.insert(i + off, row);
        off += 1;
    }

    let mut cols: Vec<(usize, Vec<char>)> = Vec::new();

    for i in 0..universe[0].len() {
        let column: Vec<char> = universe.iter().map(|row| row[i]).collect();
        if !column.contains(&'#') {
            cols.push((i, column));
        }
    }

    let mut offset = 0;
    for (loc, col) in cols {
        for (row_index, row) in universe.iter_mut().enumerate() {
            row.insert(loc + offset, col[row_index]);
        }
        offset += 1;
    }

}

fn part2(_file_contents: &str) -> usize {
    return 0;
}
