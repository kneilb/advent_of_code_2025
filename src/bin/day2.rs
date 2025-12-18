// https://adventofcode.com/2025/day/2

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .flat_map(|line| line.split(',')) // Uses flatten to turn the Vec<Vec<str>> into Vec<str>
        .map(|range| range.split('-'))
        .map(|mut ends| {
            (
                ends.next().unwrap().parse().unwrap(),
                ends.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn day_2_part_1(input: &str) -> usize {
    let mut total = 0;

    for (start, end) in parse(input) {
        for current in start..=end {
            let string_num = format!("{}", current);
            // Check string has an even number of characters.
            // Makes no sense otherwise!
            if string_num.chars().count() % 2 == 0 {
                let (s1, s2) = string_num.split_at(string_num.chars().count() / 2);
                if s1 == s2 {
                    // println!("{} {}", s1, s2);
                    total += current;
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_2_part_1_example() {
        let data = std::fs::read_to_string("data/day2_example.txt").unwrap();
        let result = day_2_part_1(&data);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn day_2_part_1_real() {
        let data = std::fs::read_to_string("data/day2.txt").unwrap();
        let result = day_2_part_1(&data);
        assert_eq!(result, 44487518055);
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day2.txt").unwrap();

    let result_1 = day_2_part_1(&data);
    println!("Part 1: {}", result_1);
}
