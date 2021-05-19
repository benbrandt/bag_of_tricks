#![allow(clippy::default_trait_access)]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Copy, Clone, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum ArtisansTools {
    #[strum(serialize = "Alchemist's Supplies")]
    AlchemistsSupplies,
    #[strum(serialize = "Brewer's Supplies")]
    BrewersSupplies,
    #[strum(serialize = "Calligrapher's Supplies")]
    CalligraphersSupplies,
    #[strum(serialize = "Carpenter's Tools")]
    CarpentersTools,
    #[strum(serialize = "Cartographer's Tools")]
    CartographersTools,
    #[strum(serialize = "Cobbler's Tools")]
    CobblersTools,
    #[strum(serialize = "Cook's Utensils")]
    CooksUtensils,
    #[strum(serialize = "Glassblower's Tools")]
    GlassblowersTools,
    #[strum(serialize = "Jeweler's Tools")]
    JewelersTools,
    #[strum(serialize = "Leatherworker's Tools")]
    LeatherworkersTools,
    #[strum(serialize = "Mason's Tools")]
    MasonsTools,
    #[strum(serialize = "Painter's Supplies")]
    PaintersSupplies,
    #[strum(serialize = "Potter's Tools")]
    PottersTools,
    #[strum(serialize = "Smith's Tools")]
    SmithsTools,
    #[strum(serialize = "Tinker's Tools")]
    TinkersTools,
    #[strum(serialize = "Weaver's Tools")]
    WeaversTools,
    #[strum(serialize = "Woodcarver's Tools")]
    WoodcarversTools,
}

impl Default for ArtisansTools {
    fn default() -> Self {
        Self::AlchemistsSupplies
    }
}

#[derive(
    Copy, Clone, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum GamingSet {
    #[strum(serialize = "Dice Set")]
    Dice,
    #[strum(serialize = "Dragonchess Set")]
    Dragonchess,
    #[strum(serialize = "Playing Card Set")]
    PlayingCard,
    #[strum(serialize = "Three-Dragon Ante Set")]
    ThreeDragonAnte,
}

impl Default for GamingSet {
    fn default() -> Self {
        Self::Dice
    }
}

#[derive(
    Copy, Clone, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum MusicalInstrument {
    Bagpipes,
    Birdpipes,
    Drum,
    Dulcimer,
    Flute,
    Glaur,
    Longhorn,
    Lute,
    Lyre,
    #[strum(serialize = "Hand Drum")]
    HandDrum,
    Horn,
    #[strum(serialize = "Pan Flute")]
    PanFlute,
    Shawm,
    Songhorn,
    Tantan,
    Thelarr,
    Tocken,
    Viol,
    Wargong,
    Yarting,
    Zulkoon,
}

impl Default for MusicalInstrument {
    fn default() -> Self {
        Self::Bagpipes
    }
}

#[derive(
    Copy, Clone, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub enum Tool {
    ArtisansTools(ArtisansTools),
    #[strum(serialize = "Disguise Kit")]
    DisguiseKit,
    #[strum(serialize = "Forgery Set")]
    ForgerySet,
    GamingSet(GamingSet),
    #[strum(serialize = "Herbalism Kit")]
    HerbalismKit,
    MusicalInstrument(MusicalInstrument),
    #[strum(serialize = "Navigator's Tools")]
    NavigatorsTools,
    #[strum(serialize = "Poisoner's Kit")]
    PoisonerKit,
    #[strum(serialize = "Thieves' Tools")]
    ThievesTools,
}
