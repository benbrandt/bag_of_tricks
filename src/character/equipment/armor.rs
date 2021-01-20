use strum_macros::Display;

#[derive(Debug, Display)]
pub(crate) enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
}
