use clap::{crate_version, App, Arg};

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate simple_logger;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let matches = App::new("Async Telegram Notifier")
        .about("Pull Request notification sender (Github only)")
        .version(crate_version!())
        .arg(
            Arg::from_usage("<level> 'logging messages level'")
                .possible_values(&["trace", "debug", "info", "warning", "error"]) 
                .default_value("info")
                .required(false)
        )
        .arg(
            Arg::with_name("repository name")
                .short("r")
                .long("repository")
                .help("Name of target repository")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("github token")
                .short("g")
                .long("github-token")
                .help("Github token with access to target repository")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("telegram token")
                .short("t")
                .long("telegram-token")
                .help("Telegram token of your bot")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("configuration")
                .help("Path to configuration file in your repository (or somewhere else)\
                      \nExamples:\
                      \n\t.github/tg_notifier.yml\
                      \n\thttps://path_to_config.com/file.yml\n"
                )
                .takes_value(true)
                .default_value(".github/tg_notifier.yml")
                .required(false)
        )
        .arg(
            Arg::with_name("proxy")
                .short("p")
                .long("proxy")
                .help("Proxy for telegram bot (if it blocked in your country")
                .takes_value(true)
                .required(false)
        )
        .get_matches();

    let log_level = value_t!(matches.value_of("level"), log::Level)
        .unwrap_or(log::Level::Info);

    simple_logger::init_with_level(log_level).expect("Cannot initialize logger");

    debug!("Command line args parsed:\n{:#?}", matches);

    let repository_name = 
        matches.value_of("repository name")
            .expect("Repository name is required")
            .to_string();
    
    let github_token = 
        matches.value_of("github token")
            .expect("Github token is required")
            .to_string();
    
    let telegram_token =
        matches.value_of("telegram token")
            .expect("Telegram token is required")
            .to_string();
    
    let config_file = 
        matches.value_of("config")
            .expect("Path to configuration file is required")
            .to_string();
    let is_absolute_path = config_file.contains("https");
    
    let proxy_params: Option<&str> = matches.value_of("proxy");
    
    let repository = GithubRepository::new(repository_name, github_token);
    info!("Repository initialized");
    let pull_requests = repository
        .get_pull_requests()
        .await
        .expect("Failed to get pull requests");
    
    let notifier = NotificationService::new(telegram_token, proxy_params);
    notifier.process_incoming_messages().await;
    
    let raw_config = repository
        .get_file(config_file, is_absolute_path)
        .await
        .expect("Failed to get configuration file");
    let configuration: Configuration = Configuration::load_from_str(&raw_config);
    debug!("Got configuration:\n{:#?}", configuration);
    
    let reminder = NotificationReminder::new(notifier, configuration);
    reminder.remind(pull_requests).await;

    info!("Done :)");
    
    Ok(())
}
