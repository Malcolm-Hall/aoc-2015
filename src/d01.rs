use std::fs;

pub fn solutions() {
    let input = fs::read_to_string("input/d01.txt").unwrap();
    let floor = part_1(&input).unwrap();
    let position = part_2(&input).unwrap();
    println!("Santa must go to floor {floor}");
    println!("Santa will first go to the basement at position {position}");
}

fn part_1(input: &str) -> Option<i32> {
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => return None,
        }
    }
    Some(floor)
}

fn part_2(input: &str) -> Option<usize> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => break,
        }
        if floor == -1 {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_examples() {
        let examples = [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];
        for (input, expected) in examples {
            assert_eq!(part_1(input).unwrap(), expected)
        }
    }

    #[test]
    fn part_2_examples() {
        let examples = [(")", 1), ("()())", 5)];
        for (input, expected) in examples {
            assert_eq!(part_2(input).unwrap(), expected)
        }
    }
}
