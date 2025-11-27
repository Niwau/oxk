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
    Produce {
        #[arg(short, long)]
        topic: String,
        #[arg(short, long)]
        key: Option<String>,
        #[arg(short, long)]
        payload: Option<String>,
        #[arg(short, long)]
        file: Option<String>,
    },
    Consume {
        #[arg(short, long)]
        topic: String,
        #[arg(short, long, default_value = "oxk-group")]
        group: String,
    },
    List,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::Produce {
            topic,
            key,
            payload,
            file,
        } => {
            commands::produce(
                &args.brokers,
                topic,
                key.as_ref(),
                payload.as_ref(),
                file.as_ref(),
            )
            .await?
        }
        Commands::Consume { topic, group } => {
            commands::consume(&args.brokers, group, topic).await?
        }
        Commands::List => commands::list_topics(&args.brokers).await?,
    }

    Ok(())
}
