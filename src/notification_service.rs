use crate::developer::Developer;
use crate::pull_request::GithubPullRequest;

use teloxide::requests::Request;

use std::sync::Arc;
use teloxide;

#[derive(Debug)]
pub struct NotificationService {
    bot: Arc<teloxide::Bot>,
}

impl NotificationService {
    pub fn new(token: String, proxy_params: Option<&str>) -> Self {
        let bot = match proxy_params {
            Some(proxy_params) => {
                let proxy =
                    reqwest::Proxy::all(proxy_params).expect("Valid proxy param string expected");
                let client = reqwest::Client::builder().proxy(proxy).build().unwrap();
                info!("Bot initialized with proxy");
                teloxide::Bot::with_client(token, client)
            }
            None => {
                info!("Bot initialized");
                teloxide::Bot::new(token)
            },
        };
        Self { bot: bot }
    }

    pub async fn send_message(&self, developer: Developer, pull_request: GithubPullRequest) {
        let time_ago = pull_request.updated_at;
        let message = format!(
            "{reviewer}, {developer} requested your review on \"{title}\" ({url}) {time_ago} hours ago.",
            reviewer=developer.username, 
            developer=pull_request.user.login,
            title=pull_request.title,
            url=pull_request.html_url,
            time_ago=time_ago
        );
        debug!(
            "Sending message to {}({}) about {}", 
            developer.username, 
            developer.tg_chat_id, 
            pull_request.title
        );
        match self.bot.send_message(
            developer.tg_chat_id, 
            &message
        ).send().await {
            Ok(_) => info!("Message sended to {}({})", developer.username, developer.tg_chat_id),
            Err(e) => error!("{:?} {:?} {:?}", e, developer, message),
        }
    }
}
