use std::fs;

pub fn d02_solutions() {
    let all_boxes = parse_input("input/d02.txt");
    let total_wrapping_paper = part_1(&all_boxes);
    let total_ribbon = part_2(&all_boxes);
    println!("The elves should order {total_wrapping_paper} square feet of wrapping paper");
    println!("The elves should order {total_ribbon} feet of ribbon");
}

type Area = u32;
type Length = u32;

#[derive(PartialEq, Debug)]
struct OrderedSides {
    min: u32,
    mid: u32,
    max: u32,
}

fn parse_input(path: &str) -> Vec<OrderedSides> {
    let input = fs::read_to_string(path).unwrap();
    let dimensions = input.lines().map(|line| line.to_owned());
    dimensions.map(|line| parse_box_dimensions(&line)).collect()
}

fn parse_box_dimensions(dimensions: &str) -> OrderedSides {
    let mut sides = dimensions
        .split('x')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    sides.sort_unstable();

    assert!(sides.len() == 3, "Expected 3 dimensions");

    OrderedSides {
        min: sides[0],
        mid: sides[1],
        max: sides[2],
    }
}

fn part_1(all_boxes: &Vec<OrderedSides>) -> Area {
    all_boxes
        .iter()
        .map(|ordered_sides| get_wrapping_paper_area(&ordered_sides))
        .sum::<Area>()
}

fn part_2(all_boxes: &Vec<OrderedSides>) -> Length {
    all_boxes
        .iter()
        .map(|ordered_sides| get_ribbon_length(&ordered_sides))
        .sum::<Length>()
}

fn get_wrapping_paper_area(ordered_sides: &OrderedSides) -> Area {
    let OrderedSides { min, mid, max } = ordered_sides;
    3 * (min * mid) + 2 * (mid * max + max * min)
}

fn get_ribbon_length(ordered_sides: &OrderedSides) -> Length {
    let OrderedSides { min, mid, max } = ordered_sides;
    2 * (min + mid) + min * mid * max
}

#[cfg(test)]
mod tests {
    use super::*;

    fn examples() -> Vec<OrderedSides> {
        vec![
            OrderedSides {
                min: 2,
                mid: 3,
                max: 4,
            },
            OrderedSides {
                min: 1,
                mid: 1,
                max: 10,
            },
        ]
    }

    #[test]
    fn parse_box_dimensions_examples() {
        let examples = examples();
        assert_eq!(parse_box_dimensions("2x3x4"), examples[0]);
        assert_eq!(parse_box_dimensions("1x1x10"), examples[1]);
    }

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&examples()), 58 + 43);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&examples()), 34 + 14);
    }

    #[test]
    fn get_wrapping_paper_area_examples() {
        let examples = examples();
        assert_eq!(get_wrapping_paper_area(&examples[0]), 58);
        assert_eq!(get_wrapping_paper_area(&examples[1]), 43);
    }

    #[test]
    fn get_ribbon_length_examples() {
        let examples = examples();
        assert_eq!(get_ribbon_length(&examples[0]), 34);
        assert_eq!(get_ribbon_length(&examples[1]), 14);
    }
}
