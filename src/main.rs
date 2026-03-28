mod languages;
mod ops;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "ast-cli",
    about = "Multi-language structural code navigation for AI agents"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show structural outline of a file (definitions, signatures, line ranges)
    Outline {
        /// Source file to analyze
        file: PathBuf,

        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },

    /// Show file with function/method bodies stripped (80-90% size reduction)
    Skeleton {
        /// Source file to analyze
        file: PathBuf,
    },

    /// Extract a specific definition by name or line range
    Read {
        /// Source file
        file: PathBuf,

        /// Address: symbol name (e.g. "Parser::parse_field") or line range (e.g. "150:200")
        address: String,
    },

    /// Find symbol definitions across a project
    Find {
        /// Directory to search
        dir: PathBuf,

        /// Symbol name to find
        name: String,

        /// Filter by kind (function, struct, class, enum, trait, etc.)
        #[arg(long)]
        kind: Option<String>,
    },

    /// Run a tree-sitter query against a file
    Query {
        /// Source file
        file: PathBuf,

        /// Tree-sitter S-expression query
        query: String,
    },

    /// Show project-wide file summary
    Project {
        /// Directory to scan
        dir: PathBuf,

        /// Exclude patterns (glob, prefix, or substring)
        #[arg(long)]
        exclude: Vec<String>,

        /// Output format
        #[arg(long, default_value = "text")]
        format: OutputFormat,
    },
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Outline { file, format } => {
            let items = ops::outline::run(&file)?;
            match format {
                OutputFormat::Text => ops::outline::print_text(&items),
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&items)?);
                }
            }
        }
        Commands::Skeleton { file } => {
            let output = ops::skeleton::run(&file)?;
            print!("{output}");
        }
        Commands::Read { file, address } => {
            let output = ops::read::run(&file, &address)?;
            print!("{output}");
        }
        Commands::Find { dir, name, kind } => {
            let results = ops::find::run(&dir, &name, kind.as_deref())?;
            ops::find::print_text(&results);
        }
        Commands::Query { file, query } => {
            let results = ops::query::run(&file, &query)?;
            ops::query::print_text(&results);
        }
        Commands::Project {
            dir,
            exclude,
            format,
        } => {
            let summary = ops::project::run(&dir, &exclude)?;
            match format {
                OutputFormat::Text => ops::project::print_text(&summary),
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&summary)?);
                }
            }
        }
    }

    Ok(())
}
