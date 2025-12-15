// https://adventofcode.com/2025/day/1

fn day1(input: &str) -> i32 {
    let mut position : i32 = 50;
    let mut zeroes : i32 = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        let is_forward : bool = chars.next().unwrap() == 'R';
        let count : i32 = chars.as_str().parse::<i32>().unwrap();
        let delta : i32 = count * if is_forward { 1 } else { -1 };
        position += delta;
        while position < 0 {
            position += 100;
        }
        while position > 99 {
            position -= 100;
        }
        println!("Position is {}", position);
        if position == 0 {
            zeroes += 1;
        }
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
        assert_eq!(result, 2196996);
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day1.txt").unwrap();
    let result = day1(&data);
    println!("The result is {}", result);
}
