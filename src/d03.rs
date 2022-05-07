use std::{collections::HashSet, fs};

pub fn solutions() {
    let input = fs::read_to_string("input/d03.txt").unwrap();
    let houses_visited_part_1 = part_1(&input);
    let houses_visited_part_2 = part_2(&input);
    println!("{houses_visited_part_1} houses receive at least one present");
    println!("{houses_visited_part_2} houses receive at least one present using Robo-Santa")
}

fn part_1(input: &str) -> usize {
    let mut visited_houses = HashSet::new();
    let instructions = input.chars();
    visit_houses(instructions, &mut visited_houses);
    visited_houses.len()
}

fn part_2(input: &str) -> usize {
    let mut visited_houses = HashSet::new();
    let real = input.chars().step_by(2);
    let robo = input.chars().skip(1).step_by(2);
    visit_houses(real, &mut visited_houses);
    visit_houses(robo, &mut visited_houses);
    visited_houses.len()
}

fn visit_houses<I>(instructions: I, visited_houses: &mut HashSet<(i32, i32)>)
where
    I: Iterator<Item = char>,
{
    let origin = (0, 0);
    visited_houses.insert(origin);
    instructions.fold(origin, |pos, direction| {
        let new_pos = match direction {
            '>' => (pos.0 + 1, pos.1),
            '<' => (pos.0 - 1, pos.1),
            '^' => (pos.0, pos.1 + 1),
            'v' => (pos.0, pos.1 - 1),
            _ => panic!("Unexpected direction {direction}"),
        };
        visited_houses.insert(new_pos);
        new_pos
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn examples() -> Vec<String> {
        vec!["^v".to_owned(), "^>v<".to_owned(), "^v^v^v^v^v".to_owned()]
    }

    #[test]
    fn visit_houses_examples() {
        let examples = examples();
        let results = examples
            .iter()
            .map(|example| {
                let mut visited_houses = HashSet::new();
                let instructions = example.chars();
                visit_houses(instructions, &mut visited_houses);
                visited_houses.len()
            })
            .collect::<Vec<_>>();
        assert_eq!(results, vec![2, 4, 2]);
    }

    #[test]
    fn part_1_examples() {
        let examples = examples();
        let results = examples
            .iter()
            .map(|example| part_1(example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![2, 4, 2]);
    }

    #[test]
    fn part_2_examples() {
        let examples = examples();
        let results = examples
            .iter()
            .map(|example| part_2(example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![3, 3, 11]);
    }
}
