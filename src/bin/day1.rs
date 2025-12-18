// https://adventofcode.com/2025/day/1

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let (dir, rest) = line.trim().split_at(1);
            let count: i32 = rest.parse().unwrap();

            match dir.chars().next() {
                Some('L') => -count,
                Some('R') => count,
                _ => panic!("Unknown direction"),
            }
        })
        .collect()
}

fn day_1_part_1(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut zeroes: i32 = 0;

    for delta in parse(input) {
        position += delta;
        while position < 0 {
            position += 100;
        }
        while position > 99 {
            position -= 100;
        }
        if position == 0 {
            zeroes += 1;
        }
        println!("{} : Position is {}, Zeroes is {}", delta, position, zeroes);
    }

    zeroes
}

fn day_1_part_2(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut zeroes: i32 = 0;

    for delta in parse(input) {
        let mut count: i32 = i32::abs(delta);
        let change = delta / count;
        while count > 0 {
            position += change;
            if position > 99 {
                position -= 100;
            }
            if position < 0 {
                position += 100;
            }
            if position == 0 {
                zeroes += 1;
            }
            count -= 1;
        }
        println!("{} : Position is {}, Zeroes is {}", delta, position, zeroes);
    }

    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_1_part_1_example() {
        let data = std::fs::read_to_string("data/day1_example.txt").unwrap();
        let result = day_1_part_1(&data);
        assert_eq!(result, 3);
    }

    #[test]
    fn day_1_part_1_real() {
        let data = std::fs::read_to_string("data/day1.txt").unwrap();
        let result = day_1_part_1(&data);
        assert_eq!(result, 1191);
    }

    #[test]
    fn day_1_part_2_example() {
        let data = std::fs::read_to_string("data/day1_example.txt").unwrap();
        let result = day_1_part_2(&data);
        assert_eq!(result, 6);
    }

    #[test]
    fn day_1_part_2_real() {
        let data = std::fs::read_to_string("data/day1.txt").unwrap();
        let result = day_1_part_2(&data);
        assert_eq!(result, 6858);
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day1.txt").unwrap();

    let result_1 = day_1_part_1(&data);
    println!("Part 1: {}", result_1);

    let result_2 = day_1_part_2(&data);
    println!("Part 2: {}", result_2);
}
