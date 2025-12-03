use std::fs;
use std::ops::Range;

pub fn run() {
    let input = fs::read_to_string("data/two").expect("We need input");
    println!("Part one: {}", compute(&input));
    println!("Part two: {}", compute_two(&input));
}

fn compute(input: &str) -> u64 {
    input
        .split(',')
        .map(|line| line.strip_suffix('\n').unwrap_or(&line))
        .map(|x| to_range(x))
        .flat_map(|range| range.filter(is_invalid))
        .sum()
}

fn to_range(input: &str) -> Range<u64> {
    let (start_str, end_str) = input.split_once('-').expect("Invalid range");
    let start = str::parse::<u64>(start_str).expect("Invalid start");
    let end = str::parse::<u64>(end_str)
        .inspect_err(|x| {
            dbg!("END", &input, &x, &end_str);
        })
        .expect("Invalid end");
    Range {
        start: start,
        end: end + 1,
    } // Offset because range is exclusive
}

fn is_invalid(input: &u64) -> bool {
    let st = input.to_string();
    if st.chars().nth(0) == Some('0') {
        return true;
    }

    let char_count = st.chars().count();

    if char_count % 2 != 0 {
        return false;
    }

    let front: String = st.chars().take(char_count / 2).collect();
    let back: String = st
        .chars()
        .rev() // Start taking from the back
        .take(char_count / 2) // We cannot reverse this again due to traits..
        .collect::<Vec<char>>()
        .iter()
        .rev()
        .collect();

    front == back
}

fn is_n_repeated(input: &u64) -> bool {
    let text = input.to_string();

    for count in 1..text.len() {
        if text.len() % count != 0 {
            continue;
        }

        let times = text.len() / count;

        let part: String = text.chars().take(count).collect();
        if part.repeat(times) == text {
            return true;
        }
    }
    false
}

fn compute_two(input: &str) -> u64 {
    input
        .split(',')
        .map(|line| line.strip_suffix('\n').unwrap_or(&line))
        .map(|x| to_range(x))
        .flat_map(|range| range.filter(is_n_repeated))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_validity() {
        assert_eq!(
            vec![11, 22],
            to_range("11-22").filter(is_invalid).collect::<Vec<u64>>()
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(1227775554, compute(&INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4174379265, compute_two(&INPUT));
    }

    #[test]
    fn test_repetition() {
        assert_eq!(true, is_n_repeated(&11));
    }
}
