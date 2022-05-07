use std::fs;

pub fn solutions() {
    let input = fs::read_to_string("input/d05.txt").unwrap();
    let lines = input.lines().map(|line| line.to_owned()).collect();
    let total_nice_strings_p1 = part_1(&lines);
    let total_nice_strings_p2 = part_2(&lines);
    println!("There are {total_nice_strings_p1} nice strings using part 1 rules");
    println!("There are {total_nice_strings_p2} nice strings using part 2 rules");
}

fn part_1(lines: &Vec<String>) -> usize {
    lines.iter().fold(0, |acc, line| {
        if !has_at_least_3_vowels(line)
            || !has_repeated_letter(line, 0)
            || has_forbidden_substrings(line)
        {
            return acc;
        }
        acc + 1
    })
}

fn part_2(lines: &Vec<String>) -> usize {
    lines.iter().fold(0, |acc, line| {
        if !has_repeated_pair_of_letters(line) || !has_repeated_letter(line, 1) {
            return acc;
        }
        acc + 1
    })
}

fn has_at_least_3_vowels(string: &str) -> bool {
    let total_vowels = string.chars().fold(0, |acc, c| match c {
        'a' | 'e' | 'i' | 'o' | 'u' => acc + 1,
        _ => acc,
    });
    total_vowels >= 3
}

fn has_repeated_letter(string: &str, separation: usize) -> bool {
    let window_size = separation + 2;
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(window_size)
        .any(|window| window[0] == window[window_size - 1])
}

fn has_forbidden_substrings(string: &str) -> bool {
    string
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .any(|window| match window {
            ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => true,
            _ => false,
        })
}

fn has_repeated_pair_of_letters(string: &str) -> bool {
    let pairs = string
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|window| format!("{}{}", window[0], window[1]))
        .collect::<Vec<_>>();

    pairs.iter().enumerate().any(|(i, pair)| {
        let rest = i + 2;
        if rest > pairs.len() {
            return false;
        }
        pairs[rest..].contains(pair)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn examples_p1() -> Vec<String> {
        vec![
            "ugknbfddgicrmopn".to_owned(),
            "aaa".to_owned(),
            "jchzalrnumimnmhp".to_owned(),
            "haegwjzuvuyypxyu".to_owned(),
            "dvszwmarrgswjxmb".to_owned(),
        ]
    }

    fn examples_p2() -> Vec<String> {
        vec![
            "qjhvhtzxzqqjkmpb".to_owned(),
            "xxyxx".to_owned(),
            "uurcxstgmygtbstg".to_owned(),
            "ieodomkazucvgmuy".to_owned(),
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
            .map(|example| has_at_least_3_vowels(example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![true, true, true, true, false])
    }

    #[test]
    fn has_repeated_letter_examples_p1() {
        let examples = examples_p1();
        let results = examples
            .iter()
            .map(|example| has_repeated_letter(example, 0))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![true, true, false, true, true])
    }

    #[test]
    fn has_repeated_letter_examples_p2() {
        let examples = examples_p2();
        let results = examples
            .iter()
            .map(|example| has_repeated_letter(example, 1))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![true, true, false, true])
    }

    #[test]
    fn has_forbidden_substrings_examples() {
        let examples = examples_p1();
        let results = examples
            .iter()
            .map(|example| has_forbidden_substrings(example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![false, false, false, true, false])
    }

    #[test]
    fn has_repeated_pair_of_letters_examples() {
        let examples = examples_p2();
        let results = examples
            .iter()
            .map(|example| has_repeated_pair_of_letters(example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![true, true, true, false])
    }
}
