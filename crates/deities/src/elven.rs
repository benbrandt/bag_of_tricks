use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Elven;

impl<'a> Deities<'a> for Elven {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity<'a>> {
        vec![
            Deity {
                name: "Aerdrie Faenya",
                titles: vec![
                    "goddess of the sky",
                    "wild goddess of the winds and weather",
                    "patron of the avariel",
                    "goddess of air, rain, fertility, and birth",
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
                    "goddess of wisdom, growth, and protection",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life, Domain::War],
                symbols: vec!["Triangle with three interlocking circles within", "Three interlocking circles"],
            },
            Deity {
                name: "Alathrien Druanna",
                titles: vec!["deity of runes, writing, and spellcasting"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbols: vec!["A quill or glyph"],
            },
            Deity {
                name: "Alobal Lorfiril",
                titles: vec!["deity of revelry and mirth"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Trickery],
                symbols: vec!["Wine Glass"],
            },
            Deity {
                name: "Araleth Letheranil",
                titles: vec!["deity of light, stars, and revelations"],
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
                titles: vec!["deity of fire, earth, and metalwork"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Forge, Domain::Light],
                symbols: vec!["Flame between hands"],
            },
            Deity {
                name: "Deep Sashelas",
                titles: vec!["god of the sea", "lord of the sea elves and of dolphins", "god of creativity, knowledge, and the sea"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Nature, Domain::Tempest],
                symbols: vec!["Dolphin"],
            },
            Deity {
                name: "Elebrin Liothiel",
                titles: vec!["deity of abundance, gardens, and the harvest"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Acorn"],
            },
            Deity {
                name: "Erevan Ilesere",
                titles: vec!["god of mischief", "trickster-god", "god of mischief and change"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Asymmetrical eight-armed star", "Asymmetrical starburst"],
            },
            Deity {
                name: "Fenmarel Mestarine",
                titles: vec!["god of outcasts and solitude"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Nature, Domain::Trickery],
                symbols: vec!["Two peering elven eyes"],
            },
            Deity {
                name: "Gadhelyn",
                titles: vec!["deity of independence and outlawry"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Nature, Domain::Trickery],
                symbols: vec!["Leaf-shaped arrowhead"],
            },
            Deity {
                name: "Hanali Celanil",
                titles: vec![
                    "goddess of love, beauty, art, and enchantment",
                    "the Winsome Rose",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Golden heart"],
            },
            Deity {
                name: "Kirith Sotheril",
                titles: vec!["deity of divination and illusion"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Rainbow sphere"],
            },
            Deity {
                name: "Labelas Enoreth",
                titles: vec!["god of time, history, and philosophy", "the philosopher god", "god of time, history, and memory"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Knowledge, Domain::Life],
                symbols: vec!["Setting sun"],
            },
            Deity {
                name: "Melira Taralen",
                titles: vec!["deity of poetry and songs"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life, Domain::Trickery],
                symbols: vec!["Lute"],
            },
            Deity {
                name: "Mythrien Sarath",
                titles: vec!["deity of abjuration and mythals"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Forge, Domain::Knowledge],
                symbols: vec!["Row of three intertwined rings"],
            },
            Deity {
                name: "Naralis Analor",
                titles: vec!["deity of healing, suffering, and death"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Grave],
                symbols: vec!["White dove"],
            },
            Deity {
                name: "Rellavar Danuvien",
                titles: vec!["deity of winter and harsh weather"],
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
                    "god of nature, beasts, and the seasons"
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Nature],
                symbols: vec!["Oak"],
            },
            Deity {
                name: "Sarula Iliene",
                titles: vec!["deity of lakes and streams"],
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
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Grave, Domain::Knowledge, Domain::Light],
                symbols: vec!["Full moon under a moonbow", "crescent moon"],
            },
            Deity {
                name: "Shevarash",
                titles: vec!["god of vengeance", "god of vengeance, loss, and hatred"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["Broken arrow over a tear"],
            },
            Deity {
                name: "Solonor Thelandira",
                titles: vec!["god of hunting, archery, and woodcraft", "god of archery, hunting, and survival"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Nature, Domain::War],
                symbols: vec!["Silver arrow with green fletching"],
            },
            Deity {
                name: "Tarsellis Meunniduin",
                titles: vec!["deity of mountains, rivers, and wild places"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Mountain with a river"],
            },
            Deity {
                name: "Tethrin Varald\u{e9}",
                titles: vec!["deity of battle, and sword fighting"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Crossed swords beneath a quarter moon and above a full moon"],
            },
            Deity {
                name: "Vandria Gilmadrith",
                titles: vec!["deity of war, grief, justice, and vigilance"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Grave, Domain::War],
                symbols: vec!["Weeping eye"],
            },
            Deity {
                name: "Ye'Cind",
                titles: vec!["deity of music and enchantment"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Trickery],
                symbols: vec!["Recorder"],
            },
            Deity {
                name: "Zandilar",
                titles: vec!["deity of romance, lust, and dance"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Life],
                symbols: vec!["Lips"],
            },
        ]
    }
}

pub(crate) struct Drow;

impl<'a> Deities<'a> for Drow {
    fn deities() -> Vec<Deity<'a>> {
        vec![
            Deity {
                name: "Eilistraee",
                titles: vec![
                    "goddess of song, beauty, swordwork, hunting, and moonlight",
                    "goddess of freedom, moonlight, and song",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Light, Domain::Nature],
                symbols: vec![
                    "Sword-wielding dancing drow female silhouetted against the full moon",
                ],
            },
            Deity {
                name: "Ghaunadaur",
                titles: vec!["deity of oozes, slimes, and outcasts"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Purple eye with black sclera"],
            },
            Deity {
                name: "Keptolo",
                titles: vec!["deity of beauty, hedonism, and fertility"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Nature, Domain::Trickery],
                symbols: vec!["Mushroom"],
            },
            Deity {
                name: "Kiaransalee",
                titles: vec!["goddess of necromancy", "goddess of the undead"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Arcana, Domain::Death],
                symbols: vec!["Female drow hand wearing many silver rings"],
            },
            Deity {
                name: "Malyk",
                titles: vec!["deity of chaos, rebellion, and wild magic"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Tempest, Domain::Trickery],
                symbols: vec!["A flame in a tear or a multihued vortex"],
            },
            Deity {
                name: "Lolth",
                titles: vec![
                    "goddess of spiders",
                    "the Demon Queen of Spiders",
                    "primary god of drow",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery, Domain::War],
                symbols: vec!["Spider"],
            },
            Deity {
                name: "Selvetarm",
                titles: vec!["god of warriors", "god of warriors and slaughter"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Spider over crossed sword and mace"],
            },
            Deity {
                name: "Vhaeraun",
                titles: vec!["god of thieves", "god of arrogance and thieves"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery, Domain::War],
                symbols: vec!["Black mask with blue glass lenses inset over eyes"],
            },
            Deity {
                name: "Zinzerena",
                titles: vec!["deity of assassination, illusion, and lies"],
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
