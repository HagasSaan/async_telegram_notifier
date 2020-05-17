pub type TelegramChatId = u32;

#[derive(Debug)]
pub struct Developer {
    tg_chat_id: TelegramChatId,
    group: String,
    days: Vec<u8>,
    // started_at: datetime.time = field(default_factory=lambda: datetime.today())
    // ended_at: datetime.time = field(default_factory=lambda: datetime.today())
}

impl Developer {
    pub fn new() {}
    // pub fn is_working_time(&self) -> bool {}
}

// @dataclass
// class Developer:
//     tg_chat_id: int = -1
//     group: str = 'developers'
//     days: List[int] = field(default_factory=lambda: [0, 1, 2, 3, 4])
//     started_at: datetime.time = field(default_factory=lambda: datetime.today())
//     ended_at: datetime.time = field(default_factory=lambda: datetime.today())

//     @property
//     def is_working_time(self) -> bool:
//         now = datetime.now()
//         weekday = now.today().weekday()
//         return weekday in self.days and self.started_at <= now < self.ended_at
