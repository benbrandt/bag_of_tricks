#![allow(clippy::default_trait_access)]
use std::fmt;

use alignment::{AlignmentInfluences, Attitude, Morality};
use attack::Resistances;
use characteristics::{
    names::gith::{GITHYANKI_FEMALE, GITHYANKI_MALE, GITHZERAI_FEMALE, GITHZERAI_MALE},
    AgeRange, Appearance, CharacteristicDetails, Characteristics, Gender, HeightAndWeightTable,
    Size, Speed,
};
use citation::{Book, Citation, CitationList, Citations};
use personality::{Influence, PersonalityOptions};
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
    equipment::{armor::ArmorType, weapons::WeaponType},
    features::{Feature, Features},
    languages::{Language, Languages},
    proficiencies::{Proficiencies, Proficiency, ProficiencyOption, WeaponProficiency},
};

use super::Race;

mod height_and_weight {
    use characteristics::{in_inches, HeightAndWeightTable, WeightMod};
    use dice_roller::{Die, RollCmd};

    pub(crate) const GITHYANKI: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(5, 0),
        base_weight: 100,
        height_mod: RollCmd(2, Die::D12),
        weight_mod: WeightMod::Roll(RollCmd(2, Die::D4)),
    };
    pub(crate) const GITHZERAI: HeightAndWeightTable = HeightAndWeightTable {
        base_height: in_inches(4, 11),
        base_weight: 90,
        height_mod: RollCmd(2, Die::D12),
        weight_mod: WeightMod::Roll(RollCmd(1, Die::D4)),
    };
}

#[derive(Clone, Debug, Deserialize, Display, EnumIter, PartialEq, Serialize)]
enum GithSubrace {
    Githyanki,
    Githzerai,
}

impl GithSubrace {
    fn gen_name(
        &self,
        rng: &mut impl Rng,
        CharacteristicDetails { gender, .. }: &CharacteristicDetails,
    ) -> String {
        let first_names = match (self, gender) {
            (Self::Githyanki, Gender::Female) => GITHYANKI_FEMALE,
            (Self::Githyanki, Gender::Male) => GITHYANKI_MALE,
            (Self::Githzerai, Gender::Female) => GITHZERAI_FEMALE,
            (Self::Githzerai, Gender::Male) => GITHZERAI_MALE,
        };
        (*first_names.choose(rng).unwrap()).to_string()
    }
}

impl PersonalityOptions for GithSubrace {
    fn bonds(&self) -> Vec<String> {
        (match self {
            GithSubrace::Githyanki => [
                "There is no greater duty than to serve the Revered Queen.",
                "Humanity thrives only because we conquered the illithids. Therefore, what is theirs is ours.",
                "Without battle, life has no purpose.",
                "Life is but a spark in the dark. We all go dark, but those who dare can burn bright.",
            ],
            GithSubrace::Githzerai => [
                "Zerthimon provides an example of conduct that I strive to duplicate.",
                "Menyar-Ag hand-picked me for my duties, and I will never betray the trust he showed in me.",
                "Vlaakith and her toadies will be defeated, if not by me then by those who follow in my footsteps.",
                "I will not rest until the last elder brain is destroyed.",
            ],
        })
        .iter()
        .map(|&s| s.to_string())
        .collect()
    }

    fn flaws(&self) -> Vec<String> {
        (match self {
            GithSubrace::Githyanki => [
                "Hunger and thirst are unbearable pains to me.",
                "I can't see a non-githyanki as a real threat.",
                "I follow orders, regardless of their implications.",
                "I start projects but never finish them.",
            ],
            GithSubrace::Githzerai => [
                "I see githyanki machinations behind every threat.",
                "I believe in the supremacy of the gith and that githzerai and githyanki will align to rule the multiverse.",
                "I respond to even minor threats with overwhelming displays of force.",
                "The next time I laugh will be the first. The sound of merriment takes me to the edge of violence.",
            ],
        })
        .iter()
        .map(|&s| s.to_string())
        .collect()
    }

    fn ideals(&self) -> Vec<(String, Influence)> {
        (match self {
            GithSubrace::Githyanki => [
                ("Fidelity. Warriors are only as good as the vows they keep.", Influence::Lawful),
                ("Power. The weak rule the strong.", Influence::Evil),
                ("Duty. It is by Vlaakith's will alone that I act.", Influence::Lawful),
                ("Freedom. No strong soul should be enslaved. Better to die first than live as another's puppet.", Influence::Chaotic),
            ],
            GithSubrace::Githzerai => [
                ("Faith. Zerthimon shall return, and I will be worthy to walk beside him.", Influence::Lawful),
                ("Courage. The mind can master anything if it is unfettered by fear.", Influence::Any),
                ("Duty. My people survive only because those like me place their needs above our own.", Influence::Lawful),
                ("Freedom. No strong soul should be enslaved. Better to die first than live as another's puppet.", Influence::Chaotic),
            ],
        })
        .iter()
        .map(|&(s, i)| (s.to_string(), i))
        .collect()
    }

    fn traits(&self) -> Vec<String> {
        (match self {
            GithSubrace::Githyanki => [
                "When I'm bored I make my own excitement, and I'm always bored.",
                "I treat others as if they were animals that simply don't know any better.",
                "Violence is a spice that makes life worth living.",
                "Old age is a concept that I find fascinating. Maybe someday I too will be aged.",
            ],
            GithSubrace::Githzerai => [
                "All energy must be expended to a useful end. Frivolity is the first step to defeat.",
                "Patience in all things. The first step in any venture is the most treacherous.",
                "Emotions are a trap, meant to weaken the intellect and disturb the nerves. Pay them no heed.",
                "Begin only those tasks you will finish. Strike only that which you will kill.",
            ],
        })
        .iter()
        .map(|&s| s.to_string())
        .collect()
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Gith {
    /// Randomly chosen subrace
    subrace: GithSubrace,
}

impl AlignmentInfluences for Gith {
    fn attitude(&self) -> Vec<Attitude> {
        vec![Attitude::Lawful]
    }

    fn morality(&self) -> Vec<Morality> {
        match self.subrace {
            GithSubrace::Githyanki => vec![Morality::Evil],
            GithSubrace::Githzerai => vec![Morality::Neutral],
        }
    }
}

impl Appearance for Gith {}

impl Backstory for Gith {}

impl Characteristics for Gith {
    const SIZE: Size = Size::Medium;

    fn get_age_range(&self) -> AgeRange {
        AgeRange(10..=100)
    }

    fn get_base_speeds(&self) -> Vec<Speed> {
        vec![Speed::Walking(30)]
    }

    fn get_height_and_weight_table(&self) -> &HeightAndWeightTable {
        match self.subrace {
            GithSubrace::Githyanki => &height_and_weight::GITHYANKI,
            GithSubrace::Githzerai => &height_and_weight::GITHZERAI,
        }
    }
}

impl Citations for Gith {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Mtof, 96)])
    }
}

impl Features for Gith {
    fn features(&self) -> Vec<Feature> {
        match self.subrace {
            GithSubrace::Githyanki => vec![
                // You know the mage hand cantrip, and the hand is invisible when you cast the cantrip with this trait.
                // When you reach 3rd level, you can cast the jump spell once with this trait, and you regain the ability to do so when you finish a long rest. When you reach 5th level, you can cast the misty step spell once with this trait, and you regain the ability to do so when you finish a long rest.
                // Intelligence is your spellcasting ability for these spells. When you cast them with this trait, they don't require components.
                Feature {
                    title: "Githyanki Psionics",
                    citation: Citation(Book::Mtof, 96),
                },
            ],

            GithSubrace::Githzerai => vec![
                // You have advantage on saving throws against the charmed and frightened conditions. Under the tutelage of monastic masters, githzerai learn to govern their own minds.
                Feature {
                    title: "Mental Discipline",
                    citation: Citation(Book::Mtof, 96),
                },
                // You know the mage hand cantrip, and the hand is invisible when you cast the cantrip with this trait.
                // When you reach 3rd level, you can cast the shield spell once with this trait, and you regain the ability to do so when you finish a long rest. When you reach 5th level, you can cast the detect thoughts spell once with this trait, and you regain the ability to do so when you finish a long rest.
                // Wisdom is your spellcasting ability for these spells. When you cast them with this trait, they don't require components.
                Feature {
                    title: "Githzerai Psionics",
                    citation: Citation(Book::Mtof, 96),
                },
            ],
        }
    }
}

impl Languages for Gith {
    fn languages(&self) -> Vec<Language> {
        vec![Language::Common, Language::Gith]
    }

    fn addl_languages(&self) -> usize {
        match self.subrace {
            GithSubrace::Githyanki => 1,
            GithSubrace::Githzerai => 0,
        }
    }
}

impl PersonalityOptions for Gith {
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

impl Proficiencies for Gith {
    fn proficiencies(&self) -> Vec<Proficiency> {
        match self.subrace {
            GithSubrace::Githyanki => vec![
                Proficiency::Armor(ArmorType::Light),
                Proficiency::Armor(ArmorType::Medium),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Greatsword)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Longsword)),
                Proficiency::Weapon(WeaponProficiency::Specific(WeaponType::Shortsword)),
            ],
            GithSubrace::Githzerai => vec![],
        }
    }

    fn addl_proficiencies(&self) -> Vec<ProficiencyOption> {
        match self.subrace {
            GithSubrace::Githyanki => vec![ProficiencyOption::FromOptions(
                vec![
                    ProficiencyOption::Skill(None, 1),
                    ProficiencyOption::Tool(1),
                ],
                1,
            )],
            GithSubrace::Githzerai => vec![],
        }
    }
}

#[typetag::serde]
impl Race for Gith {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, CharacteristicDetails) {
        let subrace = GithSubrace::iter().choose(rng).unwrap();
        let race = Box::new(Self {
            subrace: subrace.clone(),
        });
        let characteristics = race.gen_characteristics(rng);
        let name = subrace.gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> Vec<AbilityScore> {
        vec![
            AbilityScore(AbilityScoreType::Intelligence, 1),
            match self.subrace {
                GithSubrace::Githyanki => AbilityScore(AbilityScoreType::Strength, 2),
                GithSubrace::Githzerai => AbilityScore(AbilityScoreType::Wisdom, 2),
            },
        ]
    }
}

impl Resistances for Gith {}

impl Trinkets for Gith {}

impl fmt::Display for Gith {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.subrace)
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
        let gith = Gith::gen(&mut rng);
        insta::assert_yaml_snapshot!(gith);
    }

    #[test]
    fn test_snapshot_display() {
        insta::assert_snapshot!(GithSubrace::iter()
            .map(|subrace| format!("{}", Gith { subrace }))
            .collect::<Vec<String>>()
            .join("\n\n"));
    }

    #[test]
    fn test_snapshot_abilities() {
        insta::assert_yaml_snapshot!(GithSubrace::iter()
            .map(|subrace| (Gith { subrace }).abilities())
            .collect::<Vec<Vec<AbilityScore>>>());
    }

    #[test]
    fn test_snapshot_citations() {
        insta::assert_yaml_snapshot!(GithSubrace::iter()
            .map(|subrace| (Gith { subrace }).citations())
            .collect::<Vec<CitationList>>());
    }

    #[test]
    fn test_snapshot_features() {
        insta::assert_yaml_snapshot!(GithSubrace::iter()
            .map(|subrace| (Gith { subrace }).features())
            .collect::<Vec<Vec<Feature>>>());
    }
}
