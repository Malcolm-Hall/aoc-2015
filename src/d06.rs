use std::fs;

pub fn solutions() {
    println!("hello, d06!");
    let instructions = parse_input();
    let total_lit = part_1(&instructions);
    println!("There are {total_lit} lit lights after following the instructions");
}

fn parse_input() -> Vec<Instruction> {
    let input = fs::read_to_string("input/d06.txt").unwrap();
    input
        .lines()
        .map(|line| {
            let instruction = line.split(&[' ', ',']).collect::<Vec<_>>();
            let (action, area) = match instruction.as_slice() {
                ["turn", "on", rest @ ..] => (Action::ON, rest),
                ["turn", "off", rest @ ..] => (Action::OFF, rest),
                ["toggle", rest @ ..] => (Action::TOGGLE, rest),
                _ => panic!("Unexpected instruction: {line}"),
            };
            let rectangle = match area[..] {
                [x1, y1, "through", x2, y2] => Rectangle::from_str(x1, y1, x2, y2).unwrap(),
                _ => panic!("Unexpected area: {line}"),
            };
            Instruction::new(action, rectangle)
        })
        .collect()
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    rectangle: Rectangle,
}

#[derive(Debug, PartialEq)]
enum Action {
    ON,
    OFF,
    TOGGLE,
}

#[derive(Debug)]
struct Rectangle {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Instruction {
    fn new(action: Action, rectangle: Rectangle) -> Instruction {
        Instruction { action, rectangle }
    }

    fn affects(&self, x: i32, y: i32) -> bool {
        self.rectangle.contains(x, y)
    }
}

impl Rectangle {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Rectangle {
        Rectangle { x1, y1, x2, y2 }
    }

    fn from_str(x1: &str, y1: &str, x2: &str, y2: &str) -> Result<Rectangle, String> {
        match (x1.parse(), y1.parse(), x2.parse(), y2.parse()) {
            (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) => Ok(Rectangle::new(x1, y1, x2, y2)),
            _ => Err(format!("Could not parse rectangle: x1: {x1}, y1: {y1}, x2: {x2}, y2: {y2}")),
        }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2
    }
}

fn part_1(instructions: &Vec<Instruction>) -> usize {
    let mut total = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            let index = find_last_non_toggle_instuction(x, y, instructions);
            if light_is_on(x, y, &instructions[index..]) {
                total += 1;
            }
        }
    }
    total
}

fn find_last_non_toggle_instuction(x: i32, y: i32, instructions: &Vec<Instruction>) -> usize {
    instructions
        .iter()
        .rposition(|instruction| {
            if instruction.action == Action::TOGGLE || !instruction.affects(x, y) {
                return false;
            }
            true
        })
        .unwrap_or(0)
}

fn light_is_on(x: i32, y: i32, instructions: &[Instruction]) -> bool {
    instructions.iter().fold(false, |state, instruction| {
        if !instruction.affects(x, y) {
            return state;
        }
        match instruction.action {
            Action::ON => true,
            Action::OFF => false,
            Action::TOGGLE => !state,
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn examples() -> Vec<Instruction> {
        vec![
            Instruction::new(Action::ON, Rectangle::new(0, 0, 9, 9)),
            Instruction::new(Action::TOGGLE, Rectangle::new(0, 0, 9, 0)),
            Instruction::new(Action::OFF, Rectangle::new(4, 4, 5, 5)),
            Instruction::new(Action::ON, Rectangle::new(4, 4, 5, 5)),
        ]
    }

    #[test]
    fn light_is_on_examples() {
        let examples = examples();
        assert_eq!(light_is_on(0, 0, &examples), false);
        assert_eq!(light_is_on(0, 1, &examples), true);
        assert_eq!(light_is_on(5, 5, &examples), true);
    }

    #[test]
    fn find_last_non_toggle_instuction_examples() {
        let examples = examples();
        assert_eq!(find_last_non_toggle_instuction(0, 0, &examples), 0);
        assert_eq!(find_last_non_toggle_instuction(5, 5, &examples), 3);
    }
}
