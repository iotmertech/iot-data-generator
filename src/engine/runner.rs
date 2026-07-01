use crate::config::model::Config;
use crate::device::{DeviceContext, DevicePool};
use crate::error::Result;
use crate::payload::{PayloadGenerator, SimClock};
use crate::protocols::sender::{OutboundMessage, Sender};
use crate::report::metrics::Metrics;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct Runner {
    config: Config,
    sender: Box<dyn Sender>,
    generator: PayloadGenerator,
    metrics: Arc<Metrics>,
}

impl Runner {
    pub fn new(
        config: Config,
        sender: Box<dyn Sender>,
        generator: PayloadGenerator,
        metrics: Arc<Metrics>,
    ) -> Self {
        Self {
            config,
            sender,
            generator,
            metrics,
        }
    }

    pub async fn run(&mut self) -> Result<Duration> {
        let mut pool = DevicePool::new(self.config.device.count, &self.config.device.id_prefix);

        let mut clock = SimClock::from_config(&self.config.time)?;
        let ts_field = clock.field().to_string();

        let interval = Duration::from_millis(self.config.run.interval_ms);
        let total = self.config.run.total_messages;
        let max_duration = self.config.run.duration_secs.map(Duration::from_secs);

        let start = Instant::now();
        let mut seq: usize = 0;

        println!(
            "Sending {} messages with interval {}ms ...",
            total, self.config.run.interval_ms
        );

        for i in 0..total {
            if let Some(max_dur) = max_duration {
                if start.elapsed() >= max_dur {
                    println!("Duration limit reached after {} messages.", i);
                    break;
                }
            }

            let device: DeviceContext = pool.next().clone();
            let ts = clock.timestamp(seq);
            let payload = match self.generator.generate(&device, seq, ts, &ts_field) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Payload generation error: {}", e);
                    self.metrics.record_failure();
                    seq += 1;
                    continue;
                }
            };

            let msg = OutboundMessage {
                topic_or_path: String::new(),
                payload,
                device_id: device.device_id.clone(),
            };

            match self.sender.send(&msg).await {
                Ok(result) => {
                    if result.success {
                        self.metrics.record_success(result.bytes_sent);
                    } else {
                        if let Some(err) = &result.error {
                            eprintln!("[{}] Send error: {}", device.device_id, err);
                        }
                        self.metrics.record_failure();
                    }
                }
                Err(e) => {
                    eprintln!("[{}] Send error: {}", device.device_id, e);
                    self.metrics.record_failure();
                }
            }

            seq += 1;

            if i + 1 < total {
                sleep(interval).await;
            }
        }

        let _ = self.sender.close().await;
        Ok(start.elapsed())
    }
}
