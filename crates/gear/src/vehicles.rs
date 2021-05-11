use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VehicleProficiency {
    Land,
    Water,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Mount {
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
pub enum LandVehicle {
    Carriage,
    Cart,
    Chariot,
    Sled,
    Wagon,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum WaterVehicle {
    Galley,
    Keelboat,
    Longship,
    Rowboat,
    #[strum(serialize = "Sailing ship")]
    SailingShip,
    Warship,
}

#[derive(Copy, Clone, Debug, Deserialize, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Vehicle {
    Land(LandVehicle),
    Mount(Mount),
    Water(WaterVehicle),
}
