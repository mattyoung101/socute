use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use env_logger::{Builder, Env};
use log::{error, info};

pub mod tokeniser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Subcommand)]
enum Commands {
    /// Compiles a single SCU DSP assembly file
    Compile {
        #[arg(long)]
        /// Source file
        src: PathBuf,

        #[arg(long)]
        /// Destination file
        dest: PathBuf,
    },

    /// Prints version information.
    #[command()]
    Version {},
}

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "socute")]
#[command(
    about = format!("SoCUte v{VERSION}: Sega Saturn SCU DSP Assembler"),
)]
struct SoCuteCli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> color_eyre::Result<()> {
    let args = SoCuteCli::parse();
    let env = Env::new().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();
    color_eyre::install()?;

    match args.command {
        Commands::Compile { src, dest } => {}
        Commands::Version {} => {
            println!("SoCUte v{VERSION}: Sega Saturn SCU DSP Assembler",);
            println!("Copyright (c) 2025 Matt Young. Mozilla Public License v2.0.");
        }
    }

    Ok(())
}
