use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::IntoEnumIterator;

use super::Race;
use crate::{
    character::{
        ability::{AbilityScore, AbilityScoreType, AbilityScores},
        characteristics::{AgeRange, Characteristics},
        features::Feature,
    },
    citation::{Book, Citation, Citations},
};

const AGE_RANGE: AgeRange = AgeRange(1..=100);
mod names {
    use rand::{prelude::IteratorRandom, Rng};
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    use crate::character::characteristics::{Characteristics, Gender};

    struct Names<'a> {
        female: &'a [&'a str],
        male: &'a [&'a str],
        surname: &'a [&'a str],
    }

    #[derive(EnumIter)]
    enum Ethnicity {
        Calishite,
        Chondathan,
        Damaran,
        Illuskan,
        Mulan,
        Rashemi,
        Shou,
        Tethyrian,
        Turami,
    }

    const CALISHITE: Names = Names {
        female: &[
            "Atala", "Ceidil", "Hama", "Jasmal", "Meilil", "Seipora", "Yasheira", "Zasheida",
        ],
        male: &[
            "Aseir", "Bardeid", "Haseid", "Khemed", "Mehmen", "Sudeiman", "Zasheir",
        ],
        surname: &[
            "Basha", "Dumein", "Jassan", "Khalid", "Mostana", "Pashar", "Rein",
        ],
    };

    const CHONDATHAN: Names = Names {
        female: &[
            "Arveene", "Esvele", "Jhessail", "Kerri", "Lureene", "Miri", "Rowan", "Shandri",
            "Tessele",
        ],
        male: &[
            "Darvin", "Dorn", "Evendur", "Gorstag", "Grim", "Helm", "Malark", "Morn", "Randal",
            "Stedd",
        ],
        surname: &[
            "Amblecrown",
            "Buckman",
            "Dundragon",
            "Evenwood",
            "Greycastle",
            "Tallstag",
        ],
    };

    const DAMARAN: Names = Names {
        female: &[
            "Alethra", "Kara", "Katernin", "Mara", "Natali", "Olma", "Tana", "Zora",
        ],
        male: &[
            "Bor", "Fodel", "Glar", "Grigor", "Igan", "Ivor", "Kosef", "Mival", "Orel", "Pavel",
            "Sergor",
        ],
        surname: &[
            "Bersk", "Chernin", "Dotsk", "Kulenov", "Marsk", "Nemetsk", "Shemov", "Starag",
        ],
    };

    const ILLUSKAN: Names = Names {
        female: &[
            "Amafrey", "Betha", "Cefrey", "Kethra", "Mara", "Olga", "Silifrey", "Westra",
        ],
        male: &[
            "Ander", "Blath", "Bran", "Frath", "Geth", "Lander", "Luth", "Malcer", "Stor", "Taman",
            "Urth",
        ],
        surname: &[
            "Brightwood",
            "Helder",
            "Hornraven",
            "Lackman",
            "Stormwind",
            "Windrivver",
        ],
    };

    const MULAN: Names = Names {
        female: &[
            "Arizima", "Chathi", "Nephis", "Nulara", "Murithi", "Sefris", "Thola", "Umara", "Zolis",
        ],
        male: &[
            "Aoth",
            "Bareris",
            "Ehput-Ki",
            "Kethoth",
            "Mumed",
            "Ramas",
            "So-Kehur",
            "Thazar-De",
            "Urhur",
        ],
        surname: &[
            "Ankhalab",
            "Anskuld",
            "Fezim",
            "Hahpet",
            "Nathandem",
            "Sepret",
            "Uuthrakt",
        ],
    };

    const RASHEMI: Names = Names {
        female: &[
            "Fyevarra", "Hulmarra", "Immith", "Imzel", "Navarra", "Shevarra", "Tammith", "Yuldra",
        ],
        male: &[
            "Borivik",
            "Faurgar",
            "Jandar",
            "Kanithar",
            "Madislak",
            "Ralmevik",
            "Shaumar",
            "Vladislak",
        ],
        surname: &[
            "Chergoba",
            "Dyernina",
            "Iltazyara",
            "Murnyethara",
            "Stayanoga",
            "Ulmokina",
        ],
    };

    const SHOU: Names = Names {
        female: &["Bai", "Chao", "Jia", "Lei", "Mei", "Qiao", "Shui", "Tai"],
        male: &[
            "An", "Chen", "Chi", "Fai", "Jiang", "Jun", "Lian", "Long", "Meng", "On", "Shan",
            "Shui", "Wen",
        ],
        surname: &[
            "Chien", "Huang", "Kao", "Kung", "Lao", "Ling", "Mei", "Pin", "Shin", "Sum", "Tan",
            "Wan",
        ],
    };

    const TETHYRIAN: Names = CHONDATHAN;

    const TURAMI: Names = Names {
        female: &[
            "Balama", "Dona", "Faila", "Jalana", "Luisa", "Marta", "Quara", "Selise", "Vonda",
        ],
        male: &[
            "Anton", "Diero", "Marcon", "Pieron", "Rimardo", "Romero", "Salazar", "Umbero",
        ],
        surname: &[
            "Agosto",
            "Astorio",
            "Calabra",
            "Domine",
            "Falone",
            "Marivaldi",
            "Pisacar",
            "Ramondo",
        ],
    };

    pub(crate) fn gen_name(
        rng: &mut impl Rng,
        Characteristics { gender, .. }: &Characteristics,
    ) -> String {
        let ethnicity = match Ethnicity::iter().choose(rng).unwrap() {
            Ethnicity::Calishite => CALISHITE,
            Ethnicity::Chondathan => CHONDATHAN,
            Ethnicity::Damaran => DAMARAN,
            Ethnicity::Illuskan => ILLUSKAN,
            Ethnicity::Mulan => MULAN,
            Ethnicity::Rashemi => RASHEMI,
            Ethnicity::Shou => SHOU,
            Ethnicity::Tethyrian => TETHYRIAN,
            Ethnicity::Turami => TURAMI,
        };
        let first_names = match gender {
            Gender::Female => ethnicity.female,
            Gender::Male => ethnicity.male,
        };
        format!(
            "{} {}",
            first_names.iter().choose(rng).unwrap(),
            ethnicity.surname.iter().choose(rng).unwrap(),
        )
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Human;

#[typetag::serde]
impl Race for Human {
    fn gen(rng: &mut impl Rng) -> (Box<dyn Race>, String, Characteristics) {
        let race = Box::new(Self);
        let characteristics = Characteristics::gen(rng, &AGE_RANGE);
        let name = names::gen_name(rng, &characteristics);
        (race, name, characteristics)
    }

    fn abilities(&self) -> AbilityScores {
        AbilityScores(
            AbilityScoreType::iter()
                .map(|t| AbilityScore(t, 1))
                .collect(),
        )
    }

    fn citations(&self) -> Citations {
        Citations(vec![Citation {
            book: Book::PHB,
            page: 29,
        }])
    }

    fn features(&self) -> Vec<Feature> {
        vec![Feature {
            title: "Ability Score Increase",
            description: "Your ability scores each increase by 1.",
            citation: Citation {
                book: Book::PHB,
                page: 31,
            },
        }]
    }
}

impl fmt::Display for Human {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Human")
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
        let human = Human::gen(&mut rng);
        insta::assert_yaml_snapshot!(human);
    }

    #[test]
    fn test_snapshot_display() {
        let mut rng = Pcg64::seed_from_u64(1);
        let (human, _name, _characteristics) = Human::gen(&mut rng);
        insta::assert_snapshot!(format!("{}", human));
    }

    #[test]
    fn test_snapshot_abilities() {
        let human = Human;
        insta::assert_yaml_snapshot!(human.abilities());
    }

    #[test]
    fn test_snapshot_citations() {
        let human = Human;
        insta::assert_yaml_snapshot!(human.citations());
    }

    #[test]
    fn test_snapshot_features() {
        let human = Human;
        insta::assert_yaml_snapshot!(human.features());
    }
}
