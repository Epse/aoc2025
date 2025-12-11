use super::Compute;
use super::MathOp;
use super::transpose;

pub fn part_two(input: &str) -> i64 {
    let input = input.strip_suffix("\n").unwrap_or(input); // We don't want trailing newlines, they get confusing
    let line_count = input.lines().count();
    let ops: Vec<MathOp> = input
        .lines()
        .last()
        .expect("We need a last line lol")
        .chars()
        .filter_map(|c| c.try_into().ok())
        .collect();

    let number_section = input
        .lines().take(line_count - 1)
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let number_run = transpose(number_section).into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .map(|line| line.trim_ascii().to_string())
        .map(|line| line.parse::<i64>().ok()); // Now we have Some(num), Some(num), None, repeat. Each run needs to be one group!

    let mut nums: Vec<Vec<i64>> = vec![];
    let mut row: Vec<i64> = vec![];
    for item in number_run {
        if let Some(number) = item {
            row.push(number);
        } else if row.len() > 0 {
            nums.push(std::mem::take(&mut row));
        }
    }
    nums.push(row);

    nums.into_iter()
        .enumerate()
        .map(|(idx, nums)| Compute {
            operation: ops[idx],
            numbers: nums
        })
        .fold(0_i64, |acc, c| c + acc)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_two() {
        assert_eq!(3263827, part_two(INPUT));
    }
}
