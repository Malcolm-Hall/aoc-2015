use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input/d19.txt").unwrap();
    let (replacements_str, molecule) = input.split_once("\n\n").unwrap();
    let mut manipulator = Manipulator::from_str(replacements_str).unwrap();
    println!("Part 1: {}", manipulator.calibrate(molecule));
    println!("Part 2: {}", manipulator.construct_molecule("e", molecule));
}

type Replacements = Vec<(String, String)>;

struct Manipulator {
    replacements: Replacements,
}

impl Manipulator {
    fn new(replacements: Replacements) -> Self {
        Self { replacements }
    }

    fn from_str(replacements_str: &str) -> Option<Self> {
        let mut replacements = Vec::new();
        for line in replacements_str.lines() {
            let rule = line.split(' ').collect::<Vec<_>>();
            match rule[..] {
                [target, "=>", replacement] => {
                    replacements.push((target.to_owned(), replacement.to_owned()))
                }
                _ => return None,
            }
        }
        Some(Self::new(replacements))
    }

    fn calibrate(&self, molecule: &str) -> usize {
        let matches = self.get_distinct_replacements(molecule);
        matches.len()
    }

    fn get_distinct_replacements(&self, molecule: &str) -> HashSet<String> {
        let mut matches = HashSet::new();
        for (target, replacement) in &self.replacements {
            let target_size = target.len();
            for (idx, _) in molecule.match_indices(target) {
                let replaced_molecule =
                    molecule[..idx].to_owned() + &replacement + &molecule[idx + target_size..];
                matches.insert(replaced_molecule);
            }
        }
        matches
    }

    fn construct_molecule(&mut self, base_molecule: &str, target_molecule: &str) -> usize {
        let mut prospective_molecules = HashSet::from([base_molecule.to_owned()]);
        let mut step = 0;
        while !prospective_molecules.contains(target_molecule) {
            let mut new_prospective_molecules = HashSet::new();
            for molecule in prospective_molecules {
                let replaced_molecules = self.get_distinct_replacements(&molecule);
                new_prospective_molecules.extend(replaced_molecules.into_iter());
            }
            let largest = new_prospective_molecules
                .iter()
                .map(|m| molecule_len(m))
                .max()
                .unwrap();
            if largest > molecule_len(target_molecule) {
                panic!("Oh No!")
            }
            prospective_molecules = new_prospective_molecules
                .into_iter()
                .filter(|molecule| molecule_len(molecule) == largest)
                .collect();
            step += 1;
        }
        step
    }
}

fn molecule_len(molecule: &str) -> usize {
    molecule.chars().filter(|c| c.is_ascii_uppercase()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_replacements() -> Replacements {
        vec![
            ("e".to_owned(), "H".to_owned()),
            ("e".to_owned(), "O".to_owned()),
            ("H".to_owned(), "HO".to_owned()),
            ("H".to_owned(), "OH".to_owned()),
            ("O".to_owned(), "HH".to_owned()),
        ]
    }

    #[test]
    fn part_1_example_1() {
        let example_replacements = example_replacements();
        let base_molecule = "HOH";
        let manipulator = Manipulator::new(example_replacements);
        let result = manipulator.calibrate(base_molecule);
        assert_eq!(4, result);
    }

    #[test]
    fn part_1_example_2() {
        let example_replacements = example_replacements();
        let base_molecule = "HOHOHO";
        let manipulator = Manipulator::new(example_replacements);
        let result = manipulator.calibrate(base_molecule);
        assert_eq!(7, result);
    }

    #[test]
    fn part_2_example_1() {
        let example_replacements = example_replacements();
        let base_molecule = "e";
        let target_molecule = "HOH";
        let mut manipulator = Manipulator::new(example_replacements);
        let result = manipulator.construct_molecule(base_molecule, target_molecule);
        assert_eq!(3, result);
    }

    #[test]
    fn part_2_example_2() {
        let example_replacements = example_replacements();
        let base_molecule = "e";
        let target_molecule = "HOHOHO";
        let mut manipulator = Manipulator::new(example_replacements);
        let result = manipulator.construct_molecule(base_molecule, target_molecule);
        assert_eq!(6, result);
    }
}
