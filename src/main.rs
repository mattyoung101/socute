// SoCUte: An assembler for the Sega Saturn SCU DSP.
//
// Copyright (c) 2025 Matt Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
use std::{fs::File, io::Read, path::PathBuf};

use clap::{Parser, Subcommand};
use color_eyre::{
    Section, SectionExt,
    owo_colors::{AnsiColors, OwoColorize},
};
use env_logger::{Builder, Env};
use log::warn;

use crate::{emitter::Program, parser::document, tokeniser::lex};

pub mod emitter;
pub mod parser;
pub mod tokeniser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Subcommand)]
enum Commands {
    /// Assemble a single SCU DSP source file
    Asm {
        /// Source file
        src: PathBuf,

        /// Destination file
        dest: Option<PathBuf>,

        #[arg(long, action)]
        /// Relaxes some parsing rules to compile files written for the original assembler on a
        /// best-effort basis
        relaxed: bool,

        #[arg(long, action)]
        /// Print internal parser debug information
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
        Commands::Asm {
            src,
            dest,
            relaxed,
            debug,
        } => {
            if relaxed {
                warn!("Running in relaxed mode; use only to parse legacy documents.");
            }

            let mut f = File::open(src)?;
            let mut string = String::new();
            f.read_to_string(&mut string)?;
            // add extra newline in case file doesn't have its own
            string += "\n";

            let lines: Vec<String> = string.lines().map(|x| x.into()).collect();

            let mut tokens = lex(string.as_str());
            let mut prog = Program::default();
            let result = document(&mut tokens, &mut prog, relaxed);

            match result {
                Ok(_) => {}
                Err(error) => {
                    let index = prog.line;
                    let line = match lines.get::<usize>(index as usize) {
                        Some(l) => l,
                        None => "error fetching context",
                    };
                    // TODO if we're not in --relaxed mode, suggest running --relaxed
                    return Err(error.with_section(move || {
                        format!("{} |    {}", index + 1, line.trim())
                            .header("Assembly context:".color(AnsiColors::Green))
                    }));
                }
            }
        }
        Commands::Version {} => {
            println!(
                "SoCUte v{VERSION}: Sega Saturn SCU DSP Assembler <https://github.com/mattyoung101/socute>"
            );
            println!("Copyright (c) 2025 Matt Young. Mozilla Public License v2.0.");
        }
    }

    Ok(())
}
