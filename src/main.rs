use clap::{Parser, Subcommand};
mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "localhost:9092")]
    brokers: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Topic {
        #[command(subcommand)]
        command: TopicCommands,
    },
}

#[derive(Subcommand)]
enum TopicCommands {
    List,
    Produce {
        /// Name of the topic to produce to
        name: String,
        /// Optional key for the message
        #[arg(short, long)]
        key: Option<String>,
        /// Optional payload for the message
        #[arg(short, long)]
        payload: Option<String>,
        /// Optional file to read the payload from
        #[arg(short, long)]
        file: Option<String>,
    },
    Consume {
        /// Name of the topic to consume from
        name: String,
        /// Consumer group ID
        #[arg(short, long, default_value = "oxk-group")]
        group: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Topic { command } => match command {
            TopicCommands::List => {
                commands::topic::list(&args.brokers).await?;
            }
            TopicCommands::Produce {
                name,
                key,
                payload,
                file,
            } => {
                commands::topic::produce(
                    &args.brokers,
                    &name,
                    key.as_ref(),
                    payload.as_ref(),
                    file.as_ref(),
                )
                .await?;
            }
            TopicCommands::Consume { name, group } => {
                commands::topic::consume(&args.brokers, &group, &name).await?;
            }
        },
    }

    Ok(())
}
