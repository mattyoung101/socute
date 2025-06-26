# SoCUte
_SoCUte_ is an assembler for the Sega Saturn's SCU (System Control Unit) DSP chip, written in Rust.

The SCU-DSP is poorly documented, [notoriously hard to program](https://www.youtube.com/watch?v=n8plen8cLro)
chip with all sorts of quirks, running at 14.3 MHz. However, it does have some horsepower to it (at least, for
its day).

The goal of SoCUte is to write a more modern and portable assembler for the SCU-DSP for the purposes of
homebrew and experimenting, whilst maintaining 100% compatibility with original source files. Eventually, I'm
hoping to turn this into a relatively advanced macro assembler that's better than Sega's official tool.

I'm hoping that one day myself or others will be able to use this tool to unlock more power from the Saturn.

## Terminology
In user-facing error messages, I refer to a single line of instructions, as visible below, as a _bundle_:

```asm
ad2    mov mc1,x   mov mul,p   mov mc0,y   clr a   mov a11,mc2
; the above is one single bundle
```

The term "bundle" is commonly used in Very-Long-Instruction-Word (VLIW) CPUs, which the SCU-DSP isn't _really_
since the instruction word size is 32-bits. However, the ability to issue multiple instructions in a single
line and the very particular and poorly documented requirements these have means that I've decided to use a
bit of VLIW terminology and call these things bundles.

If your program produces error messages like this:

```
Illegal program: Bundle contains more than one flow control instruction
```

It means that you have attempted to pack more than the valid number of a particular instruction type into a
single line. In this case, you would have tried to pack more than one flow control instruction (e.g. issuing
two `END` instructions, or `JMP somewhere   END`).

## Compatibility
SoCUte removes a number of limitations from Sega's original assembler (`dspasm`):
- Lines may be longer than 255 characters
- Identifiers may be longer than 32 characters
- Nested `IFDEF` directives may be nested more than 16 levels deep

SoCUte will compile all valid programs written for the original assembler `dspasm`, but programs written
specifically for SoCUte probably won't compile on the original `dspasm`.

In the future I will probably add a `--strict` flag that will raise warnings when an incompatibility of this
sort is detected.

## Information sources
- SCU User's Manual, Third edition (Sega Doc. # ST-97-R5-072694), pp. 75-173
- SCU DSP Assembler User's Manual (Sega Doc. # ST-240-A-042795)
- SCU DSP Assembler User's Manual Addendum (Sega Doc. # ST-240-A-SP1-052295)

The above are technically Sega confidential, but easily obtainable e.g.
[here](https://segaretro.org/Saturn_official_documentation), [here](https://antime.kapsi.fi/sega/docs.html)

## Licence
Copyright (c) 2025 Matt Young.

SoCUte is licenced under the Mozilla Public License v2.0.
