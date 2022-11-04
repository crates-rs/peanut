mod connection;
mod message;
mod server;
mod client;

use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 服务
    Start {
        // cargo run -p peanut start --port 3333
        #[clap(name = "hostname", long, default_value = "127.0.0.1")]
        host: String,
        /// 端口
        #[arg(short, long, default_value_t = 6379)]
        port: u16,
    },
    Stop, // cargo run -p peanut stop
    Connect {
        // cargo run -p peanut connect --port 3333
        #[clap(name = "hostname", long, default_value = "127.0.0.1")]
        host: String,
        /// 端口
        #[arg(short, long, default_value_t = 6379)]
        port: u16,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).try_init()?;
    let args = Args::parse();
    println!("Hello, world! {:?}", args);

    match args.command {
        Commands::Start {host, port} => {
            let addr = format!("{}:{}", host, port);
            server::start_server(addr).await?;
        },
        Commands::Stop => {},
        Commands::Connect {host, port} => {
            let addr = format!("{}:{}", host, port);
            client::connect_server(addr).await?;
        },
    }

    Ok(())
}
