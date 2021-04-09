use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;

use super::{elf::Elf, human::Human, Race};
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType},
        alignment::{AlignmentInfluences, Attitude},
        attack::Resistances,
        backstory::Backstory,
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, HeightAndWeightTable,
            Size, Speed, WeightMod,
        },
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{human::Names, Name},
        proficiencies::{Proficiencies, ProficiencyOption},
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 9),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D8),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

#[derive(Deserialize, Serialize)]
pub(crate) struct HalfElf {
    /// Randomly chosen additional ability score increases
    addl_increases: Vec<AbilityScore>,
}

impl HalfElf {
    /// Generate 2 random ability increases other than charisma
    fn gen_ability_increases(rng: &mut impl Rng) -> Vec<AbilityScore> {
        AbilityScoreType::iter()
            .filter(|s| s != &AbilityScoreType::Charisma)
            .choose_multiple(rng, 2)
            .into_iter()
            .map(|t| AbilityScore(t, 1))
            .collect()
    }
}

impl AlignmentInfluences for HalfElf {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }
}

impl Backstory for HalfElf {}

impl Characteristics for HalfElf {
    const AGE_RANGE: AgeRange = AgeRange(10..=180);
    const SIZE: Size = Size::Medium;

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for HalfElf {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 38)])
    }
}

impl Features for HalfElf {
    fn features(&self) -> Vec<Feature> {
        vec![
            // Thanks to your elf blood, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Phb, 39),
            },
            // You have advantage on saving throws against being charmed, and magic can't put you to sleep.
            Feature {
                title: "Fey Ancestry",
                citation: Citation(Book::Phb, 39),
            },
        ]
    }
}

impl Languages for HalfElf {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Elvish]
    }

    fn addl_languages(&self) -> usize {
        1
    }
}

impl Name for HalfElf {
    /// First and last names can be either elven or human
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        let names = Names::gen_names(rng);
        let first_name = *[
            Elf::gen_first_name(rng, characteristics),
            Human::gen_first_name(rng, &names, characteristics),
        ]
        .choose(rng)
        .unwrap();
        let surname = *[Elf::gen_family_name(rng), Human::gen_surname(rng, &names)]
            .choose(rng)
            .unwrap();
        format!("{} {}", first_name, surname)
    }
}

impl Proficiencies for HalfElf {
    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![
            ProficiencyOption::Skill(None, 1),
            ProficiencyOption::Skill(None, 1),
        ]
    }
}

#[typetag::serde]
impl Race for HalfElf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Self {
            addl_increases: Self::gen_ability_increases(rng),
        };
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (Box::new(race), name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        let mut abilities = vec![AbilityScore(AbilityScoreType::Charisma, 2)];
        abilities.extend(self.addl_increases.clone());
        abilities
    }
}

impl Resistances for HalfElf {}

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
