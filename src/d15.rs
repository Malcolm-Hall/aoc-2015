use std::fs;

pub fn solutions() {
    let input = fs::read_to_string("input/d15.txt").unwrap();
    let recipie = Recipie::from_str(&input);
    let best_score_part_1 = find_best_score_part_1(&recipie);
    let best_score_part_2 = find_best_score_part_2(&recipie);
    println!("Best score is {best_score_part_1}");
    println!("Best score with 500 calories is {best_score_part_2}");
}

struct Recipie {
    capacities: Vec<i32>,
    durabilities: Vec<i32>,
    flavors: Vec<i32>,
    textures: Vec<i32>,
    calories: Vec<i32>,
}

impl Recipie {
    fn from_str(recipie_str: &str) -> Recipie {
        let mut capacities = Vec::new();
        let mut durabilities = Vec::new();
        let mut flavors = Vec::new();
        let mut textures = Vec::new();
        let mut calories = Vec::new();

        recipie_str.lines().for_each(|l| {
            let ingredient = l.split(&[' ', ',']).filter(|split| split.len() > 0).collect::<Vec<_>>();
            match ingredient[..] {
                [_, "capacity", capacity, "durability", durability, "flavor", flavor, "texture", texture, "calories", kalories] =>
                {
                    capacities.push(capacity.parse::<i32>().unwrap());
                    durabilities.push(durability.parse::<i32>().unwrap());
                    flavors.push(flavor.parse::<i32>().unwrap());
                    textures.push(texture.parse::<i32>().unwrap());
                    calories.push(kalories.parse::<i32>().unwrap());
                }
                _ => panic!("Unexpected ingredient"),
            }
        });

        Recipie {
            capacities,
            durabilities,
            flavors,
            textures,
            calories,
        }
    }

    fn get_weighted_totals(self: &Self, weights: &[i32]) -> (i32, i32, i32, i32, i32) {
        let mut totals = (0, 0, 0, 0, 0);
        for (((((idx, c), d), f), t), k) in (0..self.capacities.len())
            .zip(&self.capacities)
            .zip(&self.durabilities)
            .zip(&self.flavors)
            .zip(&self.textures)
            .zip(&self.calories)
        {
            let w = weights.get(idx).unwrap();
            let (ct, dt, ft, tt, kt) = totals;
            totals = (ct + w * c, dt + w * d, ft + w * f, tt + w * t, kt + w * k)
        }
        totals
    }

    fn get_totals(self: &Self, weights: &[i32]) -> (u32, u32, u32, u32, u32) {
        let (c, d, f, t, k) = self.get_weighted_totals(weights);
        (
            c.max(0) as u32,
            d.max(0) as u32,
            f.max(0) as u32,
            t.max(0) as u32,
            k.max(0) as u32,
        )
    }

    fn total_score_with_calories(self: &Self, weights: &[i32]) -> (u32, u32) {
        let (c, d, f, t, k) = self.get_totals(weights);
        (c * d * f * t, k)
    }

    fn total_score(self: &Self, weights: &[i32]) -> u32 {
        let (score, _) = self.total_score_with_calories(weights);
        score
    }
}

fn find_best_score_part_1(recipie: &Recipie) -> u32 {
    let mut best_score = 0;
    for i in 0..101 {
        for j in 0..101 {
            for k in 0..101 {
                for l in 0..101 {
                    if i as u32 + j as u32 + k as u32 + l as u32 == 100 {
                        let score = recipie.total_score(&[i, j, k, l]);
                        best_score = best_score.max(score);
                    }
                }
            }
        }
    }
    best_score
}

fn find_best_score_part_2(recipie: &Recipie) -> u32 {
    let mut best_score = 0;
    for i in 0..101 {
        for j in 0..101 {
            for k in 0..101 {
                for l in 0..101 {
                    if i as u32 + j as u32 + k as u32 + l as u32 == 100 {
                        let (score, calories) = recipie.total_score_with_calories(&[i, j, k, l]);
                        if calories == 500 {
                            best_score = best_score.max(score);
                        }
                    }
                }
            }
        }
    }
    best_score
}

#[cfg(test)]
mod test {
    use super::*;

    fn example() -> Recipie {
        let recipie_str =
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        Recipie::from_str(recipie_str)
    }

    #[test]
    fn test_total_score() {
        let recipie = example();
        assert_eq!(recipie.total_score(&vec![44, 56]), 62842880)
    }

    #[test]
    fn test_total_score_with_calories() {
        let recipie = example();
        assert_eq!(
            recipie.total_score_with_calories(&vec![40, 60]),
            (57600000, 500)
        )
    }
}
