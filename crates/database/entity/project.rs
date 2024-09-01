#[derive(Debug, Clone)]
pub struct Project {
    pub account: String,
    pub name: String,
    pub tags: Vec<String>,
    pub archive: bool,
}

impl Project {}
