use crate::config::model::Protocol;
use crate::config::{load_config, validate_config};
use crate::engine::Runner;
use crate::error::{MerError, Result};
use crate::payload::PayloadGenerator;
use crate::protocols::http::HttpSender;
use crate::protocols::mqtt::MqttSender;
use crate::protocols::sender::Sender;
use crate::protocols::tcp::TcpSender;
use crate::report::{print_summary, Metrics};
use std::path::Path;

pub async fn run_mqtt(file: &str) -> Result<()> {
    run_protocol(file, Protocol::Mqtt).await
}

pub async fn run_http(file: &str) -> Result<()> {
    run_protocol(file, Protocol::Http).await
}

pub async fn run_tcp(file: &str) -> Result<()> {
    run_protocol(file, Protocol::Tcp).await
}

async fn run_protocol(file: &str, expected_protocol: Protocol) -> Result<()> {
    let config = load_config(Path::new(file))?;

    if config.protocol != expected_protocol {
        return Err(MerError::Validation(format!(
            "Config protocol '{}' does not match command protocol '{}'",
            config.protocol, expected_protocol
        )));
    }

    validate_config(&config)?;

    let generator = PayloadGenerator::from_config(&config)?;
    let metrics = Metrics::new();

    let sender: Box<dyn Sender> = match config.protocol {
        Protocol::Mqtt => {
            let s = MqttSender::from_config(&config).await?;
            s.start_event_loop().await;
            Box::new(s)
        }
        Protocol::Http => Box::new(HttpSender::from_config(&config)?),
        Protocol::Tcp => Box::new(TcpSender::from_config(&config)?),
    };

    let protocol_name = config.protocol.to_string();
    let mut runner = Runner::new(config, sender, generator, metrics.clone());

    println!("Starting {} run...", protocol_name.to_uppercase());
    let elapsed = runner.run().await?;
    let snapshot = metrics.snapshot();
    print_summary(&snapshot, elapsed, &protocol_name);

    Ok(())
}
