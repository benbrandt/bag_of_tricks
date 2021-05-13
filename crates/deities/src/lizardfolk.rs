use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Lizardfolk;

impl<'a> Deities<'a> for Lizardfolk {
    fn deities() -> Vec<Deity<'a>> {
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
