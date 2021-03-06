use std::fmt;

use alignment::{AlignmentInfluences, Attitude};
use attack::Resistances;
use backstory::Backstory;
use characteristics::{
    in_inches, names::Name, AgeRange, Appearance, CharacteristicDetails, Characteristics,
    HeightAndWeightTable, Size, Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use deities::{Pantheon, PantheonWeight, Pantheons};
use dice_roller::{Die, RollCmd};
use features::{Feature, Features};
use languages::{Language, LanguageType, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScore, AbilityScoreType},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption},
};
use strum::IntoEnumIterator;
use trinkets::{TrinketOption, Trinkets};

use super::{
    elf::{Elf, ElfSubrace},
    human::Human,
    Race,
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 9),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D8),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

/// SCAG p116
#[derive(Clone, Copy, Deserialize, Serialize)]
enum Variant {
    Cantrip,
    DrowMagic,
    ElfWeaponTraining,
    FleetOfFoot,
    MaskOfTheWild,
    SkillVersatility,
    Swimming,
}

impl Default for Variant {
    fn default() -> Self {
        Self::SkillVersatility
    }
}

impl Variant {
    fn gen(rng: &mut impl Rng, subrace: &ElfSubrace) -> Self {
        let mut choices = vec![Self::SkillVersatility];
        choices.extend(match subrace {
            ElfSubrace::Dark(_) => vec![Self::DrowMagic],
            ElfSubrace::High(_) => {
                vec![Self::ElfWeaponTraining, Self::Cantrip]
            }
            ElfSubrace::Sea => vec![Self::Swimming],
            ElfSubrace::Wood => vec![
                Self::ElfWeaponTraining,
                Self::FleetOfFoot,
                Self::MaskOfTheWild,
            ],
            ElfSubrace::Eladrin(_) | ElfSubrace::ShadarKai => vec![],
        });
        *choices.choose(rng).unwrap()
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct HalfElf {
    /// Randomly chosen additional ability score increases
    addl_increases: Vec<AbilityScore>,
    /// Randomly chosen subrace
    subrace: ElfSubrace,
    /// Half-Elf Variant
    variant: Variant,
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

impl Appearance for HalfElf {}

impl Backstory for HalfElf {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Elven ancestry: {} Elf", self.subrace)]
    }
}

impl Characteristics for HalfElf {
    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=180)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        let mut speeds = if matches!(self.variant, Variant::FleetOfFoot) {
            vec![Speed::Walking(35)]
        } else {
            vec![Speed::Walking(30)]
        };
        if matches!(self.variant, Variant::Swimming) {
            speeds.push(Speed::Swimming(30));
        }
        speeds
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

impl Citations for HalfElf {
    fn citations(&self) -> CitationList {
        let mut citations = vec![Citation(Book::Phb, 38)];
        if !matches!(self.variant, Variant::SkillVersatility) {
            citations.push(Citation(Book::Scag, 116))
        }
        CitationList(citations)
    }
}

impl Features for HalfElf {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
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
        ];
        features.extend(match self.variant {
            Variant::DrowMagic => vec![
                // You know the dancing lights cantrip. When you reach 3rd level, you can cast the faerie fire spell once with this trait and regain the ability to do so when you finish a long rest. When you reach 5th level, you can cast the darkness spell once with this trait and regain the ability to do so when you finish a long rest. Charisma is your spellcasting ability for these spells.
                Feature {
                    title: "Drow Magic",
                    citation: Citation(Book::Phb, 24),
                },
            ],
            Variant::MaskOfTheWild => vec![
                // You can attempt to hide even when you are only lightly obscured by foliage, heavy rain, falling snow, mist, and other natural phenomena.
                Feature {
                    title: "Mask of the Wild",
                    citation: Citation(Book::Phb, 24),
                },
            ],
            Variant::Cantrip => vec![
                // You know one cantrip of your choice from the wizard spell list. Intelligence is your spellcasting ability for it.
                Feature {
                    title: "Cantrip",
                    citation: Citation(Book::Phb, 24),
                },
            ],
            _ => vec![],
        });
        features
    }
}

impl Languages for HalfElf {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Elvish]
    }

    fn addl_languages(&self) -> (usize, Option<LanguageType>) {
        (1, None)
    }
}

impl Name for HalfElf {
    /// First and last names can be either elven or human
    fn gen_name(&self, rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        let first_name = *[
            Elf::gen_first_name(rng, characteristics),
            Human::gen_first_name(rng, characteristics),
        ]
        .choose(rng)
        .unwrap();
        let surname = *[
            Elf::gen_family_name(rng),
            Human::gen_surname(rng, characteristics),
        ]
        .choose(rng)
        .unwrap();
        format!("{} {}", first_name, surname)
    }
}

impl Pantheons for HalfElf {
    fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {
        self.subrace.addl_pantheons()
    }
}

impl PersonalityOptions for HalfElf {
    fn bonds(&self) -> Vec<String> {
        self.subrace.bonds()
    }

    fn flaws(&self) -> Vec<String> {
        self.subrace.flaws()
    }

    fn ideals(&self) -> Vec<(String, Influence)> {
        self.subrace.ideals()
    }

    fn traits(&self) -> Vec<String> {
        self.subrace.traits()
    }
}

impl Proficiencies for HalfElf {
    fn proficiencies(&self) -> Vec<Proficiency> {
        match self.variant {
            Variant::ElfWeaponTraining => Elf::weapon_training(&self.subrace),
            _ => vec![],
        }
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        if matches!(self.variant, Variant::SkillVersatility) {
            vec![ProficiencyOption::Skill(None, 2)]
        } else {
            vec![]
        }
    }
}

impl Race for HalfElf {
    fn gen(rng: &mut impl Rng) -> Self {
        let subrace = ElfSubrace::gen(rng);
        let variant = Variant::gen(rng, &subrace);
        Self {
            addl_increases: Self::gen_ability_increases(rng),
            subrace,
            variant,
        }
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        let mut abilities = vec![AbilityScore(AbilityScoreType::Charisma, 2)];
        abilities.extend(self.addl_increases.clone());
        abilities
    }
}

impl Resistances for HalfElf {}

impl Trinkets for HalfElf {
    fn trinket_options(&self) -> Vec<trinkets::TrinketOption> {
        vec![TrinketOption::Elven]
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
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", half_elf));
    }

    #[test]
    fn test_snapshot_abilities() {
        let mut rng = Pcg64::seed_from_u64(1);
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let mut rng = Pcg64::seed_from_u64(1);
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let mut rng = Pcg64::seed_from_u64(1);
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.features());
    }

    #[test]
    fn test_snapshot_addl_pantheons() {
        let mut rng = Pcg64::seed_from_u64(1);
        let half_elf = HalfElf::gen(&mut rng);
        insta::assert_yaml_snapshot!(half_elf.addl_pantheons());
    }
}
