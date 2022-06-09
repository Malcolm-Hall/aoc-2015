use std::fs;

pub fn solutions() {
    let input = fs::read_to_string("input/d10.txt").unwrap();
    let mut part_1_result = input;
    for _ in 0..40 {
        part_1_result = process(&part_1_result);
    }
    println!("The length after 40 iterations is {}", part_1_result.len());
    let mut part_2_result = part_1_result;
    for _ in 0..10 {
        part_2_result = process(&part_2_result);
    }
    println!("The length after 50 iterations is {}", part_2_result.len());
}

fn process(input: &str) -> String {
    let (_, output) = input.chars().chain(['0']).fold(
        (None, "".to_owned()),
        |(acc, mut output): (Option<(char, usize)>, String), next| match acc {
            None => (Some((next, 1)), output),
            Some((prev, count)) => {
                if next == prev {
                    (Some((next, count + 1)), output)
                } else {
                    count.to_string().chars().for_each(|c| output.push(c));
                    output.push(prev);
                    (Some((next, 1)), output)
                }
            }
        },
    );
    output
}

#[cfg(test)]
mod test {
    use super::process;

    fn examples() -> Vec<String> {
        vec![
            "1".to_string(),
            "11".to_owned(),
            "21".to_owned(),
            "1211".to_owned(),
            "111221".to_owned(),
            "312211".to_owned(),
        ]
    }

    #[test]
    fn process_examples() {
        let examples = examples();
        for i in 0..examples.len() - 1 {
            let result = process(&examples[i]);
            assert_eq!(result, examples[i + 1])
        }
    }
}
