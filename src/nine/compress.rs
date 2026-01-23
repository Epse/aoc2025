use std::collections::{HashMap, HashSet};

pub fn coordinate_compress(input: &Vec<(u32, u32)>) -> Vec<(usize, usize)> {
    let (x, y) = input.iter().cloned().unzip();

    compress_axis(x)
        .into_iter()
        .zip(compress_axis(y).into_iter())
        .collect::<Vec<(usize, usize)>>()
}

fn compress_axis(input: Vec<u32>) -> Vec<usize> {
    let mut s = HashSet::new();
    for x in &input {
        s.insert(x);
    }
    let mut s: Vec<u32> = s.into_iter().map(|x| *x).collect();
    s.sort();

    let mapping = s
        .into_iter()
        .enumerate()
        .map(|(idx, y)| (y, idx))
        .collect::<HashMap<u32, usize>>();

    input
        .into_iter()
        .filter_map(|x| mapping.get(&x))
        .map(|x| *x)
        .collect()
}

pub fn decompress(point: &(usize, usize), compressed: &Vec<(usize, usize)>, original: &Vec<(u32, u32)>) -> Option<(u32, u32)> {
    let point_index = compressed.iter()
        .enumerate()
        .find(|(_idx, candidate)| *candidate == point)
        .map(|(idx, _pt)| idx)?;

    original.get(point_index).copied()
}

#[cfg(test)]
mod test {
    use crate::nine::two::*;

    use super::*;

    #[test]
    fn test_compression() {
        let input: Vec<(u32, u32)> = vec![(100, 700), (203, 900), (300, 700), (300, 900)];
        let output: Vec<(usize, usize)> = vec![(0, 0), (1, 1), (2, 0), (2, 1)];
        assert_eq!(output, coordinate_compress(&input));
    }

    #[test]
    fn test_decompress() {
        let input: Vec<(u32, u32)> = vec![(100, 700), (203, 900), (300, 700), (300, 900)];
        let compressed: Vec<(usize, usize)> = vec![(0, 0), (1, 1), (2, 0), (2, 1)];
        assert_eq!((203, 900), decompress(&(1, 1), &compressed, &input).expect("Lol gimme response"));
    }

    #[test]
    fn compare_compressed() {
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

        let result =
"·#→#
##·↓
#←#↓
··##
";

        let compressed = coordinate_compress(&input);

        let input: Vec<(usize, usize)> = input.into_iter()
            .map(|(a, b)| (a as usize, b as usize))
            .collect();

        let mut grid_orig = vec![vec![Field::Outside; 8];  12];
        mark_edges(&input, &mut grid_orig);

        let x_max = *compressed.iter().map(|(x, _y)| x).max().unwrap();
        let y_max = *compressed.iter().map(|(_x, y)| y).max().unwrap();
        let mut grid_compressed = vec![vec![Field::Outside; y_max + 1];  x_max + 1];
        mark_edges(&compressed, &mut grid_compressed);

        println!("{}", map_to_string(&grid_orig));
        println!("{}", map_to_string(&grid_compressed));
        assert_eq!(result, map_to_string(&grid_compressed));
    }
}
