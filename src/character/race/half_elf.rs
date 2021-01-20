use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;

use super::{elf::Elf, human::Human, Race};
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores, Skill},
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, HeightAndWeightTable,
            Size, WeightMod,
        },
        features::{Feature, Features},
        languages::Language,
        names::{human::Names, Name},
        proficiencies::Proficiency,
    },
    citation::{Book, Citation, Citations},
    dice_roller::{Die, RollCmd},
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 9),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D8),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};
const BASE_LANGUAGES: &[Language] = &[Language::Common, Language::Elvish];

#[derive(Deserialize, Serialize)]
pub(crate) struct HalfElf {
    addl_increases: Vec<AbilityScore>,
    addl_proficiencies: Vec<Proficiency>,
    extra_language: Language,
}

impl HalfElf {
    fn gen_ability_increases(rng: &mut impl Rng) -> Vec<AbilityScore> {
        AbilityScoreType::iter()
            .filter(|s| s != &AbilityScoreType::Charisma)
            .choose_multiple(rng, 2)
            .into_iter()
            .map(|t| AbilityScore(t, 1))
            .collect()
    }

    fn gen_extra_language(rng: &mut impl Rng) -> Language {
        Language::iter()
            .filter(|l| !BASE_LANGUAGES.contains(l))
            .choose(rng)
            .unwrap()
    }

    fn gen_proficiences(rng: &mut impl Rng) -> Vec<Proficiency> {
        // TODO: Filter out existing proficiencies
        Skill::iter()
            .choose_multiple(rng, 2)
            .into_iter()
            .map(Proficiency::Skill)
            .collect()
    }
}

impl Characteristics for HalfElf {
    const AGE_RANGE: AgeRange = AgeRange(1..=180);
    const SIZE: Size = Size::Medium;

    fn get_base_speed(&self) -> u8 {
        30
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Features for HalfElf {
    fn features(&self) -> Vec<Feature> {
        vec![
            Feature {
                title: "Alignment",
                description: "Half-elves share the chaotic bent of their elven heritage. They value both personal freedom and creative expression, demonstrating neither love of leaders nor desire for followers. They chafe at rules, resent others' demands, and sometimes prove unreliable, or at least unpredictable.",
                citation: Citation {
                    book: Book::PHB,
                    page: 39,
                },
            },
            Feature {
                title: "Darkvision",
                description: "Thanks to your elf blood, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
                citation: Citation {
                    book: Book::PHB,
                    page: 39,
                },
            },
            Feature {
                title: "Fey Ancestry",
                description: "You have advantage on saving throws against being charmed, and magic can't put you to sleep.",
                citation: Citation {
                    book: Book::PHB,
                    page: 39,
                },
            },
        ]
    }
}

impl Name for HalfElf {
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        let names = Names::gen_names(rng);
        let first_name = *[
            Elf::gen_first_name(rng, characteristics),
            Human::gen_first_name(rng, &names, characteristics),
        ]
        .iter()
        .choose(rng)
        .unwrap();
        let surname = *[Elf::gen_family_name(rng), Human::gen_surname(rng, &names)]
            .iter()
            .choose(rng)
            .unwrap();
        format!("{} {}", first_name, surname)
    }
}

#[typetag::serde]
impl Race for HalfElf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Self {
            addl_increases: Self::gen_ability_increases(rng),
            addl_proficiencies: Self::gen_proficiences(rng),
            extra_language: Self::gen_extra_language(rng),
        };
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (Box::new(race), name, characteristics)
    }

    fn abilities(&self) -> AbilityScores {
        let mut abilities = vec![AbilityScore(AbilityScoreType::Charisma, 2)];
        abilities.extend(self.addl_increases.clone());
        AbilityScores(abilities)
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 38,
        }])
    }

    fn languages(&self) -> Vec<Language> {
        let mut languages = BASE_LANGUAGES.to_vec();
        languages.push(self.extra_language);
        languages
    }

    fn proficiencies(&self) -> Vec<Proficiency> {
        self.addl_proficiencies.clone()
    }
}

impl fmt::Display for HalfElf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Half-Elf")
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
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (half_elf, _name, _characteristics) = HalfElf::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", half_elf));
    }

    #[test]
    fn test_snapshot_abilities() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (half_elf, _name, _characteristics) = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (half_elf, _name, _characteristics) = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (half_elf, _name, _characteristics) = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.features());
    }
}
