use base64;
use crate::pull_request::GithubPullRequest;
use std::collections::HashMap;

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
        Some(response)
    }
    async fn get_reviewers(&self, raw_pull_request: &serde_json::Value) -> Option<HashMap<String, String>> {
        // println!("{:?}", raw_pull_request["url"]);
        let response = self.get_request(
            raw_pull_request["url"].as_str()?.to_string() + "/reviews"
        ).await.unwrap();
        let mut result: HashMap<String, String> = HashMap::new();
        let reviews: Vec<serde_json::Value> = serde_json::from_str(&response).unwrap();
        // println!("Reviews: {:?}", reviews);
        for review in reviews.iter() {
            result.insert(
                review["user"]["login"].as_str()?.to_string(), 
                review["state"].as_str()?.to_string()
            );
        }
        Some(result)
    }
    pub async fn get_pull_requests(&self) -> Option<Vec<GithubPullRequest>>{
        let response = self.get_request(self.pulls_url()).await.unwrap();
        let raw_pull_requests: Vec<serde_json::Value> = serde_json::from_str(&response).unwrap();
        let mut pull_requests = Vec::<GithubPullRequest>::new();
        // lets make it really async, plz
        for raw_pull_request in raw_pull_requests.iter() {
            let reviewers = self.get_reviewers(raw_pull_request).await;
            // println!("{:?}", reviewers);
            pull_requests.push(GithubPullRequest::init(raw_pull_request, reviewers));
        }
        Some(pull_requests)
    }
    pub async fn get_file(&self, path_to_file: String) -> Option<String> {
        let response = self.get_request(self.files_url(path_to_file)).await.unwrap();
        let json: serde_json::Value = serde_json::from_str(&response).unwrap();
        let content = json["content"]
            .as_str()?
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|chunk|{ base64::decode(chunk).unwrap() })
            .collect::<Vec<Vec<u8>>>()
            .iter()
            .map(|chunk|{ String::from_utf8(chunk.to_vec()).unwrap() })
            .collect::<Vec<String>>()
            .concat();
        Some(content)
    }
}
// @dataclass
// class GithubRepository:
//     _repository: str
//     _token: str
//     _pull_requests: List[GithubPullRequest] = field(default_factory=list, init=False)

//     def _get_request(self, url: str) -> Any:
//         logger.bind(url=url)
//         logger.info('Sending request')
//         response = requests.get(url, headers={'Authorization': f'Bearer {self._token}'})
//         logger.info('Got response', status_code=response.status_code,)
//         if response.status_code != 200:
//             logger.error('Bad response', context=response.content)

//         return json.loads(response.content)

//     def _get_reviewers(
//         self,
//         raw_pull_request: Dict[str, Any]
//     ) -> Dict[str, str]:
//         reviews = self._get_request(raw_pull_request['url'] + '/reviews')

//         return {
//             review['user']['login']: review['state']
//             for review in reviews
//         }

//     def get_pull_requests(self) -> List[GithubPullRequest]:
//         self._pull_requests.clear()
//         raw_pull_requests = self._get_request(
//             self.URL_PULLS.format(repository=self._repository)
//         )
//         for raw_pr in raw_pull_requests:
//             raw_pr['reviewers'] = self._get_reviewers(raw_pr)
//             self._pull_requests.append(GithubPullRequest.init(raw_pr))
//         logger.info(f'Pull requests downloaded, count of requests: {len(self._pull_requests)}')
//         return self._pull_requests

//     def get_file(self, path_to_file: str) -> str:
//         content = self._get_request(
//             self.URL_FILES.format(repository=self._repository, path_to_file=path_to_file)
//         )['content']

//         return base64.b64decode(content).decode()
