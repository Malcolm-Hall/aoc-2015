use std::fs;

fn main() {
    let input = fs::read_to_string("input/d17.txt").unwrap();
    let containers = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut combinator = Combinator::new(150);
    combinator.combinations(&containers);
    println!("Part 1: {}", combinator.total_combinations);
    println!("Part 2: {}", combinator.min_containers_used.unwrap().1);
}

struct Combinator {
    target: u32,
    total_combinations: u32,
    min_containers_used: Option<(u32, u32)>,
}

impl Combinator {
    fn new(target: u32) -> Self {
        Self {
            target,
            total_combinations: 0,
            min_containers_used: None,
        }
    }

    fn combinations(&mut self, containers: &[u32]) {
        self.total_combinations = self.combinations_impl(0, containers, 0);
    }

    fn combinations_impl(&mut self, acc: u32, remaining: &[u32], containers_used: u32) -> u32 {
        if acc == self.target {
            self.update_min_containers_used(containers_used);
            return 1;
        } else if remaining.len() == 0 {
            return 0;
        }
        let mut total = 0;
        for (idx, next) in remaining.iter().enumerate() {
            let new_acc = acc + next;
            let new_remaining = &remaining[idx + 1..];
            let new_containers_used = containers_used + 1;
            total += self.combinations_impl(new_acc, &new_remaining, new_containers_used);
        }
        total
    }

    fn update_min_containers_used(&mut self, containers_used: u32) {
        match self.min_containers_used {
            Some((min, acc)) => {
                if containers_used == min {
                    self.min_containers_used = Some((min, acc + 1));
                } else if containers_used < min {
                    self.set_min_containers_used(containers_used);
                }
            }
            None => {
                self.set_min_containers_used(containers_used);
            }
        }
    }

    fn set_min_containers_used(&mut self, containers_used: u32) {
        self.min_containers_used = Some((containers_used, 1));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_containers() -> Vec<u32> {
        vec![20, 15, 10, 5, 5]
    }

    #[test]
    fn part_1_example() {
        let example_containers = example_containers();
        let mut combinator = Combinator::new(25);
        combinator.combinations(&example_containers);
        assert_eq!(4, combinator.total_combinations);
    }

    #[test]
    fn part_2_example() {
        let example_containers = example_containers();
        let mut combinator = Combinator::new(25);
        combinator.combinations(&example_containers);
        assert_eq!((2, 3), combinator.min_containers_used.unwrap());
    }
}
