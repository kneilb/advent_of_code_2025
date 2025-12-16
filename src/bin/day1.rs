// https://adventofcode.com/2025/day/1

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let (dir, rest) = line.trim().split_at(1);
            let count: i32 = rest.parse().unwrap();

            match dir.chars().next() {
                Some('L') => count * -1,
                Some('R') => count,
                _ => panic!("Unknown direction"),
            }
        })
        .collect()
}

fn day1(input: &str) -> i32 {
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

fn day1b(input: &str) -> i32 {
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
    fn day1_example() {
        let data = std::fs::read_to_string("data/day1_example.txt").unwrap();
        let result = day1(&data);
        assert_eq!(result, 3);
    }

    #[test]
    fn day1_real() {
        let data = std::fs::read_to_string("data/day1.txt").unwrap();
        let result = day1(&data);
        assert_eq!(result, 1191);
    }

    #[test]
    fn day1b_example() {
        let data = std::fs::read_to_string("data/day1_example.txt").unwrap();
        let result = day1b(&data);
        assert_eq!(result, 6);
    }

    #[test]
    fn day1b_real() {
        let data = std::fs::read_to_string("data/day1.txt").unwrap();
        let result = day1b(&data);
        assert_eq!(result, 6858);
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day1.txt").unwrap();
    let result = day1(&data);
    println!("The result is {}", result);
}
