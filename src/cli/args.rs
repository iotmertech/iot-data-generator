use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "mer",
    version,
    about = "A developer-friendly IoT test data generator CLI",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// MQTT commands
    Mqtt {
        #[command(subcommand)]
        command: MqttCommands,
    },
    /// HTTP commands
    Http {
        #[command(subcommand)]
        command: HttpCommands,
    },
    /// TCP commands
    Tcp {
        #[command(subcommand)]
        command: TcpCommands,
    },
    /// Validate a config file
    Validate {
        #[command(subcommand)]
        command: ValidateCommands,
    },
    /// Preview generated payloads without sending
    Preview {
        #[command(subcommand)]
        command: PreviewCommands,
    },
    /// Generate a starter config file
    Init(InitArgs),
}

#[derive(Subcommand, Debug)]
pub enum MqttCommands {
    /// Send MQTT messages
    Run(RunArgs),
}

#[derive(Subcommand, Debug)]
pub enum HttpCommands {
    /// Send HTTP messages
    Run(RunArgs),
}

#[derive(Subcommand, Debug)]
pub enum TcpCommands {
    /// Send TCP messages
    Run(RunArgs),
}

#[derive(Subcommand, Debug)]
pub enum ValidateCommands {
    /// Validate the config file
    Config(ConfigFileArgs),
}

#[derive(Subcommand, Debug)]
pub enum PreviewCommands {
    /// Preview a generated payload
    Payload(PreviewArgs),
}

#[derive(Parser, Debug)]
pub struct RunArgs {
    /// Path to config file
    #[arg(short, long, default_value = "mer.yaml")]
    pub file: String,
}

#[derive(Parser, Debug)]
pub struct ConfigFileArgs {
    /// Path to config file
    #[arg(short, long, default_value = "mer.yaml")]
    pub file: String,
}

#[derive(Parser, Debug)]
pub struct PreviewArgs {
    /// Path to config file
    #[arg(short, long, default_value = "mer.yaml")]
    pub file: String,
    /// Number of sample payloads to show
    #[arg(short = 'n', long, default_value = "3")]
    pub count: usize,
}

#[derive(Parser, Debug)]
pub struct InitArgs {
    /// Protocol to generate config for
    #[arg(long, default_value = "mqtt", value_parser = ["mqtt", "http", "tcp"])]
    pub protocol: String,
}
