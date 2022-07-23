use std::fs;

use json::JsonValue;

fn main() {
    println!("hello d12!");
    let input = fs::read_to_string("input/d12.txt").unwrap();
    let part_1 = sum_all_numbers(&input);
    let part_2 = sum_non_red_numbers(&input);
    println!("The sum of all numbers is {part_1}");
    println!("The sum of all non red numbers is {part_2}");
}

fn sum_all_numbers(input: &str) -> i32 {
    let mut sum = 0;
    input.chars().fold("".to_string(), |mut buffer, next| {
        if next == '-' || next.is_numeric() {
            buffer.push(next);
        } else if buffer.len() > 0 {
            let number = buffer.parse::<i32>().unwrap();
            sum += number;
            buffer.clear();
        }
        buffer
    });
    sum
}

fn sum_non_red_numbers(input: &str) -> i32 {
    let parsed_input = json::parse(input).unwrap();
    sum_non_red_numbers_impl(&parsed_input)
}

fn sum_non_red_numbers_impl(json: &JsonValue) -> i32 {
    if json.is_array() {
        json.members()
            .map(|entry| sum_non_red_numbers_impl(entry))
            .sum()
    } else if json.is_object() && json.entries().all(|(_, value)| value != "red") {
        json.entries()
            .map(|(_, value)| sum_non_red_numbers_impl(value))
            .sum()
    } else if json.is_number() {
        json.as_i32().unwrap()
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn examples() -> Vec<String> {
        vec![
            r#"[1,2,3]"#.to_owned(),
            r#"{"a":2,"b":4}"#.to_owned(),
            r#"[[[3]]]"#.to_owned(),
            r#"{"a":{"b":4},"c":-1}"#.to_owned(),
            r#"{"a":[-1,1]}"#.to_owned(),
            r#"[-1,{"a":1}]"#.to_owned(),
            r#"[]"#.to_owned(),
            r#"{}"#.to_owned(),
            r#"[1,{"c":"red","b":2},3]"#.to_owned(),
            r#"{"d":"red","e":[1,2,3,4],"f":5}"#.to_owned(),
            r#"[1,"red",5]"#.to_owned(),
        ]
    }

    #[test]
    fn sum_all_numbers_examples() {
        let examples = examples();
        let results = examples
            .into_iter()
            .map(|example| sum_all_numbers(&example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![6, 6, 3, 3, 0, 0, 0, 0, 6, 15, 6])
    }

    #[test]
    fn sum_non_red_numbers_examples() {
        let examples = examples();
        let results = examples
            .into_iter()
            .map(|example| sum_non_red_numbers(&example))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![6, 6, 3, 3, 0, 0, 0, 0, 4, 0, 6])
    }
}
