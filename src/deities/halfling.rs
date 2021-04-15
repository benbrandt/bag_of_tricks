use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct Halfling;

impl Pantheon for Halfling {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Arvoreen",
                titles: vec!["god of vigilance and war"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Crossed short swords"],
            },
            Deity {
                name: "Brandobaris",
                titles: vec!["god of thievery and adventure"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Halfling footprint"],
            },
            Deity {
                name: "Cyrrollalee",
                titles: vec!["goddess of hearth and home"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["An open door"],
            },
            Deity {
                name: "Sheela Peryroyl",
                titles: vec!["goddess of agriculture and weather"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Flower"],
            },
            Deity {
                name: "Urogalan",
                titles: vec!["god of earth and death"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Death, Domain::Knowledge],
                symbols: vec!["Silhouette of a dog's head"],
            },
            Deity {
                name: "Yondalla",
                titles: vec!["goddess of fertility and protection"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Cornucopia on a shield", "shield"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(Halfling::deities());
    }
}
