// SoCUte: An assembler for the Sega Saturn SCU DSP.
//
// Copyright (c) 2025 Matt Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use color_eyre::eyre::eyre;
use std::collections::HashMap;

use bit_ops::BitOps;
use log::{debug, info};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum InstrType {
    Alu,
    XBus,
    YBus,
    D1Bus,
    FlowControl,
}

#[derive(Default, Clone, Debug)]
pub struct Program {
    /// Program code, vector of 32-bit words
    prog: Vec<u32>,

    /// Current position in prog vec
    pc: u32,

    /// Mapping between labels and PC
    labels: HashMap<String, u32>,

    /// Current word being processed
    word: u32,

    /// True if emitting
    is_emitting: bool,

    /// Number of emitted instructions in the current bundle
    emitted: u32,

    /// Counts for each instruction type that was emitted
    instr_type_counts: HashMap<InstrType, u32>,

    /// Current line, starting at 0
    pub line: u32
}

impl Program {
    fn ensure_emitting(&mut self) {
        if !self.is_emitting {
            panic!("Internal error: Emitter should be emitting");
        }
    }

    fn ensure_not_emitting(&mut self) {
        if self.is_emitting {
            panic!("Internal error: Emitter should NOT be emitting");
        }
    }

    /// Starts emitting a new bundle
    pub fn begin(&mut self) {
        debug!("Begin new bundle");
        self.ensure_not_emitting();
        self.word = 0;
        self.emitted = 0;
        self.is_emitting = true;
        self.instr_type_counts.clear();
    }

    pub fn begin_if_not_begun(&mut self) {
        if !self.is_emitting {
            self.begin();
        }
    }

    /// Adds the instruction word to the current bundle
    pub fn emit(&mut self, word: u32) {
        self.ensure_emitting();
        self.word |= word;
        self.emitted += 1;
    }

    /// Adds a single bit to the current bundle
    pub fn emit_bit(&mut self, bit: u32) {
        self.ensure_emitting();
        self.word = self.word.set_bit(bit);
        self.emitted += 1;
    }

    /// Adds all the bits to the current bundle
    pub fn emit_bits(&mut self, bits: Vec<u32>) {
        self.ensure_emitting();
        for bit in bits {
            self.word = self.word.set_bit(bit);
        }
        self.emitted += 1;
    }

    /// Registers with the emitter that a particular type of instruction was just emitted
    pub fn register_emitted(&mut self, instr_type: InstrType) {
        if let Some(count) = self.instr_type_counts.get(&instr_type) {
            self.instr_type_counts.insert(instr_type, count + 1);
        } else {
            self.instr_type_counts.insert(instr_type, 1);
        }
    }

    /// Validates the current bundle
    fn validate_bundle(&self) -> color_eyre::Result<()> {
        // ensure only one flow control (JMP, BTM/LOOP, etc)
        if self
            .instr_type_counts
            .get(&InstrType::FlowControl)
            .is_some_and(|it| *it > 1)
        {
            return Err(eyre!(
                "Illegal program: Bundle contains more than one flow control instruction"
            ));
        }

        // ensure only one ALU instr per bundle
        if self
            .instr_type_counts
            .get(&InstrType::Alu)
            .is_some_and(|it| *it > 1)
        {
            return Err(eyre!(
                "Illegal program: Bundle contains more than one ALU instruction"
            ));
        }

        // So, here's where things get interesting. In the manual, pp. 91 (PDF page 107) it very
        // clear states that only 4 instructions can be issued in a bundle. However, real world
        // usage clearly uses up to 6 instructions.
        //
        // See John's very good video on the topic (which inspired this assembler):
        // https://www.youtube.com/watch?v=lxpp3KsA3CI
        //
        // Basically, Jon came to the conclusion (he says it a bit differently in the video, but
        // this is my understanding) that the manual is *wrong*, and X-Bus/Y-Bus instrs are one-hot
        // coded, and hence you can issue multiple X-Bus/Y-Bus instructions in a single bundle
        // without problems.
        //
        // So, for SoCUte, we allow 2 X-Bus and 2 Y-Bus instructions per bundle. D1-BUS TBA.

        if self
            .instr_type_counts
            .get(&InstrType::XBus)
            .is_some_and(|it| *it > 2)
        {
            return Err(eyre!(
                "Illegal program: Bundle contains more than 2 X-Bus instructions"
            ));
        }

        if self
            .instr_type_counts
            .get(&InstrType::YBus)
            .is_some_and(|it| *it > 2)
        {
            return Err(eyre!(
                "Illegal program: Bundle contains more than 2 Y-Bus instructions"
            ));
        }

        // finally, let's also check to make sure they're not issuing more than 6 instructions per
        // bundle
        if self.instr_type_counts.values().sum::<u32>() > 6 {
            return Err(eyre!(
                "Illegal program: More than 6 instructions issued in a single bundle"
            ));
        }

        Ok(())
    }

    /// Flushes and commits the current bundle
    pub fn flush(&mut self) -> color_eyre::Result<()> {
        debug!("Finalise bundle");

        // we only want to actually write an instruction if we emitted anything
        // this is to handle the case of blank programs full of newlines
        if self.emitted > 0 {
            // if we have instructions in the bundle, we better validate the bundle
            self.validate_bundle()?;

            self.prog.push(self.word);
            self.pc += 4; // sizeof(uint32)
        }
        debug!("Flushed {} instructions to bundle", self.emitted);

        self.is_emitting = false;
        self.word = 0;
        self.emitted = 0;
        self.instr_type_counts.clear();

        Ok(())
    }

    pub fn add_label(&mut self, label: String) {
        self.labels.insert(label, self.pc);
    }

    pub fn debug_dump(&self) {
        for (i, opcode) in self.prog.iter().enumerate() {
            info!("[{}] {:#034b} {:#010x}", i, opcode, opcode);
        }
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }
}
