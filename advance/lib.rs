//! This layer acts as a go between for the emulator itself and the browser.
//! Because of the additional abstraction layer, it should be relatively easy to
//! reuse the emulator module with another compatability layer for use outside
//! of WebAssembly. All of the actual rendering and sound generation is done in
//! JavaScript. Only the hardware itself is emulated inside of Rust.

// This should be removed when things are much closer to finalized
#![allow(dead_code, unused_imports, unused_variables)]

/// The core logic of the emulator is within this module.
pub mod emulator;
pub mod memory;

use emulator::Emulator;
use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
	static ref EMULATION: Mutex<Emulator> = Mutex::new(Emulator::new());
}

/// Starts the emulation of the provided ROM.
#[wasm_bindgen]
pub fn init_emulation(rom: &[u8]) {
	let mut emulation = EMULATION.lock().unwrap();

	emulation.load_rom(&rom);
	emulation.test();
}

/// Returns a pointer to the beginning of the IO memory section.
#[wasm_bindgen]
pub fn get_io_address() -> *mut u8 {
	let mut emulation = EMULATION.lock().unwrap();
	&mut emulation.memory.io[0] as *mut u8
}

/// Returns a pointer to the beginning of the palette memory section.
#[wasm_bindgen]
pub fn get_palette_address() -> *const u8 {
	let emulation = EMULATION.lock().unwrap();
	&emulation.memory.palette[0] as *const u8
}

/// Returns a pointer to the beginning of the VRAM memory section.
#[wasm_bindgen]
pub fn get_vram_address() -> *const u8 {
	let emulation = EMULATION.lock().unwrap();
	&emulation.memory.vram[0] as *const u8
}

/// Returns a pointer to the beginning of the object attribute memory section.
#[wasm_bindgen]
pub fn get_object_address() -> *const u8 {
	let emulation = EMULATION.lock().unwrap();
	&emulation.memory.object[0] as *const u8
}

/// Called from JavaScript when it is time to produce the next frame.
#[wasm_bindgen]
pub fn step_frames(frames: u32) {
	let mut emulation = EMULATION.lock().unwrap();
	for _ in 0..frames {
		emulation.step_frame();
	}
}

/// Step forward by one instruction
#[wasm_bindgen]
pub fn step_instruction() {
	let mut emulation = EMULATION.lock().unwrap();
	emulation.step_instruction();
}

/// Get the values of the current register bank
#[wasm_bindgen]
pub fn read_registers() -> Vec<u32> {
	use emulator::Reg::*;

	let emulation = EMULATION.lock().unwrap();
	vec![
		emulation.cpu.registers.get_value(r0),
		emulation.cpu.registers.get_value(r1),
		emulation.cpu.registers.get_value(r2),
		emulation.cpu.registers.get_value(r3),
		emulation.cpu.registers.get_value(r4),
		emulation.cpu.registers.get_value(r5),
		emulation.cpu.registers.get_value(r6),
		emulation.cpu.registers.get_value(r7),
		emulation.cpu.registers.get_value(r8),
		emulation.cpu.registers.get_value(r9),
		emulation.cpu.registers.get_value(r10),
		emulation.cpu.registers.get_value(r11),
		emulation.cpu.registers.get_value(r12),
		emulation.cpu.registers.get_value(r13),
		emulation.cpu.registers.get_value(r14),
		emulation.cpu.registers.get_value(r15),
	]
}

/// Get the status of the cpsr register.
#[wasm_bindgen]
pub fn read_cpsr() -> u32 {
	use emulator::Reg::cpsr;

	let emulation = EMULATION.lock().unwrap();
	emulation.cpu.registers.get_value(cpsr)
}

/// Allows us to inspect parts of memory the way that the emulator sees them.
// todo: Needs to be robustified for Thumb instructions.
#[wasm_bindgen]
pub fn read_next_instruction() -> u32 {
	let emulation = EMULATION.lock().unwrap();
	emulation.memory.read_word(emulation.cpu.registers.r15)
}
