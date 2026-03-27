use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "shadow", about = "Your AI identity, observable and portable.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the MCP server
    Serve {
        /// Use SSE transport instead of stdio
        #[arg(long)]
        sse: bool,

        /// Port for SSE transport
        #[arg(long, default_value = "3847")]
        port: u16,
    },

    /// Initialize ~/.shadow/ directory structure
    Init,

    /// Search the persona store
    Search {
        /// Natural language query
        query: String,

        /// Filter by memory type
        #[arg(long)]
        filter: Option<String>,

        /// Max results
        #[arg(long, default_value = "6")]
        limit: usize,
    },

    /// Force reindex
    Index {
        /// Full rebuild (re-embed everything)
        #[arg(long)]
        rebuild: bool,
    },

    /// Show index health and stats
    Status,

    /// Regenerate SHADOW.md snapshot
    Snapshot,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("shadow=info".parse()?),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { sse, port } => {
            if sse {
                tracing::info!("Starting Shadow MCP server (SSE on port {port})");
                todo!("SSE transport")
            } else {
                tracing::info!("Starting Shadow MCP server (stdio)");
                todo!("stdio transport")
            }
        }
        Commands::Init => {
            todo!("init ~/.shadow/")
        }
        Commands::Search { query, filter, limit } => {
            tracing::info!(%query, ?filter, %limit, "Searching");
            todo!("search")
        }
        Commands::Index { rebuild } => {
            tracing::info!(%rebuild, "Indexing");
            todo!("index")
        }
        Commands::Status => {
            todo!("status")
        }
        Commands::Snapshot => {
            todo!("snapshot")
        }
    }
}
