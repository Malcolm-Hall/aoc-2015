use std::fs;

fn main() {
    let input = fs::read_to_string("input/d18.txt").unwrap();
    let mut light_grid_1 = LightGrid::from_str(Grid::Regular, &input).unwrap();
    let mut light_grid_2 = LightGrid::from_str(Grid::Broken, &input).unwrap();
    light_grid_1.step(100);
    light_grid_2.step(100);
    println!("Part 1: {}", light_grid_1.count_total_lit());
    println!("Part 2: {}", light_grid_2.count_total_lit());
}

type State = Vec<Vec<bool>>;

struct LightGrid {
    grid: Grid,
    state: State,
}

impl LightGrid {
    fn new(grid: Grid, state: State) -> Self {
        let state = grid.set_state(state);
        Self { grid, state }
    }

    fn from_str(grid: Grid, grid_str: &str) -> Option<Self> {
        let grid_size = grid_str.lines().count();
        let mut state = Vec::new();
        for line in grid_str.lines() {
            if line.len() != grid_size {
                return None;
            }
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '#' => row.push(true),
                    '.' => row.push(false),
                    _ => return None,
                }
            }
            state.push(row);
        }
        let state = grid.set_state(state);
        Some(Self::new(grid, state))
    }

    fn step(&mut self, n: usize) {
        for _ in 0..n {
            self.step_once();
        }
    }

    fn step_once(&mut self) {
        let mut next_state = self.state.clone();
        for (i, row) in self.state.iter().enumerate() {
            for (j, light) in row.iter().enumerate() {
                let ln = self.count_lit_neighbours(i, j);
                if *light && (ln < 2 || ln > 3) {
                    next_state[i][j] = false;
                } else if !light && ln == 3 {
                    next_state[i][j] = true;
                }
            }
        }
        let next_state = self.grid.set_state(next_state);
        self.state = next_state;
    }

    fn count_lit_neighbours(&self, i: usize, j: usize) -> u8 {
        let mut lit_neighbours = 0;
        let size = self.state.len();
        for x in 0.max(i as i32 - 1) as usize..size.min(i + 2) {
            for y in 0.max(j as i32 - 1) as usize..size.min(j + 2) {
                if (x != i || y != j) && self.state[x][y] {
                    lit_neighbours += 1;
                }
            }
        }
        lit_neighbours
    }

    fn count_total_lit(&self) -> usize {
        self.state.iter().fold(0, |acc, row| {
            acc + row.iter().filter(|light| **light).count()
        })
    }
}

enum Grid {
    Regular,
    Broken,
}

impl Grid {
    fn set_state(&self, mut state: State) -> State {
        match self {
            Grid::Regular => (),
            Grid::Broken => {
                let end = state.len() - 1;
                state[0][0] = true;
                state[0][end] = true;
                state[end][0] = true;
                state[end][end] = true;
            }
        }
        state
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_state() -> Vec<Vec<bool>> {
        vec![
            vec![false, true, false, true, false, true],
            vec![false, false, false, true, true, false],
            vec![true, false, false, false, false, true],
            vec![false, false, true, false, false, false],
            vec![true, false, true, false, false, true],
            vec![true, true, true, true, false, false],
        ]
    }

    #[test]
    fn from_str() {
        let expected_state = example_state();
        let grid_str = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";
        let light_grid = LightGrid::from_str(Grid::Regular, grid_str);
        assert!(light_grid.is_some());
        let light_grid = light_grid.unwrap();
        assert_eq!(light_grid.state, expected_state);
    }

    #[test]
    fn count_lit_neighbours() {
        let example_state = example_state();
        let end = example_state.len() - 1;
        let light_grid = LightGrid::new(Grid::Regular, example_state);
        assert_eq!(1, light_grid.count_lit_neighbours(0, 0));
        assert_eq!(1, light_grid.count_lit_neighbours(0, end));
        assert_eq!(2, light_grid.count_lit_neighbours(end, 0));
        assert_eq!(1, light_grid.count_lit_neighbours(end, end));
        assert_eq!(2, light_grid.count_lit_neighbours(1, 1));
        assert_eq!(4, light_grid.count_lit_neighbours(1, end - 1));
        assert_eq!(6, light_grid.count_lit_neighbours(end - 1, 1));
        assert_eq!(2, light_grid.count_lit_neighbours(end - 1, end - 1));
    }

    #[test]
    fn count_total_lit() {
        let example_state = example_state();
        let light_grid = LightGrid::new(Grid::Regular, example_state);
        assert_eq!(15, light_grid.count_total_lit());
    }

    #[test]
    fn part_1_example() {
        let example_state = example_state();
        let mut light_grid = LightGrid::new(Grid::Regular, example_state);
        light_grid.step(4);
        let total_lit = light_grid.count_total_lit();
        assert_eq!(4, total_lit);
    }

    #[test]
    fn part_2_example() {
        let example_state = example_state();
        let mut light_grid = LightGrid::new(Grid::Broken, example_state);
        light_grid.step(5);
        let total_lit = light_grid.count_total_lit();
        assert_eq!(17, total_lit);
    }
}
