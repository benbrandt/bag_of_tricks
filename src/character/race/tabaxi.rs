use serde::{Deserialize, Serialize};

use crate::character::alignment::{AlignmentInfluences, Attitude};

const OBSESSIONS: &[&str] = &[
    "a god or planar entity",
    "a monster",
    "a lost civilization",
    "a wizard's secrets",
    "a mundane item",
    "a magic item",
    "a location",
    "a legend or tale",
];

const QUIRKS: &[&str] = &[
    "You miss your tropical home and complain endlessly about the freezing weather, even in summer.",
    "You never wear the same outfit twice, unless you absolutely must.",
    "You have a minor phobia of water and hate getting wet.",
    "Your tail always betrays your inner thoughts.",
    "You purr loudly when you are happy.",
    "You keep a small ball of yarn in your hand, which you constantly fidget with.",
    "You are always in debt, since you spend your gold on lavish parties and gifts for friends.",
    "When talking about something you’re obsessed with, you speak quickly and never pause and other’s can’t understand you.",
    "You are a font of random trivia from the lore and stories you have discovered.",
    "You can’t help but pocket interesting objects you come across.",
];

#[derive(Deserialize, Serialize)]
pub(crate) struct Tabaxi {
    obsession: String,
    quirk: String,
}

impl AlignmentInfluences for Tabaxi {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }
}
