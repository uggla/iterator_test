#![allow(dead_code)]

mod enemy;
mod player;
use rand::{Rng, rng};

use crate::{enemy::Enemy, player::Player};

use std::{
    error::Error,
    io::{self, Write},
};

trait Poisoned {
    fn poisoned(&mut self);
    fn is_poisoned(&self) -> bool;
}

struct CharacterCollection<T> {
    data: Vec<T>,
}

impl<T> CharacterCollection<T> {
    fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    fn iter(&self) -> std::slice::Iter<T> {
        self.into_iter()
    }

    fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.into_iter()
    }
}

impl<T> IntoIterator for CharacterCollection<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a CharacterCollection<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut CharacterCollection<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl CharacterCollection<Player> {
    fn add_player_score(&mut self, id: usize) -> Result<(), Box<dyn Error>> {
        match self.data.get_mut(id) {
            Some(v) => v.add_score(),
            None => return Err(format!("Player id:'{id}' not found.").into()),
        }

        Ok(())
    }
    fn get_player_mut(&mut self, name: &str) -> &mut Player {
        self.data.iter_mut().find(|n| n.name == name).unwrap()
    }

    fn remove_killed(&mut self) {
        self.data.retain(|o| {
            if o.life > 0 {
                true
            } else {
                println!();
                println!("💀 Player {} killed 💀", o.name);
                println!();
                false
            }
        });
    }
}

impl CharacterCollection<Enemy> {
    fn get_enemy_mut(&mut self, name: &str) -> &mut Enemy {
        self.data.iter_mut().find(|n| n.name == name).unwrap()
    }

    fn remove_killed(&mut self) {
        self.data.retain(|o| {
            if o.life > 0 {
                true
            } else {
                println!();
                println!("💀 Enemy {} killed 💀", o.name);
                println!();
                false
            }
        });
    }
}

fn fight(player: &mut Player, enemy: &mut Enemy) {
    println!(
        "⚔️  Entering the fight between {} and {} ! ⚔️",
        player.name, enemy.name
    );
    let player_dice = rng().random_range(1..=6);
    let enemy_dice = rng().random_range(1..=6);

    if player_dice > enemy_dice {
        enemy.life = enemy.life.saturating_sub(player_dice);
        if enemy.life == 0 {}
    } else {
        player.life = player.life.saturating_sub(enemy_dice);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut players = CharacterCollection::new(
        [
            Player::new("Uggla"),
            Player::new("Rose"),
            Player::new("Taupie"),
        ]
        .to_vec(),
    );

    let mut enemies = CharacterCollection::new(
        [Enemy::new("Wham"), Enemy::new("Acdc"), Enemy::new("Kars")].to_vec(),
    );

    players.get_player_mut("Uggla").poisoned();

    players.add_player_score(0)?; // poisoned player so score is stuck
    players.add_player_score(0)?;
    players.add_player_score(1)?;
    players.add_player_score(1)?;
    players.add_player_score(2)?;

    loop {
        if players.data.is_empty() || enemies.data.is_empty() {
            println!("Gameover !");
            break;
        }

        println!();
        show_stats(&players, &enemies);

        let player = rng().random_range(0..players.data.len());
        let enemy = rng().random_range(0..enemies.data.len());
        fight(&mut players.data[player], &mut enemies.data[enemy]);

        players.remove_killed();
        enemies.remove_killed();
        println!();
        show_stats(&players, &enemies);

        println!();
        println!("Fight again ([Y]/Quit)");
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let input = s.trim().to_lowercase();
        match input.as_ref() {
            "quit" => break,
            "y" => {}
            _ => {}
        }
    }

    Ok(())
}

fn show_stats(players: &CharacterCollection<Player>, enemies: &CharacterCollection<Enemy>) {
    for player in players.iter() {
        println!("{player}");
    }

    for enemy in enemies.iter() {
        println!("{enemy}");
    }
}
