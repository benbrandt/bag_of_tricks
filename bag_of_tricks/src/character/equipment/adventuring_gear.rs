use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum ArcaneFocus {
    Crystal,
    Orb,
    Rod,
    Staff,
    Wand,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum DruidicFocus {
    #[strum(serialize = "Sprig of mistletoe")]
    SprigOfMistletoe,
    Totem,
    #[strum(serialize = "Wooden staff")]
    WoodenStaff,
    #[strum(serialize = "Yew wand")]
    YewWand,
}

#[derive(
    Copy, Clone, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
pub(crate) enum HolySymbol {
    Amulet,
    Emblem,
    Reliquary,
}

#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum OtherGear {
    Abacus,
    #[strum(serialize = "Acid (vial)")]
    Acid,
    #[strum(serialize = "Alchemist's fire (flask)")]
    AlchemistsFire,
    #[strum(serialize = "Antitoxin (vial)")]
    Antitoxin,
    Backpack,
    #[strum(serialize = "Ball bearings (bag of 1,000)")]
    BallBearings,
    Barrel,
    Basket,
    Bedroll,
    Bell,
    Blanket,
    #[strum(serialize = "Block and tackle")]
    BlockAndTackle,
    Book,
    #[strum(serialize = "Glass bottle")]
    BottleGlass,
    Bucket,
    #[strum(serialize = "Caltrops (bag of 20)")]
    Caltrops,
    Candle,
    #[strum(serialize = "Crossbow bolt case")]
    CaseCrossbowBolt,
    #[strum(serialize = "Map or scroll case")]
    CaseMapOrScroll,
    #[strum(serialize = "Chain (10 feet)")]
    Chain,
    #[strum(serialize = "Chalk (1 piece)")]
    Chalk,
    Chest,
    #[strum(serialize = "Climber's kit")]
    ClimbersKit,
    #[strum(serialize = "a set of common clothes")]
    ClothesCommon,
    #[strum(serialize = "a set of costume clothes")]
    ClothesCostume,
    #[strum(serialize = "a set of fine clothes")]
    ClothesFine,
    #[strum(serialize = "a set of traveler's clothes")]
    ClothesTravelers,
    #[strum(serialize = "Component pouch")]
    ComponentPouch,
    Crowbar,
    #[strum(serialize = "Fishing tackle")]
    FishingTackle,
    #[strum(serialize = "Flask or tankard")]
    FlaskOrTankard,
    #[strum(serialize = "Grappling hook")]
    GrapplingHook,
    Hammer,
    #[strum(serialize = "Sledge hammer")]
    HammerSledge,
    #[strum(serialize = "Healer's kit")]
    HealersKit,
    #[strum(serialize = "Holy water (flask)")]
    HolyWater,
    Hourglass,
    #[strum(serialize = "Hunting trap")]
    HuntingTrap,
    #[strum(serialize = "Ink (1 ounce bottle)")]
    Ink,
    #[strum(serialize = "Ink pen")]
    InkPen,
    #[strum(serialize = "Jug or pitcher")]
    JugOrPitcher,
    #[strum(serialize = "Ladder (10 foot)")]
    Ladder,
    Lamp,
    #[strum(serialize = "Bullseye lantern")]
    LanternBullseye,
    #[strum(serialize = "Hooded lantern")]
    LanternHooded,
    Lock,
    #[strum(serialize = "Magnifying glass")]
    MagnifyingGlass,
    Manacles,
    #[strum(serialize = "Mess kit")]
    MessKit,
    #[strum(serialize = "Steel mirror")]
    MirrorSteel,
    #[strum(serialize = "Oil (flask)")]
    Oil,
    #[strum(serialize = "Paper (one sheet)")]
    Paper,
    #[strum(serialize = "Parchment (one sheet)")]
    Parchment,
    #[strum(serialize = "Perfume (vial)")]
    Perfume,
    #[strum(serialize = "Miner's pick")]
    PickMiners,
    Piton,
    #[strum(serialize = "Basic poison (vial)")]
    PoisonBasic,
    #[strum(serialize = "Pole (10-foot)")]
    Pole,
    #[strum(serialize = "Iron pot")]
    PotIron,
    #[strum(serialize = "Potion of healing")]
    PotionOfHealing,
    Pouch,
    Quiver,
    #[strum(serialize = "Portable ram")]
    RamPortable,
    #[strum(serialize = "Rations (1 day)")]
    Rations,
    Robes,
    #[strum(serialize = "Hempen rope (50 feet)")]
    RopeHempen,
    #[strum(serialize = "Silk rope (50 feet)")]
    RopeSilk,
    Sack,
    #[strum(serialize = "Merchant's scale")]
    ScaleMerchants,
    #[strum(serialize = "Sealing Wax")]
    SealingWax,
    Shovel,
    #[strum(serialize = "Signal whistle")]
    SignalWhistle,
    #[strum(serialize = "Signet ring")]
    SignetRing,
    Soap,
    Spellbook,
    #[strum(serialize = "Iron spikes (10)")]
    SpikesIron,
    Spyglass,
    #[strum(serialize = "Two-person tent")]
    TentTwoPerson,
    Tinderbox,
    Torch,
    Vial,
    Waterskin,
    Whetstone,
}

#[derive(Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum Gear {
    ArcaneFocus(ArcaneFocus),
    DruidicFocus(DruidicFocus),
    HolySymbol(HolySymbol),
    Other(OtherGear),
}
