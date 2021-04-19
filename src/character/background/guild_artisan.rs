use std::fmt;

use rand::{prelude::IteratorRandom, Rng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    character::{
        ability::Skill,
        backstory::Backstory,
        equipment::{
            adventuring_gear::{Gear, OtherGear},
            currency::Coin,
            tools::{ArtisansTools, Tool},
            vehicles::{LandVehicle, Mount, Vehicle},
            Equipment, EquipmentOption, StartingEquipment,
        },
        features::{Feature, Features},
        languages::Languages,
        proficiencies::{Proficiencies, Proficiency},
        Character,
    },
    citation::{Book, Citation, CitationList, Citations},
};

use super::{Background, Influence, Personality, PersonalityOptions};

const SKILLS: &[Skill] = &[Skill::Insight, Skill::Persuasion];

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Business {
    #[strum(serialize = "Alchemists and apothecaries")]
    Alchemist,
    #[strum(serialize = "Armorers, locksmiths, and finesmiths")]
    Armorer,
    #[strum(serialize = "Brewers, distillers, and vintners")]
    Brewer,
    #[strum(serialize = "Calligraphers, scribes, and scriveners")]
    Calligrapher,
    #[strum(serialize = "Carpenters, roofers, and plasterers")]
    Carpenter,
    #[strum(serialize = "Cartographers, surveyors, and chart-makers")]
    Cartographer,
    #[strum(serialize = "Cobblers and shoemakers")]
    Cobbler,
    #[strum(serialize = "Cooks and Bakers")]
    Cook,
    #[strum(serialize = "Glassblowers and glaziers")]
    Glassblower,
    #[strum(serialize = "Jewelers and gemcutters")]
    Jeweler,
    #[strum(serialize = "Leatherworkers, skinners, and tanners")]
    Leatherworker,
    #[strum(serialize = "Masons and stonecutters")]
    Mason,
    #[strum(serialize = "Painters, limners, and sign-makers")]
    Painter,
    #[strum(serialize = "Potters and tile-makers")]
    Potter,
    #[strum(serialize = "Shipwrights and sailmakers")]
    Shipwright,
    #[strum(serialize = "Smiths and metal-forgers")]
    Smith,
    #[strum(serialize = "Tinkers, pewterers, and casters")]
    Tinker,
    #[strum(serialize = "Wagon-makers and wheelwrights")]
    WagonMaker,
    #[strum(serialize = "Weavers and dyers")]
    Weaver,
    #[strum(serialize = "Woodcarvers, coopers, and bowyers")]
    Woodcarver,
}

impl Business {
    fn tools(&self) -> ArtisansTools {
        match self {
            Business::Alchemist => ArtisansTools::AlchemistsSupplies,
            Business::Armorer | Business::Smith => ArtisansTools::SmithsTools,
            Business::Brewer => ArtisansTools::BrewersSupplies,
            Business::Calligrapher => ArtisansTools::CalligraphersSupplies,
            Business::Carpenter | Business::Shipwright | Business::WagonMaker => {
                ArtisansTools::CarpentersTools
            }
            Business::Cartographer => ArtisansTools::CartographersTools,
            Business::Cobbler => ArtisansTools::CobblersTools,
            Business::Cook => ArtisansTools::CooksUtensils,
            Business::Glassblower => ArtisansTools::GlassblowersTools,
            Business::Jeweler => ArtisansTools::JewelersTools,
            Business::Leatherworker => ArtisansTools::LeatherworkersTools,
            Business::Mason => ArtisansTools::MasonsTools,
            Business::Painter => ArtisansTools::PaintersSupplies,
            Business::Potter => ArtisansTools::PottersTools,
            Business::Tinker => ArtisansTools::TinkersTools,
            Business::Weaver => ArtisansTools::WeaversTools,
            Business::Woodcarver => ArtisansTools::WoodcarversTools,
        }
    }
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum MerchantVariant {
    Language,
    NavigatorsTools,
}

#[derive(Deserialize, Display, EnumIter, Serialize)]
enum Variant {
    Artisan,
    Merchant,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct GuildArtisan {
    business: Business,
    proficiency: Option<MerchantVariant>,
    variant: Variant,
}

#[typetag::serde]
impl Background for GuildArtisan {
    fn gen(rng: &mut impl Rng, _: &Character) -> (Box<dyn Background>, Personality) {
        let variant = Variant::iter().choose(rng).unwrap();
        let background = Box::new(Self {
            business: Business::iter().choose(rng).unwrap(),
            proficiency: match variant {
                Variant::Artisan => None,
                Variant::Merchant => Some(MerchantVariant::iter().choose(rng).unwrap()),
            },
            variant,
        });
        (background, Self::gen_personality(rng))
    }

    fn skills() -> Vec<Skill> {
        SKILLS.to_vec()
    }
}

impl Backstory for GuildArtisan {
    fn backstory(&self) -> Vec<String> {
        vec![format!("Guild Business: {}", self.business)]
    }
}

impl Citations for GuildArtisan {
    fn citations(&self) -> CitationList {
        CitationList(vec![Citation(Book::Phb, 132)])
    }
}

impl Features for GuildArtisan {
    fn features(&self) -> Vec<Feature> {
        // As an established and respected member of a guild, you can rely on certain benefits that membership provides. Your fellow guild members will provide you with lodging and food if necessary, and pay for your funeral if needed. In some cities and towns, a guildhall offers a central place to meet other members of your profession, which can be a good place to meet potential patrons, allies, or hirelings.
        // Guilds often wield tremendous political power. If you are accused of a crime, your guild will support you if a good case can be made for your innocence or the crime is justifiable. You can also gain access to powerful political figures through the guild, if you are a member in good standing. Such connections might require the donation of money or magic items to the guild's coffers.
        // You must pay dues of 5 gp per month to the guild. If you miss payments, you must make up back dues to remain in the guild's good graces.
        vec![Feature {
            title: "Guild Membership",
            citation: Citation(Book::Phb, 133),
        }]
    }
}

impl Languages for GuildArtisan {
    fn addl_languages(&self) -> usize {
        match self.proficiency {
            Some(MerchantVariant::Language) => 2,
            _ => 1,
        }
    }
}

impl PersonalityOptions for GuildArtisan {
    const BONDS: [&'static str; 6] = [
        "The workshop where I learned my trade is the most important place in the world to me.",
        "I created a great work for someone, and then found them unworthy to receive it. I'm still looking for someone worthy.",
        "I owe my guild a great debt for forging me into the person I am today.",
        "I pursue wealth to secure someone's love.",
        "One day I will return to my guild and prove that I am the greatest artisan of them all.",
        "I will get revenge on the evil forces that destroyed my place of business and ruined my livelihood.",
    ];
    const FLAWS: [&'static str; 6] = [
        "I'll do anything to get my hands on something rare or priceless.",
        "I'm quick to assume that someone is trying to cheat me.",
        "No one must ever learn that I once stole money from guild coffers.",
        "I'm never satisfied with what I have \u{2014} I always want more.",
        "I would kill to acquire a noble title.",
        "I'm horribly jealous of anyone who can outshine my handiwork. Everywhere I go, I'm surrounded by rivals.",
    ];
    const IDEALS: [(&'static str, Influence); 6] = [
        ("Community. It is the duty of all civilized people to strengthen the bonds of community and the security of civilization.", Influence::Lawful),
        ("Generosity. My talents were given to me so that I could use them to benefit the world.", Influence::Good),
        ("Freedom. Everyone should be free to pursue his or her own livelihood.", Influence::Chaotic),
        ("Greed. I'm only in it for the money.", Influence::Evil),
        ("People. I'm committed to the people I care about, not to ideals.", Influence::Neutral),
        ("Aspiration. I work hard to be the best there is at my craft.", Influence::Any),
    ];
    const TRAITS: [&'static str; 8] = [
        "I believe that anything worth doing is worth doing right. I can't help it \u{2014} I'm a perfectionist.",
        "I'm a snob who looks down on those who can't appreciate fine art.",
        "I always want to know how things work and what makes people tick.",
        "I'm full of witty aphorisms and have a proverb for every occasion.",
        "I'm rude to people who lack my commitment to hard work and fair play.",
        "I like to talk at length about my profession.",
        "I don't part with my money easily and will haggle tirelessly to get the best deal possible.",
        "I'm well known for my work, and I want to make sure everyone appreciates it. I'm always taken aback when people haven't heard of me.",
    ];
}

impl Proficiencies for GuildArtisan {
    fn proficiencies(&self) -> Vec<Proficiency> {
        let mut proficiencies: Vec<Proficiency> =
            SKILLS.iter().map(|&s| Proficiency::Skill(s)).collect();
        if let Variant::Artisan = self.variant {
            proficiencies.push(Proficiency::Tool(Tool::ArtisansTools(
                self.business.tools(),
            )));
        }
        if let Some(MerchantVariant::NavigatorsTools) = self.proficiency {
            proficiencies.push(Proficiency::Tool(Tool::NavigatorsTools))
        }
        proficiencies
    }
}

impl StartingEquipment for GuildArtisan {
    fn coins(&self) -> (Coin, u8) {
        (Coin::Gold, 15)
    }

    fn equipment(&self) -> Vec<Equipment> {
        let mut equipment = vec![
            Equipment::Other(String::from("a letter of introduction from your guild")),
            Equipment::Gear(Gear::Other(OtherGear::ClothesTravelers)),
            Equipment::Gear(Gear::Other(OtherGear::Pouch)),
        ];
        if let Variant::Merchant = self.variant {
            equipment.extend(vec![
                Equipment::Vehicle(Vehicle::Mount(Mount::Mule)),
                Equipment::Vehicle(Vehicle::Land(LandVehicle::Cart)),
            ])
        }
        equipment
    }

    fn addl_equipment(&self) -> Vec<EquipmentOption> {
        match self.variant {
            Variant::Artisan => vec![EquipmentOption::ArtisansTools],
            Variant::Merchant => vec![],
        }
    }
}

impl fmt::Display for GuildArtisan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Guild {}", self.variant)
    }
}
