// https://adventofcode.com/2025/day/4

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|char| char == '@')
                .collect()
        })
        .collect()
}

fn day4(input: &str) -> u32 {
    let mut accessible_rolls = 0;

    let grid = parse(input);

    for (y, row) in grid.clone().into_iter().enumerate() {
        for (x, col) in row.into_iter().enumerate() {
            println!("FOUND {} at {}, {}", col, x, y);
            if col && is_accessible(&grid, x, y) {
                accessible_rolls += 1;
            }
        }
    }

    accessible_rolls
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    // no (0, 0) - that's us
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1)
];

fn is_accessible(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    let mut surrounding_rolls = 0;

    for dir in DIRECTIONS {
        let search_x: i32 = x as i32 + dir.0;
        let search_y: i32 = y as i32 + dir.1;

        if search_x < 0 || search_y < 0 {
            continue;
        }

        let search_x: usize = search_x as usize;
        let search_y: usize = search_y as usize;
        if search_y >= grid.len() || search_x >= grid[0].len() {
            continue;
        }

        if grid[search_y][search_x] {
            surrounding_rolls += 1;
        }
    }

    surrounding_rolls < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_example() {
        let data = std::fs::read_to_string("data/day4_example.txt").unwrap();
        let result = day4(&data);
        assert_eq!(result, 13);
    }

    #[test]
    fn day4_real() {
        let data = std::fs::read_to_string("data/day4.txt").unwrap();
        let result = day4(&data);
        assert_eq!(result, 1604);
    }
}
