use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct Elven;

impl Pantheon for Elven {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Aerdrie Faenya",
                titles: vec!["goddess of the sky"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Tempest, Domain::Trickery],
                symbols: vec!["Bird silhouetted against a cloud"],
            },
            Deity {
                name: "Angharradh",
                titles: vec!["triple goddess of wisdom and protection"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life],
                symbols: vec!["Triangle with three interlocking circles within"],
            },
            Deity {
                name: "Corellon Larathian",
                titles: vec!["god of art and magic"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Light],
                symbols: vec!["Crescent moon", "quarter moon", "starburst"],
            },
            Deity {
                name: "Deep Sashelas",
                titles: vec!["god of the sea"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Dolphin"],
            },
            Deity {
                name: "Erevan Ilesere",
                titles: vec!["god of mischief"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Asymmetrical eight-armed star"],
            },
            Deity {
                name: "Fenmarel Mestarine",
                titles: vec!["god of outcasts"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Two peering elven eyes"],
            },
            Deity {
                name: "Hanali Celanil",
                titles: vec!["goddess of love and beauty"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Golden heart"],
            },
            Deity {
                name: "Labelas Enoreth",
                titles: vec!["god of time, history, and philosophy"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbols: vec!["Setting sun"],
            },
            Deity {
                name: "Rillifane Rallathil",
                titles: vec!["god of nature"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Nature],
                symbols: vec!["Oak"],
            },
            Deity {
                name: "Sehanine Moonbow",
                titles: vec![
                    "goddess of the moon",
                    "goddess of divination, dreams, travel, and death",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Full moon under a moonbow", "crescent moon"],
            },
            Deity {
                name: "Shevarash",
                titles: vec!["god of vengeance"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["Broken arrow over a tear"],
            },
            Deity {
                name: "Solonor Thelandira",
                titles: vec!["god of archery"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Silver arrow with green fletching"],
            },
        ]
    }
}

pub(crate) struct Drow;

impl Pantheon for Drow {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Eilistraee",
                titles: vec!["goddess of song and moonlight"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Light, Domain::Nature],
                symbols: vec![
                    "Sword-wielding dancing drow female silhouetted against the full moon",
                ],
            },
            Deity {
                name: "Kiaransalee",
                titles: vec!["goddess of necromancy"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Arcana],
                symbols: vec!["Female drow hand wearing many silver rings"],
            },
            Deity {
                name: "Lolth",
                titles: vec!["goddess of spiders"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Spider"],
            },
            Deity {
                name: "Selvetarm",
                titles: vec!["god of warriors"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Spider over crossed sword-and-mace"],
            },
            Deity {
                name: "Vhaeraun",
                titles: vec!["god of thieves"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Black mask with blue glass lenses inset over eyes"],
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
