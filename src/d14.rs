use std::{cmp::min, fs, num::ParseIntError};

pub fn solutions() {
    let input = fs::read_to_string("input/d14.txt").unwrap();
    let end_time = 2503;
    let contestants = input
        .lines()
        .map(|line| Reindeer::from_str(line).unwrap())
        .collect::<Vec<_>>();
    let part_1_winner_distance = part_1_winner(end_time, &contestants).unwrap();
    let part_2_winner_points = part_2_winner(end_time, &contestants).unwrap();
    println!("The part 1 winning reindeer travelled a distance of {part_1_winner_distance}");
    println!("The part 2 winning reindeer finished with {part_2_winner_points} points");
}

#[derive(PartialEq, Debug)]
struct Reindeer {
    name: String,
    fly_speed: usize,
    fly_time: usize,
    rest_time: usize,
}

impl Reindeer {
    fn new(name: &str, fly_speed: usize, fly_time: usize, rest_time: usize) -> Reindeer {
        Reindeer {
            name: name.to_string(),
            fly_speed,
            fly_time,
            rest_time,
        }
    }

    fn from_str(riendeer_description: &str) -> Result<Reindeer, ParseIntError> {
        let description = riendeer_description.split(" ").collect::<Vec<_>>();
        match description[..] {
            [name, "can", "fly", fly_speed, "km/s", "for", fly_time, "seconds,", "but", "then", "must", "rest", "for", rest_time, "seconds."] =>
            {
                let fly_speed = fly_speed.parse::<usize>()?;
                let fly_time = fly_time.parse::<usize>()?;
                let rest_time = rest_time.parse::<usize>()?;
                Ok(Reindeer::new(name, fly_speed, fly_time, rest_time))
            }
            _ => panic!("Unexpected description"),
        }
    }

    fn part_1_distance(self: &Self, mut t: usize) -> usize {
        let mut distance = 0;
        let mut is_resting = false;
        while t > 0 {
            if is_resting {
                let time_rested = min(t, self.rest_time);
                t -= time_rested;
                is_resting = false;
            } else {
                let time_traveled = min(t, self.fly_time);
                distance += self.fly_speed * time_traveled;
                t -= time_traveled;
                is_resting = true;
            }
        }
        distance
    }

    fn part_2_distance<'a>(self: &'a Self, t: usize) -> impl Iterator<Item = usize> + 'a {
        let mut distance = 0;
        let mut step = 0;
        let mut is_resting = false;
        let mut phase_time = self.fly_time;

        std::iter::from_fn(move || {
            if step < t {
                if phase_time == 0 {
                    match is_resting {
                        true => phase_time = self.fly_time,
                        false => phase_time = self.rest_time,
                    }
                    is_resting = !is_resting
                }
                let step_size = self.step(is_resting);
                distance += step_size;
                phase_time -= 1;
                step += 1;
                Some(distance)
            } else {
                None
            }
        })
    }

    fn step(self: &Self, is_resting: bool) -> usize {
        if is_resting {
            0
        } else {
            self.fly_speed
        }
    }
}

fn part_1_winner(end_time: usize, contestants: &[Reindeer]) -> Option<usize> {
    contestants
        .iter()
        .map(|r| r.part_1_distance(end_time))
        .max()
}

fn part_2_winner(end_time: usize, contestants: &[Reindeer]) -> Option<usize> {
    let mut distance_iters = contestants
        .iter()
        .map(|r| r.part_2_distance(end_time))
        .collect::<Vec<_>>();
    let mut totals = vec![0usize; distance_iters.len()];
    for _ in 0..end_time {
        let distances_at_i = distance_iters
            .iter_mut()
            .map(|d| d.next().unwrap_or(0))
            .collect::<Vec<_>>();
        let max = distances_at_i.iter().max()?;
        distances_at_i
            .iter()
            .enumerate()
            .filter(|(_, d)| *d == max)
            .map(|(idx, _)| idx)
            .for_each(|idx| {
                let total = totals.get_mut(idx).unwrap();
                *total += 1;
            })
    }
    totals.into_iter().max()
}

#[cfg(test)]
mod test {
    use super::*;

    fn examples() -> Vec<Reindeer> {
        vec![
            Reindeer::new("Comet", 14, 10, 127),
            Reindeer::new("Dancer", 16, 11, 162),
        ]
    }

    #[test]
    fn distance_examples() {
        let examples = examples();
        let results = examples
            .into_iter()
            .map(|r| r.part_1_distance(1000))
            .collect::<Vec<_>>();
        assert_eq!(results, vec![1120, 1056]);
    }

    #[test]
    fn part_1_winner_examples() {
        let examples = examples();
        let result = part_1_winner(1000, &examples);
        if let Some(d) = result {
            assert_eq!(d, 1120);
        } else {
            assert!(false, "Should be a winner")
        }
    }

    #[test]
    fn part_2_winner_examples() {
        let examples = examples();
        let result = part_2_winner(1000, &examples);
        if let Some(t) = result {
            assert_eq!(t, 689);
        } else {
            assert!(false, "Should be a winner")
        }
    }
}
