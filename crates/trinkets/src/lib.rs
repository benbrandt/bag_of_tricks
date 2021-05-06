mod default;
mod goblin;

use default::TRINKETS;
use goblin::GOBLIN_STATUS_SYMBOLS;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TrinketOption {
    Default,
    Goblin,
    Custom(Vec<String>),
}

impl TrinketOption {
    pub fn trinkets(&self) -> Vec<String> {
        match self {
            Self::Default => TRINKETS.iter().map(|&t| t.to_string()).collect(),
            Self::Goblin => GOBLIN_STATUS_SYMBOLS
                .iter()
                .map(|&t| t.to_string())
                .collect(),
            Self::Custom(l) => l.clone(),
        }
    }
}

pub trait Trinkets {
    fn trinket_options(&self) -> Vec<TrinketOption> {
        vec![]
    }
}
