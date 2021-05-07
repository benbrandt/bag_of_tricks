#![allow(clippy::default_trait_access)]
use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::{DamageType, Resistances};
use characteristics::{
    names::{
        elf::{CHILD, FAMILY, FEMALE, MALE},
        Name,
    },
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
use trinkets::{TrinketOption, Trinkets};

use crate::{
    ability::{AbilityScore, AbilityScoreType, Skill},
    backstory::Backstory,
    equipment::weapons::WeaponType,
    features::{Feature, Features},
    languages::{Language, Languages},
    proficiencies::{Proficiencies, Proficiency, WeaponProficiency},
};

use super::Race;

mod height_and_weight {
    use characteristics::{in_inches, HeightAndWeightTable, WeightMod};
    use dice_roller::{Die, RollCmd};

    pub(crate) const DARK: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 5),
        base_weight: 75,
        height_mod: RollCmd(2, Die::D6),
        weight_mod: WeightMod::Roll(RollCmd(1, Die::D6)),
    };
    pub(crate) const ELADRIN: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 6),
        base_weight: 90,
        height_mod: RollCmd(2, Die::D12),
        weight_mod: WeightMod::Roll(RollCmd(1, Die::D4)),
    };
    pub(crate) const HIGH: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 6),
        base_weight: 90,
        height_mod: RollCmd(2, Die::D10),
        weight_mod: WeightMod::Roll(RollCmd(1, Die::D4)),
    };
    pub(crate) const SEA: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 6),
        base_weight: 90,
        height_mod: RollCmd(2, Die::D8),
        weight_mod: WeightMod::Roll(RollCmd(1, Die::D4)),
    };
    pub(crate) const SHADAR_KAI: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 8),
        base_weight: 90,
        height_mod: RollCmd(2, Die::D8),
        weight_mod: WeightMod::Roll(RollCmd(1, Die::D4)),
    };
    pub(crate) const WOOD: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 6),
        base_weight: 100,
        height_mod: RollCmd(2, Die::D10),
        weight_mod: WeightMod::Roll(RollCmd(1, Die::D4)),
    };
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
pub(crate) enum DrowHouseSpecialty {
    #[strum(serialize = "Adamantine weapons")]
    Adamantine,
    #[strum(serialize = "Assassinations")]
    Assassinations,
    #[strum(serialize = "Giant spiders subject to magical control")]
    Spiders,
    #[strum(serialize = "Hallucinogenic substances")]
    Substances,
    #[strum(serialize = "High-status slaves and sacrificial victims")]
    HighStatusSlaves,
    #[strum(serialize = "Items taken from surface world in raids")]
    Items,
    #[strum(serialize = "Low-cost, humanoid slaves")]
    LowCostSlaves,
    #[strum(serialize = "Maps of the Underdark")]
    Maps,
    #[strum(serialize = "Poisons")]
    Poisons,
    #[strum(serialize = "Reptilian beasts of burden")]
    Beasts,
}

impl Default for DrowHouseSpecialty {
    fn default() -> Self {
        Self::Adamantine
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
pub(crate) enum EladrinVariant {
    Autumn,
    Winter,
    Spring,
    Summer,
}

impl PersonalityOptions for EladrinVariant {
    fn flaws(&self) -> Vec<String> {
        (match self {
            Self::Autumn => [
                "You trust others without a second thought.",
                "You give to others, to the point that you leave yourself without necessary supplies.",
                "Everyone is your friend, or a potential friend.",
                "You spend excessively on creature comforts.",
            ],
            Self::Winter => [
                "Everything dies eventually. Why bother building anything that is supposedly meant to last?",
                "Nothing matters to you, and you allow others to guide your actions.",
                "Your needs come first. In winter, all must watch out for themselves.",
                "You speak only to point out the flaws in others' plans.",
            ],
            Self::Spring => [
                "You overdrink.",
                "Toil is for drudges. Yours should be a life of leisure.",
                "A pretty face infatuates you in an instant, but your fancy passes with equal speed.",
                "Anything worth doing is worth doing again and again.",
            ],
            Self::Summer => [
                "You are stubborn. Let others change.",
                "The best option is one that is swift, unexpected, and overwhelming.",
                "Punch first. Talk later.",
                "Your fury can carry you through anything.",
            ]
        }).iter().map(|&s| s.to_string()).collect()
    }

    fn traits(&self) -> Vec<String> {
        (match self {
            Self::Autumn => [
                "If someone is in need, you never withhold aid.",
                "You share what you have, with little regard for your own needs.",
                "There are no simple meals, only lavish feasts.",
                "You stock up on fine food and drink. You hate going without such comforts.",
            ],
            Self::Winter => [
                "The worst case is the most likely to occur.",
                "You preserve what you have. Better to be hungry today and have food for tomorrow.",
                "Life is full of dangers, but you are ready for them.",
                "A penny spent is a penny lost forever.",
            ],
            Self::Spring => [
                "Every day is the greatest day of your life.",
                "You approach everything with enthusiasm, even the most mundane chores.",
                "You love music and song. You supply a tune yourself if no one else can.",
                "You can't stay still.",
            ],
            Self::Summer => [
                "You believe that direct confrontation is the best way to solve problems.",
                "Overwhelming force can accomplish almost anything. The tougher the problem, the more force you apply.",
                "You stand tall and strong so that others can lean on you.",
                "You maintain an intimidating front. It's better to prevent fights with a show of force than to harm others.",
            ]
        }).iter().map(|&s| s.to_string()).collect()
    }
}

impl Default for EladrinVariant {
    fn default() -> Self {
        Self::Autumn
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
pub(crate) enum HighVariant {
    High,
    Moon,
    Sun,
}

impl Default for HighVariant {
    fn default() -> Self {
        Self::High
    }
}

#[derive(Deserialize, EnumIter, Serialize)]
pub(crate) enum ElfSubrace {
    Dark(DrowHouseSpecialty),
    Eladrin(EladrinVariant),
    High(HighVariant),
    Sea,
    ShadarKai,
    Wood,
}

impl ElfSubrace {
    pub(crate) fn gen(rng: &mut impl Rng) -> Self {
        let subrace = Self::iter().choose(rng).unwrap();
        match subrace {
            Self::Dark(_) => Self::Dark(DrowHouseSpecialty::iter().choose(rng).unwrap()),
            Self::Eladrin(_) => Self::Eladrin(EladrinVariant::iter().choose(rng).unwrap()),
            Self::High(_) => Self::High(HighVariant::iter().choose(rng).unwrap()),
            Self::Sea | Self::ShadarKai | Self::Wood => subrace,
        }
    }

    fn story_hook(&self, rng: &mut impl Rng) -> String {
        (*(match self {
            ElfSubrace::Dark(_) => [
                "You overheard members of your own house plotting to poison you, so you fled from the Underdark to save yourself. You won't return until you've amassed enough fortune to surround yourself with loyal mercenary bodyguards.",
                "You were enslaved as punishment for trying to poison an influential rival, but you escaped and fled to the surface. If you return to the Underdark and are captured, you'll be re-enslaved.",
                "You were the lover of a high-ranking priestess of Lolth as a means of enhancing your status. When she tired of you, the loss of status was humiliating, so you left.",
                "You killed a drow from a more powerful house in a duel over a public insult. The slain drow's house vowed to destroy your house unless you were handed over. Your kin urged you to leave the Underdark. You wonder what became of them.",
                "A close friend of yours was revealed to be a worshiper of Eilistraee. Suspicion fell on everyone in her circle. Running was a tacit admission of guilt, even though you knew nothing about it, but you'd have been sacrificed to Lolth if you stayed.",
                "You were among a group of surface raiders that was ambushed, and you were captured. During years of captivity, you learned that most of what Lolth's priestesses taught about the outer world was lies. Now you're experiencing the truth for yourself.",
                "All your life, you were alienated and terrified by the cruelty of your kin. The first chance you got, you volunteered to go on a surface raid, then deserted the group and remained behind. Now you're hated and feared wherever you go, but at least you've found a small group of adventurous friends who trust and support each other.",
                "You were part of a delegation carrying diplomatic messages to another drow city when duergar attacked the caravan for slaves and treasure. Only you and one other guard escaped. If you'd returned home, you'd have been poisoned or worse for failure. Becoming a mercenary was your best option.",
            ],
            ElfSubrace::Eladrin(_) |
            ElfSubrace::High(_) |
            ElfSubrace::Sea |
            ElfSubrace::ShadarKai |
            ElfSubrace::Wood => [
                "You believe the key to reuniting the elves with Corellon lies somewhere in the wider world, not within elven society, and you're determined to find it.",
                "Your sibling was killed by a rampaging monster. You won't rest until you track it down and slay it.",
                "A raven brought you a cryptic message from an old friend who needs your help, but the message was vague about the friend's location. You're trying to follow a years-old trail and save your friend.",
                "A beautiful elf won your heart, then broke it. If you earn enough gold and glory by adventuring, perhaps you can win back your love.",
                "Your father thought you too weak to survive as an adventurer, but he's wrong, and you'll prove it.",
                "Only those who perform great deeds are remembered long after their death. Bards will honor your exploits for generations to come.",
                "You're secretly in love with one of the other members of your adventuring group, and you can't bear the thought of any harm befalling that person.",
                "When you were born, your grandmother prophesied you would one day rule a human kingdom. You've gone in search of that destiny.",
            ]
        }).choose(rng).unwrap()).to_string()
    }
}

impl PersonalityOptions for ElfSubrace {
    fn flaws(&self) -> Vec<String> {
        match self {
            Self::Eladrin(v) => v.flaws(),
            Self::Dark(_) | Self::High(_) | Self::Sea | Self::ShadarKai | Self::Wood => vec![],
        }
    }

    fn traits(&self) -> Vec<String> {
        match self {
            Self::Eladrin(v) => v.traits(),
            Self::Dark(_) | Self::High(_) | Self::Sea | Self::ShadarKai | Self::Wood => vec![],
        }
    }
}

impl fmt::Display for ElfSubrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dark(_) => write!(f, "Dark"),
            Self::Eladrin(v) => write!(f, "{} Eladrin", v),
            Self::High(v) => write!(f, "{}", v),
            Self::Sea => write!(f, "Sea"),
            Self::ShadarKai => write!(f, "Shadar-kai"),
            Self::Wood => write!(f, "Wood"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Elf {
    story_hook: String,
    /// Randomly chosen subrace
    subrace: ElfSubrace,
}

impl Elf {
    /// Before the age of 100, elves go by their child name
    pub(crate) fn gen_first_name<'a>(
        rng: &mut impl Rng,
        CharacteristicDetails { age, gender, .. }: &CharacteristicDetails,
    ) -> &'a str {
        let first_names = match age {
            1..=100 => CHILD,
            _ => match gender {
                Gender::Female => FEMALE,
                Gender::Male => MALE,
            },
        };
        first_names.choose(rng).unwrap()
    }

    pub(crate) fn gen_family_name<'a>(rng: &mut impl Rng) -> &'a str {
        FAMILY.choose(rng).unwrap()
    }

    pub(crate) fn weapon_training(subrace: &ElfSubrace) -> Vec<Proficiency> {
        match subrace {
            ElfSubrace::Dark(_) => vec![
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::CrossbowHand)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Rapier)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortsword)),
            ],
            ElfSubrace::High(_) => vec![
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Longbow)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Longsword)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortsword)),
            ],
            ElfSubrace::Sea => {
                vec![
                    Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::CrossbowLight)),
                    Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Net)),
                    Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Spear)),
                    Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Trident)),
                ]
            }
            ElfSubrace::Wood => vec![
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Longbow)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Longsword)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortbow)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortsword)),
            ],
            ElfSubrace::Eladrin(_) | ElfSubrace::ShadarKai => vec![],
        }
    }
}

impl AlignmentInfluences for Elf {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Chaotic]
    }

    fn morality(&self) -> Vec<Morality> {
        vec![match self.subrace {
            ElfSubrace::Dark(_) | ElfSubrace::ShadarKai => Morality::Evil,
            ElfSubrace::Eladrin(_) | ElfSubrace::High(_) | ElfSubrace::Sea | ElfSubrace::Wood => {
                Morality::Good
            }
        }]
    }
}

impl Appearance for Elf {}

impl Backstory for Elf {
    fn backstory(&self) -> Vec<String> {
        let mut backstory = vec![format!("Reason for Adventuring: {}", self.story_hook)];
        if let ElfSubrace::Dark(s) = &self.subrace {
            backstory.push(format!("Drow House Specialty: {}", s));
        }
        backstory
    }
}

impl Characteristics for Elf {
    const SIZE: Size = Size::Medium;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=750)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        let mut speeds = vec![match self.subrace {
            ElfSubrace::Dark(_)
            | ElfSubrace::Eladrin(_)
            | ElfSubrace::High(_)
            | ElfSubrace::Sea
            | ElfSubrace::ShadarKai => Speed::Walking(30),
            ElfSubrace::Wood => Speed::Walking(35),
        }];
        if matches!(self.subrace, ElfSubrace::Sea) {
            speeds.push(Speed::Swimming(30));
        }
        speeds
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        match self.subrace {
            ElfSubrace::Dark(_) => &height_and_weight::DARK,
            ElfSubrace::Eladrin(_) => &height_and_weight::ELADRIN,
            ElfSubrace::High(_) => &height_and_weight::HIGH,
            ElfSubrace::Sea => &height_and_weight::SEA,
            ElfSubrace::ShadarKai => &height_and_weight::SHADAR_KAI,
            ElfSubrace::Wood => &height_and_weight::WOOD,
        }
    }
}

impl Citations for Elf {
    fn citations(&self) -> CitationList {
        let race = Citation(Book::Phb, 21);
        let subrace = match self.subrace {
            ElfSubrace::Dark(_) | ElfSubrace::Wood => Citation(Book::Phb, 24),
            ElfSubrace::Eladrin(_) => Citation(Book::Mtof, 61),
            ElfSubrace::High(HighVariant::High) => Citation(Book::Phb, 23),
            ElfSubrace::High(HighVariant::Moon) => Citation(Book::Scag, 105),
            ElfSubrace::High(HighVariant::Sun) => Citation(Book::Scag, 106),
            ElfSubrace::Sea | ElfSubrace::ShadarKai => Citation(Book::Mtof, 62),
        };
        CitationList(vec![race, subrace])
    }
}

impl Features for Elf {
    fn features(&self) -> Vec<Feature> {
        let mut features = vec![
            // Accustomed to twilit forests and the night sky, you have superior vision in dark and dim conditions. You can see in dim light within 60 feet of you as if it were bright light, and in darkness as if it were dim light. You can't discern color in darkness, only shades of gray.
            Feature {
                title: "Darkvision",
                citation: Citation(Book::Phb, 23),
            },
            // You have advantage on saving throws against being charmed, and magic can't put you to sleep.
            Feature {
                title: "Fey Ancestry",
                citation: Citation(Book::Phb, 23),
            },
            // Elves don't need to sleep. Instead, they meditate deeply, remaining semiconscious, for 4 hours a day. (The Common word for such meditation is \"trance.\") While meditating, you can dream after a fashion; such dreams are actually mental exercises that have become reflexive through years of practice. After resting in this way, you gain the same benefit that a human does from 8 hours of sleep.
            Feature {
                title: "Trance",
                citation: Citation(Book::Phb, 23),
            },
        ];
        features.extend(match self.subrace {
            ElfSubrace::Dark(_) => vec![
                // Your darkvision has a radius of 120 feet.
                Feature {
                    title: "Superior Darkvision",
                    citation: Citation(Book::Phb, 24),
                },
                // You have disadvantage on attack rolls and on Wisdom (Perception) checks that rely on sight when you, the target of your attack, or whatever you are trying to perceive is in direct sunlight.
                Feature {
                    title: "Sunlight Sensitivity",
                    citation: Citation(Book::Phb, 24),
                },
                // You know the dancing lights cantrip. When you reach 3rd level, you can cast the faerie fire spell once with this trait and regain the ability to do so when you finish a long rest. When you reach 5th level, you can cast the darkness spell once with this trait and regain the ability to do so when you finish a long rest. Charisma is your spellcasting ability for these spells.
                Feature {
                    title: "Drow Magic",
                    citation: Citation(Book::Phb, 24),
                },
            ],
            ElfSubrace::Eladrin(_) => vec![
                // As a bonus action, you can magically teleport up to 30 feet to an unoccupied space you can see. Once you use this trait, you can't do so again until you finish a short or long rest.
                // When you reach 3rd level, your Fey Step gains an additional effect based on your season; if the effect requires a saving throw, the DC equals 8 + your proficiency bonus + your Charisma modifier:
                // Autumn. Immediately after you use your Fey Step, up to two creatures of your choice that you can see within 10 feet of you must succeed on a Wisdom saving throw or be charmed by you for 1 minute, or until you or your companions deal any damage to it.
                // Winter. When you use your Fey Step, one creature of your choice that you can see within 5 feet of you before you teleport must succeed on a Wisdom saving throw or be frightened of you until the end of your next turn.
                // Spring. When you use your Fey Step, you can touch one willing creature within 5 feet of you. That creature then teleports instead of you, appearing in an unoccupied space of your choice that you can see within 30 feet of you.
                // Summer. Immediately after you use your Fey Step, each creature of your choice that you can see within 5 feet of you takes fire damage equal to your Charisma modifier (minimum of 1 damage).
                Feature {
                    title: "Fey Step",
                    citation: Citation(Book::Mtof, 62),
                },
            ],
            // You know one cantrip of your choice from the wizard spell list. Intelligence is your spellcasting ability for it.
            ElfSubrace::High(_) => vec![Feature {
                title: "Cantrip",
                citation: Citation(Book::Phb, 24),
            }],
            ElfSubrace::Sea => vec![
                // You have a swimming speed of 30 feet, and you can breath air and water.
                Feature {
                    title: "Child of the Sea",
                    citation: Citation(Book::Mtof, 62),
                },
                // Using gestures and sounds, you can communicate simple ideas with any beast that has an innate swimming speed.
                Feature {
                    title: "Friend of the Sea",
                    citation: Citation(Book::Mtof, 62),
                },
            ],
            ElfSubrace::ShadarKai => vec![
                // As a bonus action, you can magically teleport up to 30 feet to an unoccupied space you can see. Once you use this trait, you can't do so again until you finish a long rest.
                // Starting at 3rd level, you also gain resistance to all damage when you teleport using this trait. The resistance lasts until the start of your next turn. During that time, you appear ghostly and translucent.
                Feature {
                    title: "Blessing of the Raven Queen",
                    citation: Citation(Book::Mtof, 63),
                },
            ],
            // You can attempt to hide even when you are only lightly obscured by foliage, heavy rain, falling snow, mist, and other natural phenomena.
            ElfSubrace::Wood => vec![Feature {
                title: "Mask of the Wild",
                citation: Citation(Book::Phb, 24),
            }],
        });
        features
    }
}

impl Languages for Elf {
    fn languages(&self) -> Vec<Language> {
        let mut languages = vec![Language::Common, Language::Elvish];
        if matches!(self.subrace, ElfSubrace::Sea) {
            languages.push(Language::Aquan);
        }
        languages
    }

    fn addl_languages(&self) -> usize {
        match self.subrace {
            ElfSubrace::High(_) => 1,
            ElfSubrace::Dark(_)
            | ElfSubrace::Eladrin(_)
            | ElfSubrace::Sea
            | ElfSubrace::ShadarKai
            | ElfSubrace::Wood => 0,
        }
    }
}

impl Name for Elf {
    fn gen_name(rng: &mut impl Rng, characteristics: &CharacteristicDetails) -> String {
        format!(
            "{} {}",
            Self::gen_first_name(rng, characteristics),
            Self::gen_family_name(rng),
        )
    }
}

impl PersonalityOptions for Elf {
    fn flaws(&self) -> Vec<String> {
        self.subrace.flaws()
    }

    fn traits(&self) -> Vec<String> {
        self.subrace.traits()
    }
}

impl Proficiencies for Elf {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies = vec![Proficiency::Skill(Skill::Perception)];
        proficiencies.extend(Self::weapon_training(&self.subrace));
        proficiencies
    }
}

#[typetag::serde]
impl Race for Elf {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let subrace = ElfSubrace::gen(rng);
        let race = Box::new(Self {
            story_hook: subrace.story_hook(rng),
            subrace,
        });
        let characteristics = race.gen_characteristics(rng);
        let name = Self::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Dexterity, 2),
            match self.subrace {
                ElfSubrace::Dark(_) | ElfSubrace::Eladrin(_) => {
                    AbilityScore(AbilityScoreType::Charisma, 1)
                }
                ElfSubrace::High(_) => AbilityScore(AbilityScoreType::Intelligence, 1),
                ElfSubrace::Sea | ElfSubrace::ShadarKai => {
                    AbilityScore(AbilityScoreType::Constitution, 1)
                }
                ElfSubrace::Wood => AbilityScore(AbilityScoreType::Wisdom, 1),
            },
        ]
    }
}

impl Resistances for Elf {
    fn resistances(&self) -> Vec<DamageType> {
        match self.subrace {
            ElfSubrace::ShadarKai => vec![DamageType::Necrotic],
            ElfSubrace::Dark(_)
            | ElfSubrace::Eladrin(_)
            | ElfSubrace::High(_)
            | ElfSubrace::Sea
            | ElfSubrace::Wood => vec![],
        }
    }
}

impl Trinkets for Elf {
    fn trinket_options(&self) -> Vec<TrinketOption> {
        vec![TrinketOption::Elven]
    }
}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Elf", self.subrace)
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
        let elf = Elf::gen(&mut rng);
        insta::assert_yaml_snapshot!(elf);
    }

    #[test]
    fn test_snapshot_display() {
        insta::assert_snapshot!(ElfSubrace::iter()
            .map(|subrace| format!(
                "{}",
                Elf {
                    story_hook: String::new(),
                    subrace
                }
            ))
            .collect::<Vec<String>>()
            .join("\n\n"));
    }

    #[test]
    fn test_snapshot_abilities() {
        insta::assert_yaml_snapshot!(ElfSubrace::iter()
            .map(|subrace| (Elf {
                story_hook: String::new(),
                subrace
            })
            .abilities())
            .collect::<Vec<Vec<AbilityScore>>>());
    }

    #[test]
    fn test_snapshot_citations() {
        insta::assert_yaml_snapshot!(ElfSubrace::iter()
            .map(|subrace| (Elf {
                story_hook: String::new(),
                subrace
            })
            .citations())
            .collect::<Vec<CitationList>>());
    }

    #[test]
    fn test_snapshot_features() {
        insta::assert_yaml_snapshot!(ElfSubrace::iter()
            .map(|subrace| (Elf {
                story_hook: String::new(),
                subrace
            })
            .features())
            .collect::<Vec<Vec<Feature>>>());
    }
}
