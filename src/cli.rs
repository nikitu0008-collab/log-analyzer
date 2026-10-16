use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "log-analyzer")]
#[command(about = "Анализ Nginx и JSON логов: фильтрация, статистика", long_about = None)]
pub struct Cli {
    /// Путь к файлу с логами
    #[arg(short, long)]
    pub file: PathBuf,

    /// Формат логов
    #[arg(long, value_enum, default_value = "nginx")]
    pub format: LogFormat,

    /// Фильтр по уровню (ERROR, WARN, INFO)
    #[arg(short, long)]
    pub level: Option<String>,

    /// Фильтр по времени: RFC-3339 ("2025-01-01T12:00:00Z"),
    /// "ГГГГ-ММ-ДД" или "ГГГГ-ММ-ДД ЧЧ:ММ:СС"
    #[arg(long)]
    pub since: Option<String>,

    /// Фильтр по IP-адресу (для Nginx)
    #[arg(long)]
    pub ip: Option<String>,

    /// Фильтр по regex-паттерну в сообщении
    #[arg(short, long)]
    pub pattern: Option<String>,

    /// Формат вывода
    #[arg(short, long, value_enum, default_value = "table")]
    pub output: OutputFormat,

    /// Показать только статистику, без строк
    #[arg(long)]
    pub stats_only: bool,

    /// Показать топ-N самых частых ошибок
    #[arg(long, default_value = "10")]
    pub top: usize,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum LogFormat {
    Nginx,
    Json,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}
