use std::fs;
use std::mem;
use std::time::Instant;

pub fn run() {
    let input = fs::read_to_string("data/four").expect("need a file");
    {
        let grid = string_to_grid(&input);
        let start = Instant::now();
        let count = count_reachable(&map_accessible(grid));
        let elapsed = start.elapsed();
        println!("Part one: {} , time taken: {:.2?}", count, elapsed);
    }
    {
        let start = Instant::now();
        let count = part_two(&input);
        let elapsed = start.elapsed();
        println!("Part one: {} , time taken (including gridify): {:.2?}", count, elapsed);
    }
}

fn display_grid(grid: &Vec<Vec<char>>) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect::<String>()
}

fn string_to_grid(input: &str) -> Vec<Vec<char>> {
    let mut row: Vec<char> = vec![];
    let mut grid: Vec<Vec<char>> = vec![];
    input.chars().for_each(|c| {
        if c == '\n' {
            let r = mem::take(&mut row);
            grid.push(r);
        } else {
            row.push(c);
        }
    });
    if row.len() > 1 {
        // Ignore trailing newlines
        grid.push(row);
    }
    grid
}

fn map_accessible(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, elem)| {
                    if *elem != '@' {
                        return *elem;
                    }

                    let first_col = x == 0;
                    let last_col = x == row.len() - 1;
                    let first_row = y == 0;
                    let last_row = y == input.len() - 1;

                    let mut adjacent = 0;

                    if !first_col {
                        // Left
                        if row[x - 1] != '.' {
                            adjacent += 1;
                        }

                        if !first_row {
                            // Top left
                            if input[y - 1][x - 1] != '.' {
                                adjacent += 1;
                            }
                        }

                        if !last_row {
                            // Bottom left
                            if input[y + 1][x - 1] != '.' {
                                adjacent += 1;
                            }
                        }
                    }

                    if !last_col {
                        // Right
                        if row[x + 1] != '.' {
                            adjacent += 1;
                        }

                        if !first_row {
                            // Top right
                            if input[y - 1][x + 1] != '.' {
                                adjacent += 1;
                            }
                        }

                        if !last_row {
                            // Bottom right
                            if input[y + 1][x + 1] != '.' {
                                adjacent += 1;
                            }
                        }
                    }

                    // Top
                    if !first_row && input[y - 1][x] != '.' {
                        adjacent += 1;
                    }

                    // Bottom
                    if !last_row && input[y + 1][x] != '.' {
                        adjacent += 1;
                    }

                    if adjacent < 4 {
                        'x'
                    } else {
                        *elem
                    }
                })
                .collect()
        })
        .collect()
}

fn count_reachable(input: &Vec<Vec<char>>) -> u64 {
    input.iter().fold(0_u64, |acc, row| {
        row.iter()
            .fold(acc, |acc, char| if *char == 'x' { acc + 1 } else { acc })
    })
}

fn erase_reachable(input: &Vec<Vec<char>>) -> (u64, Vec<Vec<char>>) {
    let mut erased: u64 = 0;

    let result = input
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| {
                        if *c == 'x' {
                            erased += 1;
                            '.'
                        } else {
                            *c
                        }
                    })
                    .collect()
            })
        .collect();
    (erased, result)
}

// Not the most efficient implementation,what with all the looping
fn part_two(input: &str) -> u64 {
    let mut removed: u64 = 0;
    let mut grid = string_to_grid(input);
    loop {
        grid = map_accessible(grid);
        let (removed_this_round, new_grid) = erase_reachable(&grid);
        removed += removed_this_round;
        grid = new_grid; // ARgh inefficient
        if removed_this_round == 0 {
            break;
        }
    }
    removed
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    const RESULT: &str = "..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
"; // Trailing newline for consistency

    #[test]
    fn test_string_to_grid() {
        let result = string_to_grid(INPUT);

        assert_eq!(10, result.len());
        assert_eq!(10, result[0].len());
    }

    #[test]
    fn test_accessible() {
        let result = display_grid(&map_accessible(string_to_grid(INPUT)));
        assert_eq!(RESULT, result);
    }

    #[test]
    fn test_part_one() {
        let result = count_reachable(&map_accessible(string_to_grid(INPUT)));
        assert_eq!(13, result);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(43, result);
    }
}
