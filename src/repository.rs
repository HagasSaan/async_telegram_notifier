use crate::pull_request::GithubPullRequest;
// use std::collections::HashMap;

#[derive(Debug)]
struct GithubRepository {
    repository_name: String,
    access_token: String,
    pull_requests: Vec<GithubPullRequest>,
}

impl GithubRepository {
    const URL_REPOSITORY: &'static str = "https://api.github.com/repos/{repository}";
    const URL_PULLS: &'static str = "{url_repository}/pulls";
    const URL_FILES: &'static str = "{url_repository}/contents/{path_to_file}";
    pub fn new(repository_name: String, token: String) {}
    // fn get_request(&self, url: String) -> HashMap<String, String> {}
    // fn get_reviewers(&self, raw_pull_request: HashMap<String, String>) -> HashMap<String, String> {}
    // pub fn get_pull_requests(&self) -> Vec<GithubPullRequest> {}
    // pub fn get_file(&self, path_to_file: String) -> String {}
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
