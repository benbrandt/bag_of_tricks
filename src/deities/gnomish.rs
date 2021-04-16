use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct Gnomish;

impl Pantheon for Gnomish {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Baervan Wildwanderer",
                titles: vec!["god of forests and woodlands"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Nature],
                symbols: vec!["Face of a raccoon"],
            },
            Deity {
                name: "Baravar Cloakshadow",
                titles: vec!["god of illusion and deception", "Sly One"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Trickery],
                symbols: vec!["Dagger against a hooded cloak"],
            },
            Deity {
                name: "Callarduran Smoothhands",
                titles: vec!["god of mining and carving stone"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Nature],
                symbols: vec!["Golden signet ring wiht a six-pointed star"],
            },
            Deity {
                name: "Flandal Steelskin",
                titles: vec![
                    "god of metalwork",
                    "god of mining and smithcraft",
                    "god of physical improvement and good health",
                    "the Steelsmith",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Flaming hammer"],
            },
            Deity {
                name: "Gaerdal Ironhand",
                titles: vec!["god of protection", "god of war, vigilance, and defense"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Iron band"],
            },
            Deity {
                name: "Garl Glittergold",
                titles: vec![
                    "god of humor, gemcutting, protection, and trickery",
                    "god of trickery and wiles",
                    "the Watchful Protector",
                    "the king of the gnomish gods",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Trickery],
                symbols: vec!["Gold nugget"],
            },
            Deity {
                name: "Nebelun",
                titles: vec!["god of invention and luck", "the Meddler"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Bellows and a lizard tail"],
            },
            Deity {
                name: "Segojan Earthcaller",
                titles: vec![
                    "god of earth and the dead",
                    "god of the wilds beneath the earth",
                    "god of the burrows",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Light],
                symbols: vec!["Glowing gemstone"],
            },
            Deity {
                name: "Urdlen",
                titles: vec![
                    "god of greed and murder",
                    "great-clawed god of bloodlust and evil, of greed and uncontrolled impulses",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Death, Domain::War],
                symbols: vec!["White clawed mole emerging from ground"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(Gnomish::deities());
    }
}
