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
