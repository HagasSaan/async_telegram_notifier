use futures::future::join_all;

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
        Self { notifier, config }
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

            let required_to_be_approved_by = pull_request.get_required_approves_usernames();
            debug!(
                "PR {:#?} required to be approved by: {:?}", 
                pull_request.title,
                required_to_be_approved_by
            );
            
            if required_to_be_approved_by.is_empty() {
                info!("PR {} doesn't need approves, skipped", &pull_request.title);
            }

            let approved_by = pull_request.get_approves_usernames();


            for user in required_to_be_approved_by {
                if approved_by.contains(&user) {
                    info!("User {} already approved PR {:?}, skipped", user.login, pull_request.title);
                    continue;
                }
                let developer = match self.config.get_developer(&user.login) {
                    Some(developer) => developer,
                    None => {
                        error!(
                            "Developer not exists in config: {}, can't send message, PR: {}", 
                            user.login, pull_request.title
                        );
                        continue;
                    }
                };

                if !developer.is_working_time() {
                    info!("Not working time for {}, skipped", developer.username);
                    continue;
                }
                info!("PR processed: {}, sending message to {}",
                    &pull_request.title,
                    &developer.username
                );

                futures_messages_to_send.push(
                    self.notifier.send_message(developer, pull_request.clone())
                );
            };
        }
        info!(
            "{} messages sent. Pull requests processed", 
            join_all(futures_messages_to_send).await.len()
        );
    }
}

