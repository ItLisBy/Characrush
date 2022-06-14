use std::path::Path;

pub trait CharacterSheet {
    fn new(path: &Path) -> Self;
    fn get_calculated(&self) -> Self;
}