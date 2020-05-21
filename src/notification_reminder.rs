use futures::future::join_all;
use std::collections::{HashSet, HashMap};
use crate::pull_request::GithubUser;
use crate::configuration::Configuration;
use crate::notification_service::NotificationService;
use crate::pull_request::GithubPullRequest;

#[derive(Debug)]
pub struct NotificationReminder {
    notifier: NotificationService,
    config: Configuration,
}

impl NotificationReminder {
    pub fn new(notifier: NotificationService, config: Configuration) -> Self {
        Self {
            notifier: notifier,
            config: config,
        }
    }

    pub async fn remind(&self, pull_requests: Vec<GithubPullRequest>) {
        info!("Start processing pull requests");
        let mut futures_messages_to_send = Vec::new();
        'pull_requests: for pull_request in pull_requests {
            info!("Started processing PR: {}", pull_request.title);
            for label in &pull_request.labels {
                if self.config.skip_keywords.contains(&label.name){
                    info!("PR have skip keywords, skipped. PR: {}", pull_request.title); 
                    continue 'pull_requests;
                }
            }

            let mut need_to_be_approved_by: HashSet<GithubUser> = HashSet::new();
            let approved_by: HashSet<GithubUser> = match &pull_request.reviews {
                Some(reviews) => {
                    let mut approved_by = HashSet::new();
                    for review in reviews {
                        if review.state == "COMMENTED" { continue; }
                        else if review.state == "APPROVED" { approved_by.insert(review.user.clone()); }
                        else { need_to_be_approved_by.insert(review.user.clone()); }
                    }
                    approved_by
                },
                None => HashSet::new()
            };
            let need_to_be_approved_by = need_to_be_approved_by.difference(&approved_by);

            let mut approves_count_by_assignee_groups: HashMap<String, u8> = HashMap::new();

            for username in &approved_by {
                let username_group = 
                    match self.config.get_developer(&username.login) {
                        Some(developer) => developer.group,
                        None => {
                            error!("Developer not exists in config: {:?}", username.login);
                            continue;
                        }
                    };
                let approves_count = approves_count_by_assignee_groups
                    .entry(username_group)
                    .or_insert(0);
                *approves_count += 1;
            }

            let mut approved_by_assignee_groups: HashMap<String, bool> = HashMap::new();
            for (group, approves_count) in approves_count_by_assignee_groups {
                approved_by_assignee_groups.insert(
                    group, 
                    approves_count >= self.config.number_of_reviewers
                );
            }


            for username in need_to_be_approved_by {
                let developer = match self.config.get_developer(&username.login) {
                    Some(developer) => developer,
                    None => {
                        error!("Developer not exists in config: {:?}", username.login);
                        continue;
                    }
                };
                if !developer.is_working_time() {
                    info!("Not working time for {:?}, skipped", developer.username);
                    continue;
                }
                info!("PR processed: {}, added to messages to send", &pull_request.title);

                futures_messages_to_send.push(
                    self.notifier.send_message(developer.tg_chat_id, pull_request.clone())
                );
            };
        }
        join_all(futures_messages_to_send).await;
        info!("Pull requests processed");
    }
}

