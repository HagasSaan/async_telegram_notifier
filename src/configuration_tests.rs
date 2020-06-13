use crate::configuration::Configuration;
use crate::developer::Developer;
use crate::developer::Timetable;
use chrono::prelude::*;
use rstest::*;

use std::fs::File;
use std::io::prelude::*;

#[fixture]
fn f_user1() -> Developer {
    Developer {
        username: "user1".to_string(),
        tg_chat_id: 123,
        timetable: Timetable {
            days: vec![
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
            ],
            started_at: chrono::NaiveTime::from_hms(9, 0, 0),
            ended_at: chrono::NaiveTime::from_hms(17, 0, 0),
        },
    }
}

#[fixture]
fn f_config(f_user1: Developer) -> Configuration {
    Configuration {
        group_chat_id: Some(789),
        skip_keywords: vec!["skipmel".to_string(), "do-not-merge".to_string()]
            .into_iter()
            .collect(),
        developers: vec![
            f_user1,
            Developer {
                username: "user2".to_string(),
                tg_chat_id: 456,
                timetable: Timetable {
                    days: vec![Weekday::Mon],
                    started_at: chrono::NaiveTime::from_hms(12, 0, 0),
                    ended_at: chrono::NaiveTime::from_hms(20, 0, 0),
                },
            },
        ]
        .into_iter()
        .collect(),
    }
}

#[rstest]
fn test_load_configuration(f_config: Configuration) {
    let mut file = File::open("configuration_template.yml").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let config = Configuration::load_from_str(&content);
    assert_eq!(config, f_config);
}

#[rstest(username, developer, 
    case(&"user1".to_string(), Some(f_user1())), 
    case(&"unknown_user".to_string(), None)
)]
fn test_get_developer(
    f_config: Configuration,
    username: &String,
    developer: Option<Developer>,
) {
    assert_eq!(f_config.get_developer(username), developer);
}