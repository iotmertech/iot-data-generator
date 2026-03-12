use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Metrics {
    pub total_sent: AtomicUsize,
    pub total_success: AtomicUsize,
    pub total_failed: AtomicUsize,
    pub total_bytes: AtomicU64,
    pub error_count: AtomicUsize,
}

impl Metrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn record_success(&self, bytes: usize) {
        self.total_sent.fetch_add(1, Ordering::Relaxed);
        self.total_success.fetch_add(1, Ordering::Relaxed);
        self.total_bytes.fetch_add(bytes as u64, Ordering::Relaxed);
    }

    pub fn record_failure(&self) {
        self.total_sent.fetch_add(1, Ordering::Relaxed);
        self.total_failed.fetch_add(1, Ordering::Relaxed);
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        let sent = self.total_sent.load(Ordering::Relaxed);
        let success = self.total_success.load(Ordering::Relaxed);
        let failed = self.total_failed.load(Ordering::Relaxed);
        let bytes = self.total_bytes.load(Ordering::Relaxed);
        let errors = self.error_count.load(Ordering::Relaxed);
        let rate = if sent > 0 {
            (success as f64 / sent as f64) * 100.0
        } else {
            0.0
        };

        MetricsSnapshot {
            total_sent: sent,
            total_success: success,
            total_failed: failed,
            total_bytes: bytes,
            error_count: errors,
            success_rate: rate,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub total_sent: usize,
    pub total_success: usize,
    pub total_failed: usize,
    pub total_bytes: u64,
    pub error_count: usize,
    pub success_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_snapshot_is_zero() {
        let m = Metrics::new();
        let s = m.snapshot();
        assert_eq!(s.total_sent, 0);
        assert_eq!(s.total_success, 0);
        assert_eq!(s.total_failed, 0);
        assert_eq!(s.total_bytes, 0);
        assert_eq!(s.error_count, 0);
        assert_eq!(s.success_rate, 0.0);
    }

    #[test]
    fn test_record_success_increments_counters() {
        let m = Metrics::new();
        m.record_success(100);
        m.record_success(200);
        let s = m.snapshot();
        assert_eq!(s.total_sent, 2);
        assert_eq!(s.total_success, 2);
        assert_eq!(s.total_failed, 0);
        assert_eq!(s.total_bytes, 300);
        assert_eq!(s.success_rate, 100.0);
    }

    #[test]
    fn test_record_failure_increments_counters() {
        let m = Metrics::new();
        m.record_failure();
        m.record_failure();
        let s = m.snapshot();
        assert_eq!(s.total_sent, 2);
        assert_eq!(s.total_success, 0);
        assert_eq!(s.total_failed, 2);
        assert_eq!(s.error_count, 2);
        assert_eq!(s.success_rate, 0.0);
    }

    #[test]
    fn test_mixed_success_and_failure() {
        let m = Metrics::new();
        m.record_success(50);
        m.record_success(50);
        m.record_success(50);
        m.record_failure();
        let s = m.snapshot();
        assert_eq!(s.total_sent, 4);
        assert_eq!(s.total_success, 3);
        assert_eq!(s.total_failed, 1);
        assert_eq!(s.total_bytes, 150);
        assert!((s.success_rate - 75.0).abs() < 0.001);
    }

    #[test]
    fn test_success_rate_no_messages() {
        let m = Metrics::new();
        let s = m.snapshot();
        assert_eq!(s.success_rate, 0.0);
    }
}
