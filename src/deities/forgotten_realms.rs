use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct ForgottenRealms;

impl Pantheon for ForgottenRealms {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Adaki",
                description: "goddess of air",
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Tempest],
                symbol: "Cloud",
            },
            Deity {
                name: "Amaunator",
                description: "god of the sun",
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Life, Domain::Light],
                symbol: "Golden sun",
            },
            Deity {
                name: "Asmodeus",
                description: "god of induldence",
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbol: "Three inverted triangles arranged in a long triangle",
            },
            Deity {
                name: "Auril",
                description: "goddess of winter",
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbol: "Six-pointed snowflake",
            },
            Deity {
                name: "Azuth",
                description: "god of wizardry",
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbol: "Left hand pointing upward, outlined in fire",
            },
            Deity {
                name: "Bane",
                description: "god of tyranny",
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::War],
                symbol: "Upright black right hand, thumb and fingers together",
            },
            Deity {
                name: "Beshaba",
                description: "goddess of misfortune",
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbol: "Black antlers",
            },
            Deity {
                name: "Bhaal",
                description: "god of murder",
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbol: "Skull surrounded by a ring of blood droplets",
            },
            Deity {
                name: "Chauntea",
                description: "goddess of agriculture",
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life],
                symbol: "Sheaf of grain or a blooming rose over grain",
            },
            Deity {
                name: "Cyric",
                description: "god of lies",
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbol: "White jawless skull on black or purple sunburst",
            },
            Deity {
                name: "Deneir",
                description: "god of writing",
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbol: "Lit candle above an open eye",
            },
            Deity {
                name: "Eldath",
                description: "goddess of peace",
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Nature],
                symbol: "Waterfall plunging into still pool",
            },
            Deity {
                name: "Gond",
                description: "god of craft",
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbol: "Toothed cog with four spokes",
            },
            Deity {
                name: "Grumbar",
                description: "god of earth",
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbol: "Mountain",
            },
            Deity {
                name: "Gwaeron Windstrom",
                description: "god of tracking",
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Nature],
                symbol: "Paw print with a five-pointed star in its center",
            },
            Deity {
                name: "Helm",
                description: "god of watchfulness",
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Life, Domain::Light],
                symbol: "Staring eye on upright left gauntlet",
            },
            Deity {
                name: "Hoar",
                description: "god of revenge and retribution",
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::War],
                symbol: "A coin with a two-faced head",
            },
            Deity {
                name: "Ilmater",
                description: "god of endurance",
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life],
                symbol: "Hands bound at the wrist with red cord",
            },
            Deity {
                name: "Istishia",
                description: "god of water",
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Tempest],
                symbol: "Wave",
            },
            Deity {
                name: "Jergal",
                description: "scribe of the dead",
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Death],
                symbol: "A skull biting a scroll",
            },
            Deity {
                name: "Kelemvor",
                description: "god of the dead",
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Death],
                symbol: "Upright skeletal arm holding balanced scales",
            },
            Deity {
                name: "Kossuth",
                description: "god of fire",
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Light],
                symbol: "Flame",
            },
            Deity {
                name: "Lathander",
                description: "god of dawn and renewal",
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbol: "Road traveling into a sunrise",
            },
            Deity {
                name: "Leira",
                description: "goddess of illusion",
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbol: "Point-down triangle containing a swirl of mist",
            },
            Deity {
                name: "Lliira",
                description: "goddess of joy",
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life],
                symbol: "Triangle of three six-pointed stars",
            },
            Deity {
                name: "Loviatar",
                description: "goddess of pain",
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Death],
                symbol: "Nine-tailed barbed scourge",
            },
            Deity {
                name: "Malar",
                description: "god of the hunt",
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Nature],
                symbol: "Clawed paw",
            },
            Deity {
                name: "Mask",
                description: "god of thieves",
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbol: "Black mask",
            },
            Deity {
                name: "Mielikki",
                description: "goddess of forests",
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Nature],
                symbol: "Unicorn's head",
            },
            Deity {
                name: "Milil",
                description: "god of poetry and song",
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Light],
                symbol: "Five-stringed harp made of leaves",
            },
            Deity {
                name: "Myrkul",
                description: "god of death",
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbol: "White human skull",
            },
            Deity {
                name: "Mystra",
                description: "goddess of magic",
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbol: "Circle of seven stars, or nine stars encircling a flowing red mist, or a single star",
            },
            Deity {
                name: "Oghma",
                description: "god of knowledge",
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbol: "Blank scroll",
            },
            Deity {
                name: "The Red Knight",
                description: "goddess of strategy",
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::War],
                symbol: "Red knight lanceboard piece with stars for eyes",
            },
            Deity {
                name: "Savras",
                description: "god of divination and fate",
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbol: "Crystal ball containing many kinds of eyes",
            },
            Deity {
                name: "Sel\u{fb}ne",
                description: "goddess of the moon",
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life],
                symbol: "Pair of eyes surrounded by seven stars",
            },
            Deity {
                name: "Shar",
                description: "goddess of darkness and loss",
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death, Domain::Trickery],
                symbol: "Black disk encircled with a purple border",
            },
            Deity {
                name: "Silvanus",
                description: "god of wild nature",
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbol: "Oak leaf",
            },
            Deity {
                name: "Sune",
                description: "goddess of love and beauty",
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbol: "Face of a beautiful red-haired woman",
            },
            Deity {
                name: "Talona",
                description: "goddess of poison and disease",
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Death],
                symbol: "Three teardrops on a triangle",
            },
            Deity {
                name: "Talos",
                description: "god of storms",
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Tempest],
                symbol: "Three lightning bolts radiating from a central point",
            },
            Deity {
                name: "Tempus",
                description: "god of war",
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::War],
                symbol: "Upright flaming sword",
            },
            Deity {
                name: "Torm",
                description: "god of courage and self-sacrifice",
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbol: "White right gauntlet",
            },
            Deity {
                name: "Tymora",
                description: "goddess of good fortune",
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Trickery],
                symbol: "Face-up coin",
            },
            Deity {
                name: "Tyr",
                description: "god of justice",
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbol: "Balanced scales resting on a warhammer",
            },
            Deity {
                name: "Umberlee",
                description: "goddess of the sea",
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Tempest],
                symbol: "Wave curling left and right",
            },
            Deity {
                name: "Valkur",
                description: "Northlander god of sailors",
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Tempest, Domain::War],
                symbol: "A cloud and three lightning bolts",
            },
            Deity {
                name: "Waukeen",
                description: "goddess of trade",
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbol: "Upright coin with Waukeen's profile facing left",
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deities() {
        insta::assert_yaml_snapshot!(ForgottenRealms::deities());
    }
}
