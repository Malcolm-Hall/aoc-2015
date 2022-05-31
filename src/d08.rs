use std::fs;

pub fn solutions() {
    let input = fs::read_to_string("input/d08.txt").unwrap();
    let lines = input.lines().map(|line| line.to_owned()).collect();
    let part_1 = part_1(&lines);
    let part_2 = part_2(&lines);
    println!("The difference between total number of characters of code and characters in memory is {}", part_1);
    println!("The difference between total number of characters in the new encoding and characters of code is {}", part_2)
}

fn part_1(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| count_code(&line) - count_memory(&line))
        .reduce(|acc, item| acc + item)
        .unwrap_or(0)
}

fn part_2(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| count_new_encoding(line) - count_code(line))
        .reduce(|acc, item| acc + item)
        .unwrap_or(0)
}

fn count_code(input: &str) -> usize {
    input.len()
}

fn count_memory(input: &str) -> usize {
    let mut chars = input.chars();
    chars.next();
    chars.next_back();
    let (result, _) = chars.fold(
        (0, String::with_capacity(3)),
        |(acc, mut buffer), next| {
            match buffer.len() {
                0 => match next {
                    '\\' => {
                        buffer.push(next);
                        (acc, buffer)
                    },
                    _ => (acc + 1, buffer),
                },
                1 => match next {
                    'x' => {
                        buffer.push(next);
                        (acc, buffer)
                    },
                    '"' | '\\' => {
                        buffer.clear();
                        (acc + 1, buffer)
                    },
                    _ => panic!("Unexpected escaped character {}{}", buffer, next)
                },
                2 => {
                    buffer.push(next);
                    (acc, buffer)
                },
                3 => {
                    buffer.clear();
                    (acc + 1, buffer)
                },
                _ => panic!("Unexpected buffer {}", buffer),
            }
        },
    );
    result
}

fn count_new_encoding(input: &str) -> usize {
    let additional_quotes = 2;
    let encoded_len = input.chars().fold(0, |acc, next| match next {
        '"' | '\\' => acc + 2,
        _ => acc + 1,
    });
    encoded_len + additional_quotes
}

#[cfg(test)]
mod test {
    use super::*;

    fn examples() -> Vec<String> {
        vec![
            r#""""#.to_owned(),
            r#""abc""#.to_owned(),
            r#""aaa\"aaa""#.to_owned(),
            r#""\x27""#.to_owned(),
        ]
    }

    #[test]
    fn count_code_examples() {
        let examples = examples();
        let results = examples
            .into_iter()
            .map(|example| count_code(&example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![2, 5, 10, 6])
    }

    #[test]
    fn count_memory_examples() {
        let examples = examples();
        let results = examples
            .into_iter()
            .map(|example| count_memory(&example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![0, 3, 7, 1])
    }

    #[test]
    fn count_new_encoding_examples() {
        let examples = examples();
        let results = examples
            .into_iter()
            .map(|example| count_new_encoding(&example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![6, 9, 16, 11])
    }
}
