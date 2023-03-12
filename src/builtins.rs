use clap::ValueEnum;
use strum::EnumIter;

#[derive(Copy, Clone, PartialEq, Eq, EnumIter, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Builtins {
    KittyFace,
    GameBTW,
    LoveMyPuter,
    Hashbang,
    Creature,
}

impl Builtins {
    pub fn get(&self) -> &'static str {
        match self {
            Builtins::GameBTW => include_str!("art/gamebtw.txt"),
            Builtins::KittyFace => include_str!("art/kittyface.txt"),
            Builtins::LoveMyPuter => include_str!("art/lovemyputer.txt"),
            Builtins::Hashbang => include_str!("art/hashbang.txt"),
            Builtins::Creature => include_str!("art/creature.txt"),
        }
    }
}
