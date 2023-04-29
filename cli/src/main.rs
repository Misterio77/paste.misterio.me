use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io;

use pmis::{operations, PathBuf, Result, Url, Uuid};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(disable_help_subcommand = true)]
struct Cli {
    /// API URL to use
    #[clap(long, default_value = "https://paste.misterio.me")]
    api: Url,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lists pastes from a given user (or self, if owner is ommited)
    #[clap(alias = "l", alias = "ls")]
    List {
        owner: Option<String>,
        /// Only outputs ids, useful for scripting
        #[clap(short, long)]
        ids_only: bool,
    },
    /// Downloads and shows a given paste (gets raw paste if piped)
    #[clap(alias = "d", alias = "down", alias = "get")]
    Download {
        id: Uuid,
        /// Raw paste output, even on interactive terminals
        #[clap(short, long)]
        raw: bool,
    },
    /// Uploads a file and creates a new paste.
    #[clap(alias = "u", alias = "up", alias = "create", alias = "post")]
    Upload {
        /// File to upload. If ommited, reads from stdin
        file: Option<PathBuf>,
        /// Title for your paste. Defaults to file name or "Untitled" (if read from stdin).
        #[clap(short, long)]
        title: Option<String>,
        /// Description for your paste. Optional.
        #[clap(short, long)]
        description: Option<String>,
        /// If specified, the paste will not be listed on your profile and will only be reachable
        /// by its link. Anonymous uploads are always unlisted and are unnaffected by this flag.
        #[clap(short, long)]
        unlisted: bool,
        /// Output only new paste link, even on interactive terminals
        #[clap(short, long)]
        link_only: bool,
    },
    /// Deletes a given paste. Requires authentication
    #[clap(alias = "del")]
    Delete { id: Uuid },
    /// Authenticates using an API key
    Auth {
        /// File containing the key. If ommited, reads from stdin
        file: Option<PathBuf>,
    },
    /// Generate shell completions
    #[clap(hide = true)]
    Completions { shell: Shell },
}

fn print_completions<G: Generator>(gen: G, app: &mut Command) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let api = cli.api;
    let mut app = Cli::command();

    match cli.command {
        Commands::List { owner, ids_only } => operations::list(api, owner, ids_only).await?,
        Commands::Download { id, raw } => operations::download(api, id, raw).await?,
        Commands::Upload {
            file,
            title,
            description,
            unlisted,
            link_only,
        } => operations::upload(api, file, title, description, unlisted, link_only).await?,
        Commands::Delete { id } => operations::delete(api, id).await?,
        Commands::Auth { file } => operations::auth(api, file).await?,
        Commands::Completions { shell } => print_completions(shell, &mut app),
    }

    Ok(())
}
