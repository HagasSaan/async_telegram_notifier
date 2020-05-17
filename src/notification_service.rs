use crate::developer::TelegramChatId;
use crate::pull_request::GithubPullRequest;

#[derive(Debug)]
pub struct NotificationService {
    bot: teloxide::Bot,
}

impl NotificationService {
    pub fn new(token: String, proxy_client: Option<reqwest::Proxy>) {}
    pub fn send_message(&self, chat_id: TelegramChatId, pull_request: GithubPullRequest) {}
}

// @dataclass
// class NotificationService:
//     _telegram_token: str
//     bot: telebot.TeleBot = field(init=False)

//     MESSAGE_TEMPLATE = '{developer} requested your review on "{title}" ({url}) {time_ago} hours ago.'

//     def __post_init__(self):
//         self.bot = telebot.TeleBot(self._telegram_token)

//     def send_message(
//         self,
//         chat_id: int,
//         pull_request: GithubPullRequest
//     ) -> None:
//         time_ago = datetime.now() - pull_request.updated_at
//         time_ago -= timedelta(microseconds=time_ago.microseconds)
//         message = self.MESSAGE_TEMPLATE.format(
//             developer=pull_request.developer,
//             title=pull_request.title,
//             url=pull_request.html_url,
//             time_ago=time_ago
//         )
//         logger.info('Start sending message', chat_id=chat_id, message=message)
//         try:
//             self.bot.send_message(chat_id, message)
//             logger.info('Message sended')
//         except Exception as e:
//             logger.exception(e)
