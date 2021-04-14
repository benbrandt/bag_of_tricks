use rand::{prelude::IteratorRandom, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Ethnicity options, which determine name lists
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

/// Name options for a given ethnicity
pub(crate) struct Names<'a> {
    pub(crate) female: &'a [&'a str],
    pub(crate) male: &'a [&'a str],
    pub(crate) surname: &'a [&'a str],
}

impl Names<'_> {
    /// Choose a random ethnicity and then map to the name options
    pub(crate) fn gen_names(rng: &mut impl Rng) -> Self {
        match Ethnicity::iter().choose(rng).unwrap() {
            Ethnicity::Calishite => CALISHITE,
            Ethnicity::Chondathan | Ethnicity::Tethyrian => CHONDATHAN,
            Ethnicity::Damaran => DAMARAN,
            Ethnicity::Illuskan => ILLUSKAN,
            Ethnicity::Mulan => MULAN,
            Ethnicity::Rashemi => RASHEMI,
            Ethnicity::Shou => SHOU,
            Ethnicity::Turami => TURAMI,
        }
    }
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
        "Arveene", "Elsa", "Esvele", "Freda", "Halia", "Jhessail", "Kerri", "Linene", "Lureene",
        "Miri", "Mirna", "Nilsa", "Rowan", "Shandri", "Tessele", "Thistle", "Trilena",
    ],
    male: &[
        "Aldith", "Ander", "Daren", "Darvin", "Dorn", "Elmar", "Evendur", "Favric", "Gorstag",
        "Grim", "Harbin", "Helm", "Iarno", "Lanar", "Malark", "Morn", "Nars", "Narth", "Palien",
        "Pip", "Randal", "Sildar", "Stedd", "Thel", "Toblen",
    ],
    surname: &[
        "Albrek",
        "Amblecrown",
        "Barthen",
        "Buckman",
        "Dendrar",
        "Dundragon",
        "Edermath",
        "Evenwood",
        "Graywind",
        "Greycastle",
        "Hallwinter",
        "Stonehill",
        "Tallstag",
        "Thornton",
        "Tresendar",
        "Wester",
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
        "Amafrey", "Betha", "Cefrey", "Kethra", "Mara", "Olga", "Silifrey", "Teega", "Westra",
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
        "Hamun",
        "Kethoth",
        "Mumed",
        "Ramas",
        "Reidoth",
        "So-Kehur",
        "Thazar-De",
        "Urhur",
    ],
    surname: &[
        "Ankhalab",
        "Anskuld",
        "Fezim",
        "Hahpet",
        "Kost",
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
        "Gallio",
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
        "Elibro",
        "Iltazyara",
        "Murnyethara",
        "Stayanoga",
        "Ulmokina",
    ],
};

const SHOU: Names = Names {
    female: &["Bai", "Chao", "Jia", "Lei", "Mei", "Qiao", "Shui", "Tai"],
    male: &[
        "An", "Chen", "Chi", "Fai", "Jiang", "Jun", "Lian", "Long", "Meng", "On", "Shan", "Shui",
        "Wen",
    ],
    surname: &[
        "Chien", "Huang", "Kao", "Kung", "Lao", "Ling", "Mei", "Pin", "Shin", "Sum", "Tan", "Wan",
    ],
};

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
