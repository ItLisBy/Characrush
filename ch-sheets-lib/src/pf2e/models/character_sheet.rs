use std::path::Path;
use crate::common::character_sheet;

pub struct Pf2eCharacterSheet {
    abilities: Abilities,
    feats: Vec<Feat>
}

struct Abilities {
    str: u8,
    dex: u8,
    con: u8,
    int: u8,
    wis: u8,
    cha: u8
}

struct Feat {
    class: String,
    name: String,
    description: String
}

impl character_sheet::CharacterSheet for Pf2eCharacterSheet {
    fn new(path: &Path) -> Self {
        todo!()
    }

    fn get_calculated(&self) -> Self {
        todo!()
    }
}