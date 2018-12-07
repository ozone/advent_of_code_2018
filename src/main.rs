use std::collections::HashMap;
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

fn day2_part1(input: &str) -> i32 {
    let mut ids_with_duplicate_letter = 0;
    let mut ids_with_triplicate_letter = 0;
    for l in input.lines() {
        let mut letter_counts = HashMap::new();
        for c in l.chars() {
            let counter = letter_counts.entry(c).or_insert(0);
            *counter += 1;
        }
        if letter_counts.values().any(|&count| count == 2) {
            ids_with_duplicate_letter += 1;
        }
        if letter_counts.values().any(|&count| count == 3) {
            ids_with_triplicate_letter += 1;
        }
    }
    return ids_with_duplicate_letter * ids_with_triplicate_letter;
}

fn day2_part2(input: &str) -> String {
    // In this map, the key is a candidate string for the solution, and the value is the string
    // we derived it from.
    let mut candidates = HashMap::new();
    for l in input.lines() {
        for i in 0..l.len() {
            let candidate = l.chars()  // Iterate over all characters
                .enumerate()  // Add index
                .filter(|(pos, _)| *pos != i)  // Keep all chars except the one at index i
                .map(|(_, c)| c)  // Drop the index
                .collect::<String>();  // Reassemble the string
            let matching_line = candidates.entry(candidate.clone()).or_insert(l);
            // Avoid matching with itself. This occurs for consecutive repeated characters.
            // e.g.  "aa"
            if *matching_line != l {
                return candidate;
            }
        }
    }
    panic!("No solutions!");
}

fn main() {
    println!("# day 1");
    println!("part 1: {}", day1_part1(include_str!("day1.input")));
    println!("part 2: {}", day1_part2(include_str!("day1.input")));

    println!("# day 2");
    println!("part 1: {}", day2_part1(include_str!("day2.input")));
    println!("part 2: {}", day2_part2(include_str!("day2.input")));
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

    #[test]
    fn test_day2_part1() {
        assert_eq!(day2_part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"), 12);
    }

    #[test]
    fn test_day2_part2() {
        assert_eq!(day2_part2("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz").as_str(), "fgij");
        assert_eq!(day2_part2("aa\nbc\nbd").as_str(), "b");
    }
}
