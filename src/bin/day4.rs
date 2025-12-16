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
    (-1, -1), // up / left
    (0, -1),  // up
    (1, -1),  // up / right
    (-1, 0),  // left
    // no (0, 0) - that's us
    (1, 0),   // right
    (-1, 1),  // down / left
    (0, 1),   // down
    (1, 1)    // down / right
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

fn day4b(input: &str) -> u32 {
    let mut total_moved_rolls = 0;

    let mut grid = parse(input);

    // Assumes all row & columns have the same length...
    let height: usize = grid.len();
    let width: usize = grid[0].len();

    loop {
        let mut moved_rolls = 0;
        for y in 0..height {
            for x in 0..width {
                let mut has_roll = &grid[y][x];
                println!("FOUND {} at {}, {}", *has_roll, x, y);
                if *has_roll && is_accessible(&grid, x, y) {
                    grid[y][x] = false;
                    moved_rolls += 1;
                }
            }
        }
        total_moved_rolls += moved_rolls;
        if moved_rolls == 0 {
            break;
        }
    }

    total_moved_rolls
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

    #[test]
    fn day4b_example() {
        let data = std::fs::read_to_string("data/day4_example.txt").unwrap();
        let result = day4b(&data);
        assert_eq!(result, 43);
    }

    #[test]
    fn day4b_real() {
        let data = std::fs::read_to_string("data/day4.txt").unwrap();
        let result = day4b(&data);
        assert_eq!(result, 9397);
    }

}
