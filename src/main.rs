mod developer;
mod repository;
mod configuration;
mod pull_request;
mod notification_service;
mod notification_reminder;

use repository::GithubRepository;
use configuration::Configuration;
use notification_service::NotificationService;
use notification_reminder::NotificationReminder;

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
    let github_token = "c4dcf055cc883690a33d73908b62206b545b2854".to_string();
    let telegram_token = "1214608092:AAEe209RQ76oZDJGZA80dbV9IDoIfCkSFv0".to_string();
    // let config_file_name = "configuration_example.yml".to_string();
    let proxy_params: Option<&str> = Some("socks5://telegram_user:wqe45gfbSZ_dWWQ@ec2-34-226-247-158.compute-1.amazonaws.com:443");
    
    let repository = GithubRepository::new(repository_name, github_token);
    // let notifier = NotificationService::new(telegram_token, proxy_params);
    // let raw_config = repository.get_file(config_file_name).await.expect("Failed to get configuration file");
    // let configuration: Configuration = Configuration::load_from_str(&raw_config);

    // let reminder = NotificationReminder::new(notifier, configuration);
    let pull_requests = repository.get_pull_requests().await.expect("Failed to get pull requests");
    println!("{:?}", pull_requests);
    // reminder.remind(pull_requests).await;

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