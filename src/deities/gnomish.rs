use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct Gnomish;

impl Pantheon for Gnomish {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Baervan Wildwanderer",
                titles: vec!["god of woodlands"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Nature],
                symbols: vec!["Face of a raccoon"],
            },
            Deity {
                name: "Baravar Cloakshadow",
                titles: vec!["god of illusion and deception"],
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
                titles: vec!["god of metalwork"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Flaming hammer"],
            },
            Deity {
                name: "Gaerdal Ironhand",
                titles: vec!["god of protection"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Iron band"],
            },
            Deity {
                name: "Garl Glittergold",
                titles: vec!["god of trickery and gems", "god of trickery and wiles"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Trickery],
                symbols: vec!["Gold nugget"],
            },
            Deity {
                name: "Nebelun",
                titles: vec!["god of invention and luck"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Bellows and a lizard tail"],
            },
            Deity {
                name: "Segojan Earthcaller",
                titles: vec!["god of earth and the dead"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Light],
                symbols: vec!["Glowing gemstone"],
            },
            Deity {
                name: "Urdlen",
                titles: vec!["god of greed and murder"],
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
