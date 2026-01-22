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

    let mapping = s
        .into_iter()
        .enumerate()
        .map(|(x, y)| (*y, x))
        .collect::<HashMap<u32, usize>>();

    input
        .into_iter()
        .filter_map(|x| mapping.get(&x))
        .map(|x| *x)
        .collect()
}

#[cfg(test)]
mod test {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_compression() {
        let input: Vec<(u32, u32)> = vec![(100, 700), (203, 900), (300, 700), (300, 900)];
        let output: Vec<(usize, usize)> = vec![(0, 0), (1, 1), (2, 0), (2, 1)];
        assert_eq!(output, coordinate_compress(&input));
    }

    fn gen_test_vec(elements: usize, unique: usize) -> Vec<u32> {
        let mut rng = rand::rng();
        let elems = (0..unique)
            .map(|_| rng.random::<u32>())
            .collect::<Vec<u32>>();
        (0..elements)
            .map(|_| elems[rng.random_range(0..unique)])
            .collect()
    }
}
