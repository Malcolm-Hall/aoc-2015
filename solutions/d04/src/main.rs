use md5;
use std::fs;

fn main() {
    let secret_key = fs::read_to_string("input/d04.txt").unwrap();
    let miner = Miner::new(secret_key);
    let result_1 = part_1(&miner);
    let result_2 = part_2(&miner);
    println!("{result_1} produces a hash that starts with 5 zeros");
    println!("{result_2} produces a hash that starts with 6 zeros");
}

fn part_1(miner: &Miner) -> u32 {
    miner.mine(5)
}

fn part_2(miner: &Miner) -> u32 {
    miner.mine(6)
}

struct Miner {
    secret_key: String,
}

impl Miner {
    fn new(secret_key: String) -> Self {
        Miner { secret_key }
    }

    fn mine(&self, n: usize) -> u32 {
        let verifier = "0".repeat(n);
        let mut suffix = 1;
        loop {
            let hash = self.get_hash(suffix);
            if hash.starts_with(&verifier) {
                break;
            }
            suffix += 1
        }
        suffix
    }

    fn get_hash(&self, suffix: u32) -> String {
        let digest = md5::compute(format!("{}{}", self.secret_key, suffix));
        let hash = format!("{:?}", digest);
        hash
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn examples() -> Vec<String> {
        vec!["abcdef".to_owned(), "pqrstuv".to_owned()]
    }

    #[test]
    fn mine_examples() {
        let examples = examples();
        let results = examples
            .iter()
            .map(|example| Miner::new(example.into()))
            .map(|m| m.mine(5))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![609043, 1048970])
    }

    #[test]
    fn get_hash_examples() {
        let examples = examples();
        let results = examples
            .iter()
            .map(|example| Miner::new(example.into()))
            .map(|m| m.get_hash(1))
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec![
                "5f8b62a2dced0cd28946a9c891ff3e5e",
                "be2406f4b525648848bb8c9efa215717"
            ]
        );
    }
}
