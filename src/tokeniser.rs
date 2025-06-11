use std::iter::Peekable;

// SoCUte: An assembler for the Sega Saturn SCU DSP.
//
// Copyright (c) 2025 Matt Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
use logos::{Lexer, Logos, Skip};
use strum::AsRefStr;

/// Drops the last character from the string. Used to drop ':' from labels. Slow!
fn drop_last(string: String) -> String {
    let mut new = string.clone();
    new.pop();
    return new
}

#[derive(Logos, Debug, PartialEq, Eq, AsRefStr, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
#[logos(error = String)]
pub enum ScuDspToken {
    // Generic instrs
    #[regex("(?i)nop")]
    Nop,

    #[regex("(?i)mov")]
    Mov,

    // ALU control
    #[regex("(?i)and")]
    And,

    #[regex("(?i)or")]
    Or,

    #[regex("(?i)xor")]
    Xor,

    #[regex("(?i)add")]
    Add,

    #[regex("(?i)sub")]
    Sub,

    #[regex("(?i)ad2")]
    Ad2,

    #[regex("(?i)sr")]
    Sr,

    #[regex("(?i)rr")]
    Rr,

    #[regex("(?i)sl")]
    Sl,

    #[regex("(?i)rl")]
    Rl,

    #[regex("(?i)rl8")]
    Rl8,

    // X-Bus control
    #[regex("(?i)x", priority = 3)]
    X,

    #[regex("(?i)p", priority = 3)]
    P,

    #[regex("(?i)mul")]
    Mul,

    // Y-Bus control
    #[regex("(?i)y", priority = 3)]
    Y,

    #[regex(r#"(?i)clr[ \t\n\f]+a"#)]
    ClrA,

    #[regex("(?i)alu")]
    Alu,

    #[regex("(?i)a", priority = 3)]
    A,

    // Load Immediate
    #[regex("(?i)mvi")]
    Mvi,

    #[regex("(?i)z", priority = 3)]
    Z,

    #[regex("(?i)nz")]
    Nz,

    #[regex("(?i)s", priority = 3)]
    S,

    #[regex("(?i)ns")]
    Ns,

    #[regex("(?i)c", priority = 3)]
    C,

    #[regex("(?i)nc")]
    Nc,

    #[regex("(?i)t0")]
    T0,

    #[regex("(?i)nt0")]
    Nt0,

    #[regex("(?i)zs")]
    Zs,

    #[regex("(?i)nzs")]
    Nzs,

    // DMA
    #[regex("(?i)dma")]
    Dma,

    #[regex("(?i)d0")]
    D0,

    // Jump
    #[regex("(?i)jmp")]
    Jmp,

    // Generic tokens
    #[regex("[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Ident(String),

    #[regex("[#|\\$]?[0-9]+", |lex| lex.slice().to_owned())]
    Num(String),

    #[regex("[a-zA-Z][a-zA-Z0-9_]*:",  |lex| drop_last(lex.slice().to_owned()))]
    Label(String),

    #[regex(";[^\n]*", |_| Skip)]
    Comment,

    #[token(",")]
    Comma,
}

/// Lexes an asm document
pub fn lex(document: &'static str) -> Peekable<Lexer<'static, ScuDspToken>> {
    return ScuDspToken::lexer(document).peekable();
}

#[cfg(test)]
mod tests {
    use logos::Lexer;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_comment() {
        let mut lex = ScuDspToken::lexer("; comment");
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_mov_comment() {
        let mut lex = ScuDspToken::lexer("mov ; comment");
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Mov)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_mov_comment_case_sensitive() {
        let mut lex = ScuDspToken::lexer("MOV ; coMMeNT");
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Mov)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn test_label_or_ident() {
        let mut lex = ScuDspToken::lexer("x:");
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Label("x".into()))));

        let mut lex = ScuDspToken::lexer("xident");
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Ident("xident".into()))));
    }

    #[test]
    fn test_clr_a() {
        let mut lex = ScuDspToken::lexer("CLR A");
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::ClrA)));

        let mut lex = ScuDspToken::lexer("clr   a");
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::ClrA)));

        let mut lex = ScuDspToken::lexer("clra");
        assert_ne!(lex.next(), Some(Ok(ScuDspToken::ClrA)));
    }

    #[test]
    fn test_full_document() {
        let doc = r#"
            ; comment
            MOV $1, ident

            label:
                jmp nt0 ; inline comment
        "#;

        let mut lex = ScuDspToken::lexer(doc);
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Mov)));
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Num("$1".into()))));
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Comma)));
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Ident("ident".into()))));

        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Label("label".into()))));
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Jmp)));
        assert_eq!(lex.next(), Some(Ok(ScuDspToken::Nt0)));

        assert_eq!(lex.next(), None);
    }
}
