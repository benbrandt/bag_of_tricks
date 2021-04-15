use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct Bugbear;

impl Pantheon for Bugbear {
    fn deities() -> Vec<Deity> {
        vec![Deity {
            name: "Hruggek",
            titles: vec!["god of violence"],
            alignment: Alignment(Attitude::Chaotic, Morality::Evil),
            domains: vec![Domain::War],
            symbols: vec!["Morningstar"],
        }]
    }
}

pub(crate) struct Goblin;

impl Pantheon for Goblin {
    fn deities() -> Vec<Deity> {
        vec![Deity {
            name: "Maglubiyet",
            titles: vec!["god of war"],
            alignment: Alignment(Attitude::Lawful, Morality::Evil),
            domains: vec![Domain::War],
            symbols: vec!["Bloody axe"],
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bugbear_deities() {
        insta::assert_yaml_snapshot!(Bugbear::deities());
    }

    #[test]
    fn test_goblin_deities() {
        insta::assert_yaml_snapshot!(Goblin::deities());
    }
}
