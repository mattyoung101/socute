// SoCUte: An assembler for the Sega Saturn SCU DSP.
//
// Copyright (c) 2025 Matt Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.

// References:
// - https://en.wikipedia.org/wiki/Recursive_descent_parser#C_implementation
// - https://github.com/maciejhirsz/logos/issues/82

use bit_ops::BitOps;
use color_eyre::eyre::eyre;
use log::debug;
use logos::Lexer;
use std::{cell::RefCell, iter::Peekable, rc::Rc};

use crate::{emitter::Program, tokeniser::ScuDspToken};

type T = ScuDspToken;

/// All ALU tokens
const ALU_TOKENS: &'static [&'static T] = &[
    &T::Nop,
    &T::And,
    &T::Or,
    &T::Xor,
    &T::Add,
    &T::Sub,
    &T::Ad2,
    &T::Sr,
    &T::Rr,
    &T::Sl,
    &T::Rl,
    &T::Rl8,
];

/// All instruction tokens
const INSTR_TOKENS: &'static [&'static T] = &[
    &T::Nop,
    &T::And,
    &T::Or,
    &T::Xor,
    &T::Add,
    &T::Sub,
    &T::Ad2,
    &T::Sr,
    &T::Rr,
    &T::Sl,
    &T::Rl,
    &T::Rl8,
    &T::Mov,
    &T::Mvi,
    &T::Dma,
    &T::Jmp,
];

fn accept(tok: &ScuDspToken, lexer: &mut Peekable<Lexer<ScuDspToken>>) -> color_eyre::Result<bool> {
    if let Some(stream) = lexer.peek() {
        if stream.as_ref().is_ok_and(|x| tok == x) {
            let _ = lexer.next();
            return Ok(true);
        }
    }

    return Ok(false);
}

fn expect(tok: &ScuDspToken, lexer: &mut Peekable<Lexer<ScuDspToken>>) -> color_eyre::Result<bool> {
    if accept(&tok, lexer)? {
        return Ok(true);
    }
    return Err(eyre!("Expected {}", &tok.as_ref()));
}

/// Returns the token at the current position in the lexer
fn token(lexer: &mut Peekable<Lexer<ScuDspToken>>) -> color_eyre::Result<ScuDspToken> {
    if let Some(stream) = lexer.peek() {
        match stream {
            Ok(tok) => {
                return Ok(tok.clone());
            }
            Err(e) => {
                return Err(eyre!("Lexer error: {}", e));
            }
        }
    } else {
        return Err(eyre!("Unexpected end of input"));
    }
}

/// Converts token to string for debuugging
fn token_str(lexer: &mut Peekable<Lexer<ScuDspToken>>) -> color_eyre::Result<String> {
    let tok = token(lexer)?;

    match &tok {
        T::Label(lab) => Ok(format!("{} '{}'", tok.as_ref(), lab)),
        T::Ident(lab) => Ok(format!("{} '{}'", tok.as_ref(), lab)),
        T::Num(lab) => Ok(format!("{} '{}'", tok.as_ref(), lab)),
        _ => Ok(tok.as_ref().into()),
    }
}

// ALU control commands
fn alu(lexer: &mut Peekable<Lexer<ScuDspToken>>, prog: &mut Program) -> color_eyre::Result<()> {
    debug!("Parse ALU instr");
    if accept(&T::Nop, lexer)? {
        prog.emit(0);
    } else if accept(&T::And, lexer)? {
        prog.emit_bit(26);
    } else if accept(&T::Or, lexer)? {
        prog.emit_bit(27);
    } else if accept(&T::Xor, lexer)? {
        prog.emit_bits(vec![26, 27]);
    } else if accept(&T::Add, lexer)? {
        prog.emit_bit(28);
    } else if accept(&T::Sub, lexer)? {
        prog.emit_bits(vec![26, 28]);
    } else if accept(&T::Ad2, lexer)? {
        prog.emit_bits(vec![27, 28]);
    } else if accept(&T::Sr, lexer)? {
        prog.emit_bit(29);
    } else if accept(&T::Rr, lexer)? {
        prog.emit_bits(vec![26, 29]);
    } else if accept(&T::Sl, lexer)? {
        prog.emit_bits(vec![27, 29]);
    } else if accept(&T::Rl, lexer)? {
        prog.emit_bits(vec![26, 27, 29]);
    } else if accept(&T::Rl8, lexer)? {
        prog.emit_bits(vec![26, 27, 28, 29]);
    } else {
        return Err(eyre!(
            "Could not parse ALU command near {}",
            token_str(lexer)?
        ));
    }

    Ok(())
}

fn instr(lexer: &mut Peekable<Lexer<ScuDspToken>>, prog: &mut Program) -> color_eyre::Result<()> {
    let tok = token(lexer)?;
    debug!("Parse instr near {}", tok.as_ref());
    if ALU_TOKENS.contains(&&tok) {
        alu(lexer, prog)?;
    } else {
        return Err(eyre!(
            "Could not parse instruction near {}",
            token_str(lexer)?
        ));
    }

    Ok(())
}

fn document(lexer: &mut Peekable<Lexer<ScuDspToken>>, prog: &mut Program) -> color_eyre::Result<()> {
    while lexer.peek().is_some() {
        let tok = token(lexer)?;
        // first try match a define
        // then try a label
        // now look for instructions
        if INSTR_TOKENS.contains(&&tok) {
            instr(lexer, prog)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Lexer;

    use crate::tokeniser::lex;

    #[test]
    fn test_parse_alu() -> color_eyre::Result<()> {
        let document = r#"
            ; comment
            xor ; another one
        "#;
        let mut tokens = lex(document);
        let mut prog = Program::default();
        let _ = instr(&mut tokens, &mut prog)?;

        Ok(())
    }
}
