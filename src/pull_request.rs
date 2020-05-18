use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubPullRequest {
    pub url: String,
    pub title: String,
    pub user: GithubUser,
    pub labels: Vec<GithubLabel>,
    pub requested_reviewers: Vec<GithubUser>,
    pub updated_at: String,
}

impl GithubPullRequest {
    pub fn load_from_str(string: &str) -> Option<Vec<Self>> {
        // println!("{:?}", string);
        serde_json::from_str(string).unwrap_or(None)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubUser {
    login: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubLabel {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubReviews {
    user: GithubUser,
    state: String,
}

impl GithubReviews {
    pub fn load_from_str(string: &str) -> Option<Vec<Self>> {
        // println!("{:?}", string);
        serde_json::from_str(string).unwrap_or(None)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubFile {
    pub content: String,
}

impl GithubFile {
    pub fn load_from_str(string: &str) -> Option<Self> {
        // println!("{:?}", string);
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
