use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let input_file = File::open("input/d07.txt").unwrap();
    let gates = parse_gates(input_file);

    let mut outputs_1 = gates
        .keys()
        .map(|key| (key.to_owned(), None))
        .collect::<HashMap<_, Option<u16>>>();

    let a_output_1 = gates.get("a").unwrap().get_output(&gates, &mut outputs_1);

    let mut outputs_2 = gates
        .keys()
        .map(|key| {
            if key == "b" {
                return (key.to_owned(), Some(a_output_1));
            }
            (key.to_owned(), None)
        })
        .collect::<HashMap<_, Option<u16>>>();
    let a_output_2 = gates.get("a").unwrap().get_output(&gates, &mut outputs_2);

    println!("The signal provided to wire 'a' is {}", a_output_1);
    println!(
        "Setting the signal of wire 'b' to {} results in a signal to wire 'a' of {}",
        a_output_1, a_output_2
    );
}

fn parse_gates(input_file: File) -> HashMap<String, Gate> {
    let u16_max = u16::MAX.to_string();
    BufReader::new(input_file)
        .lines()
        .map(|result| {
            let line = result.unwrap();
            let instruction = line.split(" ").collect::<Vec<_>>();
            match instruction[..] {
                [input, "->", output] => {
                    (output.to_owned(), Gate::new(input, input, Operator::NOP))
                }
                ["NOT", input, "->", output] => {
                    (output.to_owned(), Gate::new(input, &u16_max, Operator::XOR))
                }
                [input_1, operator, input_2, "->", output] => (
                    output.to_owned(),
                    Gate::new(input_1, input_2, Operator::from_str(operator).unwrap()),
                ),
                _ => panic!("Unexpected instruction: {}", line),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Gate {
    input_1: String,
    input_2: String,
    operator: Operator,
}

#[derive(Debug)]
enum Operator {
    NOP,
    XOR,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(input: &str) -> Result<Operator, Self::Err> {
        match input {
            "NOP" => Ok(Operator::NOP),
            "XOR" => Ok(Operator::XOR),
            "AND" => Ok(Operator::AND),
            "OR" => Ok(Operator::OR),
            "LSHIFT" => Ok(Operator::LSHIFT),
            "RSHIFT" => Ok(Operator::RSHIFT),
            _ => panic!("Unexpected operator: {}", input),
        }
    }
}

impl Gate {
    fn new(input_1: &str, input_2: &str, operator: Operator) -> Self {
        Gate {
            input_1: input_1.to_string(),
            input_2: input_2.to_owned(),
            operator,
        }
    }

    fn get_input(
        input: &str,
        gates: &HashMap<String, Gate>,
        outputs: &mut HashMap<String, Option<u16>>,
    ) -> u16 {
        match input.parse::<u16>() {
            Ok(input) => input,
            Err(_) => match outputs.get(input).unwrap() {
                Some(value) => *value,
                None => {
                    let output = gates.get(input).unwrap().get_output(gates, outputs);
                    outputs.insert(input.to_owned(), Some(output));
                    output
                }
            },
        }
    }

    fn get_output(
        &self,
        gates: &HashMap<String, Gate>,
        outputs: &mut HashMap<String, Option<u16>>,
    ) -> u16 {
        use Operator::*;
        let in_1 = Gate::get_input(&self.input_1, gates, outputs);
        let in_2 = Gate::get_input(&self.input_2, gates, outputs);
        match self.operator {
            NOP => in_1,
            XOR => in_1 ^ in_2,
            AND => in_1 & in_2,
            OR => in_1 | in_2,
            LSHIFT => in_1 << in_2,
            RSHIFT => in_1 >> in_2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example() -> HashMap<String, Gate> {
        use Operator::*;
        let u16_max = u16::MAX.to_string();
        HashMap::from([
            ("x".to_owned(), Gate::new("123", "123", NOP)),
            ("y".to_owned(), Gate::new("456", "456", NOP)),
            ("d".to_owned(), Gate::new("x", "y", AND)),
            ("e".to_owned(), Gate::new("x", "y", OR)),
            ("f".to_owned(), Gate::new("x", "2", LSHIFT)),
            ("g".to_owned(), Gate::new("y", "2", RSHIFT)),
            ("h".to_owned(), Gate::new("x", &u16_max, XOR)),
            ("i".to_owned(), Gate::new("y", &u16_max, XOR)),
        ])
    }

    #[test]
    fn test_examples() {
        let example_gates = example();
        let mut outputs = example_gates
            .keys()
            .map(|key| (key.to_owned(), None))
            .collect::<HashMap<_, Option<u16>>>();

        let results = example_gates
            .iter()
            .map(|(output, gate)| {
                let gate_output = gate.get_output(&example_gates, &mut outputs);
                (output.to_owned(), Some(gate_output))
            })
            .collect::<HashMap<_, _>>();

        assert_eq!(
            results,
            HashMap::from([
                ("d".to_owned(), Some(72)),
                ("e".to_owned(), Some(507)),
                ("f".to_owned(), Some(492)),
                ("g".to_owned(), Some(114)),
                ("h".to_owned(), Some(65412)),
                ("i".to_owned(), Some(65079)),
                ("x".to_owned(), Some(123)),
                ("y".to_owned(), Some(456)),
            ])
        )
    }
}
