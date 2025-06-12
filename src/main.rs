// SoCUte: An assembler for the Sega Saturn SCU DSP.
//
// Copyright (c) 2025 Matt Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
use std::{fs::File, io::{BufReader, Read}, path::PathBuf};

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use env_logger::{Builder, Env};
use log::{debug, error, info};

use crate::{emitter::Program, parser::document, tokeniser::lex};

pub mod tokeniser;
pub mod parser;
pub mod emitter;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Subcommand)]
enum Commands {
    /// Assemble a single SCU DSP source file
    Asm {
        /// Source file
        src: PathBuf,

        /// Destination file
        dest: PathBuf,

        #[arg(long, action)]
        /// If true, enforces strict compatibility with the original assembler (not supported yet)
        strict: bool,

        #[arg(long, action)]
        /// Print parser debug information
        debug: bool,
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
    let env = Env::new().filter_or("RUST_LOG", "debug");
    Builder::from_env(env).init();
    color_eyre::install()?;

    match args.command {
        Commands::Asm { src, dest, strict, debug } => {
            let mut f = File::open(src)?;
            let mut string = String::new();
            f.read_to_string(&mut string)?;

            let mut tokens = lex(string.as_str());
            let mut prog = Program::default();
            document(&mut tokens, &mut prog)?;
        }
        Commands::Version {} => {
            println!("SoCUte v{VERSION}: Sega Saturn SCU DSP Assembler <https://github.com/mattyoung101/socute>");
            println!("Copyright (c) 2025 Matt Young. Mozilla Public License v2.0.");
        }
    }

    Ok(())
}
