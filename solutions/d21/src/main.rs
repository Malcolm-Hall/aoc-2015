use std::fs;

fn main() {
    let input = fs::read_to_string("input/d21.txt").unwrap();
    let boss_parameters = input
        .lines()
        .map(|l| l.split_once(':').unwrap().1.trim())
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let boss = Fighter::new(
        boss_parameters[0],
        0,
        boss_parameters[1],
        boss_parameters[2],
    );
    let empty = (0, 0, 0);
    let weapons = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armors = [
        empty.clone(),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];
    let rings = vec![
        empty.clone(),
        empty.clone(),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];
    let mut winners = Vec::new();
    let mut losers = Vec::new();
    for weapon in &weapons {
        for armor in &armors {
            for (index, ring_1) in rings.iter().enumerate() {
                let mut remaining_rings = rings.clone();
                remaining_rings.swap_remove(index);
                for ring_2 in remaining_rings {
                    let cost = weapon.0 + armor.0 + ring_1.0 + ring_2.0;
                    let damage = weapon.1 + armor.1 + ring_1.1 + ring_2.1;
                    let protection = weapon.2 + armor.2 + ring_1.2 + ring_2.2;
                    let player = Fighter::new(100, cost, damage, protection);
                    let mut arena = Arena::new(player, boss.clone());
                    match arena.fight() {
                        true => winners.push(arena.player.cost),
                        false => losers.push(arena.player.cost),
                    };
                }
            }
        }
    }
    println!("Part 1: {}", winners.into_iter().min().unwrap());
    println!("Part 2: {}", losers.into_iter().max().unwrap());
}

#[derive(Debug, Clone, Copy)]
struct Fighter {
    health: i32,
    cost: i32,
    damage: i32,
    protection: i32,
}

impl Fighter {
    fn new(health: i32, cost: i32, damage: i32, protection: i32) -> Self {
        Self {
            health,
            cost,
            damage,
            protection,
        }
    }

    fn defend(&mut self, attacker: &Fighter) {
        self.health -= 1.max(attacker.damage - self.protection)
    }
}

#[derive(Debug)]
struct Arena {
    player: Fighter,
    boss: Fighter,
}

impl Arena {
    fn new(player: Fighter, boss: Fighter) -> Self {
        Self { player, boss }
    }

    fn next_turn(&mut self) {
        self.boss.defend(&self.player);
        self.player.defend(&self.boss);
    }

    fn fight(&mut self) -> bool {
        loop {
            self.next_turn();
            match self.winner() {
                Some(winner) => {
                    return winner;
                }
                None => (),
            };
        }
    }

    fn winner(&self) -> Option<bool> {
        if self.boss.health <= 0 {
            return Some(true);
        } else if self.player.health <= 0 {
            return Some(false);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_fight() {
        let player = Fighter::new(8, 0, 5, 5);
        let boss = Fighter::new(12, 0, 7, 2);
        let mut arena = Arena::new(player, boss);

        let expected_player_health = [6, 4, 2, 0];
        let expected_boss_health = [9, 6, 3, 0];
        let expected_winner = [None, None, None, Some(true)];
        for i in 0..4 {
            arena.next_turn();
            assert_eq!(arena.player.health, expected_player_health[i]);
            assert_eq!(arena.boss.health, expected_boss_health[i]);
            assert_eq!(arena.winner(), expected_winner[i]);
        }
    }
}
