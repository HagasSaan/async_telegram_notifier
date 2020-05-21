use futures::future::join_all;

use crate::pull_request::{GithubPullRequest, GithubReview, GithubFile};

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
        info!("Prepare to send GET request on {:?}", url);
        let response = reqwest::Client::new()
            .get(&url)
            .bearer_auth(&self.access_token)
            .header("User-Agent", "RustReqwest/1.0")
            .send().await.unwrap();
        info!("Response status: {:?} for {:?}", response.status(), url);
        let content = response.text().await.unwrap();
        debug!("Got response: {:?}", content);
        Some(content)
    }
    async fn get_reviews(&self, pull_request: &GithubPullRequest) -> Option<Vec<GithubReview>> {
        let response = self.get_request(pull_request.url.clone() + "/reviews").await.unwrap();
        let reviews: Option<Vec<GithubReview>> = GithubReview::load_from_str(&response);
        reviews
    }

    async fn add_reviews_to_pull_request(&self, mut pull_request: GithubPullRequest) -> GithubPullRequest{
        let reviews = self.get_reviews(&pull_request);
        pull_request.reviews = reviews.await;
        pull_request
    }

    pub async fn get_pull_requests(&self) -> Option<Vec<GithubPullRequest>> {
        let response = self.get_request(self.pulls_url()).await.unwrap();
        let pull_requests: Option<Vec<GithubPullRequest>> = GithubPullRequest::load_from_str(&response);
        let mut futures_pull_requests_with_reviews = Vec::new();
        for pull_request in pull_requests? {
            futures_pull_requests_with_reviews.push(
                self.add_reviews_to_pull_request(pull_request)
            );
        }
        let pull_requests = Some(join_all(futures_pull_requests_with_reviews).await);
        debug!("Got pull requests: {:?}", pull_requests);
        pull_requests
    }
    
    pub async fn get_file(&self, path_to_file: String, is_absolute_path: bool) -> Option<String> {
        let url = match is_absolute_path {
            true => path_to_file,
            false => self.files_url(path_to_file),
        };
        let response = self.get_request(url).await.unwrap();
        let file: Option<GithubFile> = GithubFile::load_from_str(&response);
        match file {
            Some(file) => Some(file.decode_content()),
            None => None,
        }
    }
}
