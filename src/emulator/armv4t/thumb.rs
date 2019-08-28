#[macro_use]
use crate::{
  emulator::Emulator,
  log
};

pub fn process_instruction(_emulator: &mut Emulator, instruction: u16) {
  log!("Received thumb instruction {:x}", instruction); 
}

// pub fn add() {
// }
