use anyhow::Result;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use clap::Parser;
use log_analyzer::cli::{Cli, LogFormat, OutputFormat};
use log_analyzer::filters::Filter;
use log_analyzer::{output, parser, stats};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let file = File::open(&cli.file)?;
    let reader = BufReader::new(file);

    let filter = Filter {
        level: cli.level.clone(),
        since: cli.since.as_deref().and_then(parse_since),
        ip: cli.ip.clone(),
        pattern: cli.pattern.as_ref().and_then(|p| regex::Regex::new(p).ok()),
    };

    let mut stats = stats::Stats::new();
    let mut matched_entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let entry = match cli.format {
            LogFormat::Nginx => parser::parse_nginx_line(&line),
            LogFormat::Json => parser::parse_json_line(&line),
        };

        match entry {
            Ok(entry) => {
                if filter.matches(&entry) {
                    stats.add(&entry);
                    matched_entries.push(entry);
                }
            }
            Err(e) => eprintln!("⚠ Пропущена строка: {}", e),
        }
    }

    stats.finalize(cli.top);

    if !cli.stats_only {
        match cli.output {
            OutputFormat::Table => output::print_entries(&matched_entries),
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&matched_entries)?);
            }
            OutputFormat::Csv => output::print_csv(&matched_entries),
        }
    }

    output::print_stats(&stats);
    Ok(())
}

/// Разбирает значение `--since`.
///
/// Поддерживаются RFC-3339 ("2025-01-01T12:00:00Z"),
/// "ГГГГ-ММ-ДД" и "ГГГГ-ММ-ДД ЧЧ:ММ:СС".
fn parse_since(value: &str) -> Option<DateTime<Utc>> {
    let value = value.trim();

    if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
        return Some(dt.with_timezone(&Utc));
    }

    if let Ok(dt) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S") {
        return Some(dt.and_utc());
    }

    if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d") {
        return Some(date.and_hms_opt(0, 0, 0)?.and_utc());
    }

    None
}
