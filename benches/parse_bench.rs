use criterion::{Criterion, black_box, criterion_group, criterion_main};
use log_analyzer::parser::parse_nginx_line;

fn bench_nginx_parse(c: &mut Criterion) {
    let line = r#"127.0.0.1 - - [08/Nov/2013:13:39:18 +0000] "GET /api/foo HTTP/1.1" 200 612 "-" "curl/7.64.1""#;

    c.bench_function("parse_nginx_line", |b| {
        b.iter(|| parse_nginx_line(black_box(line)))
    });
}

criterion_group!(benches, bench_nginx_parse);
criterion_main!(benches);
