// https://adventofcode.com/2025/day/5

use std::collections::HashSet;

struct Range {
    start: usize,
    end: usize,
}

fn parse(input: &str) -> (Vec<Range>, Vec<usize>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    (
        ranges
            .lines()
            .map(|line| {
                let mut tokens = line.split("-");
                let start = tokens.next().unwrap().parse().unwrap();
                let end = tokens.next().unwrap().parse().unwrap();
                Range { start, end }
            })
            .collect(),
        ids.lines().map(|line| line.parse().unwrap()).collect(),
    )
}

fn day_5_part_1(input: &str) -> usize {
    let (ranges, ids) = parse(input);
    let mut fresh_ingredients = 0;

    for ingredient_id in ids {
        for fresh_range in &ranges {
            if ingredient_id >= fresh_range.start && ingredient_id <= fresh_range.end {
                fresh_ingredients += 1;
                break;
            }
        }
    }

    fresh_ingredients
}

// TODO: optimise!!! :D
fn day_5_part_2(input: &str) -> usize {
    let (ranges, _ids) = parse(input);

    let mut fresh_ids: HashSet<usize> = HashSet::new();

    for fresh_range in &ranges {
        for id in fresh_range.start..=fresh_range.end {
            fresh_ids.insert(id);
        }
    }

    fresh_ids.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_5_part_1_example() {
        let data = std::fs::read_to_string("data/day5_example.txt").unwrap();
        let result = day_5_part_1(&data);
        assert_eq!(result, 3);
    }

    #[test]
    fn day_5_part_1_real() {
        let data = std::fs::read_to_string("data/day5.txt").unwrap();
        let result = day_5_part_1(&data);
        assert_eq!(result, 661);
    }

    #[test]
    fn day_5_part_2_example() {
        let data = std::fs::read_to_string("data/day5_example.txt").unwrap();
        let result = day_5_part_2(&data);
        assert_eq!(result, 14);
    }

    // #[test]
    // fn day_5_part_2_real() {
    //     let data = std::fs::read_to_string("data/day5.txt").unwrap();
    //     let result = day_5_part_2(&data);
    //     assert_eq!(result, 661);
    // }
}

fn main() {
    let data = std::fs::read_to_string("data/day5.txt").unwrap();

    let result_1 = day_5_part_1(&data);
    println!("Part 1: {}", result_1);

    // let result_2 = day_5_part_2(&data);
    // println!("Part 2: {}", result_2);
}
