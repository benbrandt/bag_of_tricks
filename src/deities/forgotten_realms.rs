use crate::alignment::{Alignment, Attitude, Morality};

use super::{Deity, Domain, Pantheon};

pub(crate) struct ForgottenRealms;

impl Pantheon for ForgottenRealms {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Adaki",
                titles: vec!["goddess of air"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Tempest],
                symbols: vec!["Cloud"],
            },
            Deity {
                name: "Amaunator",
                titles: vec!["god of the sun"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Golden sun"],
            },
            Deity {
                name: "Asmodeus",
                titles: vec!["god of induldence"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Three inverted triangles arranged in a long triangle"],
            },
            Deity {
                name: "Auril",
                titles: vec!["goddess of winter"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Six-pointed snowflake"],
            },
            Deity {
                name: "Azuth",
                titles: vec!["god of wizardry"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbols: vec!["Left hand pointing upward, outlined in fire"],
            },
            Deity {
                name: "Bane",
                titles: vec!["god of tyranny"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Upright black right hand, thumb and fingers together"],
            },
            Deity {
                name: "Beshaba",
                titles: vec!["goddess of misfortune"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Black antlers"],
            },
            Deity {
                name: "Bhaal",
                titles: vec!["god of murder"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Skull surrounded by a ring of blood droplets"],
            },
            Deity {
                name: "Chauntea",
                titles: vec!["goddess of agriculture"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Sheaf of grain or a blooming rose over grain"],
            },
            Deity {
                name: "Cyric",
                titles: vec!["god of lies"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["White jawless skull on black or purple sunburst"],
            },
            Deity {
                name: "Deneir",
                titles: vec!["god of writing"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbols: vec!["Lit candle above an open eye"],
            },
            Deity {
                name: "Eldath",
                titles: vec!["goddess of peace"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Waterfall plunging into still pool"],
            },
            Deity {
                name: "Gond",
                titles: vec!["god of craft"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Toothed cog with four spokes"],
            },
            Deity {
                name: "Grumbar",
                titles: vec!["god of earth"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Mountain"],
            },
            Deity {
                name: "Gwaeron Windstrom",
                titles: vec!["god of tracking"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Nature],
                symbols: vec!["Paw print with a five-pointed star in its center"],
            },
            Deity {
                name: "Helm",
                titles: vec!["god of watchfulness"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Staring eye on upright left gauntlet"],
            },
            Deity {
                name: "Hoar",
                titles: vec!["god of revenge and retribution"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["A coin with a two-faced head"],
            },
            Deity {
                name: "Ilmater",
                titles: vec!["god of endurance"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Hands bound at the wrist with red cord"],
            },
            Deity {
                name: "Istishia",
                titles: vec!["god of water"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Tempest],
                symbols: vec!["Wave"],
            },
            Deity {
                name: "Jergal",
                titles: vec!["scribe of the dead"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Death],
                symbols: vec!["A skull biting a scroll"],
            },
            Deity {
                name: "Kelemvor",
                titles: vec!["god of the dead"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Death],
                symbols: vec!["Upright skeletal arm holding balanced scales"],
            },
            Deity {
                name: "Kossuth",
                titles: vec!["god of fire"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Light],
                symbols: vec!["Flame"],
            },
            Deity {
                name: "Lathander",
                titles: vec!["god of dawn and renewal"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Road traveling into a sunrise"],
            },
            Deity {
                name: "Leira",
                titles: vec!["goddess of illusion"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Point-down triangle containing a swirl of mist"],
            },
            Deity {
                name: "Lliira",
                titles: vec!["goddess of joy"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Triangle of three six-pointed stars"],
            },
            Deity {
                name: "Loviatar",
                titles: vec!["goddess of pain"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Nine-tailed barbed scourge"],
            },
            Deity {
                name: "Malar",
                titles: vec!["god of the hunt"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Nature],
                symbols: vec!["Clawed paw"],
            },
            Deity {
                name: "Mask",
                titles: vec!["god of thieves"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Black mask"],
            },
            Deity {
                name: "Mielikki",
                titles: vec!["goddess of forests"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Nature],
                symbols: vec!["Unicorn's head"],
            },
            Deity {
                name: "Milil",
                titles: vec!["god of poetry and song"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Light],
                symbols: vec!["Five-stringed harp made of leaves"],
            },
            Deity {
                name: "Myrkul",
                titles: vec!["god of death"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["White human skull"],
            },
            Deity {
                name: "Mystra",
                titles: vec!["goddess of magic"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbols: vec![
                    "Circle of seven stars",
                    "nine stars encircling a flowing red mist",
                    "a single star",
                ],
            },
            Deity {
                name: "Oghma",
                titles: vec!["god of knowledge"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Blank scroll"],
            },
            Deity {
                name: "The Red Knight",
                titles: vec!["goddess of strategy"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["Red knight lanceboard piece with stars for eyes"],
            },
            Deity {
                name: "Savras",
                titles: vec!["god of divination and fate"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbols: vec!["Crystal ball containing many kinds of eyes"],
            },
            Deity {
                name: "Sel\u{fb}ne",
                titles: vec!["goddess of the moon"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life],
                symbols: vec!["Pair of eyes surrounded by seven stars"],
            },
            Deity {
                name: "Shar",
                titles: vec!["goddess of darkness and loss"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death, Domain::Trickery],
                symbols: vec!["Black disk encircled with a purple border"],
            },
            Deity {
                name: "Silvanus",
                titles: vec!["god of wild nature"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Oak leaf"],
            },
            Deity {
                name: "Sune",
                titles: vec!["goddess of love and beauty"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Face of a beautiful red-haired woman"],
            },
            Deity {
                name: "Talona",
                titles: vec!["goddess of poison and disease"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Three teardrops on a triangle"],
            },
            Deity {
                name: "Talos",
                titles: vec!["god of storms"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Tempest],
                symbols: vec!["Three lightning bolts radiating from a central point"],
            },
            Deity {
                name: "Tempus",
                titles: vec!["god of war"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["Upright flaming sword"],
            },
            Deity {
                name: "Torm",
                titles: vec!["god of courage and self-sacrifice"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["White right gauntlet"],
            },
            Deity {
                name: "Tymora",
                titles: vec!["goddess of good fortune"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Trickery],
                symbols: vec!["Face-up coin"],
            },
            Deity {
                name: "Tyr",
                titles: vec!["god of justice"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Balanced scales resting on a warhammer"],
            },
            Deity {
                name: "Umberlee",
                titles: vec!["goddess of the sea"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Tempest],
                symbols: vec!["Wave curling left and right"],
            },
            Deity {
                name: "Valkur",
                titles: vec!["Northlander god of sailors"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Tempest, Domain::War],
                symbols: vec!["A cloud and three lightning bolts"],
            },
            Deity {
                name: "Waukeen",
                titles: vec!["goddess of trade"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Upright coin with Waukeen's profile facing left"],
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
