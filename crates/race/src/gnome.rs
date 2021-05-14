use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use backstory::Backstory;
use characteristics::{
    in_inches,
    names::{
        gnome::{CLAN, FEMALE, MALE, NICKNAMES},
        Name,
    },
    AgeRange, Appearance, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable,
    Size, Speed, WeightMod,
};
use citation::{Book, Citation, CitationList, Citations};
use deities::{Pantheon, PantheonWeight, Pantheons};
use dice_roller::{Die, RollCmd};
use features::{Feature, Features};
use gear::tools::{ArtisansTools, Tool};
use languages::{Language, Languages};
use personality::{Influence, PersonalityOptions};
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
use serde::{Deserialize, Serialize};
use stats::{
    ability::{AbilityScore, AbilityScoreType},
    proficiencies::{Proficiencies, Proficiency},
};
use strum::{Display, EnumIter, IntoEnumIterator};
use trinkets::Trinkets;

use super::Race;

const BONDS: &[&str] = &[
    "You pledge to bring something of immense value back to your burrow.",
    "Anything of great quality and artisanship is to be protected, respected, and cared for.",
    "Kobolds have caused you and your people nothing but trouble. You will avenge those wrongs.",
    "You are searching for your lost love.",
    "You will recover a keepsake stolen from your clan.",
    "You are willing to take risks to learn about the world.",
];
const FLAWS: &[&str] = &[
    "You embody the typical absent-minded professor. If you could forget where you put your head, you would.",
    "You prefer to hide during a fight.",
    "There is no difference between what you think and what you say.",
    "You can't keep a secret.",
];
const IDEALS: &[(&str, Influence)] = &[
    ("Love. You love little (and big) critters and go out of your way to help them.", Influence::Good),
    ("Curiosity. You can't stand an unsolved mystery or an unopened door.", Influence::Chaotic),
    ("Knowledge. You are interested in everything. You never know when what you learn will come in handy.", Influence::Neutral),
    ("Compassion. You never turn down a plea for help.", Influence::Good),
    ("Helpfulness. Whether you see a broken contraption or a broken heart, you have to try to fix it.", Influence::Good),
    ("Excellence. You strive to be and do the best you can.", Influence::Any),
];
const TRAITS: &[&str] = &[
    "Once you develop a liking for something, you quickly become obsessed with it.",
    "You live life like a leaf on the breeze, letting it take you where it will.",
    "The world is a miraculous place, and you are fascinated by everything in it.",
    "You study your friends and take notes about how they act, jotting down things they say that interest you.",
    "Your curiosity is so wide-ranging that you sometimes have trouble concentrating on any one thing.",
    "You like to make little objects and creatures out of twigs or bits of metal and give them to friends.",
];

const HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(2, 11),
    base_weight: 35,
    height_mod: RollCmd(2, Die::D4),
    weight_mod: WeightMod::Fixed(1),
};
const SVIRFNEBLIN_HEIGHT_AND_WEIGHT: HeightAndWeightTable = HeightAndWeightTable {
    base_height: in_inches(2, 11),
    base_weight: 70,
    height_mod: RollCmd(2, Die::D4),
    weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
};

#[derive(Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum GnomeSubrace {
    Forest,
    Rock,
    Svirfneblin,
}

impl Default for GnomeSubrace {
    fn default() -> Self {
        Self::Forest
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Gnome {
    /// Randomly chosen subrace
    subrace: GnomeSubrace,
}

impl AlignmentInfluences for Gnome {
    fn attitude(&self) -> Vec<Attitude> {
        match self.subrace {
            GnomeSubrace::Forest | GnomeSubrace::Rock => vec![],
            GnomeSubrace::Svirfneblin => vec![Attitude::Neutral],
        }
    }

    fn morality(&self) -> Vec<Morality> {
        match self.subrace {
            GnomeSubrace::Forest | GnomeSubrace::Rock => vec![Morality::Good],
            GnomeSubrace::Svirfneblin => vec![Morality::Neutral],
        }
    }
}

impl Appearance for Gnome {}

impl Backstory for Gnome {}

impl Characteristics for Gnome {
    fn get_age_range(&self) -> AgeRange {
        match self.subrace {
            GnomeSubrace::Forest | GnomeSubrace::Rock => AgeRange(10..=500),
            GnomeSubrace::Svirfneblin => AgeRange(12..=250),
        }
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(25)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        match self.subrace {
            GnomeSubrace::Forest | GnomeSubrace::Rock => &HEIGHT_AND_WEIGHT,
            GnomeSubrace::Svirfneblin => &SVIRFNEBLIN_HEIGHT_AND_WEIGHT,
        }
    }

    fn get_size(&self) -> Size {
        Size::Small
    }
}

impl Citations for Gnome {
    fn citations(&self) -> CitationList {
        let race = Citation(Book::Phb, 35);
        let subrace = match self.subrace {
            GnomeSubrace::Forest | GnomeSubrace::Rock => Citation(Book::Phb, 37),
            GnomeSubrace::Svirfneblin => Citation(Book::Mtof, 113),
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
            GnomeSubrace::Svirfneblin => vec![
                // Your Darkvision has a radius of 120 ft
                Feature {
                    title: "Superior Darkvision",
                    citation: Citation(Book::Mtof, 114),
                },
                // You have advantage on Dexterity (Stealth) checks to hide in rocky terrain.
                Feature {
                    title: "Stone Camouflage",
                    citation: Citation(Book::Mtof, 114),
                },
            ],
        });
        features
    }
}

impl Languages for Gnome {
    fn languages(&self) -> Vec<Language> {
        let mut languages = vec![Language::Common, Language::Gnomish];
        if matches!(self.subrace, GnomeSubrace::Svirfneblin) {
            languages.push(Language::Undercommon);
        }
        languages
    }
}

impl Name for Gnome {
    fn gen_name(
        &self,
        rng: &mut impl Rng,
        CharacteristicDetails { gender, .. }: &CharacteristicDetails,
    ) -> String {
        let first_names = match gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        };
        format!(
            "{} \"{}\" {}",
            first_names.choose(rng).unwrap(),
            NICKNAMES.choose(rng).unwrap(),
            CLAN.choose(rng).unwrap(),
        )
    }
}

impl Pantheons for Gnome {
    fn addl_pantheons(&self) -> Vec<(Pantheon, PantheonWeight)> {
        vec![(Pantheon::Gnomish, PantheonWeight::Likely)]
    }
}

impl PersonalityOptions for Gnome {
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

impl Race for Gnome {
    fn gen(rng: &mut impl Rng) -> Self {
        Self {
            subrace: GnomeSubrace::iter().choose(rng).unwrap(),
        }
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Intelligence, 2),
            match self.subrace {
                GnomeSubrace::Forest | GnomeSubrace::Svirfneblin => {
                    AbilityScore(AbilityScoreType::Dexterity, 1)
                }
                GnomeSubrace::Rock => AbilityScore(AbilityScoreType::Constitution, 1),
            },
        ]
    }
}

impl Resistances for Gnome {}

impl Trinkets for Gnome {}

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

    #[test]
    fn test_snapshot_addl_pantheons() {
        let gnome = Gnome {
            subrace: GnomeSubrace::Forest,
        };
        insta::assert_yaml_snapshot!(gnome.addl_pantheons());
    }
}
