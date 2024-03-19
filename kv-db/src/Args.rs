use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "kv_client")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: ClientArgs,
}

#[derive(Subcommand)]
pub enum ClientArgs {
    Get {
        #[clap(long)]
        key: String,
    },
    Set {
        #[clap(long)]
        key: String,
        #[clap(long)]
        value: String,
    },
    Publish {
        #[clap(long)]
        topic: String,
        #[clap(long)]
        value: String,
    },
    Subscribe {
        #[clap(long)]
        topic: String,
    },
    Unsubscribe {
        #[clap(long)]
        topic: String,
        #[clap(long)]
        id: u32,
    },
}