# log-analyzer

[![Rust](https://img.shields.io/badge/rust-1.70%2B-red.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/nikitu0008-collab/log-analyzer/actions/workflows/ci.yml/badge.svg)](https://github.com/nikitu0008-collab/log-analyzer/actions/workflows/ci.yml)

Быстрый CLI-инструмент для анализа Nginx и JSON-логов: фильтрация, агрегация статистики и вывод в удобных форматах.

## Возможности

- **Поддержка форматов**: Nginx (common/combined), JSON (`{"level": ..., "timestamp": ..., "message": ...}`)
- **Фильтрация**: по уровню лога (`ERROR`, `WARN`, `INFO`), по IP, по времени, по regex-паттерну в сообщении
- **Статистика**: общее количество, распределение по уровням и HTTP-статусам, топ-IP, топ-ошибок, объём трафика, временной диапазон
- **Форматы вывода**: красивая таблица в терминале, JSON, CSV
- **Маленький и быстрый**: построчный парсинг, один проход файла
- **Готовые бенчмарки**: измерение производительности с помощью [`criterion`](https://github.com/bheisler/criterion.rs)

## Установка

Требуется [Rust](https://www.rust-lang.org/tools/install) (stable, 1.70+).

```bash
git clone https://github.com/nikitu0008-collab/log-analyzer.git
cd log-analyzer
cargo build --release
# бинарник: target/release/log-analyzer
```

## Быстрый старт

### Анализ Nginx-лога

```bash
log-analyzer --file /var/log/nginx/access.log
```

### Только статистика

```bash
log-analyzer --file access.log --stats-only
```

### Фильтрация

```bash
# Только ошибки с IP 10.0.0.1 после 2025-01-01T12:00:00Z
log-analyzer --file access.log --level ERROR --ip 10.0.0.1 --since 2025-01-01T12:00:00Z

# Фильтр по регулярному выражению в сообщении/запросе
log-analyzer --file access.log --pattern "api/(users|orders)"
```

### Вывод в CSV или JSON

```bash
# Экспорт в CSV для Excel/LibreOffice
log-analyzer --file access.log --output csv > report.csv

# Машинный вывод в JSON
log-analyzer --file access.log --output json > report.json
```

### Анализ JSON-логов

```bash
log-analyzer --file app.jsonl --format json --top 5
```

## Формат Nginx

Поддерживаются строки вида common и combined:

```
127.0.0.1 - - [08/Nov/2013:13:39:18 +0000] "GET /api/foo HTTP/1.1" 200 612
127.0.0.1 - - [08/Nov/2013:13:39:18 +0000] "GET /api/foo HTTP/1.1" 200 612 "-" "curl/7.64.1"
```

HTTP-статусы автоматически мапятся в уровни: `2xx → INFO`, `3xx → WARN`, `4xx → WARN`, `5xx → ERROR`.

## Формат JSON

Достаточно полей `level`, `timestamp`, `message`. Остальные поля игнорируются.

```json
{"level":"INFO","timestamp":"2025-11-03T10:00:01Z","message":"Service started"}
{"level":"ERROR","timestamp":"2025-11-03T10:00:04Z","message":"DB connection failed"}
```

## Опции

| Опция | Описание | По умолчанию |
|---|---|---|
| `-f, --file <FILE>` | Путь к файлу с логами | — (обязательный) |
| `--format <FORMAT>` | Формат: `nginx` или `json` | `nginx` |
| `-l, --level <LEVEL>` | Фильтр по уровню: `ERROR`, `WARN`, `INFO` | — |
| `--since <SINCE>` | Фильтр по времени: `RFC-3339`, `YYYY-MM-DD`, `YYYY-MM-DD HH:MM:SS` | — |
| `--ip <IP>` | Фильтр по IP-адресу (только Nginx) | — |
| `-p, --pattern <PATTERN>` | Фильтр по regex-паттерну | — |
| `-o, --output <OUTPUT>` | Вывод: `table`, `json`, `csv` | `table` |
| `--stats-only` | Показать только статистику, без строк | — |
| `--top <TOP>` | Топ-N самых частых ошибок/IP | `10` |

## Разработка

```bash
# Сборка и проверка
cargo build
cargo test
cargo clippy --all-targets

# Бенчмарки
cargo bench

# Запуск с примерами из test_data (если наполнить)
cargo run -- --file test_data/nginx_access.log
```

## Тестовые данные

В каталоге `test_data/` можно положить примеры логов для ручной проверки (`nginx_access.log`, `app.jsonl`).

## Лицензия

Проект распространяется под лицензией [MIT](LICENSE).

## Автор

[@nikitu0008-collab](https://github.com/nikitu0008-collab)
