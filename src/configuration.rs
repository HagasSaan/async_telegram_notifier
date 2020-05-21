use crate::developer::Developer;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub number_of_reviewers: u8,
    pub skip_keywords: HashSet<String>,
    pub assignee_groups: HashSet<String>,
    pub developers: HashSet<Developer>,
}

impl Configuration {
    pub fn load_from_str(content: &str) -> Self {
        debug!("Got raw config file: {:?}", content);
        serde_yaml::from_str(content).expect("Unable to load configuration")
    }

    pub fn get_developer(&self, username: &String) -> Option<Developer> {
        for developer in &self.developers {
            if &developer.username == username {
                Some(developer.clone())
            } else {
                continue;
            };
        }
        None
    }
}
