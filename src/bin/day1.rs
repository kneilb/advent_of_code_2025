// https://adventofcode.com/2025/day/1

fn day1(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut zeroes: i32 = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        let is_forward: bool = chars.next().unwrap() == 'R';
        let count: i32 = chars.as_str().parse::<i32>().unwrap();
        let delta: i32 = count * if is_forward { 1 } else { -1 };
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
        println!("{} : Position is {}, Zeroes is {}", line, position, zeroes);
    }

    zeroes
}

fn day1b(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut zeroes: i32 = 0;

    for line in input.lines() {
        let mut chars = line.chars();
        let change: i32 = if chars.next().unwrap() == 'R' { 1 } else { -1 };
        let mut count: i32 = chars.as_str().parse::<i32>().unwrap();
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
        println!("{} : Position is {}, Zeroes is {}", line, position, zeroes);
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
