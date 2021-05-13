use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Giant;

impl<'a> Deities<'a> for Giant {
    fn deities() -> Vec<Deity<'a>> {
        vec![
            Deity {
                name: "Grolantor",
                titles: vec!["hill giant god of war"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Wooden club"],
            },
            Deity {
                name: "Skoraeus Stonebones",
                titles: vec!["god of stone giants and art"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Stalactite"],
            },
            Deity {
                name: "Surtur",
                titles: vec!["god of fire giants and craft"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Knowledge, Domain::War],
                symbols: vec!["Flaming sword"],
            },
            Deity {
                name: "Thrym",
                titles: vec!["god of frost giants and strength"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["White double-bladed axe"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(Giant::deities());
    }
}
