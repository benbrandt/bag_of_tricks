use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct ForgottenRealms;

impl<'a> Deities<'a> for ForgottenRealms {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity<'a>> {
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
                titles: vec![
                    "god of the sun",
                    "the Keeper of the Eternal Sun",
                    "the Light of Law",
                    "the Yellow God",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Golden sun"],
            },
            Deity {
                name: "Asmodeus",
                titles: vec![
                    "god of induldence",
                    "the Lord of the Ninth",
                    "The Cloven",
                    "Old Hoof and Horn",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Three inverted triangles arranged in a long triangle"],
            },
            Deity {
                name: "Auril",
                titles: vec![
                    "goddess of winter",
                    "the Frostmaiden",
                    "Lady Frostkiss",
                    "Icedawn",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Six-pointed snowflake"],
            },
            Deity {
                name: "Azuth",
                titles: vec![
                    "god of wizardry",
                    "The High One",
                    "the Lord of Spellcraft",
                    "the First Magister",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbols: vec!["Left hand pointing upward, outlined in fire"],
            },
            Deity {
                name: "Bane",
                titles: vec!["god of tyranny", "the Black Hand", "the Lord of Darkness"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Upright black right hand, thumb and fingers together"],
            },
            Deity {
                name: "Beshaba",
                titles: vec![
                    "goddess of misfortune",
                    "the Maid of Misfortune",
                    "Lady Doom",
                    "Black Bess",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Black antlers"],
            },
            Deity {
                name: "Bhaal",
                titles: vec!["god of murder", "the Lord of Murder"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Skull surrounded by a ring of blood droplets"],
            },
            Deity {
                name: "Chauntea",
                titles: vec![
                    "goddess of agriculture",
                    "the Great Mother",
                    "the Grain Goddess",
                    "the Earthmother",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Sheaf of grain or a blooming rose over grain"],
            },
            Deity {
                name: "Cyric",
                titles: vec!["god of lies", "the Prince of Lies", "the Dark Sun"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["White jawless skull on black or purple sunburst"],
            },
            Deity {
                name: "Deneir",
                titles: vec![
                    "god of writing",
                    "the Lord of All Glyphs and Images",
                    "the First Scribe",
                    "the Scibe of Oghma",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbols: vec!["Lit candle above an open eye"],
            },
            Deity {
                name: "Eldath",
                titles: vec![
                    "goddess of peace",
                    "the Quiet One",
                    "the Guardian of Groves",
                    "the Mother of Waters",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Waterfall plunging into still pool"],
            },
            Deity {
                name: "Gond",
                titles: vec![
                    "god of craft",
                    "the Wonderbringer",
                    "the Inspiration Divine",
                    "the Holy Maker of All Things",
                ],
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
                titles: vec![
                    "god of tracking",
                    "the Mouth of Mielikki",
                    "the Master Tracker",
                    "the Tracker Never Led Astray",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Nature],
                symbols: vec!["Paw print with a five-pointed star in its center"],
            },
            Deity {
                name: "Helm",
                titles: vec![
                    "god of watchfulness",
                    "the Watcher",
                    "He of the Unsleeping Eyes",
                    "the Vigilant One",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Staring eye on upright left gauntlet"],
            },
            Deity {
                name: "Hoar",
                titles: vec![
                    "god of revenge and retribution",
                    "the Doombringer",
                    "Poet of Justice",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["A coin with a two-faced head"],
            },
            Deity {
                name: "Ilmater",
                titles: vec![
                    "god of endurance",
                    "the Crying God",
                    "the Rack-Broken Lord",
                    "He Who Endures",
                ],
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
                titles: vec![
                    "scribe of the dead",
                    "the Final Scribe",
                    "the Pitiless One",
                    "the Bleak Seneschal",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Death],
                symbols: vec!["A skull biting a scroll"],
            },
            Deity {
                name: "Kelemvor",
                titles: vec![
                    "god of the dead",
                    "the Lord of the Dead",
                    "the Judge of the Damned",
                ],
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
                titles: vec![
                    "god of dawn and renewal",
                    "god of birth and renewal",
                    "the Morninglord",
                    "Inspiration's Dawn",
                    "the Rose-and-Gold God",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Road traveling into a sunrise"],
            },
            Deity {
                name: "Leira",
                titles: vec![
                    "goddess of illusion",
                    "the Lady of the Mists",
                    "Mistshadow",
                    "the Lady of Deception",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Point-down triangle containing a swirl of mist"],
            },
            Deity {
                name: "Lliira",
                titles: vec![
                    "goddess of joy",
                    "Our Lady of Joy",
                    "Joybringer",
                    "the Mistress of Revels",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Triangle of three six-pointed stars"],
            },
            Deity {
                name: "Loviatar",
                titles: vec![
                    "goddess of pain",
                    "the Maiden of Pain",
                    "the Scourge Mistress",
                    "the Willing Whip",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Nine-tailed barbed scourge"],
            },
            Deity {
                name: "Malar",
                titles: vec!["god of the hunt", "the Beastlord", "the Black-Blooded One"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Nature],
                symbols: vec!["Clawed paw"],
            },
            Deity {
                name: "Mask",
                titles: vec![
                    "god of thieves",
                    "the Lord of Shadows",
                    "the Master of All Thieves",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Black mask"],
            },
            Deity {
                name: "Mielikki",
                titles: vec![
                    "goddess of forests",
                    "Our Lady of the Forest",
                    "the Forest Queen",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Nature],
                symbols: vec!["Unicorn's head"],
            },
            Deity {
                name: "Milil",
                titles: vec![
                    "god of poetry and song",
                    "the Lord of Song",
                    "the One True Hand of All-Wise Oghma",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Light],
                symbols: vec!["Five-stringed harp made of leaves"],
            },
            Deity {
                name: "Myrkul",
                titles: vec![
                    "god of death",
                    "the Lord of Bones",
                    "Old Lord Skull",
                    "the Reaper",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["White human skull"],
            },
            Deity {
                name: "Mystra",
                titles: vec![
                    "goddess of magic",
                    "the Lady of Mysteries",
                    "Our Lady of Spells",
                    "the Mother of All Magic",
                ],
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
                titles: vec!["god of knowledge", "the Binder", "the Lord of Knowledge"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Blank scroll"],
            },
            Deity {
                name: "The Red Knight",
                titles: vec![
                    "goddess of strategy",
                    "the Lady of Strategy",
                    "the Crimson General",
                    "the Grandmaster of the Lanceboard",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["Red knight lanceboard piece with stars for eyes"],
            },
            Deity {
                name: "Savras",
                titles: vec![
                    "god of divination and fate",
                    "the All-Seeing",
                    "the Third Eye",
                    "Divination's Lord",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Arcana, Domain::Knowledge],
                symbols: vec!["Crystal ball containing many kinds of eyes"],
            },
            Deity {
                name: "Sel\u{fb}ne",
                titles: vec![
                    "goddess of the moon",
                    "Our Lady of Silver",
                    "the Moonmaiden",
                    "the Night White Lady",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life],
                symbols: vec!["Pair of eyes surrounded by seven stars"],
            },
            Deity {
                name: "Shar",
                titles: vec![
                    "goddess of darkness and loss",
                    "the Mistress of the Night",
                    "the Dark Lady",
                    "Our Lady of Loss",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death, Domain::Trickery],
                symbols: vec!["Black disk encircled with a purple border"],
            },
            Deity {
                name: "Silvanus",
                titles: vec![
                    "god of wild nature",
                    "Oak Father",
                    "the Old Oak",
                    "Old Father Tree",
                ],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Oak leaf"],
            },
            Deity {
                name: "Sune",
                titles: vec![
                    "goddess of love and beauty",
                    "Lady Firehair",
                    "the Lady of Love",
                    "the Princess of Passion",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Face of a beautiful red-haired woman"],
            },
            Deity {
                name: "Talona",
                titles: vec![
                    "goddess of poison and disease",
                    "Lady of Poison",
                    "Mistress of Disease",
                    "the Plague-crone",
                ],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Three teardrops on a triangle"],
            },
            Deity {
                name: "Talos",
                titles: vec!["god of storms", "Stormlord", "the Destroyer"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Tempest],
                symbols: vec!["Three lightning bolts radiating from a central point"],
            },
            Deity {
                name: "Tempus",
                titles: vec!["god of war", "the Foehammer", "the Lord of Battles"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["Upright flaming sword"],
            },
            Deity {
                name: "Torm",
                titles: vec![
                    "god of courage and self-sacrifice",
                    "the Loyal Fury",
                    "the True",
                    "the Hand of Righteousness",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["White right gauntlet"],
            },
            Deity {
                name: "Tymora",
                titles: vec!["goddess of good fortune", "Lady Luck", "Our Smiling Lady"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Trickery],
                symbols: vec!["Face-up coin"],
            },
            Deity {
                name: "Tyr",
                titles: vec![
                    "god of justice",
                    "Grimjaws",
                    "the Maimed God",
                    "the Evenhanded",
                ],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Balanced scales resting on a warhammer"],
            },
            Deity {
                name: "Umberlee",
                titles: vec![
                    "goddess of the sea",
                    "the Bitch Queen",
                    "the Queen of the Depths",
                    "the Wavemother",
                ],
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
                titles: vec![
                    "goddess of trade",
                    "Our Lady of Gold",
                    "the Coinmaiden",
                    "the Merchant's Friend",
                ],
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
