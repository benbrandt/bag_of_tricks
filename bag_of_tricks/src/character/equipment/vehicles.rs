use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum VehicleProficiency {
    Land,
    Water,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum Mount {
    Camel,
    Donkey,
    Mule,
    #[strum(serialize = "Draft horse")]
    DraftHorse,
    Elephant,
    Mastiff,
    Pony,
    #[strum(serialize = "Riding horse")]
    RidingHorse,
    Warhorse,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum LandVehicle {
    Carriage,
    Cart,
    Chariot,
    Sled,
    Wagon,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum WaterVehicle {
    Galley,
    Keelboat,
    Longship,
    Rowboat,
    #[strum(serialize = "Sailing ship")]
    SailingShip,
    Warship,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub(crate) enum Vehicle {
    Land(LandVehicle),
    Mount(Mount),
    Water(WaterVehicle),
}
