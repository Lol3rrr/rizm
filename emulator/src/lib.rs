pub mod general;
pub mod system;
pub mod target;

mod traits;
pub use traits::{Display, Input};

mod memory;
pub use memory::Memory;

mod mock;
pub use mock::{display::MockDisplay, input::MockInput};

mod instructiontype;

mod cpu;
pub use cpu::CPU;

mod em;
pub use em::Emulator;

mod inputs;
pub use inputs::{Key, Modifier};

use sh::asm;

pub const CODE_MAPPING_OFFSET: u32 = 0x00300000;

#[cfg(feature = "wasm")]
mod wasm;

/// The Exceptions that could be thrown during
/// the Execution of Code as a result of wrong
/// behaviour of the Program
#[derive(Debug)]
pub enum Exception {
    UnknownInstruction,
    SlotIllegal,
}

pub enum DisplayBits {
    HighBits,
    LowBits,
}

/// Actually runs the Emulator
pub fn run<I, D>(emulator: Emulator<I, D>)
where
    I: Input,
    D: Display,
{
}
