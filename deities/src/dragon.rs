use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Dragon;

impl Deities for Dragon {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Bahamut",
                titles: vec!["god of good"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life, Domain::War],
                symbols: vec!["Dragon's head in profile"],
            },
            Deity {
                name: "Tiamat",
                titles: vec!["god of evil"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Dragon head with five claw marks"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(Dragon::deities());
    }
}
