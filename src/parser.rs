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
use clap::Error;
use color_eyre::eyre::eyre;
use log::debug;
use logos::Lexer;
use std::{cell::RefCell, i8, iter::Peekable, rc::Rc};

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

/// All loop tokens
const LOOP_TOKENS: &'static [&'static T] = &[&T::Btm, &T::Lps];

/// All end tokens
const END_TOKENS: &'static [&'static T] = &[&T::End, &T::Endi];

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
    &T::Clr,
    &T::Btm,
    &T::Lps,
    &T::End,
    &T::Endi,
];

#[derive(PartialEq, Eq)]
enum MovDestination {
    X,
    P,
    Y,
    A,
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

fn num(lexer: &mut Peekable<Lexer<ScuDspToken>>, prog: &mut Program) -> color_eyre::Result<u32> {
    if !token(lexer)?.is_number() {
        return Err(eyre!("Syntax error: Expected number"));
    }

    match token_pop(lexer)? {
        T::Num(mut num_str) => {
            if num_str.starts_with('$') {
                // hex
                num_str.remove(0);
                return Ok(u32::from_str_radix(num_str.as_str(), 16)?);
            } else if num_str.starts_with('#') {
                // decimal?
                num_str.remove(0);
                return Ok(num_str.parse()?);
            } else if num_str.starts_with('%') {
                // binary
                num_str.remove(0);
                return Ok(u32::from_str_radix(num_str.as_str(), 2)?);
            } else {
                // also decimal
                return Ok(num_str.parse()?);
            }
        }
        _ => Err(eyre!("Syntax error: Expected number")),
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

fn emit_mov(
    address: &ScuDspToken,
    mov: MovDestination,
    prog: &mut Program,
) -> color_eyre::Result<()> {
    let opcode: u32 = if mov == MovDestination::P {
        // MOV [s], P
        0_u32.set_bit(23).set_bit(24)
    } else if mov == MovDestination::X {
        // MOV [s], X
        0_u32.set_bit(25)
    } else if mov == MovDestination::Y {
        // MOV [s], Y
        0_u32.set_bit(19)
    } else {
        panic!("Internal error: Unhandled branch in emit_mov calc opcode");
    };

    // now calculate the offset where we set bits to encode the destination address
    // for example, SCU user manual pp. 109 (pdf pp. 125), for MOV [s], P; we start setting bits at
    // bit 20
    let offset: u32 = if mov == MovDestination::P || mov == MovDestination::X {
        20
    } else if mov == MovDestination::Y || mov == MovDestination::A {
        14
    } else {
        panic!("Internal error: Unreachable branch in emit_mov calc offset");
    };

    match address {
        ScuDspToken::M0 => {
            // DATA RAM0
            prog.emit(opcode); // 000
        }
        ScuDspToken::M1 => {
            // DATA RAM1
            prog.emit(opcode.set_bit(offset)); // 001
        }
        ScuDspToken::M2 => {
            // DATA RAM2
            prog.emit(opcode.set_bit(offset + 1)); // 010
        }
        ScuDspToken::M3 => {
            // DATA RAM3
            prog.emit(opcode.set_bit(offset).set_bit(offset + 1)); // 011
        }
        ScuDspToken::Mc0 => {
            // DATA RAM0, CT0++
            prog.emit(opcode.set_bit(offset + 2)); // 100
        }
        ScuDspToken::Mc1 => {
            // DATA RAM1, CT1++
            prog.emit(opcode.set_bit(offset + 2).set_bit(offset)); // 101
        }
        ScuDspToken::Mc2 => {
            // DATA RAM2, CT2++
            prog.emit(opcode.set_bit(offset + 2).set_bit(offset + 1)); // 110
        }
        ScuDspToken::Mc3 => {
            // DATA RAM3, CT3++
            prog.emit(
                opcode
                    .set_bit(offset + 2)
                    .set_bit(offset + 1)
                    .set_bit(offset),
            ); // 111
        }
        _ => {
            return Err(eyre!(
                "Syntax error: Illegal X-Bus MOV destination address, got: {}",
                address.as_ref()
            ));
        }
    }

    Ok(())
}

fn emit_mov_simm(
    lexer: &mut Peekable<Lexer<ScuDspToken>>,
    prog: &mut Program,
) -> color_eyre::Result<()> {
    let value = num(lexer, prog)?;

    if value >= i8::MAX as u32 {
        return Err(eyre!(
            "Error: '{value}' will not fit in signed 8-bit immediate value (in MOV SImm, [d])"
        ));
    }

    // let word = 0_u32.set_bits_exact(value as i8, 8, 0);

    Ok(())
}

// MOV instructions
fn mov(lexer: &mut Peekable<Lexer<ScuDspToken>>, prog: &mut Program) -> color_eyre::Result<()> {
    debug!("Parse bus control instr");
    if accept(&T::Mov, lexer)? {
        // MOV MUL, P
        if accept(&T::Mul, lexer)? {
            expect(&T::Comma, lexer)?;
            expect(&T::P, lexer)?;
            prog.emit_bit(24);
            return Ok(());
        }

        // MOV ALU, A
        if accept(&T::Alu, lexer)? {
            expect(&T::Comma, lexer)?;
            expect(&T::A, lexer)?;
            prog.emit_bit(18);
            return Ok(());
        }

        // Otherwise, we expect a memory address
        // take the token for now, we'll check it again later in emit_xbus_mov
        let tok = token_pop(lexer)?;
        expect(&T::Comma, lexer)?;

        // MOV [s], X
        if accept(&T::X, lexer)? {
            emit_mov(&tok, MovDestination::X, prog)?;
            return Ok(());
        }

        // MOV [s], P
        if accept(&T::P, lexer)? {
            emit_mov(&tok, MovDestination::P, prog)?;
            return Ok(());
        }

        // MOV [s], Y
        if accept(&T::Y, lexer)? {
            emit_mov(&tok, MovDestination::P, prog)?;
            return Ok(());
        }

        // MOV SImm, [d]
        if token(lexer)?.is_number() {
            emit_mov_simm(lexer, prog)?;
            return Ok(());
        }

        // otherwise, illegal
        return Err(eyre!(
            "Syntax error: Illegal source for MOV instruction, got: {}",
            token_str(lexer)?
        ));
    } else {
        return Err(eyre!(
            "Syntax error: Could not parse MOV instruction near {}",
            token_str(lexer)?
        ));
    }
}

fn clr(lexer: &mut Peekable<Lexer<ScuDspToken>>, prog: &mut Program) -> color_eyre::Result<()> {
    debug!("Parse CLR A");
    expect(&T::Clr, lexer)?;
    expect(&T::A, lexer)?;
    prog.emit_bit(17);
    Ok(())
}

fn loop_cmd(
    lexer: &mut Peekable<Lexer<ScuDspToken>>,
    prog: &mut Program,
) -> color_eyre::Result<()> {
    debug!("Parse loop");

    if accept(&T::Btm, lexer)? {
        prog.emit_bits(vec![31, 30, 29]);
    } else if accept(&T::Lps, lexer)? {
        prog.emit_bits(vec![31, 30, 29, 27]);
    } else {
        return Err(eyre!(
            "Syntax error: Could not parse loop (BTM/LPS) instruction near {}",
            token_str(lexer)?
        ));
    }

    // manual pp. 91 (pdf pp. 107) seems to imply that END and LOOP type instructions are
    // completely separate to the normal bundle. The normal bundle can contain ALU, {X,Y,D1}-bus
    // control, but it seems that END and LOOP must be on their own. Hence, we expect a newline to
    // be issued.
    if !accept(&T::Newline, lexer)? {
        return Err(eyre!(
            "Syntax error: Expected a newline after LPS/BTM. \
            These instructions must be issued on their own, not as part of a bundle."
        ));
    }

    Ok(())
}

fn end(lexer: &mut Peekable<Lexer<ScuDspToken>>, prog: &mut Program) -> color_eyre::Result<()> {
    debug!("Parse end");

    if accept(&T::End, lexer)? {
        prog.emit_bits(vec![31, 30, 29, 28]);
    } else if accept(&T::Endi, lexer)? {
        prog.emit_bits(vec![31, 30, 29, 28, 27]);
    } else {
        return Err(eyre!(
            "Syntax error: Could not parse END instruction near {}",
            token_str(lexer)?
        ));
    }

    // manual pp. 91 (pdf pp. 107) seems to imply that END and LOOP type instructions are
    // completely separate to the normal bundle. The normal bundle can contain ALU, {X,Y,D1}-bus
    // control, but it seems that END and LOOP must be on their own. Hence, we expect a newline to
    // be issued.
    if !accept(&T::Newline, lexer)? {
        return Err(eyre!(
            "Syntax error: Expected a newline after END/ENDI. \
            These instructions must be issued on their own, not as part of a bundle."
        ));
    }

    Ok(())
}

fn instr(lexer: &mut Peekable<Lexer<ScuDspToken>>, prog: &mut Program) -> color_eyre::Result<()> {
    let tok = token(lexer)?;
    debug!("Parse instr near {}", tok.as_ref());
    if ALU_TOKENS.contains(&&tok) {
        // NOTE: This will also handle NOP
        alu(lexer, prog)?;
    } else if tok == T::Mov {
        mov(lexer, prog)?;
    } else if tok == T::Clr {
        clr(lexer, prog)?;
    } else if LOOP_TOKENS.contains(&&tok) {
        loop_cmd(lexer, prog)?;
    } else if END_TOKENS.contains(&&tok) {
        end(lexer, prog)?;
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

        let doc = r#"
            MOV MC3,X
            MOV M3,P
            CLR A
        "#;
        let mut tokens = lex(doc);
        let mut prog = Program::default();
        let _ = document(&mut tokens, &mut prog)?;

        Ok(())
    }

    #[test]
    fn test_with_end() -> color_eyre::Result<()> {
        let _ = env_logger::try_init();

        let doc = r#"
            MOV MC3,X       MOV M3,P    MOV M0, Y
            CLR A
            ENDI

            CLR A
        "#;
        let mut tokens = lex(doc);
        let mut prog = Program::default();
        let _ = document(&mut tokens, &mut prog)?;
        prog.debug_dump();

        Ok(())
    }

    #[test]
    fn test_end_must_be_on_its_own() -> color_eyre::Result<()> {
        let _ = env_logger::try_init();

        let doc = r#"
            CLR A
            ENDI    CLR A
        "#;
        let mut tokens = lex(doc);
        let mut prog = Program::default();
        let res = document(&mut tokens, &mut prog);
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn test_btm_must_be_on_its_own() -> color_eyre::Result<()> {
        let _ = env_logger::try_init();

        let doc = r#"
            CLR A
            BTM     CLR A
        "#;
        let mut tokens = lex(doc);
        let mut prog = Program::default();
        let res = document(&mut tokens, &mut prog);
        assert!(res.is_err());

        Ok(())
    }
}
