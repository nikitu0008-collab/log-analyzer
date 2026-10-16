use crate::parser::LogEntry;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Stats {
    pub total: usize,
    pub by_level: HashMap<String, usize>,
    pub by_status: HashMap<u16, usize>,
    pub top_ips: Vec<(String, usize)>,
    pub top_errors: Vec<(String, usize)>,
    pub total_bytes: u64,
    pub time_range: Option<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>,

    // Промежуточные счётчики, которые сворачиваются в отсортированные топы
    // в `finalize`.
    ip_counts: HashMap<String, usize>,
    error_counts: HashMap<String, usize>,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: &LogEntry) {
        self.total += 1;
        *self.by_level.entry(entry.level.clone()).or_insert(0) += 1;

        if let Some(status) = entry.status {
            *self.by_status.entry(status).or_insert(0) += 1;
        }

        if let Some(ref ip) = entry.ip {
            *self.ip_counts.entry(ip.clone()).or_insert(0) += 1;
        }

        if entry.level.eq_ignore_ascii_case("ERROR") {
            *self.error_counts.entry(entry.message.clone()).or_insert(0) += 1;
        }

        if let Some(bytes) = entry.bytes {
            self.total_bytes += bytes;
        }

        // Обновляем временной диапазон
        match self.time_range {
            Some((start, end)) => {
                self.time_range = Some((start.min(entry.timestamp), end.max(entry.timestamp)));
            }
            None => {
                self.time_range = Some((entry.timestamp, entry.timestamp));
            }
        }
    }

    /// Сворачивает промежуточные счётчики в отсортированные топы.
    /// `top` — сколько элементов оставлять в каждом топе.
    pub fn finalize(&mut self, top: usize) {
        self.top_ips = top_n(&self.ip_counts, top);
        self.top_errors = top_n(&self.error_counts, top);
    }
}

fn top_n(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut items: Vec<(String, usize)> = counts
        .iter()
        .map(|(key, count)| (key.clone(), *count))
        .collect();
    // Сначала по убыванию частоты, при равенстве — по алфавиту (детерминированно).
    items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    items.truncate(n);
    items
}
