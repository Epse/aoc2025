use std::{fmt::{Error, Formatter, Display}, time::Instant};

use crate::util::display_grid;

pub fn run() {
    let input = std::fs::read_to_string("data/seven").expect("need data");
    {
        let start = Instant::now();
        let (result, _grid) = split_till_done(&input);
        let elapsed = start.elapsed();
        println!("Day six, part one: {} . Elapsed: {:.2?}", result, elapsed);
    }
    {
        let grid = input_to_grid(&input);
        let start = Instant::now();
        let timelines = timelines(&count_timelines(grid));
        let elapsed = start.elapsed();
        println!(
            "Day six, part two: {} . Elapsed: {:.2?}",
            timelines, elapsed
        );
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

fn input_to_grid(input: &str) -> Vec<Vec<GridCell>> {
    let input = input.strip_suffix('\n').unwrap_or(input);
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| GridCell::try_from(c).expect("Grid invalid"))
                .collect::<Vec<GridCell>>()
        })
        .collect::<Vec<Vec<GridCell>>>()
}

fn timelines(counted: &Vec<Vec<GridCell>>) -> u64 {
    counted.last().expect("Need a last row").iter()
        .filter_map(|x| {
            if let GridCell::Empty(beams) = x {
                Some(beams)
            } else {
                None
            }
        })
        .sum()
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum GridCell {
    Emitter,
    Splitter,
    Empty(u64), // Beam count
}

impl TryFrom<char> for GridCell {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(GridCell::Splitter),
            '.' => Ok(GridCell::Empty(0)),
            'S' => Ok(GridCell::Emitter),
            '|' => Ok(GridCell::Empty(1)),
            _ => Err("Not a grid cell"),
        }
    }
}

impl Display for GridCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", match self {
            GridCell::Emitter => 'S'.into(),
            GridCell::Splitter => '^'.into(),
            GridCell::Empty(0) => '.'.into(),
            GridCell::Empty(x) => x.to_string()
        })
    }
}

#[allow(dead_code)]
fn display_cell_grid(grid: &Vec<Vec<GridCell>>) -> String {
    grid.iter()
        .map(|row| row.iter().map(|x| x.to_string()).collect::<String>() + "\n")
        .collect::<String>()
}

fn count_timelines(mut input: Vec<Vec<GridCell>>) -> Vec<Vec<GridCell>> {
    for row_idx in 0..(input.len() - 1) {
        for col_idx in 0..input[row_idx].len() {
            let c = input[row_idx][col_idx];
            if c != GridCell::Splitter {
                continue;
            }
            if let GridCell::Empty(source_beams) = input[row_idx - 1][col_idx] && source_beams > 0 {
                if col_idx > 0 && let GridCell::Empty(beams) = input[row_idx][col_idx - 1] {
                    input[row_idx][col_idx - 1] = GridCell::Empty(source_beams + beams);
                }
                if col_idx < input[row_idx].len() && let GridCell::Empty(beams) = input[row_idx][col_idx + 1] {
                    input[row_idx][col_idx + 1] = GridCell::Empty(beams + source_beams);
                }
            }
        }

        for idx in 0..input[row_idx].len() {
            let c = input[row_idx][idx];
            if c == GridCell::Emitter {
                if let GridCell::Empty(beams) = input[row_idx + 1][idx] {
                    input[row_idx + 1][idx] = GridCell::Empty(beams + 1);
                }
            }
            if let GridCell::Empty(source_beams) = c {
                if let GridCell::Empty(dest_beams) = input[row_idx + 1][idx] {
                    input[row_idx + 1][idx] = GridCell::Empty(source_beams + dest_beams);
                }
            }
        }

    }

    input
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

    #[test]
    fn test_timelines() {
        let grid = input_to_grid(INPUT);
        let grid = count_timelines(grid);
        println!("{}", display_cell_grid(&grid));
        let timelines = timelines(&grid);
        assert_eq!(40, timelines);
    }
}
