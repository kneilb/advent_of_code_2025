// https://adventofcode.com/2025/day/2

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .flat_map(|line| line.split(',')) // Flat map removes the extra level of iterators!
        .map(|range| range.split('-'))
        .map(|mut ends| {
            (
                ends.next().unwrap().parse().unwrap(),
                ends.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn day2(input: &str) -> usize {
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
    fn day2_example() {
        let data = std::fs::read_to_string("data/day2_example.txt").unwrap();
        let result = day2(&data);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn day2_real() {
        let data = std::fs::read_to_string("data/day2.txt").unwrap();
        let result = day2(&data);
        assert_eq!(result, 44487518055);
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day2.txt").unwrap();
    let result = day2(&data);
    println!("The result is {}", result);
}
