use std::fs;

pub fn solutions() {
    let input = fs::read_to_string("input/d01.txt").unwrap();
    let floor = part_1(&input);
    let position = part_2(&input);
    println!("Santa must go to floor {floor}");
    println!("Santa will first go to the basement at position {position}");
}

fn part_1(input: &str) -> i32 {
    input.chars().fold(0, |floor, c| match c {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => panic!("Unexpected character: {}", c),
    })
}

fn part_2(input: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Unexpected character: {}", c),
        }
        if floor == -1 {
            return i + 1;
        }
    }
    panic!("Santa never got to the basement");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1("()()"), 0);
        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(()(()("), 3);
        assert_eq!(part_1("))((((("), 3);
        assert_eq!(part_1("())"), -1);
        assert_eq!(part_1("))("), -1);
        assert_eq!(part_1(")))"), -3);
        assert_eq!(part_1(")())())"), -3);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(")"), 1);
        assert_eq!(part_2("()())"), 5);
    }
}
