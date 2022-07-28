use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input/d20.txt")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let p1 = part_1(input);
    let p2 = part_2(input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part_1(input: u32) -> u32 {
    lowest_house_number(input, get_presents_delivered_1)
}

fn part_2(input: u32) -> u32 {
    lowest_house_number(input, get_presents_delivered_2)
}

fn lowest_house_number(input: u32, presents_delivered_fn: fn(u32) -> u32) -> u32 {
    let mut i = 0;
    loop {
        let presents_delivered = presents_delivered_fn(i);
        if presents_delivered > input {
            return i;
        }
        i += 1;
    }
}

fn get_presents_delivered_1(number: u32) -> u32 {
    let divisors = get_divisors(number);
    10 * divisors.into_iter().sum::<u32>()
}

fn get_presents_delivered_2(number: u32) -> u32 {
    let divisors = get_divisors(number);
    let actual_visited = divisors
        .into_iter()
        .filter(|i| i * 50 >= number)
        .sum::<u32>();
    11 * actual_visited
}

fn get_divisors(number: u32) -> HashSet<u32> {
    let square_root = ((number as f64).sqrt()) as u32;
    let mut divisors = HashSet::new();
    for i in 1..=square_root {
        if number % i == 0 {
            divisors.insert(i);
            divisors.insert(number / i);
        }
    }
    divisors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn divisors() {
        let results = (1..=9).map(|i| get_divisors(i)).collect::<Vec<_>>();
        assert_eq!(
            results,
            [
                HashSet::from([1]),
                HashSet::from([1, 2]),
                HashSet::from([1, 3]),
                HashSet::from([1, 2, 4]),
                HashSet::from([1, 5]),
                HashSet::from([1, 2, 3, 6]),
                HashSet::from([1, 7]),
                HashSet::from([1, 2, 4, 8]),
                HashSet::from([1, 3, 9]),
            ]
        )
    }

    #[test]
    fn part_1_presents_delivered() {
        let results = (1..=9)
            .map(|i| get_presents_delivered_1(i))
            .collect::<Vec<_>>();
        assert_eq!(results, [10, 30, 40, 70, 60, 120, 80, 150, 130])
    }

    #[test]
    fn part_2_presents_delivered() {
        let results = (1..=9)
            .chain([53])
            .map(|i| get_presents_delivered_2(i))
            .collect::<Vec<_>>();
        assert_eq!(results, [11, 33, 44, 77, 66, 132, 88, 165, 143, 53 * 11])
    }
}
