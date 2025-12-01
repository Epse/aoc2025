use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let path = Path::new("data/one");
    let file = File::open(&path).expect("Need input file");
    let instructions = io::BufReader::new(file)
        .lines()
        .filter_map(|x| convert(&x.expect("IDK I need a line")))
        .collect();
    let result = compute(&instructions);
    println!("Reached 0 {} times", result)
}

/// Calculates how often the dial is at 0 at the end of an operation.
fn compute(input: &Vec<i32>) -> u32 {
    let mut count: u32 = 0;
    let mut dial: i64 = 50;

    for line in input {
        dial = (dial + (*line as i64)) % 100;
        if dial == 0 {
            count += 1;
        }
    }
    count
}

/// Converts text representation to a vec of numbers, L being negative
fn convert(line: &str) -> Option<i32> {
    let sign = match line.chars().nth(0) {
        Some('L') => -1,
        Some('R') => 1,
        _ => unreachable!("Direction can only be Left or Right"),
    };

    let num_chars = &line[1..line.len()];
    let parsed = str::parse::<i32>(num_chars).ok()?;
    Some(parsed * sign)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_convert() {
        assert_eq!(
            vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82],
            INPUT.lines().filter_map(|x| convert(x)).collect::<Vec<i32>>()
        )
    }

    #[test]
    fn test_full_flow() {
        // Look this could all just run as iterators but I am far ,far too lazy
        assert_eq!(3, compute(&INPUT.lines().filter_map(convert).collect()))
    }
}
