use crate::developer::Developer;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    number_of_reviewers: u8,
    skip_keywords: Vec<String>,
    assignee_groups: Vec<String>,
    developers: Vec<Developer>,
}

impl Configuration {
    pub fn load_from_str(content: &str) -> Self {
        serde_yaml::from_str(content).expect("Unable to load configuration")
    }
}
