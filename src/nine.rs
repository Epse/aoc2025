use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

mod compress;
use compress::coordinate_compress;
mod two;
use two::*;

pub fn run() {
    let path = Path::new("data/nine");
    let file = File::open(&path).expect("Need input file");
    let coordinates = io::BufReader::new(file)
        .lines()
        .filter_map(|x| parse_coord(&x.expect("IDK I need a line")))
        .collect::<Vec<(u32, u32)>>();

    {
        let start = Instant::now();
        let result = part_one(&coordinates);
        let elapsed = start.elapsed();
        println!("Day 9 part 1: {}, elapsed: {:.2?}", result, elapsed);
    }

    {
        let start = Instant::now();
        let result = part_two(&coordinates);
        let elapsed = start.elapsed();
        println!("Day 9 part 2: {}, elapsed: {:.2?}", result, elapsed);
    }
}

fn parse_coord(a: &str) -> Option<(u32, u32)> {
    let (x, y) = a.split_once(',')?;
    Some((x.parse::<u32>().ok()?, y.parse::<u32>().ok()?))
}

fn part_one(input: &Vec<(u32, u32)>) -> u64 {
    let mut largest_area: u64 = 0;

    for i in 0..input.len() {
        for j in i..input.len() {
            largest_area = max(largest_area, area(input[i], input[j]));
        }
    }

    largest_area
}

fn area(x: (u32, u32), y: (u32, u32)) -> u64 {
    (x.0.abs_diff(y.0) as u64 + 1) * (x.1.abs_diff(y.1) as u64 + 1)
}

fn part_two(input: &Vec<(u32, u32)>) -> u64 {
    // We compress the coordinates. We can decompress by index lookup
    let compressed = coordinate_compress(input);

    let x_max = *compressed.iter().map(|(x, _y)| x).max().unwrap();
    let y_max = *compressed.iter().map(|(_x, y)| y).max().unwrap();

    let mut grid = vec![vec![Field::Outside; y_max + 1];  x_max + 1];

    mark_edges(&compressed, &mut grid);

    // Now my strategy is finding a point inside, flood filling and then trying every rectangle lol
    let point = find_point_inside(&grid).expect("There needs to be a point in");
    flood_fill(point, &mut grid);


    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input: Vec<(u32, u32)> = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];
        assert_eq!(50u64, part_one(&input));
    }

    #[test]
    fn test_some_areas() {
        assert_eq!(24, area((2, 5), (9, 7)));
    }
}
