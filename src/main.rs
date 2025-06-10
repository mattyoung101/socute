use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use env_logger::{Builder, Env};
use log::{error, info};
use pest_derive::Parser as PestDeriveParser;

#[derive(PestDeriveParser)]
#[grammar = "grammar.pest"]
pub struct ScuAsmParser;

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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pest::Parser;

    #[test]
    fn test_blank() -> color_eyre::Result<()> {
        ScuAsmParser::parse(Rule::document, "\n")?;
        Ok(())
    }

    #[test]
    fn test_double_newline() -> color_eyre::Result<()> {
        ScuAsmParser::parse(Rule::document, "add\n\n")?;
        Ok(())
    }

    #[test]
    fn test_comment_line() -> color_eyre::Result<()> {
        ScuAsmParser::parse(Rule::document, "; comment\n; comment 2\n")?;
        Ok(())
    }

    #[test]
    fn test_comment_inline() -> color_eyre::Result<()> {
        ScuAsmParser::parse(Rule::document, "; comment\nadd\nxor\nad2 ; comment\n")?;
        Ok(())
    }

    #[test]
    fn test_label() -> color_eyre::Result<()> {
        ScuAsmParser::parse(Rule::document, "label:\n    add\n")?;
        Ok(())
    }

    #[test]
    fn test_label_and_comment() -> color_eyre::Result<()> {
        ScuAsmParser::parse(Rule::document, "label: ; comment\n    add\n")?;
        Ok(())
    }

    #[test]
    fn test_comment_and_label() -> color_eyre::Result<()> {
        ScuAsmParser::parse(Rule::document, "; comment label:\n    add\n")?;
        Ok(())
    }

    #[test]
    fn test_alu() -> color_eyre::Result<()> {
        ScuAsmParser::parse(Rule::document, "add\nxor\nad2\n")?;
        Ok(())
    }

    #[test]
    fn test_arg_spacing() -> color_eyre::Result<()> {
        ScuAsmParser::parse(Rule::document, "jmp t0,dma_wait\n")?;
        Ok(())
    }
}
