use std::fs;
use std::ops::RangeInclusive;
use std::time::Instant;

pub fn run() {
    let input = fs::read_to_string("data/five").expect("Need input");
    let (ranges, indices) = parse_input(&input);
    {
        let start = Instant::now();
        let result = find_fresh(&ranges, &indices).len();
        let elapsed = start.elapsed();
        println!("Day 5 part one: {} , elapsed: {:.2?}", result, elapsed);
    }
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let (input_ranges, input_indices) = input.split_once("\n\n").expect("Need two parts");

    let ranges = input_ranges
        .lines()
        .map(|line| line.strip_suffix('\n').unwrap_or(line))
        .map(|line| {
            let (start_s, end_s) = line.split_once('-').expect("Need a range");
            RangeInclusive::new(
                start_s.parse::<i64>().expect("Start must be numeric"),
                end_s.parse::<i64>().expect("End must be numeric")
            )
        })
        .collect();

    let indices = input_indices
        .lines()
        .map(|line| line.strip_suffix('\n').unwrap_or(line))
        .map(|line| line.parse::<i64>().expect("Index must be numeric"))
        .collect();

    (ranges, indices)
}

fn find_fresh(ranges: &Vec<RangeInclusive<i64>>, indices: &Vec<i64>) -> Vec<i64> {
    indices.iter().filter(|idx| {
        ranges.iter().any(|range| range.contains(idx))
    })
    .cloned()
    .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn part_one() {
        let (ranges, indices) = parse_input(INPUT);
        assert_eq!(3, find_fresh(&ranges, &indices).len());
    }
}
