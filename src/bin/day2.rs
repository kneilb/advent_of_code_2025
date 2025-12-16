// https://adventofcode.com/2025/day/2

fn day2(input: &str) -> i64 {
    let mut total = 0;

    for line in input.lines() {
        for range in line.split(",") {
            let mut items = range.split("-");
            let start: i64 = items.next().unwrap().parse().unwrap();
            let end: i64 = items.next().unwrap().parse().unwrap();

            // println!("RANGE: {} to {}", start, end);

            let mut current = start;
            while current <= end {
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
                current += 1;
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
