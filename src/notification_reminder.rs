use crate::configuration::Configuration;
use crate::notification_service::NotificationService;
use crate::pull_request::GithubPullRequest;

#[derive(Debug)]
struct NotificationReminder {
    notifier: NotificationService,
    config: Configuration,
}

impl NotificationReminder {
    pub fn new(notifier: NotificationService, config: Configuration) {}
    pub fn remind(&self, pull_requests: Vec<GithubPullRequest>) {}
}

// class NotificationReminder:

//     def __init__(
//         self,
//         notifier: NotificationService,
//         config: Configuration,
//         logger: BoundLogger = structlog.get_logger(__name__),
//     ) -> None:
//         self._notifier = notifier
//         self._config = config
//         self._logger = logger

//     def remind(self, pull_requests: List[GithubPullRequest]) -> None:
//         self._logger.info('Start processing pull requests')
//         for pull_request in pull_requests:
//             self._logger.bind(pull_request=pull_request.title)

//             if pull_request.labels & self._config.skip_keywords:
//                 self._logger.info('Pull request have skipKeywords in labels, skipped')
//                 continue

//             self._logger.info('Start processing pull request')
//             approved_by_groups = list(filter(
//                 lambda group: (
//                     len(pull_request.approved_by & self._config.assignee_groups[group])
//                     >= self._config.number_of_reviewers
//                 ),
//                 self._config.assignee_groups.keys()
//             ))

//             for username, state in pull_request.reviewers.items():
//                 self._logger.bind(username=username)
//                 if self._config.developers.get(username) is None:
//                     self._logger.info('Reviewer not exists in config, skipped')
//                     continue

//                 if state == 'APPROVED':
//                     self._logger.info('Reviewer already approved PR, skipped')
//                     continue

//                 developer = self._config.developers[username]

//                 if not developer.is_working_time:
//                     self._logger.info('Not working time, skipped')
//                     continue

//                 if developer.group in approved_by_groups:
//                     if state != 'CHANGES_REQUESTED':
//                         self._logger.info('PR have enough approves by group', group=developer.group)
//                         continue

//                 self._notifier.send_message(developer.tg_chat_id, pull_request)

//                 self._logger.info('Pull requests processed')
