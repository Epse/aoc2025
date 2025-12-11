use std::time::Instant;

use crate::util::display_grid;

pub fn run() {
    let input = std::fs::read_to_string("data/seven").expect("need data");
    {
        let start = Instant::now();
        let (result, _grid) = split_till_done(&input);
        let elapsed = start.elapsed();
        println!("Day six, part one: {} . Elapsed: {:.2?}", result, elapsed);
    }
}

fn next_split(mut input: Vec<Vec<char>>, row: usize) -> (u64, Vec<Vec<char>>) {
    if row == input.len() - 1 {
        return (0, input);
    }

    let mut splits = 0;

    for idx in 0..input[row].len() {
        let c = input[row][idx];
        if c == '^' && input[row - 1][idx] == '|' {
            splits += 1;
            if idx > 0 && input[row][idx - 1] == '.' {
                input[row][idx - 1] = '|';
            }
            if idx < input[row].len() && input[row][idx + 1] == '.' {
                input[row][idx + 1] = '|';
            }
        }
    }

    if row < input.len() - 1 {
        for idx in 0..input[row].len() {
            let c = input[row][idx];
            if c == 'S' || c == '|' {
                if input[row + 1][idx] != '^' {
                    input[row + 1][idx] = '|';
                }
                continue;
            }
        }
    }

    (splits, input)
}

fn split_till_done(input: &str) -> (u64, String) {
    let mut splits = 0;
    let mut grid = input_to_chars(input);
    for i in 0..grid.len() {
        let (next_splits, next_grid) = next_split(grid, i);
        splits += next_splits;
        grid = next_grid;
    }
    (splits, display_grid(&grid))
}

fn input_to_chars(input: &str) -> Vec<Vec<char>> {
    let input = input.strip_suffix('\n').unwrap_or(input);
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_splitting() {
        const EXPECTED: &str = ".......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
.....|.|.|.....
....|^|^|^|....
....|.|.|.|....
...|^|^|||^|...
...|.|.|||.|...
..|^|^|||^|^|..
..|.|.|||.|.|..
.|^|||^||.||^|.
.|.|||.||.||.|.
|^|^|^|^|^|||^|
|.|.|.|.|.|||.|";

        let (splits, final_grid) = split_till_done(INPUT);
        println!("{}\n", EXPECTED);
        println!("{}\n", final_grid);
        assert_eq!(21, splits);
    }
}
