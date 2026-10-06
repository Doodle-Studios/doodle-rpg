use crate::game::Player;
use rand::{prelude::*, rng};
use std::fs;
use std::io::{Error, ErrorKind};

fn xor(input: &[u8], key: u8) -> Vec<u8> {
    input.iter().map(|b| b ^ key).collect()
}

pub fn save(player: Player, path: &str) -> Result<(), Error> {
    let key: u8 = rng().random_range(1..=255);
    let mut xored = xor(&serde_json::to_vec(&player)?, key);
    xored.push(key);
    fs::write(path, xored)
}

pub fn load(path: &str) -> Result<Player, Error> {
    let mut result = fs::read(path)?;
    let key = result
        .pop()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "save file is empty"))?;

    Ok(serde_json::from_slice::<Player>(&xor(&result, key))?)
}
