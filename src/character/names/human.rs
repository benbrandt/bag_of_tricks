use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

/// Ethnicity options, which determine name lists
#[derive(Clone, Copy, Deserialize, Display, EnumIter, Serialize)]
pub(crate) enum Ethnicity {
    Arkaiun,
    Bedine,
    Calishite,
    Chondathan,
    Damaran,
    Ffolk,
    Gur,
    Halruaan,
    Illuskan,
    Imaskari,
    Mulan,
    Nar,
    Rashemi,
    Shaaran,
    Shou,
    Tethyrian,
    Tuigan,
    Turami,
    Ulutiun,
}

impl Ethnicity {
    pub(crate) fn gen(rng: &mut impl Rng) -> Self {
        Ethnicity::iter().choose(rng).unwrap()
    }

    pub(crate) fn human_language(self) -> &'static str {
        match self {
            Self::Arkaiun => "Dambrathan (written in Espruar)",
            Self::Bedine => "Midani",
            Self::Calishite => "Alzhedo",
            Self::Chondathan | Self::Tethyrian => "Chondothan",
            Self::Damaran => "Damaran (written in Dethek)",
            Self::Ffolk => "Waelan",
            Self::Gur => "Guran (a patois of Roushoum and Rashemi)",
            Self::Halruaan => "Halruaan (written in Draconic)",
            Self::Illuskan => "Illuskan",
            Self::Imaskari => "Roushoum",
            Self::Mulan => "Chessentan, Mulhorandi, Untheric, or Thayan",
            Self::Nar => "Damaran",
            Self::Rashemi => "Rashemi",
            Self::Shaaran => "Shaaran (written in Dethek)",
            Self::Shou => "Shou",
            Self::Tuigan => "Tuigan",
            Self::Turami => "Turmic",
            Self::Ulutiun => "Uluik",
        }
    }

    pub(crate) fn names(self) -> Names {
        match self {
            Self::Arkaiun => ARKAIUN,
            Self::Bedine => BEDINE,
            Self::Calishite => CALISHITE,
            Self::Chondathan | Self::Tethyrian => CHONDATHAN,
            Self::Damaran => DAMARAN,
            Self::Ffolk => FFOLK,
            Self::Gur => GUR,
            Self::Halruaan => HALRUAAN,
            Self::Illuskan => ILLUSKAN,
            Self::Imaskari => IMASKARI,
            Self::Mulan => MULAN,
            Self::Nar => NAR,
            Self::Rashemi => RASHEMI,
            Self::Shaaran => SHAARAN,
            Self::Shou => SHOU,
            Self::Tuigan => TUIGAN,
            Self::Turami => TURAMI,
            Self::Ulutiun => ULUTIUN,
        }
    }
}

/// Name options for a given ethnicity
pub(crate) struct Names {
    pub(crate) female: &'static [&'static str],
    pub(crate) male: &'static [&'static str],
    pub(crate) surname: &'static [&'static str],
}

const ARKAIUN: Names = Names {
    female: &["Glouris", "Maeve", "Sevaera", "Xaemarra", "Zraela"],
    male: &["Houn", "Rhivaun", "Umbril", "Xaemar", "Zeltaebar"],
    surname: &["Lharaendo", "Mristar", "Wyndael"],
};

const BEDINE: Names = Names {
    female: &["Aisha", "Farah", "Nura", "Rashida", "Zalebyeh"],
    male: &["Aali", "Rashid", "Tahnon", "Tanzim", "Whalide"],
    surname: &[
        "Alaii",
        "Bordjia",
        "Clelarra",
        "Desai",
        "Dakawa",
        "Dursalai",
        "Goldor",
        "Iriphawa",
        "Kellordrai",
        "Lalajar",
        "Qahtan",
        "Yethai",
        "Zazalaar",
    ],
};

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
        "Aleina", "Andwe", "Arveene", "Belinda", "Belynne", "Bloeth", "Bronwyn", "Daelia", "Elsa",
        "Erliza", "Esvele", "Freda", "Gardorra", "Halia", "Jhessail", "Kerri", "Kharissa",
        "Linene", "Lureene", "Miri", "Mirna", "Morwen", "Nestra", "Nilsa", "Rowan", "Shandri",
        "Tessele", "Thalamra", "Thistle", "Tiarshe", "Trilena", "Ylienna",
    ],
    male: &[
        "Aldith",
        "Ander",
        "Daerismun",
        "Daeros",
        "Dagult",
        "Daren",
        "Darvin",
        "Dauner",
        "Derid",
        "Dillard",
        "Dorn",
        "Elmar",
        "Evendur",
        "Favric",
        "Gorstag",
        "Grauman",
        "Grim",
        "Harbin",
        "Helm",
        "Iarno",
        "Jarl",
        "Javen",
        "Kaidrod",
        "Lanar",
        "Malark",
        "Maldwyn",
        "Morn",
        "Nars",
        "Narth",
        "Nasher",
        "Palien",
        "Pip",
        "Randal",
        "Selin",
        "Sildar",
        "Stedd",
        "Taumarik",
        "Thamal",
        "Thavus",
        "Thel",
        "Toblen",
        "Torlin",
        "Ulder",
    ],
    surname: &[
        "Aerath",
        "Alagondar",
        "Albrek",
        "Amblecrown",
        "Amcathra",
        "Ammakyl",
        "Anteos",
        "Anuvien",
        "Baldasker",
        "Barthen",
        "Brightlance",
        "Buckman",
        "Burr",
        "Cadrasz",
        "Caradoon",
        "Cururen",
        "Daggerford",
        "Dendrar",
        "Dundragon",
        "Eagleshields",
        "Edermath",
        "Embuirhan",
        "Emmert",
        "Evenwood",
        "Graywind",
        "Greycastle",
        "Hallwinter",
        "Harpell",
        "Hartwick",
        "Hemzar",
        "Ilzimmer",
        "Inchtarwurn",
        "Jhansczil",
        "Kreeg",
        "Kromlor",
        "Mammlar",
        "Margaster",
        "Neverember",
        "Oglyntyr",
        "Ostever",
        "Palyr",
        "Portyr",
        "Rault",
        "Raurym",
        "Ravenguard",
        "Roaringhorn",
        "Ruthiol",
        "Sharnshield",
        "Silvershield",
        "Stelmane",
        "Stonehill",
        "Suldivver",
        "Tallstag",
        "Tarm",
        "Tarmikos",
        "Thent",
        "Thornton",
        "Tresendar",
        "Vanthampur",
        "Wester",
        "Zelorrgosz",
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

const FFOLK: Names = Names {
    female: &["Alicia", "Gennifer", "Meridith", "Elaine", "Olivia"],
    male: &["Artur", "Bern", "Colin", "Manfred", "Tristan"],
    surname: &["Archer", "Gareth", "Leed", "Kendrick", "Morgan", "Waters"],
};

const GUR: Names = Names {
    female: &["Varra", "Ulmarra", "Imza", "Navarra", "Yuldra"],
    male: &["Boriv", "Gardar", "Madevik", "Vlad"],
    surname: &["Chergoba", "Drazlad", "Tazyara", "Vargoba", "Stayankina"],
};

const HALRUAAN: Names = Names {
    female: &["Aithe", "Chalan", "Oloma", "Phaele", "Sarade"],
    male: &[
        "Aldym",
        "Chand",
        "Meleghost",
        "Presmer",
        "Sandrue",
        "Uregaunt",
    ],
    surname: &["Avhoste", "Darante", "Maurmeril", "Stamaraster"],
};

const ILLUSKAN: Names = Names {
    female: &[
        "Amafrey", "Barri", "Betha", "Cefrey", "Dagmaer", "Druette", "Kethra", "Mara", "Olga",
        "Silifrey", "Teega", "Tevya", "Throa", "Westra",
    ],
    male: &[
        "Ander",
        "Beniago",
        "Blath",
        "Bran",
        "Cashaan",
        "Frath",
        "Geth",
        "Hartouchen",
        "Jendrick",
        "Lander",
        "Luth",
        "Malcer",
        "Stor",
        "Taman",
        "Urth",
        "Zelenn",
    ],
    surname: &[
        "Baram",
        "Brightwood",
        "Helder",
        "Hornraven",
        "Kurth",
        "Lackman",
        "Rethnor",
        "Stormwind",
        "Suljack",
        "Taerl",
        "Windrivver",
    ],
};

const IMASKARI: Names = Names {
    female: &["Apret", "Bask", "Fanul", "Mokat", "Nismet", "Ril"],
    male: &["Chavra", "Duma", "Hukir", "Jama", "Pradir", "Sikhil"],
    surname: &["Datharathi", "Melpurvatta", "Nalambar", "Tiliputakas"],
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

const NAR: Names = Names {
    female: &["Anva", "Dasha", "Dima", "Olga", "Westra", "Zlatara"],
    male: &[
        "Avan", "Ostaram", "Petro", "Stor", "Taman", "Thalaman", "Urth",
    ],
    surname: &[
        "Dashkev",
        "Hargoth",
        "Laboda",
        "Lackman",
        "Stonar",
        "Stormwind",
        "Sulyma",
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

const SHAARAN: Names = Names {
    female: &["Anet", "Bes", "Idim", "Lenet", "Moqem", "Neghet", "Sihvet"],
    male: &[
        "Awar", "Cohis", "Damota", "Gewar", "Hapah", "Laskaw", "Senesaw", "Tokhis",
    ],
    surname: &["Cor Marak", "Laumee Harr", "Moq Qu Harr", "Woraw Tarak"],
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

const TUIGAN: Names = Names {
    female: &["Bolormaa", "Bortai", "Erdene", "Naran"],
    male: &["Atlan", "Bayar", "Chingis", "Chinua", "Mongke", "Temur"],
    surname: &[],
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

const ULUTIUN: Names = Names {
    female: &["Akna", "Chena", "Kaya", "Sedna", "Ublereak"],
    male: &["Amak", "Chu", "Imnek", "Kanut", "Siku"],
    surname: &[],
};
