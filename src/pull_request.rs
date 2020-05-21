use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubPullRequest {
    pub url: String,
    pub html_url: String,
    pub title: String,
    pub user: GithubUser,
    pub labels: HashSet<GithubLabel>,
    pub requested_reviewers: HashSet<GithubUser>,
    pub updated_at: String,
    pub reviews: Option<Vec<GithubReview>>,
}

impl GithubPullRequest {
    pub fn load_from_str(string: &str) -> Option<Vec<Self>> {
        debug!("Got raw pull request: {:?}", string);
        serde_json::from_str(string).unwrap_or(None)
    }
}

#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct GithubUser {
    pub login: String,
}

#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct GithubLabel {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubReview {
    pub user: GithubUser,
    pub state: String,
}

impl GithubReview {
    pub fn load_from_str(string: &str) -> Option<Vec<Self>> {
        debug!("Got raw review: {:?}", string);
        serde_json::from_str(string).unwrap_or(None)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubFile {
    pub content: String,
}

impl GithubFile {
    pub fn load_from_str(string: &str) -> Option<Self> {
        debug!("Got raw file: {:?}", string);
        serde_json::from_str(string).unwrap_or(None)
    }
    pub fn decode_content(&self) -> String {
        self.content
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|chunk| base64::decode(chunk).unwrap())
            .collect::<Vec<Vec<u8>>>()
            .iter()
            .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
            .collect::<Vec<String>>()
            .concat()
    }
}
