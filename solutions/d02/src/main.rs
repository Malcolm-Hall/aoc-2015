use std::fs;

fn main() {
    let all_boxes = parse_input("input/d02.txt").unwrap();
    let total_wrapping_paper = part_1(&all_boxes);
    let total_ribbon = part_2(&all_boxes);
    println!("The elves should order {total_wrapping_paper} square feet of wrapping paper");
    println!("The elves should order {total_ribbon} feet of ribbon");
}

#[derive(PartialEq, Debug)]
struct OrderedSides {
    min: u32,
    mid: u32,
    max: u32,
}

impl OrderedSides {
    fn new(min: u32, mid: u32, max: u32) -> Self {
        Self { min, mid, max }
    }

    fn from_unordered(sides: &mut [u32]) -> Option<Self> {
        sides.sort_unstable();
        match &sides[..] {
            [min, mid, max] => Some(Self::new(*min, *mid, *max)),
            _ => None,
        }
    }

    fn wrapping_paper_area(&self) -> u32 {
        let Self { min, mid, max } = *self;
        3 * (min * mid) + 2 * (mid * max + max * min)
    }

    fn ribbon_length(&self) -> u32 {
        let Self { min, mid, max } = *self;
        2 * (min + mid) + min * mid * max
    }

    fn from_str(dimensions: &str) -> Option<Self> {
        let mut sides = dimensions
            .split('x')
            .map(|s| s.parse().ok())
            .collect::<Option<Vec<_>>>()?;

        Self::from_unordered(&mut sides)
    }
}

fn parse_input(path: &str) -> Option<Vec<OrderedSides>> {
    let input = fs::read_to_string(path).ok()?;
    let dimensions = input.lines();
    dimensions
        .map(|line| OrderedSides::from_str(&line))
        .collect()
}

fn part_1(all_boxes: &[OrderedSides]) -> u32 {
    all_boxes
        .iter()
        .map(|ordered_sides| ordered_sides.wrapping_paper_area())
        .sum()
}

fn part_2(all_boxes: &[OrderedSides]) -> u32 {
    all_boxes
        .iter()
        .map(|ordered_sides| ordered_sides.ribbon_length())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn examples() -> Vec<OrderedSides> {
        vec![OrderedSides::new(2, 3, 4), OrderedSides::new(1, 1, 10)]
    }

    #[test]
    fn parse_box_dimensions_examples() {
        let examples = examples();
        assert_eq!(OrderedSides::from_str("2x3x4").unwrap(), examples[0]);
        assert_eq!(OrderedSides::from_str("1x1x10").unwrap(), examples[1]);
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
        assert_eq!(examples[0].wrapping_paper_area(), 58);
        assert_eq!(examples[1].wrapping_paper_area(), 43);
    }

    #[test]
    fn get_ribbon_length_examples() {
        let examples = examples();
        assert_eq!(examples[0].ribbon_length(), 34);
        assert_eq!(examples[1].ribbon_length(), 14);
    }
}
