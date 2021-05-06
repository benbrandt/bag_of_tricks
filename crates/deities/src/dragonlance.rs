use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Dragonlance;

impl Deities for Dragonlance {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Paladine",
                titles: vec!["god of rulers and guardians"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Silver triangle"],
            },
            Deity {
                name: "Branchala",
                titles: vec!["god of music"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Light],
                symbols: vec!["Bard's harp"],
            },
            Deity {
                name: "Habbakuk",
                titles: vec!["god of animal life and the sea"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Blue bird"],
            },
            Deity {
                name: "Kiri-Jolith",
                titles: vec!["god of honor and war"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Bison's horns"],
            },
            Deity {
                name: "Majere",
                titles: vec!["god of meditation and order"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Copper spider"],
            },
            Deity {
                name: "Mishakal",
                titles: vec!["goddess of healing"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life],
                symbols: vec!["Blue infinity sign"],
            },
            Deity {
                name: "Solinari",
                titles: vec!["god of good magic"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![],
                symbols: vec!["White circle or sphere"],
            },
            Deity {
                name: "Gilean",
                titles: vec!["god of knowledge"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Open book"],
            },
            Deity {
                name: "Chislev",
                titles: vec!["goddess of nature"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Feather"],
            },
            Deity {
                name: "Reorx",
                titles: vec!["god of craft"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Forging hammer"],
            },
            Deity {
                name: "Shinare",
                titles: vec!["goddess of wealth and trade"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Griffon's wing"],
            },
            Deity {
                name: "Sirrion",
                titles: vec!["god of fire and change"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Multi-colored fire"],
            },
            Deity {
                name: "Zivilyn",
                titles: vec!["god of wisdom"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Great green or gold tree"],
            },
            Deity {
                name: "Lunitari",
                titles: vec!["goddess of neutral magic"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![],
                symbols: vec!["Red circle or sphere"],
            },
            Deity {
                name: "Takhisis",
                titles: vec!["goddess of night and hatred"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Black crescent"],
            },
            Deity {
                name: "Chemosh",
                titles: vec!["god of the undead"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Yellow skull"],
            },
            Deity {
                name: "Hiddukel",
                titles: vec!["god of lies and greed"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Broken merchant's scales"],
            },
            Deity {
                name: "Morgion",
                titles: vec!["god of disease and secrecy"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Hood with two red eyes"],
            },
            Deity {
                name: "Sargonnas",
                titles: vec!["god of vengeance and fire"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Stylized red condor"],
            },
            Deity {
                name: "Zeboim",
                titles: vec!["goddess of the sea and storms"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Tempest],
                symbols: vec!["Turtle shell"],
            },
            Deity {
                name: "Nuitari",
                titles: vec!["god of evil magic"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![],
                symbols: vec!["Black circle or sphere"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(Dragonlance::deities());
    }
}
