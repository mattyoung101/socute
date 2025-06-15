// SoCUte: An assembler for the Sega Saturn SCU DSP.
//
// Copyright (c) 2025 Matt Young.
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use bit_ops::BitOps;
use log::info;

#[derive(Default, Clone, Debug)]
pub struct Program {
    /// Program code, vector of 32-bit words
    prog: Vec<u32>,
    /// Current position in prog vec
    pc: u32,
    /// Mapping between labels and PC
    labels: HashMap<String, u32>,
}

impl Program {
    pub fn emit(&mut self, word: u32) {
        self.prog.push(word);
        self.pc += 1;
    }

    pub fn emit_bit(&mut self, bit: u32) {
        self.emit(0_u32.set_bit(bit));
    }

    pub fn emit_bits(&mut self, bits: Vec<u32>) {
        let mut word = 0_u32;
        for bit in bits {
            word = word.set_bit(bit);
        }
        self.emit(word);
    }

    pub fn add_label(&mut self, label: String) {
        self.labels.insert(label, self.pc);
    }

    pub fn debug_dump(&self) {
        for (i, opcode) in self.prog.iter().enumerate() {
            info!("[{}] {:#034b} {:#010x}", i, opcode, opcode);
        }
    }
}
