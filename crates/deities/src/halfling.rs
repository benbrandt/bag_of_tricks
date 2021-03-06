use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Halfling;

impl<'a> Deities<'a> for Halfling {
    fn deities() -> Vec<Deity<'a>> {
        vec![
            Deity {
                name: "Arvoreen",
                titles: vec![
                    "god of vigilance and war",
                    "defender-god",
                    "watchful protector",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Crossed short swords"],
            },
            Deity {
                name: "Brandobaris",
                titles: vec![
                    "god of thievery, stealth, and adventure",
                    "god of adventure and thievery",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Halfling footprint"],
            },
            Deity {
                name: "Charmalaine",
                titles: vec!["deity of keen senses and luck"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Burning boot print"],
            },
            Deity {
                name: "Cyrrollalee",
                titles: vec![
                    "goddess of hearth, hospitality, and home",
                    "goddess of trust and handicrafts",
                    "goddess of hearth and home",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["An open door"],
            },
            Deity {
                name: "Sheela Peryroyl",
                titles: vec![
                    "goddess of agriculture and weather",
                    "goddess of nature",
                    "goddess of love, song, and dance",
                    "the lady of fields, streams, and the wilds found in shire and glen",
                    "the Green Sister of Yondalla",
                    "goddess of agriculture, nature, and weather",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Flower"],
            },
            Deity {
                name: "Urogalan",
                titles: vec!["god of earth and death"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Death, Domain::Grave, Domain::Knowledge],
                symbols: vec!["Silhouette of a dog's head"],
            },
            Deity {
                name: "Yondalla",
                titles: vec![
                    "goddess of bounty, fertility, and protection",
                    "protector of hearth, home, and family",
                    "the Blessed One",
                    "Primary goddess of halflings",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Cornucopia on a shield", "shield", "cornucopia"],
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
