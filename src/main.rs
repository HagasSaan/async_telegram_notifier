// use std::collections::HashMap;

// use std::env;


// use futures::StreamExt;
// use teloxide::prelude::*;
// use serde_json;
mod repository;
use repository::GithubRepository;
mod configuration;
use configuration::Configuration;
mod developer;
mod pull_request;
use pull_request::GithubPullRequest;

// mod notification_reminder;
// mod notification_service;

// async fn handle_messages(rx: DispatcherHandlerRx<Message>) {
//     let result = rx.for_each(|message| async move {
//         println!("{:?}", message);
//         println!("{:?}", message.answer("pong").send().await);
//     }).await;
//     println!("{:?}", result);
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let repository_name = "tekliner/dsas".to_string();
    let token = "94fa772c53c540f137c4db4ca18ad8b464e94735".to_string();
    let repository = GithubRepository::new(repository_name.clone(), token);
    
    let response = repository.get_request(repository.pulls_url()).await.unwrap();
    println!("{:?}", response);
    let pull_requests: Vec<GithubPullRequest> = GithubPullRequest::load_from_str(&response).unwrap();
    // for pull_request in &pull_requests {
    //     let review = repository.get_reviews(&pull_request);
    //     println!("{:?}", review.await);
    // }
    println!("{:?}", pull_requests);    
    // let pull_requests = repository.get_pull_requests().await;
    // println!("Pull requests: {:?}", pull_requests);
    let config_file_name = ".github/tg_notifier.yml".to_string();
    let raw_file = repository.get_file(config_file_name).await.unwrap();
    println!("Config file: {:?}", raw_file);
    

    // let raw_configuration = "---\nnumber_of_reviewers: 2\nskip_keywords:\n  - skip1\n  - skip2\nassignee_groups:\n  - developers\n  - codeowners\ndevelopers:\n  - username: user1\n    tg_chat_id: 123\n    group: developers\n    timetable:\n      days:\n        - Monday\n        - Friday\n      started_at: \"09:00:00\"\n      ended_at: \"09:00:00\"\n  - username: user2\n    tg_chat_id: 456\n    group: codeowners\n    timetable:\n      days:\n        - Monday\n        - Saturday\n      started_at: \"12:00:00\"\n      ended_at: \"14:00:00\"";
    // println!("Configuration: {:?}", raw_configuration);
    // let configuration: Configuration = Configuration::load_from_str(raw_configuration);
    // println!("Configuration: {:?}", configuration);



    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{:#?}", resp);

    // let token = "1214608092:AAEe209RQ76oZDJGZA80dbV9IDoIfCkSFv0";

    // // let bot = Bot::from_env_with_client(client); 
    // // Creates a new Bot with the TELOXIDE_TOKEN environmental variable (a bot's token) and your reqwest::Client.
    // let proxy = reqwest::Proxy::all("socks5://telegram_user:wqe45gfbSZ_dWWQ@ec2-34-226-247-158.compute-1.amazonaws.com:443")?;

    // let client = reqwest::Client::builder().proxy(proxy).build()?;

    // let bot = Bot::with_client(token, client);

    // Dispatcher::new(bot)
    //     .messages_handler(handle_messages)
    //     .dispatch()
    //     .await;

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