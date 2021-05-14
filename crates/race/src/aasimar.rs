use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::{DamageType, Resistances};
use backstory::Backstory;
use characteristics::{
    in_inches, names::Name, AgeRange, Appearance, CharacteristicDetails, Characteristics,
    HeightAndWeightTable, Size, Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use deities::Pantheons;
use dice_roller::{Die, RollCmd};
use features::{Feature, Features};
use languages::{Language, Languages};
use personality::PersonalityOptions;
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScore, AbilityScoreType},
    proficiencies::Proficiencies,
};
use strum::{Display, EnumIter, IntoEnumIterator};
use trinkets::Trinkets;

use super::{human::Human, Race};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 8),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D10),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum AasimarSubrace {
    Fallen,
    Protector,
    Scourge,
}

const GUIDE_NAME: &[&str] = &[
    "Tadriel",
    "Myllandra",
    "Seraphina",
    "Galladia",
    "Mykiel",
    "Valandras",
];

const GUIDE_NATURE: &[&str] = &[
    "Bookish and lecturing",
    "Compassionate and hopeful",
    "Practical and lighthearted",
    "Fierce and vengeful",
    "Stern and judgmental",
    "Kind and parental",
];

#[derive(Deserialize, Serialize)]
struct AngelicGuide {
    name: String,
    nature: String,
}

impl AngelicGuide {
    fn gen(rng: &mut impl Rng) -> Self {
        Self {
            name: (*GUIDE_NAME.choose(rng).unwrap()).to_string(),
            nature: (*GUIDE_NATURE.choose(rng).unwrap()).to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Aasimar {
    /// Angelic guide's name and nature
    guide: AngelicGuide,
    /// Randomly chosen subrace
    subrace: AasimarSubrace,
}

impl AlignmentInfluences for Aasimar {
    fn attitude(&self) -> Vec<Attitude> {
        match self.subrace {
            AasimarSubrace::Fallen => vec![Attitude::Neutral],
            AasimarSubrace::Protector | AasimarSubrace::Scourge => vec![],
        }
    }

    fn morality(&self) -> Vec<Morality> {
        match self.subrace {
            AasimarSubrace::Fallen => vec![Morality::Evil, Morality::Neutral],
            AasimarSubrace::Protector | AasimarSubrace::Scourge => vec![Morality::Good],
        }
    }
}

impl Appearance for Aasimar {}

impl Backstory for Aasimar {
    fn backstory(&self) -> Vec<String> {
        vec![
            format!(
                "{}Angelic Guide: {}",
                if self.subrace == AasimarSubrace::Fallen {
                    "Former "
                } else {
                    ""
                },
                self.guide.name
            ),
            format!("Guide's Nature: {}", self.guide.nature),
        ]
    }
}

impl Characteristics for Aasimar {
    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=160)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }

    fn get_size(&self) -> Size {
        Size::Medium
    }

    fn has_human_ancestry(&self) -> bool {
        true
    }
}

impl Citations for Aasimar {
    fn citations(&self) -> CitationList {
        let race = Citation(Book::Vgtm, 104);
        let subrace = Citation(Book::Vgtm, 105);
        CitationList(vec![race, subrace])
    }
}

impl Features for Aasimar {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            // Blessed with a radiant soul, your vision can easily cut through darkness. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Vgtm, 105),
            },
            // As an action, you can touch a creature and cause it to regain a number of hit points equal to your level. Once you use this trait, you can't use it again until you finish a long rest.
            Feature {
                title: "Healing Hands",
                citation: Citation(Book::Vgtm, 105),
            },
            // You know the light cantrip. Charisma is your spellcasting ability for it.
            Feature {
                title: "Light Bearer",
                citation: Citation(Book::Vgtm, 105),
            },
        ];
        features.push(match self.subrace {
            // Starting at 3rd level, you can use your action to unleash the divine energy within yourself, causing your eyes to turn into pools of darkness and two skeletal, ghostly, flightless wings to sprout from your back. The instant you transform, other creatures within 10 feet of you that can see you must each succeed on a Charisma saving throw (DC 8 + your proficiency bonus + your Charisma modifier) or become frightened of you until the end of your next turn.
            // Your transformation lasts for 1 minute or until you end it as a bonus action. During it, once on each of your turns, you can deal extra necrotic damage to one target when you deal damage to it with an attack or a spell. The extra necrotic damage equals your level.
            // Once you use this trait, you can't use it again until you finish a long rest.
            AasimarSubrace::Fallen => Feature {
                title: "Necrotic Shroud",
                citation: Citation(Book::Vgtm, 105),
            },
            // Starting at 3rd level, you can use your action to unleash the divine energy within yourself, causing your eyes to glimmer and two luminous, incorporeal wings to sprout from your back.
            // Your transformation lasts for 1 minute or until you end it as a bonus action. During it, you have a flying speed of 30 feet, and once on each of your turns, you can deal extra radiant damage to one target when you deal damage to it with an attack or a spell. The extra radiant damage equals your level.
            // Once you use this trait, you can't use it again until you finish a long rest.
            AasimarSubrace::Protector => Feature {
                title: "Radiant Soul",
                citation: Citation(Book::Vgtm, 105),
            },
            // Starting at 3rd level, you can use your action to unleash the divine energy within yourself, causing a searing light to radiate from you, pour out of your eyes and mouth, and threaten to char you.
            // Your transformation lasts for 1 minute or until you end it as a bonus action. During it, you shed bright light in a 10-foot radius and dim light for an additional 10 feet, and at the end of each of your turns, you and each creature within 10 feet of you take radiant damage equal to half your level (rounded up). In addition, once on each of your turns, you can deal extra radiant damage to one target when you deal damage to it with an attack or a spell. The extra radiant damage equals your level.
            // Once you use this trait, you can't use it again until you finish a long rest.
            AasimarSubrace::Scourge => Feature {
                title: "Radiant Consumption",
                citation: Citation(Book::Vgtm, 105),
            },
        });
        features
    }
}

impl Languages for Aasimar {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Celestial]
    }
}

impl Name for Aasimar {
    fn gen_name(&self, rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        format!(
            "{} {}",
            Human::gen_first_name(rng, characteristics),
            Human::gen_surname(rng, characteristics),
        )
    }
}

impl Pantheons for Aasimar {}

impl PersonalityOptions for Aasimar {}

impl Proficiencies for Aasimar {}

#[typetag::serde]
impl Race for Aasimar {
    fn gen(rng: &mut impl Rng) -> Self {
        Self {
            guide: AngelicGuide::gen(rng),
            subrace: AasimarSubrace::iter().choose(rng).unwrap(),
        }
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Charisma, 2),
            match self.subrace {
                AasimarSubrace::Fallen => AbilityScore(AbilityScoreType::Strength, 1),
                AasimarSubrace::Protector => AbilityScore(AbilityScoreType::Wisdom, 1),
                AasimarSubrace::Scourge => AbilityScore(AbilityScoreType::Constitution, 1),
            },
        ]
    }
}

impl Resistances for Aasimar {
    fn resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Necrotic, DamageType::Radiant]
    }
}

impl Trinkets for Aasimar {}

impl fmt::Display for Aasimar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Aasimar", self.subrace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use characteristics::Gender;
    use rand::SeedableRng;
    use rand_pcg::Pcg64;

    #[test]
    fn test_snapshot() {
        let mut rng = Pcg64::seed_from_u64(1);
        let aasimar = Aasimar::gen(&mut rng);
        insta::assert_yaml_snapshot!(aasimar);
    }

    #[test]
    fn test_gen_characteristics() {
        let mut rng = Pcg64::seed_from_u64(1);
        let aasimar = Aasimar::gen(&mut rng);
        insta::assert_yaml_snapshot!(aasimar.gen_characteristics(&mut rng));
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let aasimar = Aasimar::gen(&mut rng);
        insta::assert_display_snapshot!(aasimar);
    }

    #[test]
    fn test_attitude() {
        let mut rng = Pcg64::seed_from_u64(1);
        insta::assert_yaml_snapshot!(AasimarSubrace::iter()
            .map(|subrace| Aasimar {
                guide: AngelicGuide::gen(&mut rng),
                subrace
            }
            .attitude())
            .collect::<Vec<Vec<Attitude>>>());
    }

    #[test]
    fn test_morality() {
        let mut rng = Pcg64::seed_from_u64(1);
        insta::assert_yaml_snapshot!(AasimarSubrace::iter()
            .map(|subrace| Aasimar {
                guide: AngelicGuide::gen(&mut rng),
                subrace
            }
            .morality())
            .collect::<Vec<Vec<Morality>>>())
    }

    #[test]
    fn test_backstory() {
        let mut rng = Pcg64::seed_from_u64(1);
        insta::assert_yaml_snapshot!(AasimarSubrace::iter()
            .map(|subrace| Aasimar {
                guide: AngelicGuide::gen(&mut rng),
                subrace
            }
            .backstory())
            .collect::<Vec<Vec<String>>>());
    }

    #[test]
    fn test_characteristics() {
        let aasimar = Aasimar {
            guide: AngelicGuide {
                name: "Galladia".to_string(),
                nature: "Bookish".to_string(),
            },
            subrace: AasimarSubrace::Fallen,
        };
        assert_eq!(aasimar.get_base_speeds(), vec![Speed::Walking(30)]);
        assert_eq!(aasimar.get_height_and_weight_table(), &HEIGHT_AND_WEIGHT);
    }

    #[test]
    fn test_snapshot_citations() {
        let aasimar = Aasimar {
            guide: AngelicGuide {
                name: "Galladia".to_string(),
                nature: "Bookish".to_string(),
            },
            subrace: AasimarSubrace::Fallen,
        };
        insta::assert_yaml_snapshot!(aasimar.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        insta::assert_yaml_snapshot!(AasimarSubrace::iter()
            .map(|subrace| Aasimar {
                guide: AngelicGuide::gen(&mut rng),
                subrace
            }
            .features())
            .collect::<Vec<Vec<Feature>>>())
    }

    #[test]
    fn test_snapshot_languages() {
        let aasimar = Aasimar {
            guide: AngelicGuide {
                name: "Galladia".to_string(),
                nature: "Bookish".to_string(),
            },
            subrace: AasimarSubrace::Fallen,
        };
        insta::assert_yaml_snapshot!(aasimar.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let aasimar = Aasimar {
            guide: AngelicGuide {
                name: "Galladia".to_string(),
                nature: "Bookish".to_string(),
            },
            subrace: AasimarSubrace::Fallen,
        };
        let characteristics_1 = aasimar.gen_characteristics(&mut rng);
        let characteristics_2 = aasimar.gen_characteristics(&mut rng);
        let female_name = aasimar.gen_name(
            &mut rng,
            &CharacteristicDetails {
                gender: Gender::Female,
                ..characteristics_1
            },
        );
        let male_name = aasimar.gen_name(
            &mut rng,
            &CharacteristicDetails {
                gender: Gender::Male,
                ..characteristics_2
            },
        );
        insta::assert_yaml_snapshot!([female_name, male_name]);
    }

    #[test]
    fn test_snapshot_abilities() {
        let mut rng = Pcg64::seed_from_u64(1);
        insta::assert_yaml_snapshot!(AasimarSubrace::iter()
            .map(|subrace| Aasimar {
                guide: AngelicGuide::gen(&mut rng),
                subrace
            }
            .abilities())
            .collect::<Vec<Vec<AbilityScore>>>());
    }

    #[test]
    fn test_resistances() {
        let aasimar = Aasimar {
            guide: AngelicGuide {
                name: "Galladia".to_string(),
                nature: "Bookish".to_string(),
            },
            subrace: AasimarSubrace::Fallen,
        };
        insta::assert_yaml_snapshot!(aasimar.resistances());
    }
}
