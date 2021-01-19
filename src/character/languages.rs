use strum_macros::Display;

#[derive(Display)]
pub(crate) enum Language {
    Abyssal,
    Celestial,
    Common,
    #[strum(serialize = "Deep Speech")]
    DeepSpeech,
    Draconic,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Infernal,
    Orc,
    Primordial,
    Sylvan,
    Undercommon,
}
