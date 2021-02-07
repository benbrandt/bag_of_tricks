/// Basic trait to return a list of backstory descriptions
pub(crate) trait Backstory {
    /// List of backstory descriptions for this entity
    fn backstory(&self) -> Vec<String> {
        vec![]
    }
}
