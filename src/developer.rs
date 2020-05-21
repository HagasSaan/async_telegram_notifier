use serde::{Deserialize, Serialize};

pub type ChatId = i64;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
pub struct Developer {
    pub username: String,
    pub tg_chat_id: ChatId,
    pub group: String,
    pub timetable: Timetable,
}

impl Developer {
    pub fn is_working_time(&self) -> bool {
        let now: chrono::NaiveTime = chrono::Local::now().time();
        if self.timetable.started_at <= self.timetable.ended_at {
            // timerange doesn't cross midnight
            self.timetable.started_at <= now && now <= self.timetable.ended_at
        } else {
            !(self.timetable.ended_at <= now && now <= self.timetable.started_at)
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
pub struct Timetable {
    pub days: Vec<Weekdays>,
    pub started_at: chrono::NaiveTime,
    pub ended_at: chrono::NaiveTime,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
pub enum Weekdays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
