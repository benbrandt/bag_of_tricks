#![allow(clippy::default_trait_access)]
use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::{DamageType, Resistances};
use characteristics::{
    names::dwarf::{CLAN, DUERGAR_CLAN, FEMALE, MALE},
    AgeRange, Appearance, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable,
    Size, Speed,
};
use citation::{Book, Citation, CitationList, Citations};
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

#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum HillVariant {
    Gold,
    Hill,
}

impl Default for HillVariant {
    fn default() -> Self {
        Self::Hill
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum MountainVariant {
    Mountain,
    Shield,
}

impl Default for MountainVariant {
    fn default() -> Self {
        Self::Mountain
    }
}

#[derive(Clone, Copy, Debug, Deserialize, EnumIter, PartialEq, Serialize)]
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

    fn gen_name(
        self,
        rng: &mut impl Rng,
        CharacteristicDetails { gender, .. }: &CharacteristicDetails,
    ) -> String {
        let first_names = match gender {
            Gender::Female => FEMALE,
            Gender::Male => MALE,
        };
        let clan_names = match self {
            Self::Duergar => DUERGAR_CLAN,
            Self::Hill(_) | Self::Mountain(_) => CLAN,
        };
        format!(
            "{} {}",
            first_names.choose(rng).unwrap(),
            clan_names.choose(rng).unwrap()
        )
    }

    fn clan_status(self, rng: &mut impl Rng) -> String {
        (*match self {
            Self::Duergar => [
                "Mighty. Conquered several dwarven strongholds, dominates Underdark region",
                "Growing. Stronghold expanding; glory days lie ahead",
                "Declining. Clan growing stale, population falling",
                "Beleaguered. Surrounded by drow and illithid foes",
                "Scattered. Torn apart by slave rebellion or civil war",
                "Refugees. Defeated by enemies, few survivors",
            ],
            Self::Hill(_) | Self::Mountain(_) => [
                "Prosperous. Clan occupies original stronghold, currently flourishing",
                "Growing. Stronghold expanding; glory days lie ahead",
                "Declining. Clan population stagnant or decreasing",
                "Beleaguered. Victimized by goblinoid and dragon attacks, intact but severely weakened",
                "Scattered. Stronghold recently lost, many folk slain, survivors scattered",
                "Refugees. Stronghold lost, survivors occupy a neighborhood or ward in human city",
            ],
        }.choose(rng).unwrap()).to_string()
    }

    fn clan_trait(self, rng: &mut impl Rng) -> String {
        (*match self {
            Self::Duergar => vec![
                "Stole a mighty dwarven artifact",
                "Has bound many devils to service",
                "Experts in building mechanical devices",
                "Conducts trade with the City of Brass",
                "Notable for defeating many dwarves",
                "Conquered and occupied a drow enclave",
                "Is secretly controlled by mind flayers",
                "Has enslaved a colony of troglodytes",
                "Have interbred with devils",
                "Known for its extensive spy network on surface",
                "Masters of psionics",
                "Dominated by a coven of warlocks",
            ],
            Self::Hill(_) | Self::Mountain(_) => vec![
                "Founder was one of the greatest artisans in history",
                "Clan owns a powerful artifact, such as an Axe of the Dwarvish Lords",
                "Clan noted for expertise in a specific craft, such as brewing or armorsmithing",
                "Clan has a sinister reputation, history plagued by scandal and mark of Abbathor",
                "Militaristic clan, known for excellent fighting skills",
                "Unusual stronghold, such as an undersea castle, a former cloud giant fortress, or an aboveground city",
                "Prophecies indicate clan is destined to play a pivotal role in history",
                "Heretical clan has rejected dwarf teachings in favor of human deities",
                "Unique marker or curse, such as all clan members are hairless",
                "Clan is known for its evil ways or a particularly sinister, notable member",
            ],
        }.choose(rng).unwrap()).to_string()
    }

    fn clan_vocation(rng: &mut impl Rng) -> String {
        (*[
            "Armorer",
            "Blacksmith",
            "Brewer",
            "Carpenter",
            "Cook",
            "Envoy",
            "Farmer",
            "Hunter",
            "Jeweler",
            "Mason",
            "Merchant",
            "Messenger",
            "Miner",
            "Potter",
            "Scout",
            "Sculptor",
            "Shepherd",
            "Warrior",
            "Weaponsmith",
            "Weaver",
        ]
        .choose(rng)
        .unwrap())
        .to_string()
    }

    fn quirk(self, rng: &mut impl Rng) -> String {
        (*match self {
            Self::Duergar => vec![
                "A separate personality in your mind provides advice and guidance to you.",
                "Your gear must be perfectly arranged, otherwise someone must bleed.",
                "When there isn't a roof over your head, you keep your eyes on the ground.",
                "You don't talk unless you absolutely must.",
                "The outside world is a giant cave, and nothing will convince you otherwise.",
                "Humans fascinate you, and you collect odd trinkets of their culture.",
            ],
            Self::Hill(_) | Self::Mountain(_) => vec![
                "Water from the sky! It always surprises you.",
                "You have a fascination with the ocean and its chaos.",
                "Any creature larger than a human makes you nervous.",
                "You prefer to travel with a parasol or similar item that puts a comforting shelter over your head.",
                "You prefer to sleep during the day.",
                "You speak Common or any other non-dwarf language only if you must.",
                "For you, relaxation is putting in a day at the forge.",
                "You avoid contact with other dwarves, since you mistrust those who would leave their strongholds.",
            ],
        }.choose(rng).unwrap()).to_string()
    }

    fn story_hook(self, rng: &mut impl Rng) -> String {
        (*match self {
            Self::Duergar => [
                "You are a heretic, drawn to worship of Moradin.",
                "Caught stealing, you escaped imprisonment but not before torture left you with a scar or lasting injury.",
                "You were enslaved by drow or mind flayers but escaped to the surface.",
                "You seek only to test yourself in battle with monsters.",
                "Profit is all that matters to you.",
                "The best way to defeat the folk of the surface is to study them firsthand.",
            ],
            Self::Hill(_) | Self::Mountain(_) => [
                "You were accused of stealing a fellow artisan's item and claiming it as your work. Innocent or guilty, you were made an outcast.",
                "Your wanderlust prompted you to shirk your duties as a crafter in favor of wandering the world. Your clan isn't pleased with this choice.",
                "You became separated from your clan due to an earthquake, a drow slave raid, or similar event and hope to return home.",
                "You were assigned to become a merchant by the priests of Moradin and have yet to forgive them for their mistake. You should be working a forge, not wandering the outside world!",
                "You are a spy, traveling incognito to gather information for the clan elders.",
                "You struggle to resist the lure of Abbathor, but can't hold it at bay. Better to walk the world and sate your greed on non-dwarves.",
            ],
        }.choose(rng).unwrap()).to_string()
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
    clan_status: String,
    clan_trait: String,
    clan_vocation: String,
    quirk: String,
    story_hook: String,
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

impl Backstory for Dwarf {
    fn backstory(&self) -> Vec<String> {
        vec![
            format!("Clan's Status: {}", self.clan_status),
            format!("Clan's Notable Trait: {}", self.clan_trait),
            format!("Clan Vocation: {}", self.clan_vocation),
            format!("Quirk: {}", self.quirk),
            format!("Reason for Adventuring: {}", self.story_hook),
        ]
    }
}

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
            DwarfSubrace::Duergar => Citation(Book::Mtof, 81),
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
                    citation: Citation(Book::Mtof, 81),
                },
                // You have advantage on saving throws against illusions and against being charmed or paralyzed.
                Feature {
                    title: "Duergar Resiliance",
                    citation: Citation(Book::Mtof, 81),
                },
                // When you reach 3rd level, you can cast the Enlarge/Reduce spell on yourself once with this trait, using only the spell's enlarge option. When you reach 5th level, you can cast the Invisibility spell on yourself once with this trait. You don't need material components for either spell, and you can't cast them while you're in direct sunlight, although sunlight has no effect on them once cast. You regain the ability to cast these spells with this trait when you finish a long rest. Intelligence is your spellcasting ability for these spells.
                Feature {
                    title: "Duergar Magic",
                    citation: Citation(Book::Mtof, 81),
                },
                // You have disadvantage on Attack rolls and Wisdom (Perception) checks that rely on sight when you, the target of your attack, or whatever you are trying to perceive is in direct sunlight.
                Feature {
                    title: "Sunlight Sensitivity",
                    citation: Citation(Book::Mtof, 81),
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

impl PersonalityOptions for Dwarf {}

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
        let subrace = DwarfSubrace::gen(rng);
        let race = Box::new(Self {
            clan_status: subrace.clan_status(rng),
            clan_trait: subrace.clan_trait(rng),
            clan_vocation: DwarfSubrace::clan_vocation(rng),
            quirk: subrace.quirk(rng),
            story_hook: subrace.story_hook(rng),
            subrace,
        });
        let characteristics = race.gen_characteristics(rng);
        let name = subrace.gen_name(rng, &characteristics);
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

impl Trinkets for Dwarf {}

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
            .map(|subrace| format!(
                "{}",
                Dwarf {
                    subrace,
                    clan_status: String::new(),
                    clan_trait: String::new(),
                    clan_vocation: String::new(),
                    quirk: String::new(),
                    story_hook: String::new(),
                }
            ))
            .collect::<Vec<String>>()
            .join("\n\n"));
    }

    #[test]
    fn test_snapshot_abilities() {
        insta::assert_yaml_snapshot!(DwarfSubrace::iter()
            .map(|subrace| (Dwarf {
                subrace,
                clan_status: String::new(),
                clan_trait: String::new(),
                clan_vocation: String::new(),
                quirk: String::new(),
                story_hook: String::new(),
            })
            .abilities())
            .collect::<Vec<Vec<AbilityScore>>>());
    }

    #[test]
    fn test_snapshot_citations() {
        insta::assert_yaml_snapshot!(DwarfSubrace::iter()
            .map(|subrace| (Dwarf {
                subrace,
                clan_status: String::new(),
                clan_trait: String::new(),
                clan_vocation: String::new(),
                quirk: String::new(),
                story_hook: String::new(),
            })
            .citations())
            .collect::<Vec<CitationList>>());
    }

    #[test]
    fn test_snapshot_features() {
        insta::assert_yaml_snapshot!(DwarfSubrace::iter()
            .map(|subrace| (Dwarf {
                subrace,
                clan_status: String::new(),
                clan_trait: String::new(),
                clan_vocation: String::new(),
                quirk: String::new(),
                story_hook: String::new(),
            })
            .features())
            .collect::<Vec<Vec<Feature>>>());
    }
}
