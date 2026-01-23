use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Field {
    Outside,
    Red,
    Edge(Direction),
    Inside,
}

impl From<&Field> for char {
    fn from(field: &Field) -> char {
        match field {
            Field::Outside => '·',
            Field::Inside => '•',
            Field::Red => '#',
            Field::Edge(Direction::Up) => '↑',
            Field::Edge(Direction::Down) => '↓',
            Field::Edge(Direction::Right) => '→',
            Field::Edge(Direction::Left) => '←',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn mark_edges(coords: &Vec<(usize, usize)>, map: &mut Vec<Vec<Field>>) {
    for idx in 0..coords.len() {
        let current = coords[idx];
        let next: (usize, usize);
        if idx != coords.len() - 1 {
            next = coords[idx + 1];
        } else {
            next = coords[0];
        }
        map[current.0][current.1] = Field::Red;

        if current.0 == next.0 {
            // Edge along y
            let direction = if current.1 < next.1 {
                Direction::Down
            } else {
                Direction::Up
            };
            for y in (min(current.1, next.1) + 1)..max(current.1, next.1) {
                map[current.0][y] = Field::Edge(direction);
            }
        } else {
            // Edge along x
            let direction = if current.0 < next.0 {
                Direction::Right
            } else {
                Direction::Left
            };
            for x in (min(current.0, next.0) + 1)..max(current.0, next.0) {
                map[x][current.1] = Field::Edge(direction);
            }
        }
    }
}

pub fn find_point_inside(map: &Vec<Vec<Field>>) -> Option<(usize, usize)> {
    if map.len() < 1 {
        return None; // Why the f would you give me an empty array lol
    }

    // Let's start with the middle
    for x in 1..(map.len() - 1) {
        for y in 1..(map[0].len() - 1) {
            if is_inside((x, y), map) {
                return Some((x, y));
            }
        }
    }

    None
}

/// Checks if a given point is inside a polygon for the purposes of flood filling. Edges are considered _outside_
fn is_inside(point: (usize, usize), map: &Vec<Vec<Field>>) -> bool {
    match map[point.0][point.1] {
        Field::Red | Field::Edge(_) => return false,
        _ => {}
    };

    // We will be using the winding number rule, going from x=point.0 to x=max
    // If it's zero at the end, we are inside.
    // For each direction Down we subtract one, for each Up we add one.
    // Left and right we just skip.
    // Now this does pose an issue if we cross only horizontal edges and vertices...
    let mut winding_number: i64 = 0;
    let end = map.len();
    let mut x = point.0;
    while x + 1 < end {
        x += 1; // Starts at point + 1

        match map[x][point.1] {
            Field::Inside | Field::Outside => {}
            Field::Edge(Direction::Down) => {
                winding_number -= 1;
            }
            Field::Edge(Direction::Up) => {
                winding_number += 1;
            }
            Field::Edge(_) => {
                unreachable!("We can only ender collinear edges via a vertex")
            }
            Field::Red => {
                // OK so... This is not necessarily a problem, but it may mean we are about to enter a collinear edge....
                // Collinear edges give problems. Big ones. They make the result of the winding number _undefined_.
                // If we are about to enter, we need to take the direction in which we are entering it
                // and the direction to which we exit it,
                // simplifying that to a vertical.

                // Let's check if we're doing a vertical here
                if point.1 > 0 && point.1 + 1 < map[point.0].len() {
                    if map[point.0][point.1 - 1] == map[point.0][point.1 + 1] {
                        match map[point.0][point.1 - 1] {
                            Field::Edge(Direction::Down) => {
                                winding_number -= 1;
                                continue;
                            }
                            Field::Edge(Direction::Up) => {
                                winding_number += 1;
                                continue;
                            }
                            _ => {} // Not a vertical
                        }
                    }
                }

                // Ok we are entering a horizontal here of 0 or more length... Fek
                let mut exit_idx = x + 1;
                loop {
                    // This could in theory go off the end, but then we'll consider int malformed anyway..
                    match map[exit_idx][point.1] {
                        Field::Red => {
                            break;
                        } // Ok so this _will_ break if there's just a red in the middle of a horizontal, continuing horizontal...
                        Field::Edge(Direction::Left) | Field::Edge(Direction::Right) => {
                            // So we could determine the clockwiseness from here, but that doesn't work for zero-length
                            exit_idx += 1;
                        }
                        Field::Edge(_) => {
                            break;
                        }
                        _ => {
                            unreachable!("You have to exit an edge by a vertex")
                        }
                    }
                }

                // Shortcut: if we are at the top or bottom,
                // the horizontal edge _must_ exit and end in opposing directions, resulting in zero winding number influence
                //
                if point.1 == 0 || point.1 + 1 >= end {
                    x = exit_idx;
                    continue;
                }

                // OK so now, we check the connecting edge for idx and for exit_idx
                // If they mixmatch, no influence on the winding_number
                // If they match, treat winding number as if it was a normal edge!
                // We know we're not at the very top or bottom
                let entry_edge: Direction = if let Field::Edge(dir) = map[x][point.1 - 1]
                    && (dir == Direction::Up || dir == Direction::Down)
                {
                    // Above
                    dir
                } else if let Field::Edge(dir) = map[x][point.1 + 1]
                    && (dir == Direction::Up || dir == Direction::Down)
                {
                    dir
                } else {
                    unreachable!("Uhhh your vertex needs an edge man");
                };
                let exit_edge: Direction = if let Field::Edge(dir) = map[exit_idx][point.1 - 1]
                    && (dir == Direction::Up || dir == Direction::Down)
                {
                    // Above
                    dir
                } else if let Field::Edge(dir) = map[exit_idx][point.1 + 1]
                    && (dir == Direction::Up || dir == Direction::Down)
                {
                    dir
                } else {
                    unreachable!("Uhhh your vertex needs an edge man");
                };

                if entry_edge == exit_edge {
                    match entry_edge {
                        Direction::Up => {winding_number += 1;},
                        Direction::Down => {winding_number -= 1;},
                        _ => {unreachable!()},
                    };
                }

                x = exit_idx;
            }
        };
    }

    winding_number != 0
}

pub fn flood_fill(start: (usize, usize), map: &mut Vec<Vec<Field>>) {
    let mut stack = vec![start];

    while !stack.is_empty() {
        let point = stack
            .pop()
            .expect("Well if it aint empty, it can't be None");
        if map[point.0][point.1] != Field::Outside {
            continue;
        }

        map[point.0][point.1] = Field::Inside;
        if point.0 > 0 {
            stack.push((point.0 - 1, point.1)); // Left
            if point.1 > 0 {
                stack.push((point.0 - 1, point.1 - 1)); // Top left
            }
            if point.1 + 1 < map[0].len() {
                stack.push((point.0 - 1, point.1 + 1)); // Bottom left
            }
        }

        if point.0 + 1 < map.len() {
            stack.push((point.0 + 1, point.1)); // Right
            if point.1 > 0 {
                stack.push((point.0 + 1, point.1 - 1)); // Top right
            }
            if point.1 + 1 < map[0].len() {
                stack.push((point.0 + 1, point.1 + 1)); // Bottom right
            }
        }

        if point.1 > 0 {
            stack.push((point.0, point.1 - 1)); // Top
        }

        if point.1 + 1 < map[0].len() {
            stack.push((point.0, point.1 + 1)); // Bottom
        }
    }
}

#[allow(dead_code)]
pub fn map_to_string(map: &Vec<Vec<Field>>) -> String {
    let mut a = String::with_capacity(map.len() * map[0].len());
    for y in 0..map[0].len() {
        for x in 0..map.len() {
            a.push((&map[x][y]).into());
        }
        a.push('\n');
    }
    a
}

pub fn rect_valid(a: &(usize, usize), b: &(usize, usize), map: &Vec<Vec<Field>>) -> bool {
    for x in min(a.0, b.0)..=max(a.0, b.0) {
        for y in min(a.1, b.1)..=max(a.1, b.1) {
            if map[x][y] == Field::Outside {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flood_filling() {
        let input: Vec<(usize, usize)> = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];

        // I'm making the map 15
        let mut grid = vec![vec![Field::Outside; 8]; 12];

        mark_edges(&input, &mut grid);
        
        println!("{}", map_to_string(&grid));
        let point = find_point_inside(&grid).expect("Well there's certainly one");
        dbg!(point);

        flood_fill(point, &mut grid);

        println!("{}", map_to_string(&grid));
    }

    #[test]
    fn test_inside_outside() {
        let input: Vec<(usize, usize)> = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];

        // I'm making the map 15
        let mut grid = vec![vec![Field::Outside; 8]; 13];
        let mut text = String::with_capacity(8 * 12);

        mark_edges(&input, &mut grid);

        for y in 0..grid[0].len() {
            for x in 0..grid.len() {
                text.push(if is_inside((x, y), &grid) {
                    'i'
                } else {
                    (&grid[x][y]).into()
                });
            }
            text.push('\n');
        }
        println!("{}", text);
    }
}
