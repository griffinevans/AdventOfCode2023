use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::collections::VecDeque;
use std::collections::HashMap;

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
    let rows: Vec<&str> = file_contents.lines().collect();

    let grid: Vec<Vec<char>> = rows
        .iter()
        .map(|&row| row.chars().collect())
        .collect();

    let start_index = find_char(&grid, 'S').unwrap();

    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    bfs(&mut map, &grid, start_index);

    return *map.values().max().unwrap_or(&0);
}

fn bfs(map: &mut HashMap<(usize, usize), usize>, grid: &Vec<Vec<char>>, root: (usize, usize)) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut depth = 0;

    map.insert(root, depth);
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        // enqueue any valid neighbors
        
        let current_tile = get_tile(&grid, &node).unwrap();
        depth = map.get(&node).unwrap() + 1;

        let north_facing: Vec<char> = vec!['|', 'L', 'J'];
        let south_facing: Vec<char> = vec!['|', '7', 'F'];
        let east_facing: Vec<char> = vec!['-', 'L', 'F'];
        let west_facing: Vec<char> = vec!['-', 'J', '7'];

        if current_tile == 'S' || north_facing.contains(&current_tile) {
            let coords = (node.0.wrapping_sub(1), node.1);
            if let Some(north_tile) = get_tile(&grid, &coords) {
                if south_facing.contains(&north_tile) && !map.contains_key(&coords) {
                    map.insert(coords, depth);
                    queue.push_back(coords);
                }
            }
        }

        if current_tile == 'S' || south_facing.contains(&current_tile) {
            let coords = (node.0+1, node.1);
            if let Some(south_tile) = get_tile(&grid, &coords) {
                if north_facing.contains(&south_tile) && !map.contains_key(&coords) {
                    map.insert(coords, depth);
                    queue.push_back(coords);
                }
            }
        }

        if current_tile == 'S' || east_facing.contains(&current_tile) {
            let coords = (node.0, node.1+1);
            if let Some(east_tile) = get_tile(&grid, &coords) {
                if west_facing.contains(&east_tile) && !map.contains_key(&coords) {
                    map.insert(coords, depth);
                    queue.push_back(coords);
                }
            }
        }

        if current_tile == 'S' || west_facing.contains(&current_tile) {
            let coords = (node.0, node.1.wrapping_sub(1));
            if let Some(west_tile) = get_tile(&grid, &coords) {
                if east_facing.contains(&west_tile) && !map.contains_key(&coords) {
                    map.insert(coords, depth);
                    queue.push_back(coords);
                }
            }
        }
    }
}

fn get_tile(grid: &Vec<Vec<char>>, index: &(usize, usize)) -> Option<char> {
    if index.0 < grid.len() && index.1 < grid[0].len()  {
        return grid.get(index.0).and_then(|row| row.get(index.1)).copied();
    }
    return None;
}

fn find_char(grid: &Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == target) {
            return Some((i,j));
        }
    }
    return None
}

fn part2(file_contents: &str) -> usize {
    let rows: Vec<&str> = file_contents.lines().collect();

    let mut grid: Vec<Vec<char>> = rows
        .iter()
        .map(|&row| row.chars().collect())
        .collect();

    let start_index = find_char(&grid, 'S').unwrap();

    let mut distance_map: HashMap<(usize, usize), usize> = HashMap::new();

    bfs(&mut distance_map, &grid, start_index);
    // shoelace formula: 2A = x1y2 - x2y1 + x2y2 - x3y2 + ... + xny1 - x1yn
    let mut points: Vec<(usize, usize)> = Vec::new();
    dfs(&mut grid, &mut points, start_index);

    points.push(points[0]);

    let area: i64 = points
        .windows(2)
        .map(|p| {
            let left = p[0].0.wrapping_mul(p[1].1) as i64;
            let right = p[0].1.wrapping_mul(p[1].0) as i64;
            left - right
        })
        .sum();
    let a: usize = (area.abs() / 2) as usize;

    //// pick's theorem num_interior points = area - b/2 + 1
    let num_steps = distance_map.len();
    return a - (num_steps / 2) + 1;
}

fn dfs(grid: &mut Vec<Vec<char>>, points: &mut Vec<(usize, usize)>, start: (usize, usize)) {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push(start);

    while let Some(node) = stack.pop() {
        if !points.contains(&node) {
            points.push(node);

            let current_tile = get_tile(&grid, &node).unwrap();

            let north_facing: Vec<char> = vec!['|', 'L', 'J'];
            let south_facing: Vec<char> = vec!['|', '7', 'F'];
            let east_facing: Vec<char> = vec!['-', 'L', 'F'];
            let west_facing: Vec<char> = vec!['-', 'J', '7'];

            if current_tile == 'S' || north_facing.contains(&current_tile) {
                let coords = (node.0.wrapping_sub(1), node.1);
                if let Some(north_tile) = get_tile(&grid, &coords) {
                    if south_facing.contains(&north_tile) && !points.contains(&coords) {
                        stack.push(coords);
                    }
                }
            }

            if current_tile == 'S' || south_facing.contains(&current_tile) {
                let coords = (node.0+1, node.1);
                if let Some(south_tile) = get_tile(&grid, &coords) {
                    if north_facing.contains(&south_tile) && !points.contains(&coords) {
                        stack.push(coords);
                    }
                }
            }

            if current_tile == 'S' || east_facing.contains(&current_tile) {
                let coords = (node.0, node.1+1);
                if let Some(east_tile) = get_tile(&grid, &coords) {
                    if west_facing.contains(&east_tile) && !points.contains(&coords) {
                        stack.push(coords);
                    }
                }
            }

            if current_tile == 'S' || west_facing.contains(&current_tile) {
                let coords = (node.0, node.1.wrapping_sub(1));
                if let Some(west_tile) = get_tile(&grid, &coords) {
                    if east_facing.contains(&west_tile) && !points.contains(&coords) {
                        stack.push(coords);
                    }
                }
            }
        }
    }
} 
