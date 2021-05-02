use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Orc;

impl Deities for Orc {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Bahgtru",
                titles: vec!["god of pure, brute strength", "son of Gruumsh"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Broken thigh bone"],
            },
            Deity {
                name: "Gruumsh",
                titles: vec![
                    "god of storms and war",
                    "god of conquest, strength, and survival",
                    "One-Eye",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Tempest, Domain::War],
                symbols: vec!["Unblinking eye"],
            },
            Deity {
                name: "Ilneval",
                titles: vec!["god of strategy and hordes", "War Master"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Upright blood-spattered sword"],
            },
            Deity {
                name: "Luthic",
                titles: vec![
                    "mother-goddess of fertility and healding",
                    "goddess of fecundity, caverns, and witchery",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Orcish rune meaning \"cave entrance\""],
            },
            Deity {
                name: "Shargaas",
                titles: vec!["god of darkness, night, and stealth", "the Night Lord"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Red crescent moon with a skull between the moon's horns"],
            },
            Deity {
                name: "Yurtrus",
                titles: vec![
                    "god of death and disease",
                    "god of plagues and death",
                    "the White-Handed",
                    "Lord of Maggots",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["White hand, palm outward"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(Orc::deities());
    }
}
