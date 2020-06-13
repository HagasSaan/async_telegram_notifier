use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
pub struct Developer {
    pub username: String,
    pub tg_chat_id: i64,
    pub timetable: Timetable,
}

impl Developer {
    pub fn is_working_time(&self) -> bool {
        let now = chrono::Local::now();
        let time: chrono::NaiveTime = now.time();
        let today: chrono::Weekday = now.date().weekday();
        if !self.timetable.days.contains(&today) {
            return false;
        };
        if self.timetable.started_at <= self.timetable.ended_at {
            // timerange doesn't cross midnight
            self.timetable.started_at <= time && time <= self.timetable.ended_at
        } else {
            !(self.timetable.ended_at <= time && time <= self.timetable.started_at)
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
pub struct Timetable {
    pub days: Vec<chrono::Weekday>,
    pub started_at: chrono::NaiveTime,
    pub ended_at: chrono::NaiveTime,
}
