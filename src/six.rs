use std::ops::Add;

pub fn run() {
    let input = std::fs::read_to_string("data/six").expect("I need data wth");
    let result = part_one(&input);
    println!("Part one: {}", result);
}

fn part_one(input: &str) -> i64 {
    string_to_computes(input)
        .iter()
        .fold(0_i64, |acc, current| current + acc)
}

fn string_to_computes(input: &str) -> Vec<Compute> {
    let input = input.strip_suffix("\n").unwrap_or(input); // We don't want trailing newlines, they get confusing
    let line_count = input.lines().count();
    let ops: Vec<MathOp> = input
        .lines()
        .last()
        .expect("We need a last line lol")
        .chars()
        .filter_map(|c| c.try_into().ok())
        .collect();
    let number_lines = input
        .lines()
        .take(line_count - 1) // Not the ops line!
        .map(|line| {
            line.split(' ')
                .filter_map(|item| {
                    if item.len() == 0 {
                        None
                    } else {
                        item.parse::<i64>().ok()
                    }
                })
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();

    let number_lines = transpose(number_lines); // This is now a vec of columns!
    number_lines.into_iter()
        .enumerate()
        .map(|(idx, numbers)| Compute {
            numbers: numbers,
            operation: ops[idx]
        })
        .collect()
}

// Source - https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
// Posted by Netwave, modified by community. See post 'Timeline' for change history
// Retrieved 2025-12-11, License - CC BY-SA 4.0
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}


struct Compute {
    numbers: Vec<i64>,
    operation: MathOp,
}

impl Compute {
    fn compute(&self) -> i64 {
        self.numbers
            .iter()
            .cloned() // Not a fan but otherwise we fckin
            .reduce(|acc, current| match self.operation {
                MathOp::Add {} => acc + current,
                MathOp::Subtract {} => acc - current,
                MathOp::Divide {} => acc / current,
                MathOp::Multiply {} => acc * current,
            })
            .expect("If this dont work then wth we doin")
    }
}

impl Add for Compute {
    type Output = i64;

    fn add(self, other: Self) -> i64 {
        self.compute() + other.compute()
    }
}

impl Add<i64> for Compute {
    type Output = i64;

    fn add(self, other: i64) -> i64 {
        self.compute() + other
    }
}

impl<'a> Add<i64> for &'a Compute {
    type Output = i64;

    fn add(self, other: i64) -> i64 {
        self.compute() + other
    }
}

#[derive(Clone, Copy, Debug)]
enum MathOp {
    Add {},
    Subtract {},
    Divide {},
    Multiply {},
}

impl TryFrom<char> for MathOp {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(MathOp::Add {}),
            '-' => Ok(MathOp::Subtract {}),
            '*' => Ok(MathOp::Multiply {}),
            '/' => Ok(MathOp::Divide {}),
            _ => Err("Not a math operator"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_one() {
        assert_eq!(4277556, part_one(INPUT));
    }
}
