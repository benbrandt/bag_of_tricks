use strum_macros::Display;

#[derive(Debug, Display)]
pub(crate) enum WeaponCategory {
    Simple,
    Martial,
}

#[derive(Debug, Display)]
pub(crate) enum WeaponClassification {
    Melee,
    Ranged,
}

#[derive(Debug, Display)]
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
    LongBow,
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
