use crate::parser::LogEntry;
use crate::stats::Stats;
use colored::*;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct EntryRow {
    #[tabled(rename = "Время")]
    time: String,
    #[tabled(rename = "Уровень")]
    level: String,
    #[tabled(rename = "Сообщение")]
    message: String,
    #[tabled(rename = "Статус")]
    status: String,
}

pub fn print_entries(entries: &[LogEntry]) {
    let rows: Vec<EntryRow> = entries
        .iter()
        .map(|e| EntryRow {
            time: e.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            level: match e.level.as_str() {
                "ERROR" => e.level.red().bold().to_string(),
                "WARN" => e.level.yellow().to_string(),
                _ => e.level.green().to_string(),
            },
            message: e.message.chars().take(60).collect(),
            status: e.status.map(|s| s.to_string()).unwrap_or_default(),
        })
        .collect();

    let table = Table::new(rows).with(Style::rounded()).to_string();
    println!("{}", table);
}

pub fn print_csv(entries: &[LogEntry]) {
    println!("timestamp,level,status,ip,bytes,request,message");
    for e in entries {
        println!(
            "{},{},{},{},{},{},{}",
            csv_field(&e.timestamp.to_rfc3339()),
            csv_field(&e.level),
            e.status.map(|s| s.to_string()).unwrap_or_default(),
            csv_field(e.ip.as_deref().unwrap_or("")),
            e.bytes.map(|b| b.to_string()).unwrap_or_default(),
            csv_field(e.request.as_deref().unwrap_or("")),
            csv_field(&e.message),
        );
    }
}

/// Экранирует поле по правилам CSV (RFC 4180).
fn csv_field(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

pub fn print_stats(stats: &Stats) {
    println!("\n{}", "═══ СТАТИСТИКА ═══".bold().cyan());
    println!("Всего записей: {}", stats.total.to_string().bold());

    println!("\n{}", "По уровням:".bold());
    let mut levels: Vec<_> = stats.by_level.iter().collect();
    levels.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    for (level, count) in levels {
        println!("  {}: {}", level, count);
    }

    if !stats.by_status.is_empty() {
        println!("\n{}", "По HTTP-статусам:".bold());
        let mut statuses: Vec<_> = stats.by_status.iter().collect();
        statuses.sort_by_key(|(status, _)| **status);
        for (status, count) in statuses {
            println!("  {}: {}", status, count);
        }
    }

    if !stats.top_ips.is_empty() {
        println!("\n{}", "Топ IP:".bold());
        for (ip, count) in &stats.top_ips {
            println!("  {}: {}", ip, count);
        }
    }

    if !stats.top_errors.is_empty() {
        println!("\n{}", "Топ ошибок:".bold());
        for (message, count) in &stats.top_errors {
            println!("  {}: {}", message.chars().take(80).collect::<String>(), count);
        }
    }

    if stats.total_bytes > 0 {
        println!("\nВсего байт: {}", stats.total_bytes);
    }

    if let Some((start, end)) = stats.time_range {
        println!(
            "\nПериод: {} — {}",
            start.format("%Y-%m-%d %H:%M:%S"),
            end.format("%Y-%m-%d %H:%M:%S")
        );
    }
}
