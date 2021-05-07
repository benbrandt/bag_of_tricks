use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Dwarven;

impl Deities for Dwarven {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Abbathor",
                titles: vec!["god of greed", "Province: greed"],
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
                    "Province: hearth, home, truth",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Intertwined silver rings"],
            },
            Deity {
                name: "Clangeddin Silverbeard",
                titles: vec!["god of war and valor", "Province: war, strategy"],
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
                    "Province: discovery",
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
                    "Province: buried secrets",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Death, Domain::Grave, Domain::Knowledge],
                symbols: vec![
                    "Mountain silhouette with a central gemstone",
                    "gemstone in a mountain",
                ],
            },
            Deity {
                name: "Gorm Gulthyn",
                titles: vec![
                    "god of defense and vigilance",
                    "Fire Eyes",
                    "the Lord of the Bronze Mask",
                    "the protector of dwarves",
                    "Province: vigilance",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Bronze half-mask"],
            },
            Deity {
                name: "Haela Brightaxe",
                titles: vec![
                    "goddess of war-luck",
                    "patron of dwarf fighters",
                    "Province: combat prowess, luck in battle",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec![
                    "Upright sword whose blade is spiraled in flame",
                    "Upright sword with blade sheathed in flame",
                ],
            },
            Deity {
                name: "Hanseath",
                titles: vec!["Province: festivity, brewing, song"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery, Domain::War],
                symbols: vec!["Beer stein"],
            },
            Deity {
                name: "Marthammor Duin",
                titles: vec![
                    "god of wanderers",
                    "the traveler's god",
                    "patron of expatriates and guides",
                    "deity of lightning and roads",
                    "Province: explorers, wanderers, the lost",
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
                    "Primary deity of dwarves",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Forge, Domain::Knowledge],
                symbols: vec!["Hammer and anvil"],
            },
            Deity {
                name: "Muamman Duathal",
                titles: vec!["Province: storms, travel"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Tempest],
                symbols: vec!["Mace held in gauntlets"],
            },
            Deity {
                name: "Mya",
                titles: vec!["Province: clan, family, wisdom"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life],
                symbols: vec!["A faceless mother figure"],
            },
            Deity {
                name: "Roknar",
                titles: vec!["Province: lies, intrigue"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Hands filled with coins"],
            },
            Deity {
                name: "Sharindlar",
                titles: vec![
                    "goddess of healing, romantic love, and fertility",
                    "Lady of Life and Mercy",
                    "Province: healing, love",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Burning needle"],
            },
            Deity {
                name: "Thard Harr",
                titles: vec!["Province: wilderness, hunting"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Nature],
                symbols: vec!["Two clawed guantlets"],
            },
            Deity {
                name: "Tharmekh\u{fb}l",
                titles: vec!["Province: fire, forges, molten rock"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Forge, Domain::Light],
                symbols: vec!["Fiery axe"],
            },
            Deity {
                name: "Thautam",
                titles: vec!["Province: mysteries, darkness, lost treasures"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Blindfold"],
            },
            Deity {
                name: "Ulaa",
                titles: vec!["Province: mining, quarrying"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Forge],
                symbols: vec!["A miner's pick"],
            },
            Deity {
                name: "Valkauna",
                titles: vec!["Province: oaths, birth, aging, death"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Grave, Domain::Life],
                symbols: vec!["A silver ewer"],
            },
            Deity {
                name: "Vergadain",
                titles: vec![
                    "god of luck and wealth",
                    "god of thieves, luck, and chance",
                    "god of commerce and negotiation",
                    "the Merchant King",
                    "Province: luck, wealth",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec![
                    "Gold coin with the face of a dwarf",
                    "Gold coin bearing a dwarf's face",
                ],
            },
        ]
    }
}

pub(crate) struct Duergar;

impl Deities for Duergar {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Deep Duerra",
                titles: vec![
                    "goddess of conquest and psionics",
                    "Province: conquest, psionics",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Arcana, Domain::Knowledge, Domain::War],
                symbols: vec!["Mind flayer skill"],
            },
            Deity {
                name: "Laduguer",
                titles: vec!["god of magic and slavery", "Province: labor, slavery"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Arcana, Domain::Death, Domain::Forge],
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
