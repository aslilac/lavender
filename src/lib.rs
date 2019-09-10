//! This layer acts as a go between for the emulator itself and the browser.
//! Because of the additional abstraction layer, it should be relatively easy to
//! reuse the emulator module with another compatability layer for use outside
//! of WebAssembly.

pub mod audio;
pub mod emulator;
pub mod gl;

use emulator::Emulator;
use lazy_static::lazy_static;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn enable_drawing(io: *mut u8, vram: *const u8);
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

lazy_static! {
    static ref GBA: Mutex<Emulator> = Mutex::new(Emulator::new());
}

#[wasm_bindgen]
pub fn start_gba(rom: &[u8]) {
    let mut gba = GBA.lock().unwrap();

    gba.load_rom(&rom);
    gba.test();

    enable_drawing(
        &mut gba.memory.io[0] as *mut u8,
        &gba.memory.vram[0] as *const u8,
    );
    log!("Drawing enabled");
}

#[wasm_bindgen]
pub fn step_frame() {
    let mut gba = GBA.lock().unwrap();
    gba.step_frame();
}
