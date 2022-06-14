use std::path::Path;
use crate::common::character_sheet;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Pf2eCharacterSheet {
    abilities: Abilities,
    abilities_mods: AbilitiesMods,
    feats: Vec<Feat>,
    prof: u8,
    lvl: u8,
    class: String,

}

#[derive(Debug, Clone, Deserialize)]
struct Abilities {
    str: u8,
    dex: u8,
    con: u8,
    int: u8,
    wis: u8,
    cha: u8
}

#[derive(Debug, Clone, Deserialize)]
struct AbilitiesMods {
    str: u8,
    dex: u8,
    con: u8,
    int: u8,
    wis: u8,
    cha: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct Feat {
    class: String,
    name: String,
    description: String
}

impl character_sheet::CharacterSheet for Pf2eCharacterSheet {
    fn new(path: &Path) -> Self {
        todo!()
    }

    fn get_calculated(&mut self) -> Self {
        todo!()
    }
}