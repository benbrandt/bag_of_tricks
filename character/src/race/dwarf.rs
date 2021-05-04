#![allow(clippy::default_trait_access)]
use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use characteristics::{
    names::{
        dwarf::{CLAN, FEMALE, MALE},
        Name,
    },
    AgeRange, Appearance, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable,
    Size, Speed,
};
use citation::{Book, Citation, CitationList, Citations};
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::{
    ability::{AbilityScore, AbilityScoreType},
    attack::{DamageType, Resistances},
    backstory::Backstory,
    equipment::{
        armor::ArmorType,
        tools::{ArtisansTools, Tool},
        weapons::WeaponType,
    },
    features::{Feature, Features},
    languages::{Language, Languages},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption, WeaponProficiency},
};

use super::Race;

mod height_and_weight {
    use characteristics::{in_inches, HeightAndWeightTable, WeightMod};
    use dice_roller::{Die, RollCmd};

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
enum HillVariant {
    Gold,
    Hill,
}

impl Default for HillVariant {
    fn default() -> Self {
        Self::Hill
    }
}

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum MountainVariant {
    Mountain,
    Shield,
}

impl Default for MountainVariant {
    fn default() -> Self {
        Self::Mountain
    }
}

#[derive(Debug, Deserialize, EnumIter, PartialEq, Serialize)]
enum DwarfSubrace {
    Duergar,
    Hill(HillVariant),
    Mountain(MountainVariant),
}

impl DwarfSubrace {
    fn gen(rng: &mut impl Rng) -> Self {
        let subrace = Self::iter().choose(rng).unwrap();
        match subrace {
            Self::Hill(_) => Self::Hill(HillVariant::iter().choose(rng).unwrap()),
            Self::Mountain(_) => Self::Mountain(MountainVariant::iter().choose(rng).unwrap()),
            Self::Duergar => subrace,
        }
    }
}

impl fmt::Display for DwarfSubrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duergar => write!(f, "Duergar"),
            Self::Hill(v) => write!(f, "{}", v),
            Self::Mountain(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Dwarf {
    /// Randomly chosen subrace
    subrace: DwarfSubrace,
}

impl AlignmentInfluences for Dwarf {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Lawful]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![match self.subrace {
            DwarfSubrace::Duergar => Morality::Evil,
            DwarfSubrace::Hill(_) | DwarfSubrace::Mountain(_) => Morality::Good,
        }]
    }
}

impl Appearance for Dwarf {}

impl Backstory for Dwarf {}

impl Characteristics for Dwarf {
    const SIZE: Size = Size::Medium;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=350)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(25)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        match self.subrace {
            DwarfSubrace::Duergar | DwarfSubrace::Hill(_) => &height_and_weight::HILL,
            DwarfSubrace::Mountain(_) => &height_and_weight::MOUNTAIN,
        }
    }
}

impl Citations for Dwarf {
    fn citations(&self) -> CitationList {
        let race = Citation(Book::Phb, 18);
        let subrace = match self.subrace {
            DwarfSubrace::Duergar => Citation(Book::Scag, 104),
            DwarfSubrace::Hill(HillVariant::Gold)
            | DwarfSubrace::Mountain(MountainVariant::Shield) => Citation(Book::Scag, 103),
            DwarfSubrace::Hill(HillVariant::Hill)
            | DwarfSubrace::Mountain(MountainVariant::Mountain) => Citation(Book::Phb, 20),
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
                citation: Citation(Book::Phb, 20),
            },
            // You have advantage on saving throws against poison, and you have resistance against poison damage (explained in the \"Combat\" section).
            Feature {
                title: "Dwarven Resilience",
                citation: Citation(Book::Phb, 20),
            },
            // Whenever you make an Intelligence (History) check related to the origin of stonework, you are considered proficient in the History skill and add double your proficiency bonus to the check, instead of your normal proficiency bonus.
            Feature {
                title: "Stonecunning",
                citation: Citation(Book::Phb, 20),
            },
        ];
        if matches!(self.subrace, DwarfSubrace::Hill(_)) {
            features.push(
                // Your hit point maximum increases by 1, and it increases by 1 every time you gain a level.
                Feature {
                    title: "Dwarven Toughness",
                    citation: Citation(Book::Phb, 20),
                },
            );
        }
        if matches!(self.subrace, DwarfSubrace::Duergar) {
            features.extend(vec![
                // Your darkvision has a radius of 120 feet.
                Feature {
                    title: "Superior Darkvision",
                    citation: Citation(Book::Scag, 104),
                },
                // You have advantage on saving throws against illusions and against being charmed or paralyzed.
                Feature {
                    title: "Duergar Resiliance",
                    citation: Citation(Book::Scag, 104),
                },
                // When you reach 3rd level, you can cast the Enlarge/Reduce spell on yourself once with this trait, using only the spell's enlarge option. When you reach 5th level, you can cast the Invisibility spell on yourself once with this trait. You don't need material components for either spell, and you can't cast them while you're in direct sunlight, although sunlight has no effect on them once cast. You regain the ability to cast these spells with this trait when you finish a long rest. Intelligence is your spellcasting ability for these spells.
                Feature {
                    title: "Duergar Magic",
                    citation: Citation(Book::Scag, 104),
                },
                // You have disadvantage on Attack rolls and Wisdom (Perception) checks that rely on sight when you, the target of your attack, or whatever you are trying to perceive is in direct sunlight.
                Feature {
                    title: "Sunlight Sensitivity",
                    citation: Citation(Book::Scag, 104),
                },
            ]);
        }
        features
    }
}

impl Languages for Dwarf {
    fn languages(&self) -> Vec<Language> {
        let mut languages = vec![Language::Common, Language::Dwarvish];
        if matches!(self.subrace, DwarfSubrace::Duergar) {
            languages.push(Language::Undercommon);
        }
        languages
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
            first_names.choose(rng).unwrap(),
            CLAN.choose(rng).unwrap()
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
        if matches!(self.subrace, DwarfSubrace::Mountain(_)) {
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
            1,
        )]
    }
}

#[typetag::serde]
impl Race for Dwarf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            subrace: DwarfSubrace::gen(rng),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Constitution, 2),
            match self.subrace {
                DwarfSubrace::Duergar => AbilityScore(AbilityScoreType::Strength, 1),
                DwarfSubrace::Hill(_) => AbilityScore(AbilityScoreType::Wisdom, 1),
                DwarfSubrace::Mountain(_) => AbilityScore(AbilityScoreType::Strength, 2),
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
