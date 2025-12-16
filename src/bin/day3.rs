// https://adventofcode.com/2025/day/3

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect()
}

fn day3(input: &str) -> u32 {
    let mut total = 0;

    for battery in parse(input) {
        let (mut one, mut two) = (battery[0], battery[1]);

        for new in &battery[2..] {
            if two > one {
                one = two;
                two = *new;
            } else if *new > two {
                two = *new;
            }
        }

        total += one * 10 + two;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_example() {
        let data = std::fs::read_to_string("data/day3_example.txt").unwrap();
        let result = day3(&data);
        assert_eq!(result, 357);
    }

    #[test]
    fn day3_real() {
        let data = std::fs::read_to_string("data/day3.txt").unwrap();
        let result = day3(&data);
        assert_eq!(result, 17166);
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day3.txt").unwrap();
    let result = day3(&data);
    println!("The result is {}", result);
}
