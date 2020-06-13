use crate::developer::Developer;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Configuration {
    pub skip_keywords: HashSet<String>,
    pub developers: HashSet<Developer>,
    pub group_chat_id: Option<i64>,
}

impl Configuration {
    pub fn load_from_str(content: &str) -> Self {
        info!("Got raw config file");
        debug!("{}", content);
        serde_yaml::from_str(content).expect("Unable to load configuration")
    }

    pub fn get_developer(&self, username: &String) -> Option<Developer> {
        for developer in &self.developers {
            if &developer.username == username {
                return Some(developer.clone());
            } else {
                continue;
            };
        }
        None
    }
}
