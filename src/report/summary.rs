use crate::report::metrics::MetricsSnapshot;
use std::time::Duration;

pub fn print_summary(snapshot: &MetricsSnapshot, elapsed: Duration, protocol: &str) {
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Run Summary — {}", protocol.to_uppercase());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Total sent    : {}", snapshot.total_sent);
    println!("  Successful    : {}", snapshot.total_success);
    println!("  Failed        : {}", snapshot.total_failed);
    println!("  Success rate  : {:.1}%", snapshot.success_rate);
    println!("  Errors        : {}", snapshot.error_count);
    println!("  Bytes sent    : {}", format_bytes(snapshot.total_bytes));
    println!("  Elapsed       : {:.2}s", elapsed.as_secs_f64());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
