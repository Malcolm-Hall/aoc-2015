use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input/d22.txt").unwrap();
    let boss_parameters = input
        .lines()
        .map(|l| l.split_once(':').unwrap().1.trim())
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let boss = Boss::new(boss_parameters[0], boss_parameters[1]);
    let player = Mage::new(50, 500);
    let arena = Arena::new(player, boss);

    let part_1 = get_least_mana(arena.clone(), false).unwrap();
    let part_2 = get_least_mana(arena.clone(), true).unwrap();
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_least_mana(start_arena: Arena, hardmode: bool) -> Option<i32> {
    let mut is_players_turn = true;
    let mut arenas = vec![start_arena];
    let mut next_arenas = Vec::new();
    let mut winners = Vec::new();
    while arenas.len() > 0 {
        for mut arena in arenas {
            if arena.boss.health <= 0 {
                winners.push(arena.player.total_mana_used);
                continue;
            }
            if hardmode && is_players_turn {
                arena.player.health -= 1;
            }
            if arena.player.health <= 0 {
                continue;
            }
            arena.before_turn_effects();
            if arena.boss.health <= 0 {
                winners.push(arena.player.total_mana_used);
                continue;
            }

            if is_players_turn {
                for spell in &arena.allowed_spells {
                    let mut next_arena = arena.clone();
                    if next_arena.player_cast(spell) {
                        next_arenas.push(next_arena);
                    }
                }
            } else {
                arena.boss_attack();
                next_arenas.push(arena);
            }
        }
        is_players_turn = !is_players_turn;
        arenas = next_arenas;
        next_arenas = Vec::new();
    }

    winners.into_iter().min()
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn all() -> HashSet<Spell> {
        HashSet::from([
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ])
    }

    fn get_mana_cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

#[derive(Debug, Clone)]
struct Boss {
    health: i32,
    damage: i32,
}

impl Boss {
    fn new(health: i32, damage: i32) -> Self {
        Self { health, damage }
    }

    fn defend(&mut self, damage: i32) {
        self.health -= 1.max(damage)
    }
}

#[derive(Debug, Clone)]
struct Mage {
    health: i32,
    mana: i32,
    protection: i32,
    total_mana_used: i32,
}

impl Mage {
    fn new(health: i32, mana: i32) -> Self {
        Self {
            health,
            mana,
            protection: 0,
            total_mana_used: 0,
        }
    }

    fn defend(&mut self, damage: i32) {
        self.health -= 1.max(damage - self.protection)
    }

    fn use_mana(&mut self, mana_cost: i32) -> bool {
        if mana_cost > self.mana {
            return false;
        }
        self.mana -= mana_cost;
        self.total_mana_used += mana_cost;
        true
    }
}

#[derive(Debug, Clone)]
struct Arena {
    player: Mage,
    boss: Boss,
    shield_effect: i32,
    poison_effect: i32,
    recharge_effect: i32,
    allowed_spells: HashSet<Spell>,
}

impl Arena {
    fn new(player: Mage, boss: Boss) -> Self {
        Self {
            player,
            boss,
            shield_effect: 0,
            poison_effect: 0,
            recharge_effect: 0,
            allowed_spells: Spell::all(),
        }
    }

    fn before_turn_effects(&mut self) {
        self.allowed_spells = Spell::all();

        if self.poison_effect > 0 {
            self.boss.health -= 3;
            self.poison_effect -= 1;
            if self.poison_effect > 0 {
                self.allowed_spells.remove(&Spell::Poison);
            }
        }
        if self.recharge_effect > 0 {
            self.player.mana += 101;
            self.recharge_effect -= 1;
            if self.recharge_effect > 0 {
                self.allowed_spells.remove(&Spell::Recharge);
            }
        }
        if self.shield_effect > 0 {
            self.player.protection = 7;
            self.shield_effect -= 1;
            if self.shield_effect > 0 {
                self.allowed_spells.remove(&Spell::Shield);
            }
        } else {
            self.player.protection = 0;
        }
    }

    fn player_cast(&mut self, spell: &Spell) -> bool {
        if !self.player.use_mana(spell.get_mana_cost()) {
            return false;
        }
        match spell {
            Spell::MagicMissile => {
                self.boss.defend(4);
            }
            Spell::Drain => {
                self.boss.defend(2);
                self.player.health += 2;
            }
            Spell::Shield => {
                self.shield_effect = 6;
            }
            Spell::Poison => {
                self.poison_effect = 6;
            }
            Spell::Recharge => {
                self.recharge_effect = 5;
            }
        }
        true
    }

    fn boss_attack(&mut self) {
        self.player.defend(self.boss.damage);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_fight_1() {
        let player = Mage::new(10, 250);
        let boss = Boss::new(13, 8);
        let mut arena = Arena::new(player, boss);

        let mut is_player_turn = true;
        let player_attacks = [Spell::Poison, Spell::MagicMissile];
        let expected_player_health = [10, 10, 2, 2];
        let expected_player_mana = [250, 77, 77, 24];
        let expected_boss_health = [13, 13, 10, 3];
        let expected_poison_timer = [0, 5, 4, 3];
        for i in 0..4 {
            assert_eq!(arena.player.health, expected_player_health[i]);
            assert_eq!(arena.player.mana, expected_player_mana[i]);
            assert_eq!(arena.boss.health, expected_boss_health[i]);
            arena.before_turn_effects();
            assert_eq!(arena.poison_effect, expected_poison_timer[i]);
            if i == 3 {
                assert!(arena.boss.health <= 0);
                break;
            }
            if is_player_turn {
                arena.player_cast(&player_attacks[i / 2]);
            } else {
                arena.boss_attack();
            }
            is_player_turn = !is_player_turn;
        }
    }

    #[test]
    fn example_fight_2() {
        let player = Mage::new(10, 250);
        let boss = Boss::new(14, 8);
        let mut arena = Arena::new(player, boss);

        let mut is_player_turn = true;
        let player_attacks = [
            Spell::Recharge,
            Spell::Shield,
            Spell::Drain,
            Spell::Poison,
            Spell::MagicMissile,
        ];
        let expected_player_health = [10, 10, 2, 2, 1, 3, 2, 2, 1, 1];
        let expected_player_mana = [250, 21, 122, 110, 211, 239, 340, 167, 167, 114];
        let expected_player_protection = [0, 0, 0, 7, 7, 7, 7, 7, 7, 0];
        let expected_boss_health = [14, 14, 14, 14, 14, 12, 12, 12, 9, 2];
        let expected_poison_timer = [0, 0, 0, 0, 0, 0, 0, 5, 4, 3];
        let expected_recharge_timer = [0, 4, 3, 2, 1, 0, 0, 0, 0, 0];
        let expected_shield_timer = [0, 0, 0, 5, 4, 3, 2, 1, 0, 0];
        for i in 0..10 {
            assert_eq!(arena.player.health, expected_player_health[i]);
            assert_eq!(arena.player.mana, expected_player_mana[i]);
            assert_eq!(arena.boss.health, expected_boss_health[i]);
            arena.before_turn_effects();
            assert_eq!(arena.player.protection, expected_player_protection[i]);
            assert_eq!(arena.recharge_effect, expected_recharge_timer[i]);
            assert_eq!(arena.shield_effect, expected_shield_timer[i]);
            assert_eq!(arena.poison_effect, expected_poison_timer[i]);
            if i == 9 {
                assert!(arena.boss.health <= 0);
                break;
            }
            if is_player_turn {
                arena.player_cast(&player_attacks[i / 2]);
            } else {
                arena.boss_attack();
            }
            is_player_turn = !is_player_turn;
        }
    }
}
