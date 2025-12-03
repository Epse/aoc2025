use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

pub fn run() {
    let path = Path::new("data/one");
    let file = File::open(&path).expect("Need input file");
    let instructions = io::BufReader::new(file)
        .lines()
        .filter_map(|x| convert(&x.expect("IDK I need a line")))
        .collect();
    let result_one = compute_part_one(&instructions);
    println!("Reached 0 {} times", result_one);

    {
        let start = Instant::now();
        let result_two = compute_part_two(&instructions);
        let elapsed = start.elapsed();
        println!("Part two: {} zeroes, elapsed: {:.2?}", result_two, elapsed);
    }

    {
        let start = Instant::now();
        let result_fast = compute_faster(&instructions);
        let elapsed = start.elapsed();
        println!("Part two faster: {} , time: {:.2?}", result_fast, elapsed);
    }
}

/// Calculates how often the dial is at 0 at the end of an operation.
fn compute_part_one(input: &Vec<i32>) -> u32 {
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

/// Returns dial, zeroes
fn zeroes_in_one_click(current_pos: i32, movement: i32, current_zeroes: u32) -> (i32, u32) {
    // This is the most lazy implementation I could think of
    let mut zeroes: u32 = current_zeroes;
    let mut dial = current_pos;

    let sign = movement.signum();

    for _ in 0..movement.abs() {
        dial = (dial + sign) % 100;
        if dial == 0 {
            zeroes += 1;
        }
    }

    (dial, zeroes)
}

fn zeroes_but_faster(mut dial: i32, movement: i32, mut zeroes: u32) -> (i32, u32) {
    let started_at_zero = dial == 0;
    
    dial += movement;

    zeroes += (dial.abs() as u32) / 100;

    if dial <= 0 && !started_at_zero {
        zeroes += 1;
    }

    let result = (dial.rem_euclid(100), zeroes);
    result
}

fn compute_part_two(input: &Vec<i32>) -> u32 {
    input
        .iter()
        .fold((50, 0), |(dial, zeroes), motion| {
            zeroes_in_one_click(dial, *motion, zeroes)
        })
        .1
}

fn compute_faster(input: &Vec<i32>) -> u32 {
    input
        .iter()
        .fold((50, 0), |(dial, zeroes), motion| {
            zeroes_but_faster(dial, *motion, zeroes)
        })
        .1
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
            INPUT
                .lines()
                .filter_map(|x| convert(x))
                .collect::<Vec<i32>>()
        )
    }

    #[test]
    fn test_full_flow() {
        // Look this could all just run as iterators but I am far ,far too lazy
        assert_eq!(
            3,
            compute_part_one(&INPUT.lines().filter_map(convert).collect())
        )
    }

    #[test]
    fn test_zeroes_one_click() {
        assert_eq!(
            6,
            compute_part_two(&INPUT.lines().filter_map(|x| convert(x)).collect())
        )
    }

    #[test]
    fn test_two_faster() {
        assert_eq!(
            6,
            compute_faster(&INPUT.lines().filter_map(|x| convert(x)).collect())
        )
    }
}
