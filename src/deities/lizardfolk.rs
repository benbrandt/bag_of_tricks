use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct Lizardfolk;

impl Pantheon for Lizardfolk {
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
