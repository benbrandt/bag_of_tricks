use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct Orc;

impl Pantheon for Orc {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Bahgtru",
                titles: vec!["god of strength"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Broken thigh bone"],
            },
            Deity {
                name: "Gruumsh",
                titles: vec!["god of storms and war"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Tempest, Domain::War],
                symbols: vec!["Unblinking eye"],
            },
            Deity {
                name: "Ilneval",
                titles: vec!["god of strategy and hordes"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Upright blood-spattered sword"],
            },
            Deity {
                name: "Luthic",
                titles: vec!["mother-goddess of fertility and healding"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Orcish rune meaning \"cave entrance\""],
            },
            Deity {
                name: "Shargaas",
                titles: vec!["god of stealth and darkness"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Red crescent moon with a skull between the moon's horns"],
            },
            Deity {
                name: "Yurtrus",
                titles: vec!["god of death and disease"],
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