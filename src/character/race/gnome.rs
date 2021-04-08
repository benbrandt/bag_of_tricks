use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType},
        alignment::{AlignmentInfluences, Morality},
        attack::Resistances,
        backstory::Backstory,
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, WeightMod,
        },
        equipment::tools::{ArtisansTools, Tool},
        features::{Feature, Features},
        languages::{Language, Languages},
        names::{
            gnome::{CLAN, FEMALE, MALE, NICKNAMES},
            Name,
        },
        proficiencies::{Proficiencies, Proficiency},
    },
    citation::{Book, Citation, CitationList, Citations},
    dice_roller::{Die, RollCmd},
};

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(2, 11),
    base_weight: 35,
    height_mod: RollCmd(2, Die::D4),
    weight_mod: WeightMod::Fixed(1),
};

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum GnomeSubrace {
    Forest,
    Rock,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Gnome {
    /// Randomly chosen subrace
    subrace: GnomeSubrace,
}

impl AlignmentInfluences for Gnome {
    fn morality(&self) -> Vec<Morality> {
        vec![Morality::Good]
    }
}

impl Backstory for Gnome {}

impl Characteristics for Gnome {
    const AGE_RANGE: AgeRange = AgeRange(10..=500);
    const SIZE: Size = Size::Small;

    fn get_base_speed(&self) -> u8 {
        25
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Citations for Gnome {
    fn citations(&self) -> CitationList {
        let race = Citation(Book::Phb, 35);
        let subrace = match self.subrace {
            GnomeSubrace::Forest | GnomeSubrace::Rock => Citation(Book::Phb, 37),
        };
        CitationList(vec![race, subrace])
    }
}

impl Features for Gnome {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            // Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Phb, 37),
            },
            // You have advantage on all Intelligence, Wisdom, and Charisma saving throws against magic.
            Feature {
                title: "Gnome Cunning",
                citation: Citation(Book::Phb, 37),
            },
        ];
        features.extend(match self.subrace {
            GnomeSubrace::Forest => vec![
                // You know the minor illusion cantrip. Intelligence is your spellcasting ability for it.
                Feature {
                    title: "Natural Illusionist",
                    citation: Citation(Book::Phb, 37),
                },
                // Through sounds and gestures, you can communicate simple ideas with Small or smaller beasts. Forest gnomes love animals and often keep squirrels, badgers, rabbits, moles, woodpeckers, and other creatures as beloved pets.
                Feature {
                    title: "Speak with Small Beasts",
                    citation: Citation(Book::Phb, 37),
                },
            ],
            GnomeSubrace::Rock => vec![
                Feature {
                    // Whenever you make an Intelligence (History) check related to magic items, alchemical objects, or technological devices, you can add twice your proficiency bonus, instead of any proficiency bonus you normally apply.
                    title: "Artificer's Lore",
                    citation: Citation(Book::Phb, 37),
                },
                // You have proficiency with artisan's tools (tinker's tools). Using those tools, you can spend 1 hour and 10 gp worth of materials to construct a Tiny clockwork device (AC 5, 1 hp). The device ceases to function after 24 hours (unless you spend 1 hour repairing it to keep the device functioning), or when you use your action to dismantle it; at that time, you can reclaim the materials used to create it. You can have up to three such devices active at a time.
                // When you create a device, choose one of the following options:
                // Clockwork Toy. This toy is a clockwork animal, monster, or person, such as a frog, mouse, bird, dragon, or soldier. When placed on the ground, the toy moves 5 feet across the ground on each of your turns in a random direction. It makes noises as appropriate to the creature it represents.
                // Fire Starter. The device produces a miniature flame, which you can use to light a candle, torch, or campfire. Using the device requires your action.
                // Music Box. When opened, this music box plays a single song at a moderate volume.
                // The box stops playing when it reaches the song's end or when it is closed.
                Feature {
                    title: "Tinker",
                    citation: Citation(Book::Phb, 37),
                },
            ],
        });
        features
    }
}

impl Languages for Gnome {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Gnomish]
    }
}

impl Name for Gnome {
    fn gen_name(
        rng: &mut impl Rng,
        CharacteristicDetails { gender, .. }: &CharacteristicDetails,
    ) -> String {
        let first_names = match gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        };
        format!(
            "{} \"{}\" {}",
            first_names.iter().choose(rng).unwrap(),
            NICKNAMES.iter().choose(rng).unwrap(),
            CLAN.iter().choose(rng).unwrap(),
        )
    }
}

impl Proficiencies for Gnome {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![];
        if let GnomeSubrace::Rock = self.subrace {
            proficiencies.push(Proficiency::Tool(Tool::ArtisansTools(
                ArtisansTools::TinkersTools,
            )))
        }
        proficiencies
    }
}

#[typetag::serde]
impl Race for Gnome {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let race = Box::new(Self {
            subrace: GnomeSubrace::iter().choose(rng).unwrap(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Intelligence, 2),
            match self.subrace {
                GnomeSubrace::Forest => AbilityScore(AbilityScoreType::Dexterity, 1),
                GnomeSubrace::Rock => AbilityScore(AbilityScoreType::Constitution, 1),
            },
        ]
    }
}

impl Resistances for Gnome {}

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
        insta::assert_snapshot!(GnomeSubrace::iter()
            .map(|subrace| format!("{}", Gnome { subrace }))
            .collect::<Vec<String>>()
            .join("\n\n"));
    }

    #[test]
    fn test_snapshot_abilities() {
        insta::assert_yaml_snapshot!(GnomeSubrace::iter()
            .map(|subrace| (Gnome { subrace }).abilities())
            .collect::<Vec<Vec<AbilityScore>>>());
    }

    #[test]
    fn test_snapshot_citations() {
        insta::assert_yaml_snapshot!(GnomeSubrace::iter()
            .map(|subrace| (Gnome { subrace }).citations())
            .collect::<Vec<CitationList>>());
    }

    #[test]
    fn test_snapshot_features() {
        insta::assert_yaml_snapshot!(GnomeSubrace::iter()
            .map(|subrace| (Gnome { subrace }).features())
            .collect::<Vec<Vec<Feature>>>());
    }
}
