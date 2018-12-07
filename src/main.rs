use std::collections::HashSet;

fn day1_part1(input: &str) -> i32 {
    // let sum = content.lines().fold(0, |acc, x| acc + &x.parse().unwrap());
    return input.lines().map(|x| x.parse::<i32>().unwrap()).sum();
}

fn day1_part2(input: &str) -> i32 {
    let mut already_seen_frequencies = HashSet::new();
    already_seen_frequencies.insert(0);
    let mut frequency: i32 = 0;
    for l in input.lines().cycle() {
        frequency += l.parse::<i32>().unwrap();
        if already_seen_frequencies.contains(&frequency) {
            return frequency;
        }
        already_seen_frequencies.insert(frequency);
    }
    panic!("Should not be reached");
}

fn main() {
    println!("# day 1");
    println!("part 1: {}", day1_part1(include_str!("day1.input")));
    println!("part 2: {}", day1_part2(include_str!("day1.input")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        assert_eq!(day1_part1("+1\n+1\n+1"), 3);
        assert_eq!(day1_part1("+1\n+1\n-2"), 0);
        assert_eq!(day1_part1("-1\n-2\n-3"), -6);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(day1_part2("+1\n-1"), 0);
        assert_eq!(day1_part2("+3\n+3\n+4\n-2\n-4"), 10);
        assert_eq!(day1_part2("-6\n+3\n+8\n+5\n-6"), 5);
        assert_eq!(day1_part2("+7\n+7\n-2\n-7\n-4"), 14);
    }
}
