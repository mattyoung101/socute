// SoCUte: An assembler for the Sega Saturn SCU DSP.
//
// Copyright (c) 2025 Matt Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.

// References:
// - https://en.wikipedia.org/wiki/Recursive_descent_parser#C_implementation
// - https://github.com/maciejhirsz/logos/issues/82

use std::iter::Peekable;

use color_eyre::eyre::eyre;
use logos::Lexer;

use crate::tokeniser::ScuDspToken;

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
        _ => Ok(tok.as_ref().into())
    }
}

// ALU control commands
fn alu(lexer: &mut Peekable<Lexer<ScuDspToken>>) -> color_eyre::Result<()> {
    if accept(&T::Nop, lexer)? {
        // emit
    } else if accept(&T::And, lexer)? {
        // emit
    } else if accept(&T::Or, lexer)? {
        // emit
    } else if accept(&T::Xor, lexer)? {
        // emit
    } else if accept(&T::Add, lexer)? {
        // emit
    } else if accept(&T::Sub, lexer)? {
        // emit
    } else if accept(&T::Ad2, lexer)? {
        // emit
    } else if accept(&T::Sr, lexer)? {
        // emit
    } else if accept(&T::Rr, lexer)? {
        // emit
    } else if accept(&T::Sl, lexer)? {
        // emit
    } else if accept(&T::Rl, lexer)? {
        // emit
    } else if accept(&T::Rl8, lexer)? {
        // emit
    } else {
        return Err(eyre!(
            "Could not parse ALU command at {}",
            token_str(lexer)?
        ));
    }

    Ok(())
}

fn instr(lexer: &mut Peekable<Lexer<ScuDspToken>>) -> color_eyre::Result<()> {
    let tok= token(lexer)?;
    if ALU_TOKENS.contains(&&tok) {
        alu(lexer)?;
    } else {
        return Err(eyre!(
            "Could not parse instruction at {}",
            token_str(lexer)?
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use logos::Lexer;
    use super::*;

    use crate::tokeniser::lex;

    #[test]
    fn test_parse_alu() -> color_eyre::Result<()> {
        let document = r#"
            ; comment
            xor ; another one
        "#;
        let mut tokens = lex(document);
        let _ = instr(&mut tokens)?;

        Ok(())
    }
}
