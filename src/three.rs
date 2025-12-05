use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

pub fn run() {
    let path = Path::new("data/three");
    {
        let file = File::open(&path).expect("Need input file");
        let banks = io::BufReader::new(file)
            .lines()
            .map(|line| line.expect("WDYM its not a string"));
        let start = Instant::now();
        let result = part_one(banks);
        let elapsed = start.elapsed();
        println!("Part one, result: {} , time: {:.2?}", result, elapsed);
    }
    {
        let file = File::open(&path).expect("Need input file");
        let banks = io::BufReader::new(file)
            .lines()
            .map(|line| line.expect("WDYM its not a string"));
        let start = Instant::now();
        let result = part_two(banks);
        let elapsed = start.elapsed();
        println!("Part two, result: {} , time: {:.2?}", result, elapsed);
    }
}

fn part_one<T: Iterator>(input: T) -> u32
where
    T::Item: ToString,
{
    input.map(|line| bank_max_joltage(&line.to_string())).sum()
}

fn part_two<T: Iterator>(input: T) -> u64
where
    T::Item: ToString,
{
    input.map(|x| max_for_n_digits(&x.to_string(), 12, 0)).sum()
}

fn bank_max_joltage(bank: &str) -> u32 {
    let (max_idx, max_dec) = bank
        .char_indices()
        .take(bank.len() - 1) // Ignore last character
        .map(|(i, c)| (i, c.to_digit(10).expect(&format!("Not a digit?? {}", c))))
        .max_by(|x, y| {
            let cmp = x.1.cmp(&y.1);
            // max_by keeps the last one, so if theyre equal we keep the first one..
            if cmp.is_eq() {
                Ordering::Greater
            } else {
                cmp
            }
        })
        .expect("No biggest digit??");
    let max_single = bank
        .chars()
        .skip(max_idx + 1)
        .map(|c| c.to_digit(10).expect("Not a digit???"))
        .max()
        .expect("Need biggest single");

    max_dec * 10 + max_single
}

fn max_for_n_digits(bank: &str, n: usize, start_val: u64) -> u64 {
    if n == 0 {
        return start_val;
    }

    // Consider all the digits from 0..(bank.len() - n)
    // Pick the first highest digit
    // Recurse with the remaining substring
    let (idx, digit) = bank
        .char_indices()
        .take(bank.len() - n + 1)
        .map(|(idx, char)| (idx, char.to_digit(10).expect("Must be a digit") as u64))
        .max_by(|x, y| {
            let cmp = x.1.cmp(&y.1);
            // max_by keeps the last one, so if theyre equal we keep the first one..
            if cmp.is_eq() {
                Ordering::Greater
            } else {
                cmp
            }
        })
        .expect("We need a max");
    let val: u64 = digit * 10_u64.pow(n as u32 - 1);
    let substr = bank.chars().skip(idx + 1).collect::<String>();
    max_for_n_digits(&substr, n - 1, start_val + val)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_one() {
        assert_eq!(357, part_one(INPUT.lines()));
    }

    #[test]
    fn test_example_banks() {
        assert_eq!(98, bank_max_joltage("987654321111111"));
        assert_eq!(89, bank_max_joltage("811111111111119"));
        assert_eq!(78, bank_max_joltage("234234234234278"));
        assert_eq!(92, bank_max_joltage("818181911112111"));
    }

    #[test]
    fn test_ordering() {
        assert_eq!(88, bank_max_joltage("878161111112111"));
    }

    #[test]
    fn test_big_n() {
        assert_eq!(987654321111, max_for_n_digits("987654321111111", 12, 0));
        assert_eq!(811111111119, max_for_n_digits("811111111111119", 12, 0));
        assert_eq!(434234234278, max_for_n_digits("234234234234278", 12, 0));
        assert_eq!(888911112111, max_for_n_digits("818181911112111", 12, 0));
    }
}
