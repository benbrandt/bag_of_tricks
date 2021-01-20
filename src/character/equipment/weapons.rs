use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Debug, Deserialize, Display, Serialize)]
pub(crate) enum WeaponCategory {
    Simple,
    Martial,
}

#[derive(Debug, Deserialize, Display, Serialize)]
pub(crate) enum WeaponClassification {
    Melee,
    Ranged,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Display, Serialize)]
pub(crate) enum WeaponType {
    Battleaxe,
    Blowgun,
    Club,
    CrossbowHand,
    CrossbowHeavy,
    CrossbowLight,
    Dagger,
    Dart,
    Flail,
    Glaive,
    Greataxe,
    Greatsword,
    Greatclub,
    Halberd,
    Handaxe,
    Javelin,
    Lance,
    Longbow,
    Longsword,
    LightHammer,
    Mace,
    Maul,
    Morningstar,
    Net,
    Pike,
    Quarterstaff,
    Rapier,
    Scimitar,
    Shortbow,
    Shortsword,
    Sickle,
    Sling,
    Spear,
    Trident,
    WarPick,
    Warhammer,
    Whip,
}
