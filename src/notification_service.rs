use crate::developer::ChatId;
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

    pub async fn send_message(&self, chat_id: ChatId, pull_request: GithubPullRequest) {
        let time_ago = pull_request.updated_at;
        let message = format!(
            "{developer} requested your review on \"{title}\" ({url}) {time_ago} hours ago.", 
            developer=pull_request.user.login,
            title=pull_request.title,
            url=pull_request.html_url,
            time_ago=time_ago
        );
        info!("Sending message to {:?}", chat_id);
        match self.bot.send_message(
            chat_id, 
            &message
        ).send().await {
            Ok(_) => info!("Message sended to {:?}", chat_id),
            Err(e) => error!("{:?} {:?} {:?}", e, chat_id, message),
        }
    }
}
