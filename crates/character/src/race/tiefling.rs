#![allow(clippy::default_trait_access)]
use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::{DamageType, Resistances};
use backstory::Backstory;
use characteristics::{
    in_inches,
    names::{
        tiefling::{FEMALE_ABYSSAL, MALE_ABYSSAL, SURNAMES, VIRTUE_NAMES},
        Name,
    },
    AgeRange, Appearance, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable,
    Size, Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use dice_roller::{Die, RollCmd};
use languages::{Language, Languages};
use personality::PersonalityOptions;
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use trinkets::Trinkets;

use crate::{
    ability::{AbilityScore, AbilityScoreType},
    features::{Feature, Features},
    proficiencies::Proficiencies,
};

use super::{human::Human, Race};

/// Tiefling height and weight table
const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(4, 9),
    base_weight: 110,
    height_mod: RollCmd(2, Die::D8),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum HornShape {
    Antelope,
    Gazelle,
    Ram,
}

impl Default for HornShape {
    fn default() -> Self {
        Self::Antelope
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum HornSize {
    Large,
    Small,
}

impl Default for HornSize {
    fn default() -> Self {
        Self::Large
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum SkinColor {
    #[strum(serialize = "Dark Blue")]
    DarkBlue,
    Red,
}

impl Default for SkinColor {
    fn default() -> Self {
        Self::DarkBlue
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum SkinTexture {
    Leathery,
    Scaly,
}

impl Default for SkinTexture {
    fn default() -> Self {
        Self::Leathery
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Teeth {
    Fangs,
    #[strum(serialize = "sharp teeth")]
    Sharp,
}

impl Default for Teeth {
    fn default() -> Self {
        Self::Fangs
    }
}

#[derive(Deserialize, EnumIter, Serialize)]
enum PhysicalAppearance {
    Brimstone,
    CatlikeEyes,
    ClovenHooves,
    ForkedTail,
    ForkedTongue,
    GoatLegs,
    Horns(HornSize, HornShape),
    NoShadow,
    SkinColor(SkinColor),
    SkinTexture(SkinTexture),
    SixFingers,
    Teeth(Teeth),
}

impl PhysicalAppearance {
    fn gen(rng: &mut impl Rng) -> Vec<Self> {
        let amount = RollCmd(1, Die::D4).roll(rng).total() + 1;
        PhysicalAppearance::iter()
            .choose_multiple(rng, amount)
            .into_iter()
            .map(|a| match a {
                Self::Brimstone
                | Self::CatlikeEyes
                | Self::ClovenHooves
                | Self::ForkedTail
                | Self::ForkedTongue
                | Self::GoatLegs
                | Self::NoShadow
                | Self::SixFingers => a,
                Self::Horns(_, _) => Self::Horns(
                    HornSize::iter().choose(rng).unwrap(),
                    HornShape::iter().choose(rng).unwrap(),
                ),
                Self::SkinColor(_) => Self::SkinColor(SkinColor::iter().choose(rng).unwrap()),
                Self::SkinTexture(_) => Self::SkinTexture(SkinTexture::iter().choose(rng).unwrap()),
                Self::Teeth(_) => Self::Teeth(Teeth::iter().choose(rng).unwrap()),
            })
            .collect()
    }
}

impl fmt::Display for PhysicalAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Brimstone => "exude a smell of brimstone".to_string(),
                Self::CatlikeEyes => "catlike eyes".to_string(),
                Self::ClovenHooves => "cloven hooves".to_string(),
                Self::ForkedTail => "a forked tail".to_string(),
                Self::ForkedTongue => "a forked tongue".to_string(),
                Self::GoatLegs => "goatlike legs".to_string(),
                Self::Horns(size, shape) => format!("{} {} horns", size, shape),
                Self::NoShadow => "cast no shadow or reflection".to_string(),
                Self::SkinColor(c) => format!("{}", c),
                Self::SkinTexture(t) => format!("{}", t),
                Self::SixFingers => "six fingers on each hand".to_string(),
                Self::Teeth(t) => format!("{}", t),
            }
        )
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum FeralVariant {
    #[strum(serialize = "Devil's Tongue")]
    DevilsTongue,
    Hellfire,
    Winged,
}

impl FeralVariant {
    fn features(&self) -> Vec<Feature> {
        match self {
            // You know the Vicious Mockery cantrip. Once you reach 3rd level, you can cast the Charm Person spell once as a 2nd-level spell. Once you reach 5th level, you can also cast the Enthrall spell once. You must finish a long rest to cast these spells again with this trait. Charisma is your spellcasting ability for these spells. This trait replaces the Infernal Legacy trait
            Self::DevilsTongue => vec![Feature {
                title: "Devil's Tongue",
                citation: Citation(Book::Scag, 118),
            }],
            // Once you reach 3rd level, you can cast the Burning Hands spell once as a 2nd-level spell. This trait replaces the Hellish Rebuke spell of the Infernal Legacy trait.
            Self::Hellfire => vec![Feature {
                title: "Hellfire",
                citation: Citation(Book::Scag, 118),
            }],
            Self::Winged => vec![],
        }
    }
}

impl Default for FeralVariant {
    fn default() -> Self {
        Self::DevilsTongue
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum TieflingSubrace {
    Asmodeus,
    Baalzebul,
    Dispater,
    Feral(FeralVariant),
    Fierna,
    Glasya,
    Levistus,
    Mammon,
    Mephistopheles,
    Zariel,
}

impl TieflingSubrace {
    fn gen(rng: &mut impl Rng) -> Self {
        let subrace = Self::iter().choose(rng).unwrap();
        if matches!(subrace, Self::Feral(_)) {
            Self::Feral(FeralVariant::iter().choose(rng).unwrap())
        } else {
            subrace
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Tiefling {
    appearance: Vec<PhysicalAppearance>,
    subrace: TieflingSubrace,
}

impl Tiefling {
    /// Generate first name for a tiefling. Could be abyssal, human, or virtue name.
    fn gen_first_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> &'static str {
        let abyssal_name = *(match characteristics.gender {
            Gender::Female => FEMALE_ABYSSAL,
            Gender::Male => MALE_ABYSSAL,
        })
        .choose(rng)
        .unwrap();
        let human_name = Human::gen_first_name(rng, characteristics);
        let virtue_name = *VIRTUE_NAMES.choose(rng).unwrap();
        *[abyssal_name, human_name, virtue_name].choose(rng).unwrap()
    }

    // Generate surname, could be human, or more traditional tiefling
    fn gen_surname(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> &'static str {
        *[
            Human::gen_surname(rng, characteristics),
            SURNAMES.choose(rng).unwrap(),
        ]
        .choose(rng)
        .unwrap()
    }
}

impl AlignmentInfluences for Tiefling {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Evil]
    }
}

impl Appearance for Tiefling {
    fn appearance(&self) -> Vec<String> {
        let mut appearance: Vec<String> =
            self.appearance.iter().map(|a| format!("{}", a)).collect();
        if matches!(self.subrace, TieflingSubrace::Feral(FeralVariant::Winged)) {
            appearance.push("Bat-like wings jut from your shoulder blades. You have a flying speed of 30ft while not wearing heavy armor".to_string());
        }
        appearance
    }
}

impl Backstory for Tiefling {}

impl Characteristics for Tiefling {
    const HUMAN_ANCESTRY: bool = true;
    const SIZE: Size = Size::Medium;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=100)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        let mut speeds = vec![Speed::Walking(30)];
        if matches!(self.subrace, TieflingSubrace::Feral(FeralVariant::Winged)) {
            speeds.push(Speed::Flying(30));
        }
        speeds
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Tiefling {
    fn citations(&self) -> CitationList {
        let mut citations = vec![Citation(Book::Phb, 42)];
        citations.push(match self.subrace {
            TieflingSubrace::Asmodeus
            | TieflingSubrace::Baalzebul
            | TieflingSubrace::Dispater
            | TieflingSubrace::Fierna => Citation(Book::Mtof, 21),
            TieflingSubrace::Feral(_) => Citation(Book::Scag, 118),
            TieflingSubrace::Glasya | TieflingSubrace::Levistus | TieflingSubrace::Mammon => {
                Citation(Book::Mtof, 22)
            }
            TieflingSubrace::Mephistopheles | TieflingSubrace::Zariel => Citation(Book::Mtof, 23),
        });
        CitationList(citations)
    }
}

impl Features for Tiefling {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            // Thanks to your infernal heritage, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Phb, 43),
            },
        ];
        features.extend(match &self.subrace {
            // You know the thaumaturgy cantrip. When you reach 3rd level, you can cast the hellish rebuke spell as a 2nd-level spell once with this trait and regain the ability to do so when you finish a long rest. When you reach 5th level, you can cast the darkness spell once with this trait and regain the ability to do so when you finish a long rest. Charisma is your spellcasting ability for these spells.
            TieflingSubrace::Asmodeus => {
                vec![Feature {
                    title: "Infernal Legacy",
                    citation: Citation(Book::Phb, 43),
                }]
            }
            // You know the Thaumaturgy cantrip. Once you reach 3rd level, you can cast the Ray of Sickness spell once as a 2nd-level spell. Once you reach 5th level, you can also cast the Crown of Madness spell once. You must finish a long rest to cast these spells again with this trait. Charisma is your spellcasting ability for these spells.
            TieflingSubrace::Baalzebul => vec![Feature {
                title: "Legacy of Maladomini",
                citation: Citation(Book::Mtof, 21),
            }],
            // You know the Thaumaturgy cantrip. Once you reach 3rd level, you can cast the Disguise Self spell once as a 2nd-level spell. Once you reach 5th level, you can also cast the Detect Thoughts spell once. You must finish a long rest to cast these spells again with this trait. Charisma is your spellcasting ability for these spells.
            TieflingSubrace::Dispater => vec![Feature {
                title: "Legacy of Dis",
                citation: Citation(Book::Mtof, 21),
            }],
            TieflingSubrace::Feral(v) => v.features(),
            // You know the Friends cantrip. Once you reach 3rd level, you can cast the Charm Person spell once as a 2nd-level spell. Once you reach 5th level, you can also cast the Suggestion spell once. You must finish a long rest to cast these spells again with this trait. Charisma is your spellcasting ability for these spells.
            TieflingSubrace::Fierna => vec![Feature {
                title: "Legacy of Phlegethos",
                citation: Citation(Book::Mtof, 21),
            }],
            // You know the Minor Illusion cantrip. Once you reach 3rd level, you can cast the Disguise Self spell once as a 2nd-level spell. Once you reach 5th level, you can also cast the Invisibility spell once as a 2nd-level spell. You must finish a long rest to cast these spells again with this trait. Charisma is your spellcasting ability for these spells.
            TieflingSubrace::Glasya => vec![Feature {
                title: "Legacy of Malbolge",
                citation: Citation(Book::Mtof, 22),
            }],
            // You know the Ray of Frost cantrip. Once you reach 3rd level, you can cast the Armor of Agathys spell once as a 2nd-level spell. Once you reach 5th level, you can also cast the Darkness spell once. You must finish a long rest to cast these spells again with this trait. Charisma is your spellcasting ability for these spells.
            TieflingSubrace::Levistus => vec![Feature {
                title: "Legacy of Stygia",
                citation: Citation(Book::Mtof, 22),
            }],
            // You know the Mage Hand cantrip. Once you reach 3rd level, you can cast the Tenser's Floating Disk spell once as a 2nd-level spell. Once you reach 5th level, you can also cast the Arcane Lock spell once. You must finish a long rest to cast these spells again with this trait. Charisma is your spellcasting ability for these spells.
            TieflingSubrace::Mammon => vec![Feature {
                title: "Legacy of Minauros",
                citation: Citation(Book::Mtof, 22),
            }],
            // You know the Mage Hand cantrip. Once you reach 3rd level, you can cast the Burning Hands spell once as a 2nd-level spell. Once you reach 5th level, you can also cast the Flame Blade spell once as a 3rd-level spell. You must finish a long rest to cast these spells again with this trait. Charisma is your spellcasting ability for these spells.
            TieflingSubrace::Mephistopheles => vec![Feature {
                title: "Legacy of Cania",
                citation: Citation(Book::Mtof, 23),
            }],
            // You know the Thaumaturgy cantrip. Once you reach 3rd level, you can cast the Searing Smite spell once as a 2nd-level spell. Once you reach 5th level, you can also cast the Branding Smite spell once as a 3rd-level spell. You must finish a long rest to cast these spells again with this trait. Charisma is your spellcasting ability for these spells.
            TieflingSubrace::Zariel => vec![Feature {
                title: "Legacy of Avernus",
                citation: Citation(Book::Mtof, 23),
            }],
        });

        features
    }
}

impl Languages for Tiefling {
    /// Names given by tiefling race
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Infernal]
    }
}

impl Name for Tiefling {
    /// Name also requires getting a set of human names (for human lineage)
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        format!(
            "{} {}",
            Self::gen_first_name(rng, characteristics),
            Self::gen_surname(rng, characteristics)
        )
    }
}

impl PersonalityOptions for Tiefling {}

impl Proficiencies for Tiefling {}

#[typetag::serde]
impl Race for Tiefling {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            appearance: PhysicalAppearance::gen(rng),
            subrace: TieflingSubrace::gen(rng),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        match self.subrace {
            TieflingSubrace::Asmodeus
            | TieflingSubrace::Baalzebul
            | TieflingSubrace::Mammon
            | TieflingSubrace::Mephistopheles => {
                vec![
                    AbilityScore(AbilityScoreType::Charisma, 2),
                    AbilityScore(AbilityScoreType::Intelligence, 1),
                ]
            }
            TieflingSubrace::Dispater | TieflingSubrace::Glasya => vec![
                AbilityScore(AbilityScoreType::Charisma, 2),
                AbilityScore(AbilityScoreType::Dexterity, 1),
            ],
            TieflingSubrace::Feral(_) => vec![
                AbilityScore(AbilityScoreType::Dexterity, 2),
                AbilityScore(AbilityScoreType::Intelligence, 1),
            ],
            TieflingSubrace::Fierna => vec![
                AbilityScore(AbilityScoreType::Charisma, 2),
                AbilityScore(AbilityScoreType::Wisdom, 1),
            ],
            TieflingSubrace::Levistus => vec![
                AbilityScore(AbilityScoreType::Charisma, 2),
                AbilityScore(AbilityScoreType::Constitution, 1),
            ],
            TieflingSubrace::Zariel => vec![
                AbilityScore(AbilityScoreType::Charisma, 2),
                AbilityScore(AbilityScoreType::Strength, 1),
            ],
        }
    }
}

impl Resistances for Tiefling {
    fn resistances(&self) -> Vec<DamageType> {
        vec![DamageType::Fire]
    }
}

impl Trinkets for Tiefling {}

impl fmt::Display for Tiefling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Tiefling", self.subrace)
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
        let tiefling = Tiefling::gen(&mut rng);
        insta::assert_yaml_snapshot!(tiefling);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (tiefling, _name, _characteristics) = Tiefling::gen(&mut rng);
        insta::assert_display_snapshot!(tiefling);
    }

    #[test]
    fn test_attitude() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (tiefling, _name, _characteristics) = Tiefling::gen(&mut rng);
        insta::assert_yaml_snapshot!(tiefling.attitude());
    }

    #[test]
    fn test_morality() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (tiefling, _name, _characteristics) = Tiefling::gen(&mut rng);
        insta::assert_yaml_snapshot!(tiefling.morality());
    }

    #[test]
    fn test_appearance() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (tiefling, _name, _characteristics) = Tiefling::gen(&mut rng);
        insta::assert_yaml_snapshot!(tiefling.appearance());
    }

    #[test]
    fn test_snapshot_citations() {
        insta::assert_yaml_snapshot!(TieflingSubrace::iter()
            .map(|subrace| Tiefling {
                appearance: vec![],
                subrace
            }
            .citations())
            .collect::<Vec<CitationList>>());
    }

    #[test]
    fn test_snapshot_features() {
        insta::assert_yaml_snapshot!(TieflingSubrace::iter()
            .map(|subrace| Tiefling {
                appearance: vec![],
                subrace
            }
            .features())
            .collect::<Vec<Vec<Feature>>>())
    }

    #[test]
    fn test_snapshot_languages() {
        let tiefling = Tiefling {
            appearance: vec![],
            subrace: TieflingSubrace::Asmodeus,
        };
        insta::assert_yaml_snapshot!(tiefling.languages());
    }

    #[test]
    fn test_name() {
        let mut rng = Pcg64::seed_from_u64(1);
        let tiefling = Tiefling {
            appearance: vec![],
            subrace: TieflingSubrace::Asmodeus,
        };
        let characteristics_1 = tiefling.gen_characteristics(&mut rng);
        let characteristics_2 = tiefling.gen_characteristics(&mut rng);
        let female_name = Tiefling::gen_name(
            &mut rng,
            &CharacteristicDetails {
                gender: Gender::Female,
                ..characteristics_1
            },
        );
        let male_name = Tiefling::gen_name(
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
        insta::assert_yaml_snapshot!(TieflingSubrace::iter()
            .map(|subrace| Tiefling {
                appearance: vec![],
                subrace
            }
            .abilities())
            .collect::<Vec<Vec<AbilityScore>>>());
    }

    #[test]
    fn test_resistances() {
        let tiefling = Tiefling {
            appearance: vec![],
            subrace: TieflingSubrace::Asmodeus,
        };
        insta::assert_yaml_snapshot!(tiefling.resistances());
    }
}
