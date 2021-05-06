use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Lizardfolk;

impl Deities for Lizardfolk {
    fn deities() -> Vec<Deity> {
        vec![Deity {
            name: "Semuanya",
            titles: vec!["deity of survival"],
            alignment: Alignment(Attitude::Neutral, Morality::Neutral),
            domains: vec![Domain::Life],
            symbols: vec!["Egg"],
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(Lizardfolk::deities());
    }
}
