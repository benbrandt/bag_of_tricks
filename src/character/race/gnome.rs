use rand::prelude::IteratorRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        features::Feature,
    },
    citation::{Book, Citation, Citations},
};

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum GnomeSubrace {
    Forest,
    Rock,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Gnome {
    subrace: GnomeSubrace,
}

#[typetag::serde]
impl Race for Gnome {
    fn gen(rng: &mut impl Rng) -> Self {
        Self {
            subrace: GnomeSubrace::iter()
                .choose(rng)
                .unwrap_or(GnomeSubrace::Forest),
        }
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(vec![
            AbilityScore(AbilityScoreType::Intelligence, 2),
            match self.subrace {
                GnomeSubrace::Forest => AbilityScore(AbilityScoreType::Dexterity, 1),
                GnomeSubrace::Rock => AbilityScore(AbilityScoreType::Constitution, 1),
            },
        ])
    }

    fn citations(&self) -> Citations {
        let race = Citation {
            book: Book::PHB,
            page: 35,
        };
        let subrace = match self.subrace {
            GnomeSubrace::Forest | GnomeSubrace::Rock => Citation {
                book: Book::PHB,
                page: 37,
            },
        };
        Citations(vec![race, subrace])
    }

    fn features(&self) -> Vec<Feature> {
        let mut features = vec![Feature {
            title: "Ability Score Increase",
            description: "Your Intelligence score increases by 2.",
            citation: Citation {
                book: Book::PHB,
                page: 36,
            },
        }];
        features.extend(match self.subrace {
            GnomeSubrace::Forest => vec![Feature {
                title: "Ability Score Increase",
                description: "Your Dexterity score increases by 1.",
                citation: Citation {
                    book: Book::PHB,
                    page: 37,
                },
            }],
            GnomeSubrace::Rock => vec![Feature {
                title: "Ability Score Increase",
                description: "Your Constitution score increases by 1.",
                citation: Citation {
                    book: Book::PHB,
                    page: 37,
                },
            }],
        });
        features
    }
}

impl fmt::Display for Gnome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Gnome", self.subrace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_snapshot() {
        let mut rng = Pcg64::seed_from_u64(1);
        let gnome = Gnome::gen(&mut rng);
        insta::assert_yaml_snapshot!(gnome);
    }

    #[test]
    fn test_snapshot_display() {
        for subrace in GnomeSubrace::iter() {
            let gnome = Gnome { subrace };
            insta::assert_snapshot!(format!("{}", gnome));
        }
    }

    #[test]
    fn test_snapshot_abilities() {
        for subrace in GnomeSubrace::iter() {
            let gnome = Gnome { subrace };
            insta::assert_yaml_snapshot!(gnome.abilities());
        }
    }

    #[test]
    fn test_snapshot_citations() {
        for subrace in GnomeSubrace::iter() {
            let gnome = Gnome { subrace };
            insta::assert_yaml_snapshot!(gnome.citations());
        }
    }

    #[test]
    fn test_snapshot_features() {
        for subrace in GnomeSubrace::iter() {
            let gnome = Gnome { subrace };
            insta::assert_yaml_snapshot!(gnome.features());
        }
    }
}
