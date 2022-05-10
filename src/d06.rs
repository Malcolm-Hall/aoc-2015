use std::{
    fs,
    io::{self, BufRead},
};

pub fn solutions() {
    let instructions = parse_input();
    let total_lit = part_1(1000, &instructions);
    let total_brightness = part_2(1000, &instructions);
    println!("There are {total_lit} lit lights after following the instructions");
    println!(
        "The lights have a total brightness of {total_brightness} after following the instructions",
    )
}

fn parse_input() -> Vec<Instruction> {
    let file = fs::File::open("input/d06.txt").unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|result| {
            let line = result.unwrap();
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
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Instruction {
    fn new(action: Action, rectangle: Rectangle) -> Instruction {
        Instruction { action, rectangle }
    }

    fn affects(&self, x: usize, y: usize) -> bool {
        self.rectangle.contains(x, y)
    }
}

impl Rectangle {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Rectangle {
        Rectangle { x1, y1, x2, y2 }
    }

    fn from_str(x1: &str, y1: &str, x2: &str, y2: &str) -> Result<Rectangle, String> {
        match (x1.parse(), y1.parse(), x2.parse(), y2.parse()) {
            (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) => Ok(Rectangle::new(x1, y1, x2, y2)),
            _ => Err(format!(
                "Could not parse rectangle: x1: {x1}, y1: {y1}, x2: {x2}, y2: {y2}"
            )),
        }
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2
    }
}

fn part_1(grid_size: usize, instructions: &[Instruction]) -> usize {
    let mut total = 0;
    for x in 0..grid_size {
        for y in 0..grid_size {
            let index = find_last_non_toggle_instuction(x, y, instructions);
            if light_is_on(x, y, &instructions[index..]) {
                total += 1;
            }
        }
    }
    total
}

fn part_2(grid_size: usize, instructions: &[Instruction]) -> usize {
    let mut total = 0;
    for x in 0..grid_size {
        for y in 0..grid_size {
            total += calculate_brightness(x, y, instructions)
        }
    }
    total
}

fn calculate_brightness(x: usize, y: usize, instructions: &[Instruction]) -> usize {
    instructions.iter().fold(0, |brightness, instruction| {
        if !instruction.affects(x, y) {
            return brightness;
        }
        match instruction.action {
            Action::ON => brightness + 1,
            Action::TOGGLE => brightness + 2,
            Action::OFF => match brightness {
                0 => 0,
                b => b - 1,
            },
        }
    })
}

fn find_last_non_toggle_instuction(x: usize, y: usize, instructions: &[Instruction]) -> usize {
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

fn light_is_on(x: usize, y: usize, instructions: &[Instruction]) -> bool {
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
    fn part_1_examples() {
        let examples = examples();
        assert_eq!(part_1(10, &examples), 90);
    }

    #[test]
    fn part_2_examples() {
        let examples = examples();
        assert_eq!(part_2(10, &examples), 120);
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
        assert_eq!(find_last_non_toggle_instuction(0, 1, &examples), 0);
        assert_eq!(find_last_non_toggle_instuction(5, 5, &examples), 3);
    }

    #[test]
    fn calculate_brightness_examples() {
        let examples = examples();
        assert_eq!(calculate_brightness(0, 0, &examples), 3);
        assert_eq!(calculate_brightness(0, 1, &examples), 1);
        assert_eq!(calculate_brightness(5, 5, &examples), 1);
    }
}
