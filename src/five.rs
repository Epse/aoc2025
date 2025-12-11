use std::cmp::max;
use std::fs;
use std::ops::{Range, RangeInclusive};
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
    {
        let start = Instant::now();
        let merged = merge_ranges(&ranges);
        let result = fresh_id_count(&merged);
        let elapsed = start.elapsed();
        println!("Day 5 part two: {} , elapsed: {:.2?}", result, elapsed);
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
                end_s.parse::<i64>().expect("End must be numeric"),
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
    indices
        .iter()
        .filter(|idx| ranges.iter().any(|range| range.contains(idx)))
        .cloned()
        .collect()
}

fn merge_ranges(ranges: &Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    let mut sorted = ranges
        .iter()
        .map(|x| Range {
            start: *x.start(),
            end: x.end() + 1,
        })
        .collect::<Vec<Range<i64>>>();
    sorted.sort_by_key(|range| range.start);
    /*
     * Now we must iterate over ranges.
     * We consider the current range x and the previous range p.
     * If p.contains(x.start), replace them both by {p.start, max(x.end, p.end)}
     * Otherwise, just keep both, ensuring x is the last.
     */

    let mut result = vec![sorted[0].clone()];

    sorted.iter().skip(1).for_each(|range| {
        let last = result.last_mut().unwrap(); // Will always have 1
        if last.contains(&range.start) {
            last.end = max(last.end, range.end);
        } else {
            result.push(range.clone());
        }
    });

    result
        .iter()
        .map(|x| RangeInclusive::new(x.start, x.end - 1))
        .collect()
}

/// Assumes the vec is sorted, this is important
fn fresh_id_count(ranges: &Vec<RangeInclusive<i64>>) -> usize {
    ranges.iter().fold(0_usize, |acc, range| {
        acc + (range.end() - range.start() + 1) as usize
    })
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

    #[test]
    fn part_two() {
        let (mut ranges, _indices) = parse_input(INPUT);
        ranges = merge_ranges(&ranges);
        assert_eq!(14, fresh_id_count(&ranges));
    }

    #[test]
    fn test_id_counts() {
        assert_eq!(1, fresh_id_count(&vec![0..=0]));
        assert_eq!(2, fresh_id_count(&vec![0..=1]));
        assert_eq!(3, fresh_id_count(&vec![0..=2]));
    }

    #[test]
    fn test_merging() {
        let input = vec![1..=1, 1..=2, 3..=4, 1..=1];
        let merged = merge_ranges(&input);
        assert_eq!(4, fresh_id_count(&merged));
    }

    #[test]
    fn test_internet_example() {
        const INPUT: &str = "200-300
100-101
1-1
2-2
3-3
1-3
1-3
2-2
50-70
10-10
98-99
99-99
99-99
99-100
1-1
2-1
100-100
100-100
100-101
200-300
201-300
202-300
250-251
98-99
100-100
100-101
1-101

1";
        let (mut ranges, _indices) = parse_input(INPUT);
        ranges = merge_ranges(&ranges);
        assert_eq!(202, fresh_id_count(&ranges));
    }
}
