use std::collections::HashMap;

// use std::env;

use futures::StreamExt;
use teloxide::prelude::*;

mod configuration;
mod developer;
mod notification_reminder;
mod notification_service;
mod pull_request;
mod repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);

    let token = "1214608092:AAEe209RQ76oZDJGZA80dbV9IDoIfCkSFv0";

    // let bot = Bot::from_env_with_client(client); 
    // Creates a new Bot with the TELOXIDE_TOKEN environmental variable (a bot's token) and your reqwest::Client.
    let proxy = reqwest::Proxy::all("socks5://telegram_user:wqe45gfbSZ_dWWQ@ec2-34-226-247-158.compute-1.amazonaws.com:443")?;

    let client = reqwest::Client::builder().proxy(proxy).build()?;

    let bot = Bot::with_client(token, client);

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
                message.answer("pong").send().await.log_on_error().await;
            })
        })
        .dispatch()
        .await;

    Ok(())
}


// #!/usr/bin/env python

// import os
// import sys
// import structlog

// from src.configuration import Configuration
// from src.notification_reminder import NotificationReminder
// from src.repository import GithubRepository
// from src.notification_service import NotificationService


// if __name__ == '__main__':
//     logger = structlog.get_logger(__name__)
//     logger.info('env and argv', env_vars=os.environ, argv_vars=sys.argv)

//     GITHUB_REPOSITORY = os.environ['INPUT_GITHUB_REPOSITORY']
//     TELEGRAM_TOKEN = os.environ['INPUT_TELEGRAM_TOKEN']
//     PATH_TO_FILE = os.environ['INPUT_PATH_TO_FILE']
//     GITHUB_TOKEN = os.environ['INPUT_GITHUB_TOKEN']

//     logger.info(
//         'Script started',
//         github_repository=GITHUB_REPOSITORY,
//         telegram_token=TELEGRAM_TOKEN,
//         path_to_file=PATH_TO_FILE,
//         github_token=GITHUB_TOKEN
//     )

//     repository = GithubRepository(GITHUB_REPOSITORY, GITHUB_TOKEN)
//     notifier = NotificationService(TELEGRAM_TOKEN)
//     raw_config = repository.get_file(PATH_TO_FILE)
//     config = Configuration.load_configuration(raw_config)

//     reminder = NotificationReminder(notifier, config)
//     pull_requests = repository.get_pull_requests()
//     reminder.remind(pull_requests)