use crate::config::model::{AuthConfig, Config};
use crate::error::{MerError, Result};
use crate::protocols::sender::{OutboundMessage, SendResult, Sender};
use async_trait::async_trait;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MqttSender {
    client: AsyncClient,
    event_loop: Arc<Mutex<EventLoop>>,
    topic_template: String,
    qos: QoS,
    retain: bool,
}

impl MqttSender {
    pub async fn from_config(config: &Config) -> Result<Self> {
        let broker = config
            .target
            .broker
            .as_deref()
            .ok_or_else(|| MerError::Config("target.broker is required for MQTT".to_string()))?;

        let topic_template = config
            .target
            .topic
            .clone()
            .ok_or_else(|| MerError::Config("target.topic is required for MQTT".to_string()))?;

        // Parse broker URI: mqtt://host:port
        let (host, port) = parse_broker(broker)?;

        let client_id = config
            .target
            .client_id
            .clone()
            .unwrap_or_else(|| format!("mer-{}", uuid::Uuid::new_v4()));

        let mut mqtt_options = MqttOptions::new(client_id, host, port);
        mqtt_options.set_keep_alive(std::time::Duration::from_secs(30));

        if let Some(auth) = &config.auth {
            match auth {
                AuthConfig::UsernamePassword { username, password } => {
                    mqtt_options.set_credentials(username, password);
                }
                _ => {
                    return Err(MerError::Config(
                        "MQTT only supports username/password auth".to_string(),
                    ));
                }
            }
        }

        let qos = match config.target.qos.unwrap_or(0) {
            0 => QoS::AtMostOnce,
            1 => QoS::AtLeastOnce,
            2 => QoS::ExactlyOnce,
            q => {
                return Err(MerError::Config(format!(
                    "Invalid MQTT QoS value: {}",
                    q
                )))
            }
        };

        let retain = config.target.retain.unwrap_or(false);

        let (client, event_loop) = AsyncClient::new(mqtt_options, 100);

        Ok(Self {
            client,
            event_loop: Arc::new(Mutex::new(event_loop)),
            topic_template,
            qos,
            retain,
        })
    }

    /// Start polling the event loop in the background so the client stays connected.
    pub async fn start_event_loop(&self) {
        let event_loop = Arc::clone(&self.event_loop);
        tokio::spawn(async move {
            loop {
                let mut el = event_loop.lock().await;
                match el.poll().await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("MQTT event loop error: {}", e);
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    }
                }
            }
        });
    }
}

fn parse_broker(broker: &str) -> Result<(String, u16)> {
    // Accept: mqtt://host:port  or  host:port  or  host
    let stripped = broker
        .strip_prefix("mqtt://")
        .or_else(|| broker.strip_prefix("mqtts://"))
        .unwrap_or(broker);

    if let Some((host, port_str)) = stripped.rsplit_once(':') {
        let port = port_str.parse::<u16>().map_err(|_| {
            MerError::Config(format!("Invalid MQTT broker port in URI: {}", broker))
        })?;
        Ok((host.to_string(), port))
    } else {
        Ok((stripped.to_string(), 1883))
    }
}

fn resolve_topic(template: &str, device_id: &str) -> String {
    template.replace("{device_id}", device_id)
}

#[async_trait]
impl Sender for MqttSender {
    async fn send(&self, message: &OutboundMessage) -> Result<SendResult> {
        let topic = resolve_topic(&self.topic_template, &message.device_id);
        let bytes = message.payload.as_bytes().to_vec();
        let bytes_sent = bytes.len();
        match self
            .client
            .publish(topic, self.qos, self.retain, bytes)
            .await
        {
            Ok(()) => Ok(SendResult {
                success: true,
                bytes_sent,
                error: None,
            }),
            Err(e) => Ok(SendResult {
                success: false,
                bytes_sent: 0,
                error: Some(e.to_string()),
            }),
        }
    }

    async fn close(&self) -> Result<()> {
        let _ = self.client.disconnect().await;
        Ok(())
    }
}
