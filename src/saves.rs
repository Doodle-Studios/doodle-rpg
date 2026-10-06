use crate::game::Player;
use rand::{prelude::*, rng};
use std::fs;
use std::io::Error;

fn xor(input: &str, key: &u8) -> String {
    input.bytes().map(|b| (b ^ key) as char).collect()
}

pub fn save(player: Player, path: &str) -> Result<(), Error> {
    let key: u8 = rng().random_range(1..=255);
    let mut xored = xor(&serde_json::to_string(&player)?, &key);
    xored.push(key as char);
    fs::write(path, xored)
}

pub fn load(path: &str) -> Result<Player, Error> {
    let mut result = fs::read(path)?;
    let key = result.pop().unwrap_or(255);
    let result: &str = std::str::from_utf8(&result).unwrap();

    match serde_json::from_str::<Player>(&xor(result, &key)) {
        Ok(i) => Ok(i),
        Err(e) => Err(Error::from(e)),
    }
}
