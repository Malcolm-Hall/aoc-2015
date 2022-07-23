use std::{collections::HashMap, fs};

pub fn solutions() {
    let input = fs::read_to_string("input/d05.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let total_nice_strings_p1 = part_1(&lines);
    let total_nice_strings_p2 = part_2(&lines);
    println!("There are {total_nice_strings_p1} nice strings using part 1 rules");
    println!("There are {total_nice_strings_p2} nice strings using part 2 rules");
}

fn part_1(lines: &[&str]) -> u32 {
    let mut total = 0;
    for line in lines {
        let chars = chars(line);
        if has_at_least_3_vowels(&chars)
            && has_repeated_letter(&chars, 0)
            && !has_forbidden_substrings(&chars)
        {
            total += 1;
        }
    }
    total
}

fn part_2(lines: &[&str]) -> u32 {
    let mut total = 0;
    for line in lines {
        let chars = chars(line);
        if has_repeated_pair_of_letters(&chars) && has_repeated_letter(&chars, 1) {
            total += 1;
        }
    }
    total
}

fn chars(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn has_at_least_3_vowels(chars: &[char]) -> bool {
    let mut total_vowels = 0;
    for c in chars {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => total_vowels += 1,
            _ => (),
        }
    }
    total_vowels >= 3
}

fn has_repeated_letter(chars: &[char], separation: usize) -> bool {
    let window_size = separation + 2;
    chars
        .windows(window_size)
        .any(|window| window[0] == window[window_size - 1])
}

fn has_forbidden_substrings(chars: &[char]) -> bool {
    chars.windows(2).any(|window| match window {
        ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => true,
        _ => false,
    })
}

fn has_repeated_pair_of_letters(chars: &[char]) -> bool {
    let mut found_pairs: HashMap<&[char], usize> = HashMap::new();
    for (idx, window) in chars.windows(2).enumerate() {
        if let Some(prev_idx) = found_pairs.get(window) {
            if idx != prev_idx + 1 {
                return true;
            }
        } else {
            found_pairs.insert(window, idx);
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    fn examples_p1() -> Vec<&'static str> {
        vec![
            "ugknbfddgicrmopn",
            "aaa",
            "jchzalrnumimnmhp",
            "haegwjzuvuyypxyu",
            "dvszwmarrgswjxmb",
        ]
    }

    fn examples_p2() -> Vec<&'static str> {
        vec![
            "qjhvhtzxzqqjkmpb",
            "xxyxx",
            "uurcxstgmygtbstg",
            "ieodomkazucvgmuy",
        ]
    }

    #[test]
    fn part_1_examples() {
        let examples = examples_p1();
        let result = part_1(&examples);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_2_examples() {
        let examples = examples_p2();
        let result = part_2(&examples);
        assert_eq!(result, 2)
    }

    #[test]
    fn has_at_least_3_vowels_examples() {
        let examples = examples_p1();
        let results = examples
            .iter()
            .map(|example| chars(&example))
            .map(|chars| has_at_least_3_vowels(&chars))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![true, true, true, true, false])
    }

    #[test]
    fn has_repeated_letter_examples_p1() {
        let examples = examples_p1();
        let results = examples
            .iter()
            .map(|example| chars(&example))
            .map(|chars| has_repeated_letter(&chars, 0))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![true, true, false, true, true])
    }

    #[test]
    fn has_repeated_letter_examples_p2() {
        let examples = examples_p2();
        let results = examples
            .iter()
            .map(|example| chars(&example))
            .map(|chars| has_repeated_letter(&chars, 1))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![true, true, false, true])
    }

    #[test]
    fn has_forbidden_substrings_examples() {
        let examples = examples_p1();
        let results = examples
            .iter()
            .map(|example| chars(&example))
            .map(|chars| has_forbidden_substrings(&chars))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![false, false, false, true, false])
    }

    #[test]
    fn has_repeated_pair_of_letters_examples() {
        let examples = examples_p2();
        let results = examples
            .iter()
            .map(|example| chars(&example))
            .map(|chars| has_repeated_pair_of_letters(&chars))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![true, true, true, false])
    }
}
