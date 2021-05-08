#![allow(clippy::default_trait_access)]
use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::{DamageType, Resistances};
use characteristics::{
    in_inches,
    names::{
        halfling::{FAMILY, FEMALE, MALE},
        Name,
    },
    AgeRange, Appearance, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable,
    Size, Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use dice_roller::{Die, RollCmd};
use personality::{Influence, PersonalityOptions};
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use trinkets::Trinkets;

use crate::{
    ability::{AbilityScore, AbilityScoreType},
    backstory::Backstory,
    features::{Feature, Features},
    languages::{Language, Languages},
    proficiencies::Proficiencies,
};

use super::Race;

const BONDS: &[&str] = &[
    "The safety of your village is worth any sacrifice.",
    "Nothing is more valuable than friendship and family.",
    "You are following your own path through life. No one can tell you what to do.",
    "You have a special heirloom that you never part with.",
    "You won't rob or hurt those who are weaker or less fortunate than you.",
    "No matter how small you may be, you won't back down from a bully.",
];
const FLAWS: &[&str] = &[
    "You can't resist poking your nose where it doesn't belong.",
    "You are very fidgety. Sitting still is a major challenge.",
    "You can't pass up a good time.",
    "You hate to miss a meal, and become grumpy and ill-tempered when you must.",
    "You are fascinated by shiny things and can't help \"borrowing\" them.",
    "You never settle for just one slice when you can have the whole cake.",
];
const IDEALS: &[(&str, Influence)] = &[
    ("Courage. You seek to prove that the bravest heart can be contained within the smallest of packages.", Influence::Good),
    ("Companionship. You're pretty sure you can be friends with anyone or anything.", Influence::Good),
    ("Hopeful. You will live a life of adventure and have many stories to tell.", Influence::Any),
    ("Protective. You make sure to shelter the innocent.", Influence::Lawful),
    ("Honest. Your mother told you to always tell the truth.", Influence::Lawful),
    ("Excitement. Can you steal the sleeping giant's pouch? Of course you can!", Influence::Chaotic),
];
const TRAITS: &[&str] = &[
    "You try to start every day with a smile.",
    "Why walk when you can skip?",
    "You make up songs about your friends that praise them for their bravery and intelligence.",
    "You are extremely cautious, always on the lookout for monsters and other dangers.",
    "You always see the bright side of a situation.",
    "You like to collect mementos of your travels.",
];
const REASON_FOR_ADVENTURING: &[&str] = &[
    "Peeling taters and herding goats all the time wasn't your cup of tea.",
    "You fell asleep on a raft one day and woke up near a human city. You were so thrilled with the strange sights and tasty food that you never turned back.",
    "What started off as simple pumpkin pillaging from nearby farms turned into your becoming a wandering rogue for hire.",
    "You talked to a nice faerie in the woods, and all of a sudden you were a thousand miles from home.",
    "Your village elder told you so many stories about being a rogue in an adventuring party that you couldn't resist the urge to try doing it yourself.",
    "A friend dared you to jump on the back of a sleeping horse, which turned out to be a pegasus, and your life hasn't slowed down since.",
];

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(2, 7),
    base_weight: 35,
    height_mod: RollCmd(2, Die::D4),
    weight_mod: WeightMod::Fixed(1),
};

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum StoutVariant {
    Stout,
    Strongheart,
}

impl Default for StoutVariant {
    fn default() -> Self {
        Self::Stout
    }
}

#[derive(Debug, Deserialize, EnumIter, PartialEq, Serialize)]
enum HalflingSubrace {
    Ghostwise,
    Lightfoot,
    Stout(StoutVariant),
}

impl HalflingSubrace {
    fn gen(rng: &mut impl Rng) -> Self {
        let subrace = Self::iter().choose(rng).unwrap();
        match subrace {
            Self::Stout(_) => Self::Stout(StoutVariant::iter().choose(rng).unwrap()),
            Self::Ghostwise | Self::Lightfoot => subrace,
        }
    }
}

impl fmt::Display for HalflingSubrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ghostwise => write!(f, "Ghostwise"),
            Self::Lightfoot => write!(f, "Lightfoot"),
            Self::Stout(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Halfling {
    reason_for_adventuring: String,
    /// Randomly chosen subrace
    subrace: HalflingSubrace,
}

impl AlignmentInfluences for Halfling {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Lawful]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Good]
    }
}

impl Appearance for Halfling {}

impl Backstory for Halfling {
    fn backstory(&self) -> Vec<String> {
        vec![format!(
            "Reason for Adventuring: {}",
            self.reason_for_adventuring
        )]
    }
}

impl Characteristics for Halfling {
    const SIZE: Size = Size::Small;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=150)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(25)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Halfling {
    fn citations(&self) -> CitationList {
        let race = Citation(Book::Phb, 26);
        let subrace = match self.subrace {
            HalflingSubrace::Ghostwise => Citation(Book::Scag, 110),
            HalflingSubrace::Lightfoot | HalflingSubrace::Stout(StoutVariant::Stout) => {
                Citation(Book::Phb, 28)
            }
            HalflingSubrace::Stout(StoutVariant::Strongheart) => Citation(Book::Scag, 109),
        };
        CitationList(vec![race, subrace])
    }
}

impl Features for Halfling {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            // You have advantage on saving throws against being frightened.
            Feature {
                title: "Brave",
                citation: Citation(Book::Phb, 28),
            },
            // You can move through the space of any creature that is of a size larger than yours.
            Feature {
                title: "Halfling Nimbleness",
                citation: Citation(Book::Phb, 28),
            },
        ];
        features.push(match self.subrace {
            // You can speak telepathically to any creature within 30 feet of you. The creature understands you only if the two of you share a language. You can speak telepathically in this way to one creature at a time.
            HalflingSubrace::Ghostwise => Feature {
                title: "Silent Speech",
                citation: Citation(Book::Scag, 110),
            },
            // You can attempt to hide even when you are obscured only by a creature that is at least one size larger than you.
            HalflingSubrace::Lightfoot => Feature {
                title: "Naturally Stealthy",
                citation: Citation(Book::Phb, 28),
            },
            // You have advantage on saving throws against poison, and you have resistance against poison damage.
            HalflingSubrace::Stout(_) => Feature {
                title: "Stout Resilience",
                citation: Citation(Book::Phb, 28),
            },
        });
        features
    }
}

impl Languages for Halfling {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Halfling]
    }
}

impl Name for Halfling {
    fn gen_name(
        rng: &mut impl Rng,
        CharacteristicDetails { gender, .. }: &CharacteristicDetails,
    ) -> String {
        let first_names = match gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        };
        format!(
            "{} {}",
            first_names.choose(rng).unwrap(),
            FAMILY.choose(rng).unwrap(),
        )
    }
}

impl PersonalityOptions for Halfling {
    fn bonds(&self) -> Vec<String> {
        BONDS.iter().map(|&s| s.to_string()).collect()
    }

    fn flaws(&self) -> Vec<String> {
        FLAWS.iter().map(|&s| s.to_string()).collect()
    }

    fn ideals(&self) -> Vec<(String, Influence)> {
        IDEALS.iter().map(|&(s, i)| (s.to_string(), i)).collect()
    }

    fn traits(&self) -> Vec<String> {
        TRAITS.iter().map(|&s| s.to_string()).collect()
    }
}

impl Proficiencies for Halfling {}

#[typetag::serde]
impl Race for Halfling {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            reason_for_adventuring: (*REASON_FOR_ADVENTURING.choose(rng).unwrap()).to_string(),
            subrace: HalflingSubrace::gen(rng),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            match self.subrace {
                HalflingSubrace::Ghostwise => AbilityScore(AbilityScoreType::Wisdom, 1),
                HalflingSubrace::Lightfoot => AbilityScore(AbilityScoreType::Charisma, 1),
                HalflingSubrace::Stout(_) => AbilityScore(AbilityScoreType::Constitution, 1),
            },
        ]
    }
}

impl Resistances for Halfling {
    fn resistances(&self) -> Vec<DamageType> {
        match self.subrace {
            HalflingSubrace::Ghostwise | HalflingSubrace::Lightfoot => vec![],
            HalflingSubrace::Stout(_) => vec![DamageType::Poison],
        }
    }
}

impl Trinkets for Halfling {}

impl fmt::Display for Halfling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Halfling", self.subrace)
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
        let halfling = Halfling::gen(&mut rng);
        insta::assert_yaml_snapshot!(halfling);
    }

    #[test]
    fn test_snapshot_display() {
        insta::assert_snapshot!(HalflingSubrace::iter()
            .map(|subrace| format!(
                "{}",
                Halfling {
                    subrace,
                    reason_for_adventuring: String::new()
                }
            ))
            .collect::<Vec<String>>()
            .join("\n\n"));
    }

    #[test]
    fn test_snapshot_abilities() {
        insta::assert_yaml_snapshot!(HalflingSubrace::iter()
            .map(|subrace| (Halfling {
                subrace,
                reason_for_adventuring: String::new()
            })
            .abilities())
            .collect::<Vec<Vec<AbilityScore>>>());
    }

    #[test]
    fn test_snapshot_citations() {
        insta::assert_yaml_snapshot!(HalflingSubrace::iter()
            .map(|subrace| (Halfling {
                subrace,
                reason_for_adventuring: String::new()
            })
            .citations())
            .collect::<Vec<CitationList>>());
    }

    #[test]
    fn test_snapshot_features() {
        insta::assert_yaml_snapshot!(HalflingSubrace::iter()
            .map(|subrace| (Halfling {
                subrace,
                reason_for_adventuring: String::new()
            })
            .features())
            .collect::<Vec<Vec<Feature>>>());
    }
}
