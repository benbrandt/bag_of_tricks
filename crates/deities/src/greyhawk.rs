use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Greyhawk;

impl<'a> Deities<'a> for Greyhawk {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity<'a>> {
        vec![
            Deity {
                name: "Beory",
                titles: vec!["goddess of nature"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Green disk"],
            },
            Deity {
                name: "Boccob",
                titles: vec!["god of magic"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Eye within a pentagram"],
            },
            Deity {
                name: "Celestian",
                titles: vec!["god of stars and wanderers"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Arc of seven stars inside a circle"],
            },
            Deity {
                name: "Ehlonna",
                titles: vec!["goddess of woodlands"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Unicorn horn"],
            },
            Deity {
                name: "Erythnul",
                titles: vec!["god of envy and slaughter"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Blood drop"],
            },
            Deity {
                name: "Fharlanghn",
                titles: vec!["god of horizons and travel"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Circle crossed by a curved horizon line"],
            },
            Deity {
                name: "Heironeous",
                titles: vec!["god of chivalry and valor"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Lightning bolt"],
            },
            Deity {
                name: "Hextor",
                titles: vec!["god of war and discord"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Six arrows facing downward in a fan"],
            },
            Deity {
                name: "Kord",
                titles: vec!["god of athletics and sport"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Tempest, Domain::War],
                symbols: vec!["Four spears and four maces radiating out from a central point"],
            },
            Deity {
                name: "Incabulos",
                titles: vec!["god of plague and famine"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Reptilian eye with a horizontal diamond"],
            },
            Deity {
                name: "Istus",
                titles: vec!["goddess of fate and destiny"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Weaver's spindle with three strands"],
            },
            Deity {
                name: "Iuz",
                titles: vec!["god of pain and oppression"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Grinning human skull"],
            },
            Deity {
                name: "Nerull",
                titles: vec!["god of death"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Skull with either a sickle or a scythe"],
            },
            Deity {
                name: "Obad-Hai",
                titles: vec!["god of nature"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Oak leaf and acorn"],
            },
            Deity {
                name: "Olidammara",
                titles: vec!["god of revelry"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Laughing mask"],
            },
            Deity {
                name: "Pelor",
                titles: vec!["god of the sun and healing"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Sun"],
            },
            Deity {
                name: "Pholtus",
                titles: vec!["god of light and law"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Light],
                symbols: vec![
                    "Silver sun or full moon partially eclipsed by a smaller crescent moon",
                ],
            },
            Deity {
                name: "Ralishaz",
                titles: vec!["god of ill luck and insanity"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Three bone fate-casting sticks"],
            },
            Deity {
                name: "Rao",
                titles: vec!["god of peace and reason"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["White heart"],
            },
            Deity {
                name: "St. Cuthbert",
                titles: vec!["god of common sense and zeal"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Circle at the center of a starburst of lines"],
            },
            Deity {
                name: "Tharizdun",
                titles: vec!["god of eternal darkness"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Dark spiral or inverted ziggurat"],
            },
            Deity {
                name: "Trithereon",
                titles: vec!["god of liberty and retribution"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Triskelion"],
            },
            Deity {
                name: "Ulaa",
                titles: vec!["goddess of hills and mountains"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life, Domain::War],
                symbols: vec!["Mountain with a circle at its heart"],
            },
            Deity {
                name: "Vecna",
                titles: vec!["god of evil secrets"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Hand with eye in the palm"],
            },
            Deity {
                name: "Wee Jas",
                titles: vec!["goddess of magic and death"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Death, Domain::Knowledge],
                symbols: vec!["Red skull in front of fireball"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(Greyhawk::deities());
    }
}
