use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Elven;

impl Deities for Elven {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Aerdrie Faenya",
                titles: vec![
                    "goddess of the sky",
                    "wild goddess of the winds and weather",
                    "patron of the avariel",
                    "Province: air, rain, fertility, birth",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Tempest, Domain::Trickery],
                symbols: vec!["Bird silhouetted against a cloud"],
            },
            Deity {
                name: "Angharradh",
                titles: vec![
                    "triple goddess of wisdom and protection",
                    "the fierce mother-protector of the elf people",
                    "Province: wisdom, growth, protection",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life, Domain::War],
                symbols: vec!["Triangle with three interlocking circles within", "Three interlocking circles"],
            },
            Deity {
                name: "Alathrien Druanna",
                titles: vec!["Province: runes, writing, spellcasting"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbols: vec!["A quill or glyph"],
            },
            Deity {
                name: "Alobal Lorfiril",
                titles: vec!["Province: revelry, mirth"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Trickery],
                symbols: vec!["Wine Glass"],
            },
            Deity {
                name: "Araleth Letheranil",
                titles: vec!["Province: light, stars, revelations"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Light],
                symbols: vec!["Shaft of light"],
            },
            Deity {
                name: "Corellon Larathian",
                titles: vec![
                    "god of art and magic",
                    "god of elves, magic, poetry, rulership, and warcraft",
                    "Primary god of elves",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Life, Domain::Light, Domain::War],
                symbols: vec!["Crescent moon", "quarter moon", "starburst"],
            },
            Deity {
                name: "Darahl Tilvenar",
                titles: vec!["Province: fire, earth, metalwork"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Forge, Domain::Light],
                symbols: vec!["Flame between hands"],
            },
            Deity {
                name: "Deep Sashelas",
                titles: vec!["god of the sea", "lord of the sea elves and of dolphins", "Province: creativity, knowledge, sea"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Nature, Domain::Tempest],
                symbols: vec!["Dolphin"],
            },
            Deity {
                name: "Elebrin Liothiel",
                titles: vec!["Province: abundance, gardens, the harvest"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Acorn"],
            },
            Deity {
                name: "Erevan Ilesere",
                titles: vec!["god of mischief", "trickster-god", "Province: mischief, change"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Asymmetrical eight-armed star", "Asymmetrical starburst"],
            },
            Deity {
                name: "Fenmarel Mestarine",
                titles: vec!["god of outcasts and solitude", "Province: solitude, outcasts"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Nature, Domain::Trickery],
                symbols: vec!["Two peering elven eyes"],
            },
            Deity {
                name: "Gadhelyn",
                titles: vec!["Province: independence, outlawry"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Nature, Domain::Trickery],
                symbols: vec!["Leaf-shaped arrowhead"],
            },
            Deity {
                name: "Hanali Celanil",
                titles: vec![
                    "goddess of love, beauty, art, and enchantment",
                    "the Winsome Rose",
                    "Province: love, beauty, the arts",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Golden heart"],
            },
            Deity {
                name: "Kirith Sotheril",
                titles: vec!["Province: divination, illusion"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Rainbow sphere"],
            },
            Deity {
                name: "Labelas Enoreth",
                titles: vec!["god of time, history, and philosophy", "the philosopher god", "Province: time, history, memory"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Knowledge, Domain::Life],
                symbols: vec!["Setting sun"],
            },
            Deity {
                name: "Melira Taralen",
                titles: vec!["Province: poetry, songs"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life, Domain::Trickery],
                symbols: vec!["Lute"],
            },
            Deity {
                name: "Mythrien Sarath",
                titles: vec!["Province: abjuration, mythals"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Forge, Domain::Knowledge],
                symbols: vec!["Row of three intertwined rings"],
            },
            Deity {
                name: "Naralis Analor",
                titles: vec!["Province: healing, suffering, death"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Grave],
                symbols: vec!["White dove"],
            },
            Deity {
                name: "Rellavar Danuvien",
                titles: vec!["Province: winter, harsh weather"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Tempest],
                symbols: vec!["Spear between two circles"],
            },
            Deity {
                name: "Rillifane Rallathil",
                titles: vec![
                    "god of nature",
                    "god of the woodlands and wild places",
                    "father of the wood elves",
                    "protector of druids",
                    "Province: nature, beasts, the seasons"
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Nature],
                symbols: vec!["Oak"],
            },
            Deity {
                name: "Sarula Iliene",
                titles: vec!["Province: lakes, streams"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Tempest, Domain::Trickery],
                symbols: vec!["Three lines symbolizing waves"],
            },
            Deity {
                name: "Sehanine Moonbow",
                titles: vec![
                    "goddess of the moon",
                    "goddess of divination, dreams, travel, and death",
                    "goddess of all life's mysteries, including mysticism, prophecy, death, and dreams",
                    "the Moonlit Mystery",
                    "Province: dreams, death, travel",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Grave, Domain::Knowledge, Domain::Light],
                symbols: vec!["Full moon under a moonbow", "crescent moon"],
            },
            Deity {
                name: "Shevarash",
                titles: vec!["god of vengeance", "Province: vengeance, loss, hatred"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["Broken arrow over a tear"],
            },
            Deity {
                name: "Solonor Thelandira",
                titles: vec!["god of hunting, archery, and woodcraft", "Province: archery, hunting, survival"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Nature, Domain::War],
                symbols: vec!["Silver arrow with green fletching"],
            },
            Deity {
                name: "Tarsellis Meunniduin",
                titles: vec!["Province: mountains, rivers, wild places"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Mountain with a river"],
            },
            Deity {
                name: "Tethrin Varald\u{e9}",
                titles: vec!["Province: battle, sword fighting"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Crossed swords beneath a quarter moon and above a full moon"],
            },
            Deity {
                name: "Vandria Gilmadrith",
                titles: vec!["Province: war, grief, justice, vigilance"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Grave, Domain::War],
                symbols: vec!["Weeping eye"],
            },
            Deity {
                name: "Ye'Cind",
                titles: vec!["Province: music, enchantment"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Trickery],
                symbols: vec!["Recorder"],
            },
            Deity {
                name: "Zandilar",
                titles: vec!["Province: romance, lust, dance"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Life],
                symbols: vec!["Lips"],
            },
        ]
    }
}

pub(crate) struct Drow;

impl Deities for Drow {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Eilistraee",
                titles: vec![
                    "goddess of song, beauty, swordwork, hunting, and moonlight",
                    "Province: freedom, moonlight, song",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Light, Domain::Nature],
                symbols: vec![
                    "Sword-wielding dancing drow female silhouetted against the full moon",
                ],
            },
            Deity {
                name: "Ghaunadaur",
                titles: vec!["Province: oozes, slimes, outcasts"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Purple eye with black sclera"],
            },
            Deity {
                name: "Keptolo",
                titles: vec!["Province: beauty, hedonism, fertility"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Nature, Domain::Trickery],
                symbols: vec!["Mushroom"],
            },
            Deity {
                name: "Kiaransalee",
                titles: vec![
                    "goddess of necromancy",
                    "goddess of the undead",
                    "Province: necromancy",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Arcana, Domain::Death],
                symbols: vec!["Female drow hand wearing many silver rings"],
            },
            Deity {
                name: "Malyk",
                titles: vec!["Province: chaos, rebellion, wild magic"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Tempest, Domain::Trickery],
                symbols: vec!["A flame in a tear or a multihued vortex"],
            },
            Deity {
                name: "Lolth",
                titles: vec![
                    "goddess of spiders",
                    "the Demon Queen of Spiders",
                    "Province: primary god of drow, spiders",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery, Domain::War],
                symbols: vec!["Spider"],
            },
            Deity {
                name: "Selvetarm",
                titles: vec!["god of warriors", "Province: warriors, slaughter"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Spider over crossed sword and mace"],
            },
            Deity {
                name: "Vhaeraun",
                titles: vec!["god of thieves", "Province: arrogance, thieves"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery, Domain::War],
                symbols: vec!["Black mask with blue glass lenses inset over eyes"],
            },
            Deity {
                name: "Zinzerena",
                titles: vec!["Province: assassination, illusion, lies"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Shortsword draped with cloth"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elven_deities() {
        insta::assert_yaml_snapshot!(Elven::deities());
    }

    #[test]
    fn test_drow_deities() {
        insta::assert_yaml_snapshot!(Drow::deities());
    }
}
