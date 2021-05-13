use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Eberron;

impl<'a> Deities<'a> for Eberron {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity<'a>> {
        vec![
            Deity {
                name: "Arawai",
                titles: vec!["goddess of fertility"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Sheaf of wheat tied with green ribbon"],
            },
            Deity {
                name: "Aureon",
                titles: vec!["god of law and knowledge"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Open tome"],
            },
            Deity {
                name: "Balinor",
                titles: vec!["god of beasts and the hunt"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Pair of antlers"],
            },
            Deity {
                name: "Boldrei",
                titles: vec!["goddess of community and home"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Fire in a stone hearth"],
            },
            Deity {
                name: "Dol Arrah",
                titles: vec!["goddess of sunlight and honor"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Light, Domain::War],
                symbols: vec!["Rising sun"],
            },
            Deity {
                name: "Dol Dorn",
                titles: vec!["god of strength at arms"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Longsword crossed over a shield"],
            },
            Deity {
                name: "Kol Korran",
                titles: vec!["god of trade and wealth"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Nine-sided gold coin"],
            },
            Deity {
                name: "Olladra",
                titles: vec!["goddess of good fortune"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Trickery],
                symbols: vec!["Domino"],
            },
            Deity {
                name: "Onatar",
                titles: vec!["god of craft"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Crossed hammer and tongs"],
            },
            Deity {
                name: "The Devourer",
                titles: vec!["god of nature's wrath"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Tempest],
                symbols: vec!["Bundle of five sharpened bones"],
            },
            Deity {
                name: "The Fury",
                titles: vec!["goddess of wrath and madness"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Winged wyrm with woman's head and upper body"],
            },
            Deity {
                name: "The Keeper",
                titles: vec!["god of greed and death"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Dragonshard stone in the shape of a fang"],
            },
            Deity {
                name: "The Mockery",
                titles: vec!["god of violence and treachery"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Five blood-spattered tools"],
            },
            Deity {
                name: "The Shadow",
                titles: vec!["god of dark magic"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Obsidian tower"],
            },
            Deity {
                name: "The Traveler",
                titles: vec!["deity of chaos and change"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Four crossed, rune-inscribed bones"],
            },
            Deity {
                name: "The Silver Flame",
                titles: vec!["deity of protection and good"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life, Domain::Light, Domain::War],
                symbols: vec!["Flame drawn on silver or molded from silver"],
            },
            Deity {
                name: "The Blood of Vol",
                titles: vec!["philosophy of immortality and undeath"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Death, Domain::Life],
                symbols: vec!["Stylized dragon skull on red teardrop gem"],
            },
            Deity {
                name: "Cults of the Dragon Below",
                titles: vec!["deities of madness"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Varies"],
            },
            Deity {
                name: "The Path of Light",
                titles: vec!["philosophy of light and self-improvement"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Brilliant crystal"],
            },
            Deity {
                name: "The Undying Court",
                titles: vec!["elven ancestors"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life],
                symbols: vec!["Varies"],
            },
            Deity {
                name: "The Spirits of the Past",
                titles: vec!["elven ancestors"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Varies"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(Eberron::deities());
    }
}
