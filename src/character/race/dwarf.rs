use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType},
        alignment::{AlignmentInfluences, Attitude, Morality},
        attack::{DamageType, Resistances},
        characteristics::{
            AgeRange, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable, Size,
        },
        equipment::{
            armor::ArmorType,
            tools::{ArtisansTools, Tool},
            weapons::WeaponType,
        },
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{
            dwarf::{CLAN, FEMALE, MALE},
            Name,
        },
        proficiencies::{Proficiencies, Proficiency, ProficiencyOption, WeaponProficiency},
    },
    citation::{Book, Citation, CitationList, Citations},
};

mod height_and_weight {
    use crate::{
        character::characteristics::{in_inches, HeightAndWeightTable, WeightMod},
        dice_roller::{Die, RollCmd},
    };

    pub(crate) const HILL: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(3, 8),
        base_weight: 115,
        height_mod: RollCmd(2, Die::D4),
        weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
    };
    pub(crate) const MOUNTAIN: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 1),
        base_weight: 130,
        height_mod: RollCmd(2, Die::D4),
        weight_mod: WeightMod::Roll(RollCmd(2, Die::D6)),
    };
}

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum DwarfSubrace {
    Hill,
    Mountain,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Dwarf {
    subrace: DwarfSubrace,
}

impl AlignmentInfluences for Dwarf {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Lawful]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Good]
    }
}

impl Characteristics for Dwarf {
    const AGE_RANGE: AgeRange = AgeRange(1..=350);
    const SIZE: Size = Size::Medium;

    fn get_base_speed(&self) -> u8 {
        25
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        match self.subrace {
            DwarfSubrace::Hill => &height_and_weight::HILL,
            DwarfSubrace::Mountain => &height_and_weight::MOUNTAIN,
        }
    }
}

impl Citations for Dwarf {
    fn citations(&self) -> CitationList {
        let race = Citation(Book::PHB, 18);
        let subrace = match self.subrace {
            DwarfSubrace::Hill | DwarfSubrace::Mountain => Citation(Book::PHB, 20),
        };
        CitationList(vec![race, subrace])
    }
}

impl Features for Dwarf {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            // Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::PHB, 20),
            },
            // You have advantage on saving throws against poison, and you have resistance against poison damage (explained in the \"Combat\" section).
            Feature {
                title: "Dwarven Resilience",
                citation: Citation(Book::PHB, 20),
            },
            // Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus.
            Feature {
                title: "Stonecunning",
                citation: Citation(Book::PHB, 20),
            },
        ];
        if let DwarfSubrace::Hill = self.subrace {
            features.extend(vec![
                // Your hit point maximum increases by 1, and it increases by 1 every time you gain a level.
                Feature {
                    title: "Dwarven Toughness",
                    citation: Citation(Book::PHB, 20),
                },
            ]);
        }
        features
    }
}

impl Languages for Dwarf {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Dwarvish]
    }
}

impl Name for Dwarf {
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
            first_names.iter().choose(rng).unwrap(),
            CLAN.iter().choose(rng).unwrap()
        )
    }
}

impl Proficiencies for Dwarf {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Battleaxe)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Handaxe)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::LightHammer)),
            Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Warhammer)),
        ];
        if let DwarfSubrace::Mountain = self.subrace {
            proficiencies.extend(vec![
                Proficiency::Armor(ArmorType::Light),
                Proficiency::Armor(ArmorType::Medium),
            ]);
        };
        proficiencies
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        vec![ProficiencyOption::From(
            [
                ArtisansTools::BrewersSupplies,
                ArtisansTools::MasonsTools,
                ArtisansTools::SmithsTools,
            ]
            .iter()
            .map(|t| Proficiency::Tool(Tool::ArtisansTools(*t)))
            .collect(),
        )]
    }
}

#[typetag::serde]
impl Race for Dwarf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            subrace: DwarfSubrace::iter().choose(rng).unwrap(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Constitution, 2),
            match self.subrace {
                DwarfSubrace::Hill => AbilityScore(AbilityScoreType::Wisdom, 1),
                DwarfSubrace::Mountain => AbilityScore(AbilityScoreType::Strength, 2),
            },
        ]
    }
}

impl Resistances for Dwarf {
    fn resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Poison]
    }
}

impl fmt::Display for Dwarf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Dwarf", self.subrace)
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
        let dwarf = Dwarf::gen(&mut rng);
        insta::assert_yaml_snapshot!(dwarf);
    }

    #[test]
    fn test_snapshot_display() {
        insta::assert_snapshot!(DwarfSubrace::iter()
            .map(|subrace| format!("{}", Dwarf { subrace }))
            .collect::<Vec<String>>()
            .join("\n\n"));
    }

    #[test]
    fn test_snapshot_abilities() {
        insta::assert_yaml_snapshot!(DwarfSubrace::iter()
            .map(|subrace| (Dwarf { subrace }).abilities())
            .collect::<Vec<Vec<AbilityScore>>>());
    }

    #[test]
    fn test_snapshot_citations() {
        insta::assert_yaml_snapshot!(DwarfSubrace::iter()
            .map(|subrace| (Dwarf { subrace }).citations())
            .collect::<Vec<CitationList>>());
    }

    #[test]
    fn test_snapshot_features() {
        insta::assert_yaml_snapshot!(DwarfSubrace::iter()
            .map(|subrace| (Dwarf { subrace }).features())
            .collect::<Vec<Vec<Feature>>>());
    }
}
