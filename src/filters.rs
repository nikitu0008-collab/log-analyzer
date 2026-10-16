use crate::parser::LogEntry;
use chrono::{DateTime, Utc};

pub struct Filter {
    pub level: Option<String>,
    pub since: Option<DateTime<Utc>>,
    pub ip: Option<String>,
    pub pattern: Option<regex::Regex>,
}

impl Filter {
    pub fn matches(&self, entry: &LogEntry) -> bool {
        // Уровень (регистронезависимо)
        if let Some(ref lvl) = self.level
            && !entry.level.eq_ignore_ascii_case(lvl)
        {
            return false;
        }

        // Время
        if let Some(since) = self.since
            && entry.timestamp < since
        {
            return false;
        }

        // IP (для Nginx)
        if let Some(ref ip) = self.ip
            && entry.ip.as_deref() != Some(ip.as_str())
        {
            return false;
        }

        // Regex-паттерн по сообщению
        if let Some(ref re) = self.pattern
            && !re.is_match(&entry.message)
        {
            return false;
        }

        true
    }
}
