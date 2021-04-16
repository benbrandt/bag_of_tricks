use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct Dwarven;

impl Pantheon for Dwarven {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Abbathor",
                titles: vec!["god of greed"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Jeweled dagger, point-down"],
            },
            Deity {
                name: "Berronar Truesilver",
                titles: vec![
                    "goddess of hearth and home",
                    "goddess of honesty and faithfulness",
                    "goddess of oaths, loyalty, and honor",
                    "the Revered Mother",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Intertwined silver rings"],
            },
            Deity {
                name: "Clangeddin Silverbeard",
                titles: vec!["god of war and valor"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Crossed silver battleaxes"],
            },
            Deity {
                name: "Dugmaren Brightmantle",
                titles: vec![
                    "god of invention and discovery",
                    "the Wandering Tinkerer",
                    "the Gleam in the Eye",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Open book"],
            },
            Deity {
                name: "Dumathoin",
                titles: vec![
                    "god of buried secrets",
                    "god of buried wealth, mining, gems, and exploration",
                    "the Keeper of Secrets under the Mountain",
                    "the guardian of the dead",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Death, Domain::Knowledge],
                symbols: vec!["Mountain silhouette with a central gemstone"],
            },
            Deity {
                name: "Gorm Gulthyn",
                titles: vec![
                    "god of defense and vigilance",
                    "Fire Eyes",
                    "the Lord of the Bronze Mask",
                    "the protector of dwarves",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Bronze half-mask"],
            },
            Deity {
                name: "Haela Brightaxe",
                titles: vec!["goddess of war-luck", "patron of dwarf fighters"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Upright sword whose blade is spiraled in flame"],
            },
            Deity {
                name: "Marthammor Duin",
                titles: vec![
                    "god of wanderers",
                    "the traveler's god",
                    "patron of expatriates and guides",
                    "deity of lightning and roads",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Nature, Domain::Trickery],
                symbols: vec!["Upright mace in front of a tall boot"],
            },
            Deity {
                name: "Moradin",
                titles: vec![
                    "god of creation",
                    "god of \"dwarf-crafts\" (smithing and stonework)",
                    "god of protection",
                    "the Soulforger",
                    "Dwarf-father",
                    "All-father",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Hammer and anvil"],
            },
            Deity {
                name: "Sharindlar",
                titles: vec![
                    "goddess of healing, romantic love, and fertility",
                    "Lady of Life and Mercy",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Burning needle"],
            },
            Deity {
                name: "Vergadain",
                titles: vec![
                    "god of luck and wealth",
                    "god of thieves, luck, and chance",
                    "god of commerce and negotiation",
                    "the Merchant King",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Gold coin with the face of a dwarf"],
            },
        ]
    }
}

pub(crate) struct Duergar;

impl Pantheon for Duergar {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Deep Duerra",
                titles: vec!["goddess of conquest and psionics"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Arcana, Domain::War],
                symbols: vec!["Mind flayer skill"],
            },
            Deity {
                name: "Laduguer",
                titles: vec!["god of magic and slavery"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Arcana, Domain::Death],
                symbols: vec!["Broken arrow"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dwarven_deities() {
        insta::assert_yaml_snapshot!(Dwarven::deities());
    }

    #[test]
    fn test_duergar_deities() {
        insta::assert_yaml_snapshot!(Duergar::deities());
    }
}
