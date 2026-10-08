use crate::saves::*;
use rand::{prelude::*, rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Entity {
    fn damage(&mut self, damaging: &Damaging);
}

#[derive(Serialize, Deserialize)]
pub struct RndDmg(f64, f64);
impl RndDmg {
    pub fn calculate(&self) -> f64 {
        rng().random_range(self.0..self.1)
    }
}

#[derive(Serialize, Deserialize)]
pub enum Effect {
    DamageMult(RndDmg),
}

pub enum Damaging<'a> {
    Weapon(&'a Weapon),
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    dmg: RndDmg,
    effects: HashMap<String, Effect>,
}

impl Weapon {
    pub fn new(min: f64, max: f64, effects: HashMap<String, Effect>) -> Weapon {
        Weapon {
            dmg: RndDmg(min, max),
            effects,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Armour {
    protection: f64,
    effects: HashMap<String, Effect>,
}

#[derive(Serialize, Deserialize)]
pub enum Equipment {
    Weapon(Weapon),
    Armour(Armour),
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    hp: f64,
    max_hp: f64,
    xp: f64,
    equipment: Vec<Equipment>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            hp: 100.0,
            max_hp: 100.0,
            xp: 0.0,
            equipment: vec![Equipment::Weapon(Weapon::new(0.0, 10.0, HashMap::new()))],
        }
    }

    pub fn from(file_path: &str) -> Player {
        load(file_path).unwrap_or_else(|e| {
            eprintln!("{}", e);
            Player::new()
        })
    }
}

impl Entity for Player {
    fn damage(&mut self, damaging: &Damaging) {
        match damaging {
            Damaging::Weapon(weapon) => {
                let mut final_dmg: f64 = 0.0;
                final_dmg += weapon.dmg.calculate();

                let mut effect_modifier: f64 = 1.0;

                for effect in weapon.effects.values() {
                    match effect {
                        Effect::DamageMult(mult) => effect_modifier *= mult.calculate(),
                    }
                }

                final_dmg *= effect_modifier;

                self.hp = (self.hp - final_dmg).clamp(0.0f64, self.max_hp);
            }
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new()
    }
}
