use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct GithubPullRequest {
    html_url: String,
    title: String,
    developer_username: String,
    // updated_at
    approved_by: HashSet<String>,
    reviewers: HashMap<String, String>,
    labels: HashSet<String>,
}

impl GithubPullRequest {
    pub fn init(
        raw_pull_request: &serde_json::Value,
        reviewers: Option<HashMap<String, String>>,
    ) -> Self {
        unimplemented!()
    }
}

// @dataclass
// class GithubPullRequest:
//     html_url: str
//     title: str
//     developer: str
//     updated_at: datetime
//     approved_by: Set[str] = field(default_factory=set)
//     reviewers: Dict[str, str] = field(default_factory=dict)
//     labels: Set[str] = field(default_factory=set)

//     @classmethod
//     def init(cls, raw_pull_request: Dict[str, Any]):
//         reviewers = raw_pull_request['reviewers']
//         for assignee in raw_pull_request['assignees']:
//             reviewers[assignee['login']] = reviewers.get(assignee['login'], 'N/A')

//         return cls(
//             html_url=raw_pull_request['html_url'],
//             title=raw_pull_request['title'],
//             developer=raw_pull_request['user']['login'],
//             updated_at=datetime.strptime(raw_pull_request['updated_at'], '%Y-%m-%dT%H:%M:%SZ'),
//             reviewers=reviewers,
//             approved_by={
//                 reviewer
//                 for reviewer, state in reviewers.items()
//                 if state == 'APPROVED'
//             },
//             labels={label['name'] for label in raw_pull_request['labels']}
//         )
