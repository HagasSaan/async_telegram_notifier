
use crate::pull_request::{GithubPullRequest, GithubReviews, GithubFile};

#[derive(Debug)]
pub struct GithubRepository {
    repository_name: String,
    access_token: String,
    pull_requests: Option<Vec<GithubPullRequest>>,
}

impl GithubRepository {
    fn repository_url(&self) -> String {
        format!("https://api.github.com/repos/{repository}", repository=self.repository_name)
    }
    fn pulls_url(&self) -> String {
        self.repository_url() + "/pulls"
    }
    fn files_url(&self, path_to_file: String) -> String {
        self.repository_url() + &format!("/contents/{path_to_file}", path_to_file=path_to_file)
    }
    pub fn new(repository_name: String, access_token: String) -> Self {
        Self {
            repository_name: repository_name,
            access_token: access_token,
            pull_requests: None,
        }
    }
    async fn get_request(&self, url: String) -> Option<String> {
        // println!("{:?}", url);
        let response: String = reqwest::Client::new()
            .get(&url)
            .bearer_auth(&self.access_token)
            .header("User-Agent", "RustReqwest/1.0")
            .send().await.unwrap()
            .text().await.unwrap();
        // println!("{:?}", response);
        Some(response)
    }
    async fn get_reviews(&self, raw_pull_request: &GithubPullRequest) -> Option<Vec<GithubReviews>> {
        let response = self.get_request(raw_pull_request.url.clone() + "/reviews").await.unwrap();
        let reviews: Option<Vec<GithubReviews>> = GithubReviews::load_from_str(&response);
        reviews
    }

    pub async fn get_pull_requests(&self) -> Option<Vec<GithubPullRequest>> {
        let response = self.get_request(self.pulls_url()).await.unwrap();
        // println!("{:?}", response);
        let pull_requests: Option<Vec<GithubPullRequest>> = 
            GithubPullRequest::load_from_str(&response);
                // .map(|pull_request|{ self.get_reviews })
        // add reviews here somehow
        pull_requests
    }
    
    pub async fn get_file(&self, path_to_file: String) -> Option<String> {
        let response = self.get_request(self.files_url(path_to_file)).await.unwrap();
        let file: Option<GithubFile> = GithubFile::load_from_str(&response);
        match file {
            Some(file) => Some(file.decode_content()),
            None => None,
        }
    }
}
