use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Gnomish;

impl<'a> Deities<'a> for Gnomish {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity<'a>> {
        vec![
            Deity {
                name: "Baervan Wildwanderer",
                titles: vec!["god of forests and woodlands", "Province: woodlands"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Nature],
                symbols: vec!["Face of a raccoon"],
            },
            Deity {
                name: "Baravar Cloakshadow",
                titles: vec![
                    "god of illusion and deception",
                    "Sly One",
                    "Province: illusion, deception",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Trickery],
                symbols: vec!["Dagger against a hooded cloak"],
            },
            Deity {
                name: "Bleredd",
                titles: vec!["Province: labor, craft"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Forge, Domain::Light],
                symbols: vec!["Iron mule"],
            },
            Deity {
                name: "Callarduran Smoothhands",
                titles: vec![
                    "god of mining and carving stone",
                    "Province: mining, stone carving",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Nature],
                symbols: vec!["Golden signet ring with a six-pointed star"],
            },
            Deity {
                name: "Flandal Steelskin",
                titles: vec![
                    "god of metalwork",
                    "god of mining and smithcraft",
                    "god of physical improvement and good health",
                    "the Steelsmith",
                    "Province: metalwork",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Forge, Domain::Knowledge],
                symbols: vec!["Flaming hammer"],
            },
            Deity {
                name: "Gaerdal Ironhand",
                titles: vec![
                    "god of protection",
                    "god of war, vigilance, and defense",
                    "Province: protection",
                ],
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
                    "Primary god of gnomes",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Trickery],
                symbols: vec!["Gold nugget"],
            },
            Deity {
                name: "Gelf Darkhearth",
                titles: vec!["Province: frustration, destruction"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["Broken anvil"],
            },
            Deity {
                name: "Nebelun",
                titles: vec![
                    "god of invention and luck",
                    "the Meddler",
                    "Province: invention, luck",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Forge, Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Bellows and a lizard tail"],
            },
            Deity {
                name: "Rill Cleverthrush",
                titles: vec!["Province: law, thought"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Interlocking gears"],
            },
            Deity {
                name: "Segojan Earthcaller",
                titles: vec![
                    "god of earth and the dead",
                    "god of the wilds beneath the earth",
                    "god of the burrows",
                    "Province: earth, the dead",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Grave, Domain::Light],
                symbols: vec!["Glowing gemstone"],
            },
            Deity {
                name: "Sheyanna Flaxenstrand",
                titles: vec!["Province: love, beauty, passion"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Light],
                symbols: vec!["Two silver goblets"],
            },
            Deity {
                name: "Urdlen",
                titles: vec![
                    "god of greed and murder",
                    "great-clawed god of bloodlust and evil, of greed and uncontrolled impulses",
                    "Province: greed, murder",
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
