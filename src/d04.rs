use std::fs;
use md5;

pub fn solutions() {
    let input = fs::read_to_string("input/d04.txt").unwrap();
    let result_1 = part_1(&input);
    let result_2 = part_2(&input);
    println!("{result_1} produces a hash that starts with 5 zeros");
    println!("{result_2} produces a hash that starts with 6 zeros");
}

fn part_1(input: &str) -> usize {
    mine_advent_coins(input, 5)
}

fn part_2(input: &str) -> usize {
    mine_advent_coins(input, 6)
}

fn mine_advent_coins(secret_key: &str, n: usize) -> usize {
    let mut suffix = 1;
    loop {
        let hash = calculate_hash(secret_key, suffix);
        if verify_hash(&hash, n) {
            break;
        }
        suffix += 1
    }
    suffix
}

fn calculate_hash(secret_key: &str, suffix: usize) -> String {
    let digest = md5::compute(format!("{secret_key}{suffix}"));
    let hash = format!("{:?}", digest);
    hash
}

fn verify_hash(hash: &str, n: usize) -> bool {
    let verifier = "0".repeat(n);
    hash.starts_with(&verifier)
}

#[cfg(test)]
mod test {
    use super::*;

    fn examples() -> Vec<String> {
        vec!["abcdef".to_owned(), "pqrstuv".to_owned()]
    }

    #[test]
    fn mine_advent_coins_examples() {
        let examples = examples();
        let results = examples.iter().map(|example| mine_advent_coins(example, 5)).collect::<Vec<_>>();
        assert_eq!(results, vec![609043, 1048970])
    }
    
    #[test]
    fn calculate_hash_examples() {
        let examples = examples();
        let results = examples.iter().map(|example| calculate_hash(example, 1)).collect::<Vec<_>>();
        assert_eq!(results, vec!["5f8b62a2dced0cd28946a9c891ff3e5e", "be2406f4b525648848bb8c9efa215717"]);
    }
    
        #[test]
        fn verify_hash_examples() {
            assert!(verify_hash("0123", 1));
            assert!(!verify_hash("0123", 2));
            assert!(verify_hash("00000123", 5));
            assert!(!verify_hash("00000123", 6));
            assert!(verify_hash("000000123", 6));
            assert!(!verify_hash("000000123", 7));
        }
}