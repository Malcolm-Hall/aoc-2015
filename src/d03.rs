use std::{collections::HashSet, fs};

pub fn solutions() {
    let input = fs::read_to_string("input/d03.txt").unwrap();
    let houses_visited_part_1 = part_1(&input).unwrap();
    let houses_visited_part_2 = part_2(&input).unwrap();
    println!("{houses_visited_part_1} houses receive at least one present");
    println!("{houses_visited_part_2} houses receive at least one present using Robo-Santa")
}

fn part_1(input: &str) -> Option<usize> {
    let instructions = get_instructions(input.chars())?;
    let mut visiter = Visiter::new();
    let result = visiter.follow_instructions(&instructions).unique_visits();
    Some(result)
}

fn part_2(input: &str) -> Option<usize> {
    let real = get_instructions(input.chars().step_by(2))?;
    let robo = get_instructions(input.chars().skip(1).step_by(2))?;
    let mut visiter = Visiter::new();
    let result = visiter
        .follow_instructions(&real)
        .follow_instructions(&robo)
        .unique_visits();
    Some(result)
}

fn get_instructions<T>(input: T) -> Option<Vec<Direction>>
where
    T: Iterator<Item = char>,
{
    input.map(|c| Direction::from_char(c)).collect()
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::UP),
            'v' => Some(Direction::DOWN),
            '<' => Some(Direction::LEFT),
            '>' => Some(Direction::RIGHT),
            _ => None,
        }
    }
}

struct Visiter {
    visited_houses: HashSet<(i32, i32)>,
}

impl Visiter {
    fn new() -> Self {
        Self {
            visited_houses: HashSet::new(),
        }
    }

    fn follow_instructions(&mut self, instructions: &[Direction]) -> &mut Self {
        let mut pos = (0, 0);
        self.visited_houses.insert(pos);
        for direction in instructions {
            pos = match direction {
                Direction::UP => (pos.0, pos.1 + 1),
                Direction::DOWN => (pos.0, pos.1 - 1),
                Direction::LEFT => (pos.0 - 1, pos.1),
                Direction::RIGHT => (pos.0 + 1, pos.1),
            };
            self.visited_houses.insert(pos);
        }
        self
    }

    fn unique_visits(&self) -> usize {
        self.visited_houses.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn examples() -> Vec<String> {
        vec!["^v".to_owned(), "^>v<".to_owned(), "^v^v^v^v^v".to_owned()]
    }

    #[test]
    fn follow_instructions_examples() {
        let examples = examples();
        let results = examples
            .iter()
            .map(|example| {
                let instructions = get_instructions(example.chars()).unwrap();
                let mut visiter = Visiter::new();
                visiter.follow_instructions(&instructions).unique_visits()
            })
            .collect::<Vec<_>>();
        assert_eq!(results, vec![2, 4, 2]);
    }

    #[test]
    fn part_1_examples() {
        let examples = examples();
        let results = examples
            .iter()
            .map(|example| part_1(example).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(results, vec![2, 4, 2]);
    }

    #[test]
    fn part_2_examples() {
        let examples = examples();
        let results = examples
            .iter()
            .map(|example| part_2(example).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(results, vec![3, 3, 11]);
    }
}
