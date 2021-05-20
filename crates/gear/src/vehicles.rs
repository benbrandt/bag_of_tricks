use std::fmt;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Copy, Clone, Debug, Deserialize, Display, EnumIter, Eq, Ord, PartialEq, PartialOrd, Serialize,
)]
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

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Vehicle {
    Land(LandVehicle),
    Mount(Mount),
    Water(WaterVehicle),
}

impl fmt::Display for Vehicle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Land(v) => write!(f, "{}", v),
            Self::Mount(v) => write!(f, "{}", v),
            Self::Water(v) => write!(f, "{}", v),
        }
    }
}
