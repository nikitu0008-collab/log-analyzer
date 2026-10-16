use crate::error::{LogError, Result};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// Паттерн для common/combined формата Nginx-логов.
/// Компилируется один раз на весь процесс, а не на каждую строку.
static NGINX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"^(?P<ip>\S+) \S+ \S+ \[(?P<time>[^\]]+)\] "(?P<request>[^"]*)" (?P<status>\d{3}) (?P<bytes>\d+)(?: "(?P<referer>[^"]*)" "(?P<agent>[^"]*)")?"#,
    )
    .expect("корректное регулярное выражение Nginx")
});

#[derive(Debug, Serialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub ip: Option<String>,
    pub status: Option<u16>,
    pub bytes: Option<u64>,
    pub request: Option<String>,
    pub user_agent: Option<String>,
    pub raw: String,
}

pub fn parse_nginx_line(line: &str) -> Result<LogEntry> {
    // Common: 127.0.0.1 - - [08/Nov/2013:13:39:18 +0000] "GET /api/foo HTTP/1.1" 200 612
    // Combined: + "$http_referer" "$http_user_agent"
    let caps = NGINX_RE
        .captures(line)
        .ok_or_else(|| LogError::NginxParse(format!("Не удалось разобрать строку: {}", line)))?;

    let timestamp =
        DateTime::parse_from_str(caps.name("time").unwrap().as_str(), "%d/%b/%Y:%H:%M:%S %z")
            .map_err(|e| LogError::InvalidTimestamp(e.to_string()))?
            .with_timezone(&Utc);

    let status: u16 = caps
        .name("status")
        .unwrap()
        .as_str()
        .parse()
        .map_err(|e: std::num::ParseIntError| LogError::NginxParse(e.to_string()))?;

    let level = match status {
        200..=299 => "INFO",
        300..=399 => "WARN",
        400..=499 => "WARN",
        500..=599 => "ERROR",
        _ => "UNKNOWN",
    };

    Ok(LogEntry {
        timestamp,
        level: level.to_string(),
        message: caps
            .name("request")
            .map(|m| m.as_str())
            .unwrap_or("")
            .to_string(),
        ip: caps.name("ip").map(|m| m.as_str().to_string()),
        status: Some(status),
        bytes: caps.name("bytes").and_then(|m| m.as_str().parse().ok()),
        request: caps.name("request").map(|m| m.as_str().to_string()),
        user_agent: caps.name("agent").map(|m| m.as_str().to_string()),
        raw: line.to_string(),
    })
}

#[derive(Deserialize)]
struct JsonLog {
    level: Option<String>,
    timestamp: Option<String>,
    message: Option<String>,
}

pub fn parse_json_line(line: &str) -> Result<LogEntry> {
    let json: JsonLog = serde_json::from_str(line)?;

    let timestamp = json
        .timestamp
        .as_deref()
        .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(Utc::now);

    Ok(LogEntry {
        timestamp,
        level: json.level.unwrap_or_else(|| "INFO".to_string()),
        message: json.message.unwrap_or_default(),
        ip: None,
        status: None,
        bytes: None,
        request: None,
        user_agent: None,
        raw: line.to_string(),
    })
}
