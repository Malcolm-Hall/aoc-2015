use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input/d11.txt").unwrap();
    let first_new_password = get_next_password(&input);
    let second_new_password = get_next_password(&first_new_password);
    println!("Santa's first new password is {first_new_password}");
    println!("Santa's second new password is {second_new_password}");
}

fn get_next_password(input: &str) -> String {
    let mut new_password = increment_password(input);
    while !meets_requirements(&new_password) {
        new_password = increment_password(&new_password);
    }
    new_password
}

fn increment_letter(c: char) -> char {
    if c == 'z' {
        'a'
    } else {
        (c as u8 + 1) as char
    }
}

fn increment_password(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    input.chars().rev().fold(true, |wrap, next| {
        if wrap {
            let inc = increment_letter(next);
            output.push(inc);
            inc == 'a'
        } else {
            output.push(next);
            false
        }
    });
    output.chars().rev().collect()
}

fn meets_requirements(input: &str) -> bool {
    has_increasing_straight(input)
        && !has_forbidden_letters(input)
        && has_two_different_pairs(input)
}

fn has_increasing_straight(input: &str) -> bool {
    let (result, _, _) = input
        .chars()
        .fold((false, 0, '0'), |(result, count, prev), next| {
            if !result && next as u8 == prev as u8 + 1 {
                if count >= 2 {
                    (true, 1, next)
                } else {
                    (false, count + 1, next)
                }
            } else {
                (result, 1, next)
            }
        });
    result
}

fn has_forbidden_letters(input: &str) -> bool {
    input.contains(['i', 'o', 'l'])
}

fn has_two_different_pairs(input: &str) -> bool {
    let mut pairs = HashSet::new();
    input.chars().reduce(|prev, next| {
        if prev == next {
            pairs.insert(next);
        }
        next
    });
    pairs.len() >= 2
}

#[cfg(test)]
mod test {
    use super::*;

    fn examples() -> Vec<String> {
        vec![
            "hijklmmn".to_owned(),
            "abbceffg".to_owned(),
            "abbcegjk".to_owned(),
            "szppqrra".to_owned(),
        ]
    }

    #[test]
    fn get_next_password_examples() {
        let examples = vec!["abcdefgh".to_owned(), "ghijklmn".to_owned()];
        let results = examples
            .into_iter()
            .map(|example| get_next_password(&example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec!["abcdffaa".to_owned(), "ghjaabcc".to_owned()])
    }

    #[test]
    fn increment_letter_examples() {
        let examples = vec!['a', 'z', 'u', 'r', 'e'];
        let results = examples
            .into_iter()
            .map(|example| increment_letter(example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec!['b', 'a', 'v', 's', 'f'])
    }

    #[test]
    fn increment_password_examples() {
        let examples = vec!["czzz".to_owned(), "abcxyz".to_owned(), "fwzzrp".to_owned()];
        let results = examples
            .into_iter()
            .map(|example| increment_password(&example))
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec!["daaa".to_owned(), "abcxza".to_owned(), "fwzzrq".to_owned()]
        )
    }

    #[test]
    fn meets_requirements_examples() {
        let examples = examples();
        let results = examples
            .into_iter()
            .map(|example| meets_requirements(&example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![false, false, false, true])
    }

    #[test]
    fn has_increasing_straight_examples() {
        let examples = examples();
        let results = examples
            .into_iter()
            .map(|example| has_increasing_straight(&example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![true, false, false, true])
    }

    #[test]
    fn has_forbidden_letters_examples() {
        let examples = examples();
        let results = examples
            .into_iter()
            .map(|example| has_forbidden_letters(&example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![true, false, false, false])
    }

    #[test]
    fn has_two_different_pairs_examples() {
        let examples = examples();
        let results = examples
            .into_iter()
            .map(|example| has_two_different_pairs(&example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![false, true, false, true])
    }
}
