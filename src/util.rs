#[allow(dead_code)]
pub fn display_grid(grid: &Vec<Vec<char>>) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect::<String>()
}
