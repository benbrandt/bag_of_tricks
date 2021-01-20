use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{
            in_inches, AgeRange, CharacteristicDetails, Characteristics, Gender,
            HeightAndWeightTable, Size, WeightMod,
        },
        equipment::tools::{ArtisansTools, Tool},
        features::{Feature, Features},
        languages::Language,
        names::{
            gnome::{CLAN, FEMALE, MALE, NICKNAMES},
            Name,
        },
        proficiencies::Proficiency,
    },
    citation::{Book, Citation, Citations},
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
    subrace: GnomeSubrace,
}

impl Characteristics for Gnome {
    const AGE_RANGE: AgeRange = AgeRange(1..=500);
    const SIZE: Size = Size::Small;

    fn get_base_speed(&self) -> u8 {
        25
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        &HEIGHT_AND_WEIGHT
    }
}

impl Features for Gnome {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            Feature {
                title: "Alignment",
                description: "Gnomes are most often good. Those who tend toward law are sages, engineers, researchers, scholars, investigators, or inventors. Those who tend toward chaos are minstrels, tricksters, wanderers, or fanciful jewelers. Gnomes are good-hearted, and even the tricksters among them are more playful than vicious.",
                citation: Citation {
                    book: Book::PHB,
                    page: 36,
                },
            },
            Feature {
                title: "Darkvision",
                description: "Accustomed to life underground, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.",
                citation: Citation {
                    book: Book::PHB,
                    page: 37,
                },
            },
            Feature {
                title: "Gnome Cunning",
                description: "You have advantage on all Intelligence, Wisdom, and Charisma saving throws against magic.",
                citation: Citation {
                    book: Book::PHB,
                    page: 37,
                },
            },
        ];
        features.extend(match self.subrace {
            GnomeSubrace::Forest => vec![
                Feature {
                    title: "Natural Illusionist",
                    description: "You know the minor illusion cantrip. Intelligence is your spellcasting ability for it.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 37,
                    },
                },
                Feature {
                    title: "Speak with Small Beasts",
                    description: "Through sounds and gestures, you can communicate simple ideas with Small or smaller beasts. Forest gnomes love animals and often keep squirrels, badgers, rabbits, moles, woodpeckers, and other creatures as beloved pets.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 37,
                    },
                },
            ],
            GnomeSubrace::Rock => vec![
                Feature {
                    title: "Artificer's Lore",
                    description: "Whenever you make an Intelligence (History) check related to magic items, alchemical objects, or technological devices, you can add twice your proficiency bonus, instead of any proficiency bonus you normally apply.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 37,
                    },
                },
                Feature {
                    title: "Artificer's Lore",
                    description: "You have proficiency with artisan's tools (tinker's tools). Using those tools, you can spend 1 hour and 10 gp worth of materials to construct a Tiny clockwork device (AC 5, 1 hp). The device ceases to function after 24 hours (unless you spend 1 hour repairing it to keep the device functioning), or when you use your action to dismantle it; at that time, you can reclaim the materials used to create it. You can have up to three such devices active at a time.

When you create a device, choose one of the following options:

Clockwork Toy. This toy is a clockwork animal, monster, or person, such as a frog, mouse, bird, dragon, or soldier. When placed on the ground, the toy moves 5 feet across the ground on each of your turns in a random direction. It makes noises as appropriate to the creature it represents.

Fire Starter. The device produces a miniature flame, which you can use to light a candle, torch, or campfire. Using the device requires your action.

Music Box. When opened, this music box plays a single song at a moderate volume.
The box stops playing when it reaches the song's end or when it is closed.",
                    citation: Citation {
                        book: Book::PHB,
                        page: 37,
                    },
                },
            ],
        });
        features
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

    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Gnomish]
    }

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
