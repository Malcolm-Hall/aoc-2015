use std::fs;

pub fn d02_solutions() {
    let input = fs::read_to_string("input/d02.txt").unwrap();
    let lines = input.lines().map(|line| line.to_owned());
    let box_dimensions: Vec<BoxDimensions> = lines.map(|line| parse_box_dimensions(&line)).collect();
    let total_area = part_1(box_dimensions);
    println!("The elves should order {total_area} square feet of wrapping paper")
}

type BoxDimensions = Vec<u32>;
type Area = u32;

fn part_1(box_dimensions: Vec<BoxDimensions>) -> Area {
    box_dimensions.iter()
        .map(|dimensions| get_wrapping_paper_area(&dimensions))
        .sum::<Area>()
}

fn parse_box_dimensions(dimensions: &str) -> BoxDimensions {
    dimensions.split('x').map(|s| s.parse().unwrap()).collect()
}

fn get_wrapping_paper_area(dimensions: &BoxDimensions) -> Area {
    assert!(dimensions.len() == 3, "Expected 3 dimensions");
    let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
    let areas = vec![l * w, w * h, h * l];
    let slack = areas.iter().min().unwrap();
    2 * areas.iter().sum::<u32>() + slack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_box_dimensions_examples() {
        assert_eq!(parse_box_dimensions("2x3x4"), vec![2, 3, 4]);
        assert_eq!(parse_box_dimensions("1x1x10"), vec![1, 1, 10]);
    }

    #[test]
    fn get_wrapping_paper_area_examples() {
        assert_eq!(get_wrapping_paper_area(&vec![2, 3, 4]), 58);
        assert_eq!(get_wrapping_paper_area(&vec![1, 1, 10]), 43);
    }

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(vec![ vec![2,3,4], vec![1, 1, 10] ]), 58 + 43);
    }
}
