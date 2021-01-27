use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum ArtisansTools {
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

#[derive(
    Copy, Clone, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub(crate) enum GamingSet {
    #[strum(serialize = "Dice Set")]
    Dice,
    #[strum(serialize = "Dragonchess Set")]
    Dragonchess,
    #[strum(serialize = "Playing Card Set")]
    PlayingCard,
    #[strum(serialize = "Three-Dragon Ante Set")]
    ThreeDragonAnte,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum MusicalInstrument {
    Bagpipes,
    Drum,
    Dulcimer,
    Flute,
    Lute,
    Lyre,
    Horn,
    #[strum(serialize = "Pan Flute")]
    PanFlute,
    Shawm,
    Viol,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum Vehicles {
    Land,
    Water,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum Tool {
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
    Vehicles(Vehicles),
}
