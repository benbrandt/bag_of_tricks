use alignment::{Alignment, Attitude, Morality};

use super::{Deities, Deity, Domain};

pub(crate) struct Celtic;

impl Deities for Celtic {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "The Daghdha",
                titles: vec!["god of weather and crops"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Nature, Domain::Trickery],
                symbols: vec!["Bubbling cauldron", "shield"],
            },
            Deity {
                name: "Arawn",
                titles: vec!["god of life and death"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Life, Domain::Death],
                symbols: vec!["Black star on gray background"],
            },
            Deity {
                name: "Belenus",
                titles: vec!["god of sun, light, and warmth"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Light],
                symbols: vec!["Solar disk and standing stones"],
            },
            Deity {
                name: "Brigantia",
                titles: vec!["goddess of rivers and livestock"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Footbridge"],
            },
            Deity {
                name: "Diancecht",
                titles: vec!["god of medicine and healing"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Crossed oak and mistletoe branches"],
            },
            Deity {
                name: "Dunatis",
                titles: vec!["god of mountains and peaks"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Red sun-capped mountain peak"],
            },
            Deity {
                name: "Goibhniu",
                titles: vec!["god of smiths and healing"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life],
                symbols: vec!["Giant mallet over sword"],
            },
            Deity {
                name: "Lugh",
                titles: vec!["god of arts, travel, and commerce"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::Life],
                symbols: vec!["Pair of long hands"],
            },
            Deity {
                name: "Manannan mac Lir",
                titles: vec!["god of oceans and sea creatures"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Wave of white water on green"],
            },
            Deity {
                name: "Math Mathonwy",
                titles: vec!["god of magic"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Staff"],
            },
            Deity {
                name: "Morrigan",
                titles: vec!["goddess of battle"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Two crossed spears"],
            },
            Deity {
                name: "Nuada",
                titles: vec!["god of war and warriors"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["Silver hand on black background"],
            },
            Deity {
                name: "Oghma",
                titles: vec!["god of speech and writing"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Unfurled scroll"],
            },
            Deity {
                name: "Silvanus",
                titles: vec!["god of nature and forests"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Summer oak tree"],
            },
        ]
    }
}

pub(crate) struct Greek;

impl Deities for Greek {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Zeus",
                titles: vec!["god of the sky, ruler of the gods"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Tempest],
                symbols: vec!["Fist full of lightning bolts"],
            },
            Deity {
                name: "Aphrodite",
                titles: vec!["goddess of love and beauty"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Light],
                symbols: vec!["Sea shell"],
            },
            Deity {
                name: "Apollo",
                titles: vec!["god of light, music, and healing"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life, Domain::Light],
                symbols: vec!["Lyre"],
            },
            Deity {
                name: "Ares",
                titles: vec!["god of war and strife"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Spear"],
            },
            Deity {
                name: "Artemis",
                titles: vec!["goddess of hunting and childbirth"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Bow and arrow on lunar disk"],
            },
            Deity {
                name: "Athena",
                titles: vec!["goddess of wisdom and civilization"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::War],
                symbols: vec!["Owl"],
            },
            Deity {
                name: "Demeter",
                titles: vec!["goddess of agriculture"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Mare's head"],
            },
            Deity {
                name: "Dionysus",
                titles: vec!["god of mirth and wine"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Life],
                symbols: vec!["Thyrsus (staff tipped with pine cone)"],
            },
            Deity {
                name: "Hades",
                titles: vec!["god of the underworld"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Black ram"],
            },
            Deity {
                name: "Hecate",
                titles: vec!["goddess of magic and the moon"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Knowledge, Domain::Trickery],
                symbols: vec!["Setting moon"],
            },
            Deity {
                name: "Hephaestus",
                titles: vec!["god of smithing and craft"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Hammer and anvil"],
            },
            Deity {
                name: "Hera",
                titles: vec!["goddess of marriage and intrigue"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Fan of peacock feathers"],
            },
            Deity {
                name: "Hercules",
                titles: vec!["god of strength and adventure"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Tempest, Domain::War],
                symbols: vec!["Lion's head"],
            },
            Deity {
                name: "Hermes",
                titles: vec!["god of travel and commerce"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Trickery],
                symbols: vec!["Caduceus (winged staff and serpents)"],
            },
            Deity {
                name: "Hestia",
                titles: vec!["goddess of home and family"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Hearth"],
            },
            Deity {
                name: "Nike",
                titles: vec!["goddess of victory"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::War],
                symbols: vec!["Winged woman"],
            },
            Deity {
                name: "Pan",
                titles: vec!["god of nature"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Syrinx (pan pipes)"],
            },
            Deity {
                name: "Poseidon",
                titles: vec!["god of the sea and earthquakes"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Tempest],
                symbols: vec!["Trident"],
            },
            Deity {
                name: "Tyche",
                titles: vec!["goddess of good fortune"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Red pentagram"],
            },
        ]
    }
}

pub(crate) struct Egyptian;

impl Deities for Egyptian {
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Re-Horakhty",
                titles: vec!["god of the sun, ruler of the gods"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Solar disk encircled by serpent"],
            },
            Deity {
                name: "Anubis",
                titles: vec!["god of judgment and death"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Death],
                symbols: vec!["Black jackal"],
            },
            Deity {
                name: "Apep",
                titles: vec!["god of evil, fire, and serpents"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Flaming snake"],
            },
            Deity {
                name: "Bast",
                titles: vec!["goddess of cats and vengeance"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Cat"],
            },
            Deity {
                name: "Bes",
                titles: vec!["god of luck and music"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Image of the misshapen deity"],
            },
            Deity {
                name: "Hathor",
                titles: vec!["goddess of love, music, and motherhood"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Horned cow's head with lunar disk"],
            },
            Deity {
                name: "Imhotep",
                titles: vec!["god of crafts and medicine"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Step pyramid"],
            },
            Deity {
                name: "Isis",
                titles: vec!["goddess of fertility and magic"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::Life],
                symbols: vec!["Ankh and star"],
            },
            Deity {
                name: "Nephthys",
                titles: vec!["goddess of death and grief"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Death],
                symbols: vec!["Horns around a lunar disk"],
            },
            Deity {
                name: "Osiris",
                titles: vec!["god of nature and the underworld"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Life, Domain::Nature],
                symbols: vec!["Crook and flail"],
            },
            Deity {
                name: "Ptah",
                titles: vec!["god of crafts, knowledge, and secrets"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Bull"],
            },
            Deity {
                name: "Set",
                titles: vec!["god of darkness and desert storms"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Death, Domain::Tempest, Domain::Trickery],
                symbols: vec!["Coiled cobra"],
            },
            Deity {
                name: "Sobek",
                titles: vec!["god of water and crocodiles"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Crocodile head with horns and plumes"],
            },
            Deity {
                name: "Thoth",
                titles: vec!["god of knowledge and wisdom"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Knowledge],
                symbols: vec!["Ibis"],
            },
        ]
    }
}

pub(crate) struct Norse;

impl Deities for Norse {
    #[allow(clippy::too_many_lines)]
    fn deities() -> Vec<Deity> {
        vec![
            Deity {
                name: "Odin",
                titles: vec!["god of knowledge and war"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Knowledge, Domain::War],
                symbols: vec!["Watching blue eye"],
            },
            Deity {
                name: "Aegir",
                titles: vec!["god of the sea and storms"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Tempest],
                symbols: vec!["Rough ocean waves"],
            },
            Deity {
                name: "Balder",
                titles: vec!["god of beauty and poetry"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Gem-encrusted silver chalice"],
            },
            Deity {
                name: "Forseti",
                titles: vec!["god of justice and law"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Light],
                symbols: vec!["Head of a bearded man"],
            },
            Deity {
                name: "Frey",
                titles: vec!["god of fertility and the sun"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Ice-blue greatsword"],
            },
            Deity {
                name: "Freya",
                titles: vec!["goddess of fertility and love"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Life],
                symbols: vec!["Falcon"],
            },
            Deity {
                name: "Frigga",
                titles: vec!["goddess of birth and fertility"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Life, Domain::Light],
                symbols: vec!["Cat"],
            },
            Deity {
                name: "Heimdall",
                titles: vec!["god of watchfulness and loyalty"],
                alignment: Alignment(Attitude::Lawful, Morality::Good),
                domains: vec![Domain::Light, Domain::War],
                symbols: vec!["Curling musical horn"],
            },
            Deity {
                name: "Hel",
                titles: vec!["goddess of the underworld"],
                alignment: Alignment(Attitude::Neutral, Morality::Evil),
                domains: vec![Domain::Death],
                symbols: vec!["Woman's face, rotting on one side"],
            },
            Deity {
                name: "Hermod",
                titles: vec!["god of luck"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Trickery],
                symbols: vec!["Winged scroll"],
            },
            Deity {
                name: "Loki",
                titles: vec!["god of thieves and trickery"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::Trickery],
                symbols: vec!["Flame"],
            },
            Deity {
                name: "Njord",
                titles: vec!["god of sea and wind"],
                alignment: Alignment(Attitude::Neutral, Morality::Good),
                domains: vec![Domain::Nature, Domain::Tempest],
                symbols: vec!["Gold coin"],
            },
            Deity {
                name: "Odur",
                titles: vec!["god of light and the sun"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Light],
                symbols: vec!["Solar disk"],
            },
            Deity {
                name: "Sif",
                titles: vec!["goddess of war"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::War],
                symbols: vec!["Upraised sword"],
            },
            Deity {
                name: "Skadi",
                titles: vec!["god of earth and mountains"],
                alignment: Alignment(Attitude::Neutral, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Mountain peak"],
            },
            Deity {
                name: "Surtur",
                titles: vec!["god of fire giants and war"],
                alignment: Alignment(Attitude::Lawful, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["Flaming sword"],
            },
            Deity {
                name: "Thor",
                titles: vec!["god of storms and thunder"],
                alignment: Alignment(Attitude::Chaotic, Morality::Good),
                domains: vec![Domain::Tempest, Domain::War],
                symbols: vec!["Hammer"],
            },
            Deity {
                name: "Thrym",
                titles: vec!["god of frost giants and cold"],
                alignment: Alignment(Attitude::Chaotic, Morality::Evil),
                domains: vec![Domain::War],
                symbols: vec!["White double-bladed axe"],
            },
            Deity {
                name: "Tyr",
                titles: vec!["god of courage and strategy"],
                alignment: Alignment(Attitude::Lawful, Morality::Neutral),
                domains: vec![Domain::Knowledge, Domain::War],
                symbols: vec!["Sword"],
            },
            Deity {
                name: "Uller",
                titles: vec!["god of hunting and winter"],
                alignment: Alignment(Attitude::Chaotic, Morality::Neutral),
                domains: vec![Domain::Nature],
                symbols: vec!["Longbow"],
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celtic_deities() {
        insta::assert_yaml_snapshot!(Celtic::deities());
    }

    #[test]
    fn test_greek_deities() {
        insta::assert_yaml_snapshot!(Greek::deities());
    }

    #[test]
    fn test_egyptian_deities() {
        insta::assert_yaml_snapshot!(Egyptian::deities());
    }

    #[test]
    fn test_norse_deities() {
        insta::assert_yaml_snapshot!(Norse::deities());
    }
}
