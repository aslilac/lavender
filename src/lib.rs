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
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

lazy_static! {
    static ref GBA: Mutex<Emulator> = Mutex::new(Emulator::new());
}

#[wasm_bindgen]
pub fn start_gba(rom: &[u8]) -> Result<(), JsValue> {
    let mut gba = GBA.lock().unwrap();

    gba.load_rom(&rom);
    gba.test();

    Ok(())
}

/// Used from JavaScript to get a pointer to the IO memory.
#[wasm_bindgen]
pub fn get_io_address() -> *mut u8 {
    let mut gba = GBA.lock().unwrap();
    &mut gba.memory.io[0] as *mut u8
}

/// Used from JavaScript to get a pointer to the VRAM memory.
#[wasm_bindgen]
pub fn get_vram_address() -> *const u8 {
    let gba = GBA.lock().unwrap();
    &gba.memory.vram[0] as *const u8
}

/// Used from JavaScript to step the emulator forward the given number of frames.
#[wasm_bindgen]
pub fn step_frames(frames: u32) {
    let mut gba = GBA.lock().unwrap();
    for _ in 0..frames {
        gba.step_frame();
    }
}
