# SoCUte
_SoCUte_ is an assembler for the Sega Saturn's SCU (System Control Unit) DSP chip, written in Rust.

The SCU-DSP is poorly documented, [notoriously hard to program](https://www.youtube.com/watch?v=n8plen8cLro)
chip with all sorts of quirks, running at 14.3 MHz. However, it does have some horsepower to it (at least, for
its day).

The goal of SoCUte is to write a more modern and portable assembler for the SCU-DSP for the purposes of
homebrew and experimenting, whilst maintaining 100% compatibility with original source files. Eventually, I'm
hoping to turn this into a relatively advanced macro assembler that's better than Sega's official tool.

## Differences to Sega's official assembler (dspasm)
SoCUte removes a number of limitations from the original assembler:
- Lines may be longer than 255 characters
- Identifiers may be longer than 32 characters
- Nested `IFDEF` directives may be nested more than 16 levels deep

In a future version, SoCUte will implement a `--strict` flag that will emulate the exact behaviour of the
original assembler.

## Information sources
- SCU User's Manual, Third edition (Sega Doc. # ST-97-R5-072694), pp. 75-173
- SCU DSP Assembler User's Manual (Sega Doc. # ST-240-A-042795)
- SCU DSP Assembler User's Manual Addendum (Sega Doc. # ST-240-A-SP1-052295)

The above are technically Sega confidential, but easily obtainable e.g.
[here](https://segaretro.org/Saturn_official_documentation), [here](https://antime.kapsi.fi/sega/docs.html)

## Licence
Copyright (c) 2025 Matt Young.

SoCUte is licenced under the Mozilla Public License v2.0.
