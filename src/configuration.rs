use crate::developer::Developer;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Configuration {
    number_of_reviewers: u8,
    skip_keywords: HashSet<String>,
    assignee_groups: HashMap<String, String>,
    developers: HashMap<String, Developer>,
}

impl Configuration {
    // pub fn load_configuration(content: String) -> Self {}
}

// @dataclass
// class Configuration:
//     number_of_reviewers: int
//     skip_keywords: Set[str]
//     assignee_groups: Dict[str, Any]
//     developers: Dict[str, Developer]

//     @classmethod
//     def load_configuration(cls, content: str) -> 'Configuration':
//         data = yaml.safe_load(content)
//         logger.debug('Got config file', content=data)

//         developers = {}
//         for group, devs in data['assigneeGroups'].items():
//             data['assigneeGroups'][group] = set(data['assigneeGroups'][group])
//             for dev in devs:
//                 developers[dev] = {'group': group}

//         now = datetime.now()
//         time = {
//             'year': now.year,
//             'month': now.month,
//             'day': now.day,
//         }
//         for timetable in data['timetables'].values():
//             started_at = datetime.strptime(timetable['started_at'], '%H:%M')
//             started_at = datetime(
//                 **time,
//                 hour=started_at.hour,
//                 minute=started_at.minute,
//             )
//             ended_at = datetime.strptime(timetable['ended_at'], '%H:%M')
//             ended_at = datetime(
//                 **time,
//                 hour=ended_at.hour,
//                 minute=ended_at.minute,
//             )

//             if ended_at < started_at:
//                 ended_at += timedelta(days=1)

//             for user in timetable['users']:
//                 developers[user]['days'] = timetable['days']
//                 developers[user]['started_at'] = started_at
//                 developers[user]['ended_at'] = ended_at

//         for username, tg_chat_id in data['tg_chat_ids'].items():
//             developers[username]['tg_chat_id'] = tg_chat_id

//         for username, dev_data in developers.items():
//             developers[username] = Developer(**dev_data)

//         return cls(
//             number_of_reviewers=data['numberOfReviewers'],
//             assignee_groups=data['assigneeGroups'],
//             skip_keywords=set(data['skipKeywords']),
//             developers=developers,
//         )
