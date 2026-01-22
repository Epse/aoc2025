pub fn run() {
    let input = std::fs::read_to_string("data/eight").expect("Need input file");
    {
        let start = Instant::now();
        let result = part_one(&input, 1000);
        let elapsed = start.elapsed();
        println!("Day 8 part one: {}, elapsed: {:.2?}", result, elapsed);
    }
    {
        let start = Instant::now();
        let result = part_two(&input);
        let elapsed = start.elapsed();
        println!("Day 8 part two: {}, elapsed: {:.2?}", result, elapsed);
    }
}

mod link;
mod union_find;
mod vector;

use std::collections::HashMap;
use std::time::Instant;

use link::Link;
use union_find::UnionFind;
use vector::Vector3;

fn parse_coordinates(input: &str) -> Vec<Vector3> {
    input
        .lines()
        .filter_map(|l| Vector3::try_from(l).ok())
        .collect()
}

fn part_one(input: &str, connection_count: usize) -> u64 {
    let coords = parse_coordinates(input);
    let distances = calculate_distances(&coords);
    let mut distances = distances.into_iter().collect::<Vec<(i64, Link)>>();
    // Unstable sort be zoomin more
    distances.sort_unstable_by(|(a, _link_a), (b, _link_b)| b.cmp(a));

    let mut uf = UnionFind::from_values(coords);

    for _ in 0..connection_count {
        let link = distances.pop().expect("We should have enough");
        let link = link.1.split();
        let _ = uf.union_values(&link.0, &link.1).expect("Missing items??");
    }

    println!("We have {} circuits", uf.set_count());
    let mut counts = uf.sizes().clone();
    counts.sort();
    counts
        .into_iter()
        .rev()
        .take(3)
        .fold(1u64, |acc, elem| acc * elem as u64)
}

fn part_two(input: &str) -> u64 {
    let coords = parse_coordinates(input);
    let distances = calculate_distances(&coords);
    let mut distances = distances.into_iter().collect::<Vec<(i64, Link)>>();
    // Unstable sort be zoomin more
    distances.sort_unstable_by(|(a, _link_a), (b, _link_b)| b.cmp(a));

    let mut uf = UnionFind::from_values(coords);

    loop {
        let link = distances.pop().expect("We should have enough");
        let link = link.1.split();
        let merged = uf.union_values(&link.0, &link.1).expect("Missing items??");
        if uf.set_count() == 1 && merged {
            dbg!(&link);
            return link.0.x as u64 * link.1.x as u64;
        }
    }
}

fn calculate_distances(input: &Vec<Vector3>) -> HashMap<i64, Link> {
    let mut output = HashMap::with_capacity(input.len()); // Guesswork capacity

    for i in 0..input.len() {
        for j in i..input.len() {
            if i == j {
                continue;
            }
            let link = Link::new(input[i].clone(), input[j].clone());
            output.entry(&input[i] ^ &input[j]).or_insert(link);
        }
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part_one() {
        assert_eq!(40, part_one(INPUT, 10));
    }
}
