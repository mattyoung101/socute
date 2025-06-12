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

/// All X-Bus tokens
const XBUS_TOKENS: &'static [&'static T] = &[&T::Nop, &T::Mov];

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

#[derive(PartialEq, Eq)]
enum XBusMov {
    X,
    P,
}

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
    return Err(eyre!(
        "Syntax error: Expected {} but got {}",
        &tok.as_ref(),
        token_str(lexer)?
    ));
}

/// Returns, but does not remove, the token at the current position in the lexer
fn token(lexer: &mut Peekable<Lexer<ScuDspToken>>) -> color_eyre::Result<ScuDspToken> {
    if let Some(stream) = lexer.peek() {
        match stream {
            Ok(tok) => {
                return Ok(tok.clone());
            }
            Err(_) => {
                return Err(eyre!("Lexer error"));
            }
        }
    } else {
        return Err(eyre!("Error: Unexpected end of input"));
    }
}

/// Returns, **and removes**, the token at the current position in the lexer
fn token_pop(lexer: &mut Peekable<Lexer<ScuDspToken>>) -> color_eyre::Result<ScuDspToken> {
    if let Some(stream) = lexer.next() {
        match stream {
            Ok(tok) => {
                return Ok(tok.clone());
            }
            Err(_) => {
                return Err(eyre!("Lexer error"));
            }
        }
    } else {
        return Err(eyre!("Error: Unexpected end of input"));
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
            "Syntax error: Could not parse ALU command near {}",
            token_str(lexer)?
        ));
    }

    Ok(())
}

fn emit_xbus_mov(address: &ScuDspToken, mov_type: XBusMov, prog: &mut Program) -> color_eyre::Result<()> {
    let opcode: u32 = if mov_type == XBusMov::P {
        // MOV [s], P
        0_u32.set_bit(23).set_bit(24)
    } else {
        // MOV [s], X
        0_u32.set_bit(25)
    };

    match address {
        ScuDspToken::M0 => {
            // DATA RAM0
            prog.emit(opcode); // 000
        },
        ScuDspToken::M1 => {
            // DATA RAM1
            prog.emit(opcode.set_bit(20)); // 001
        },
        ScuDspToken::M2 => {
            // DATA RAM2
            prog.emit(opcode.set_bit(21)); // 010
        },
        ScuDspToken::M3 => {
            // DATA RAM3
            prog.emit(opcode.set_bit(20).set_bit(21)); // 011
        },
        ScuDspToken::Mc0 => {
            // DATA RAM0, CT0++
            prog.emit(opcode.set_bit(22)); // 100
        },
        ScuDspToken::Mc1 => {
            // DATA RAM1, CT1++
            prog.emit(opcode.set_bit(22).set_bit(20)); // 101
        },
        ScuDspToken::Mc2 => {
            // DATA RAM2, CT2++
            prog.emit(opcode.set_bit(22).set_bit(21)); // 110
        },
        ScuDspToken::Mc3 => {
            // DATA RAM3, CT3++
            prog.emit(opcode.set_bit(22).set_bit(21).set_bit(20)); // 111
        },
        _ => {
            return Err(eyre!(
                "Syntax error: Illegal X-Bus MOV destination address, got: {}",
                address.as_ref()
            ));
        }
    }

    Ok(())
}

// X-Bus control commands
fn xbus(lexer: &mut Peekable<Lexer<ScuDspToken>>, prog: &mut Program) -> color_eyre::Result<()> {
    debug!("Parse X-Bus instr");
    if accept(&T::Nop, lexer)? {
        prog.emit(0);
    } else if accept(&T::Mov, lexer)? {
        // MOV MUL, P
        if accept(&T::Mul, lexer)? {
            expect(&T::Comma, lexer)?;
            expect(&T::P, lexer)?;
            prog.emit_bit(24);
            return Ok(())
        }

        // Otherwise, we expect a memory address
        // take the token for now, we'll check it again later in emit_xbus_mov
        let tok = token_pop(lexer)?;
        expect(&T::Comma, lexer)?;

        // MOV [s], X
        if accept(&T::X, lexer)? {
            emit_xbus_mov(&tok, XBusMov::X, prog)?;
            return Ok(());
        }
        // MOV [s], P
        if accept(&T::P, lexer)? {
            emit_xbus_mov(&tok, XBusMov::P, prog)?;
            return Ok(());
        }

        // otherwise, illegal
        return Err(eyre!(
            "Syntax error: Illegal destination for X-Bus MOV instruction, expected X or P but got: {}",
            token_str(lexer)?
        ));
    } else {
        return Err(eyre!(
            "Syntax error: Could not parse X-Bus control command near {}",
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
    } else if XBUS_TOKENS.contains(&&tok) {
        xbus(lexer, prog)?;
    } else {
        return Err(eyre!(
            "Syntax error: Could not parse instruction near {}",
            token_str(lexer)?
        ));
    }

    Ok(())
}

pub fn document(
    lexer: &mut Peekable<Lexer<ScuDspToken>>,
    prog: &mut Program,
) -> color_eyre::Result<()> {
    while lexer.peek().is_some() {
        let tok = token(lexer)?;
        debug!("document looking at {}", tok.as_ref());

        // skip newlines
        if tok == T::Newline {
            lexer.next();
            continue;
        }

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
    use log::info;
    use logos::Lexer;

    use crate::tokeniser::lex;

    #[test]
    fn test_parse_alu() -> color_eyre::Result<()> {
        let document = r#"xor ; another one"#;
        let mut tokens = lex(document);
        let mut prog = Program::default();
        let _ = instr(&mut tokens, &mut prog)?;

        Ok(())
    }

    #[test]
    fn test_mov_mul_p() -> color_eyre::Result<()> {
        let document = r#"mov mul, p ;;; another other one"#;
        let mut tokens = lex(document);
        let mut prog = Program::default();
        let _ = instr(&mut tokens, &mut prog)?;

        Ok(())
    }

    #[test]
    fn test_mov_s_p() -> color_eyre::Result<()> {
        let _ = env_logger::try_init();

        let doc = "MOV MC3,X\nMOV M3,P";
        let mut tokens = lex(doc);
        let mut prog = Program::default();
        let _ = document(&mut tokens, &mut prog)?;
        prog.debug_dump();

        Ok(())
    }
}
