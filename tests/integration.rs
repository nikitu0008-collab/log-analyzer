use std::fs;
use std::process::Command;

#[test]
fn test_nginx_parsing() {
    let log_content = r#"127.0.0.1 - - [08/Nov/2013:13:39:18 +0000] "GET /api/foo HTTP/1.1" 200 612
10.0.0.1 - - [08/Nov/2013:13:39:19 +0000] "POST /api/bar HTTP/1.1" 500 1024"#;

    let tmp = tempfile::NamedTempFile::new().unwrap();
    fs::write(tmp.path(), log_content).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_log-analyzer"))
        .args([
            "--file",
            tmp.path().to_str().unwrap(),
            "--format",
            "nginx",
            "--stats-only",
        ])
        .output()
        .expect("Failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Всего записей: 2"));
    assert!(stdout.contains("ERROR: 1"));
}

#[test]
fn test_json_parsing() {
    let log_content = r#"{"level":"INFO","timestamp":"2025-11-03T10:00:01Z","message":"Service started"}
{"level":"ERROR","timestamp":"2025-11-03T10:00:04Z","message":"Database connection failed"}"#;

    let tmp = tempfile::NamedTempFile::new().unwrap();
    fs::write(tmp.path(), log_content).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_log-analyzer"))
        .args([
            "--file",
            tmp.path().to_str().unwrap(),
            "--format",
            "json",
            "--stats-only",
        ])
        .output()
        .expect("Failed to run binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("INFO: 1"));
    assert!(stdout.contains("ERROR: 1"));
}
