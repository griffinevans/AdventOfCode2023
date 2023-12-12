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

    let file_contents = match read_file("input") {
        Ok(contents) => contents,
        Err(_) => std::process::exit(1),
    };

    println!("{}", part1(&file_contents));
    println!("{}", part2(&file_contents));
}

fn part2(file_contents: &str) -> u64 {
    let mut plays: Vec<Play> = Vec::new();
    for line in file_contents.lines() {
        let hand: Vec<char> = line.split_whitespace().next().unwrap().chars().collect();
        let mut uniq_chars: HashMap<char, u64> = HashMap::new();
        for &c in &hand {
            let count = uniq_chars.entry(c).or_insert(0);
            *count += 1;
        }

        // add joker value to highest val
        let mut joker_val = uniq_chars.get(&'J').cloned().unwrap_or(0);
        if joker_val > 0 {
            // this stupid code took me 2 hours
            let max_key = uniq_chars.iter().max_by_key(|entry| entry.1).unwrap().0.clone();
            let max_val = uniq_chars.get_mut(&max_key).unwrap();
            *max_val += joker_val;
            joker_val = *max_val;
            uniq_chars.remove(&'J');
        }

        let mut ordinality = uniq_chars.len() as u64;
        if ordinality == 0 {
            ordinality = 1;
        }
        if ordinality >= 4 {
            ordinality += 2;
        }
        // three of a kind or two pair
        else if ordinality == 3 {
            if joker_val > 0 {
                if joker_val == 2 {
                    ordinality = 5;
                } else {
                    ordinality = 4;
                }
            }
            else if uniq_chars.values().any(|&value| value == 2) {
                ordinality = 5;
            } else {
                ordinality = 4;
            }
        } 
        // either four of a kind or full house
        else if ordinality == 2 {
            if joker_val > 0 {
                if joker_val == 2 || joker_val == 3 {
                    ordinality = 3;
                }
            } else {
                let mut count = uniq_chars.get(&hand[0]).unwrap();
                if *count == 2 || *count == 3 {
                    ordinality = 3;
                }
            }
        }

        plays.push(Play {
            hand: hand,
            bet: line.split_whitespace().nth(1).unwrap().parse().unwrap(),
            ordinality: ordinality,
        });
    }

    plays.sort_by(compare_plays_p2);

    let mut sum: u64 = 0;
    for (i, play) in plays.iter().enumerate() {
        println!("{:?}", play);
        let index = i as u64 + 1;
        sum += play.bet * index;
    }
    return sum;
}

fn part1(file_contents: &str) -> u64 {
    let mut plays: Vec<Play> = Vec::new();
    for line in file_contents.lines() {
        let hand: Vec<char> = line.split_whitespace().next().unwrap().chars().collect();
        
        // get occurrences of each char
        let mut uniq_chars: HashMap<char, u64> = HashMap::new();
        for &c in &hand {
            let count = uniq_chars.entry(c).or_insert(0);
            *count += 1;
        }

        let mut ordinality = uniq_chars.len() as u64;
        if ordinality >= 4 {
            ordinality += 2;
        }
        // three of a kind or two pair
        else if ordinality == 3 {
            if uniq_chars.values().any(|&value| value == 2) {
                ordinality = 5;
            } else {
                ordinality = 4;
            }
        } 
        // either four of a kind or full house
        else if ordinality == 2 {
            let count = uniq_chars.get(&hand[0]).unwrap();
            if *count == 2 || *count == 3 {
                ordinality = 3;
            }
        }

        plays.push(Play {
            hand: hand,
            bet: line.split_whitespace().nth(1).unwrap().parse().unwrap(),
            ordinality: ordinality,
        });
    }

    plays.sort_by(compare_plays);

    let mut sum: u64 = 0;
    for (i, play) in plays.iter().enumerate() {
        println!("{:?}", play);
        let index = i as u64 + 1;
        sum += play.bet * index;
    }
    return sum;
}

#[derive(Debug)]
struct Play {
    hand: Vec<char>, 
    bet: u64,
    ordinality: u64,
}

fn card_value(card: char) -> u64 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0, // Handle unexpected characters if needed
    }
}

fn card_value_p2(card: char) -> u64 {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => 0, // Handle unexpected characters if needed
    }
}

// Sort by ordinality and then by the characters in the hand
fn compare_plays(a: &Play, b: &Play) -> std::cmp::Ordering {
    match a.ordinality.cmp(&b.ordinality) {
        std::cmp::Ordering::Equal => {
            let iter = a.hand.iter().zip(b.hand.iter());

            for (char_a, char_b) in iter {
                let val_a = card_value(*char_a);
                let val_b = card_value(*char_b);

                match val_a.cmp(&val_b) {
                    std::cmp::Ordering::Equal => continue,
                    result => return result,
                }
            }
            a.hand.len().cmp(&b.hand.len())
        }
        other => other.reverse(),
    }
}

// Sort by ordinality and then by the characters in the hand
fn compare_plays_p2(a: &Play, b: &Play) -> std::cmp::Ordering {
    match a.ordinality.cmp(&b.ordinality) {
        std::cmp::Ordering::Equal => {
            let iter = a.hand.iter().zip(b.hand.iter());

            for (char_a, char_b) in iter {
                let val_a = card_value_p2(*char_a);
                let val_b = card_value_p2(*char_b);

                match val_a.cmp(&val_b) {
                    std::cmp::Ordering::Equal => continue,
                    result => return result,
                }
            }
            a.hand.len().cmp(&b.hand.len())
        }
        other => other.reverse(),
    }
}
