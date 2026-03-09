mod cli;
mod config;
mod device;
mod engine;
mod error;
mod payload;
mod protocols;
mod report;

use clap::Parser;
use cli::args::{
    Cli, Commands, HttpCommands, MqttCommands, PreviewCommands, TcpCommands, ValidateCommands,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Mqtt {
            command: MqttCommands::Run(args),
        } => cli::run::run_mqtt(&args.file).await,

        Commands::Http {
            command: HttpCommands::Run(args),
        } => cli::run::run_http(&args.file).await,

        Commands::Tcp {
            command: TcpCommands::Run(args),
        } => cli::run::run_tcp(&args.file).await,

        Commands::Validate {
            command: ValidateCommands::Config(args),
        } => cli::validate::validate(&args.file),

        Commands::Preview {
            command: PreviewCommands::Payload(args),
        } => cli::preview::preview(&args.file, args.count),

        Commands::Init(args) => cli::init::init(&args.protocol),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
