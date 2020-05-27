use teloxide::types::Update;
use futures::future::join_all;
use crate::developer::Developer;
use crate::pull_request::GithubPullRequest;

use teloxide::requests::Request;
use teloxide::types;

use std::sync::Arc;
use teloxide;

#[derive(Debug)]
pub struct NotificationService {
    pub bot: Arc<teloxide::Bot>,
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
        let reviewer = self.get_username_by_chat_id(developer.tg_chat_id).await;
        let message = format!(
            "{reviewer}, {developer} requested your review on \"{title}\" ({url}) {time_ago} hours ago.",
            reviewer=reviewer, 
            developer=pull_request.user.login,
            title=pull_request.title,
            url=pull_request.html_url,
            time_ago=time_ago
        );
        debug!(
            "Sending message to {}({}) about {}", 
            reviewer, 
            developer.tg_chat_id, 
            pull_request.title
        );
        match self.bot.send_message(
            developer.tg_chat_id, 
            &message
        ).send().await {
            Ok(_) => info!(
                "Message sended to {}({}:{})", 
                developer.username, 
                reviewer, 
                developer.tg_chat_id
            ),
            Err(e) => error!("{:#?}\n{:#?}\n{:#?}", e, developer, message),
        }
    }

    async fn get_username_by_chat_id(&self, tg_chat_id: i64) -> String {
        let username = match self.bot.get_chat(tg_chat_id).send().await {
            Ok(result) => {
                match result.kind {
                    types::ChatKind::Private{
                        type_: _,
                        username,
                        first_name: _,
                        last_name: _,
                    } => {
                        username.unwrap_or("unknown".to_string())
                    },
                    _ => {
                        error!("Failed to get username of chat_id");
                        "unknown".to_string()
                    }
                }
            }
            Err(_) => {
                error!("Failed to get username of chat_id");
                "unknown"
            }.to_string()
        };
        username
    }

    async fn process_update(&self, update: Update) {
        debug!("Processing incoming message:\n{:#?}", update);
        let user_chat_id = update.user().unwrap().id;
        match self.bot.send_message(
            user_chat_id as i64, 
            format!("Your chat id: {:?}", user_chat_id)
        ).send().await {
            Err(e) => error!("Error while sending message: {:?}", e),
            _ => ()
        }
    }

    pub async fn process_incoming_messages(&self) {
        info!("Starting processing incoming messages");
        let updates = self.bot.get_updates().send().await.unwrap();
        let mut futures_updates = vec!();
        for update in updates {
            futures_updates.push(
                self.process_update(update.unwrap())
            );
        }
        info!(
            "{} messages sent. Incoming messages processed",
            join_all(futures_updates).await.len()
        );
        info!("Finished processing incoming messages");
    }
}
