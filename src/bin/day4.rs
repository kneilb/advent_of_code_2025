// https://adventofcode.com/2025/day/4

#[derive(Copy, Clone, Debug, PartialEq)]
enum Content {
    Empty,
    RollOfPaper,
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), // up / left
    (0, -1),  // up
    (1, -1),  // up / right
    (-1, 0),  // left
    (1, 0),   // right
    (-1, 1),  // down / left
    (0, 1),   // down
    (1, 1),   // down / right
];

struct Grid {
    grid: Vec<Vec<Content>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<Content>>) -> Grid {
        let width = grid[0].len();
        let height = grid.len();
        Grid {
            grid,
            width,
            height,
        }
    }

    fn remove_paper(&mut self, x: usize, y: usize) {
        assert!(self.grid[y][x] == Content::RollOfPaper);
        self.grid[y][x] = Content::Empty;
    }

    fn can_move_paper(&self, x: usize, y: usize) -> bool {
        if self.grid[y][x] == Content::Empty {
            return false;
        }

        let mut surrounding_rolls = 0;

        for dir in DIRECTIONS {
            let x: i32 = x as i32 + dir.0;
            let y: i32 = y as i32 + dir.1;

            if x < 0 || y < 0 {
                continue;
            }

            let x: usize = x as usize;
            let y: usize = y as usize;
            if y >= self.height || x >= self.width {
                continue;
            }

            if self.grid[y][x] == Content::RollOfPaper {
                surrounding_rolls += 1;
            }
        }

        surrounding_rolls < 4
    }
}

fn parse(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '@' => Content::RollOfPaper,
                    '.' => Content::Empty,
                    _ => panic!("Unexpected panda in the bagging area"),
                })
                .collect()
        })
        .collect();

    Grid::new(grid)
}

fn day4(input: &str) -> u32 {
    let mut accessible_rolls = 0;

    let grid = parse(input);

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.can_move_paper(x, y) {
                accessible_rolls += 1;
            }
        }
    }

    accessible_rolls
}

fn day4b(input: &str) -> u32 {
    let mut total_moved_rolls = 0;

    let mut grid = parse(input);

    loop {
        let mut moved_rolls = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.can_move_paper(x, y) {
                    grid.remove_paper(x, y);
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
