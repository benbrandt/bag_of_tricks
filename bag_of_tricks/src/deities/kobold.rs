use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct Kobold;

impl Pantheon for Kobold {
    fn deities() -> Vec<Deity> {
        vec![Deity {
            name: "Kurtulmak",
            titles: vec!["god of war and mining"],
            alignment: Alignment(Attitude::Lawful, Morality::Evil),
            domains: vec![Domain::War],
            symbols: vec!["Gnome skull"],
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(Kobold::deities());
    }
}
