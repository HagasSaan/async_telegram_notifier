use serde::{Deserialize, Serialize};

pub type ChatId = i64;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct Developer {
    pub username: String,
    pub tg_chat_id: ChatId,
    pub group: String,
    pub timetable: Timetable,
}

impl Developer {
    pub fn is_working_time(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct Timetable {
    pub days: Vec<Weekdays>,
    pub started_at: chrono::NaiveTime,
    pub ended_at: chrono::NaiveTime,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Weekdays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
