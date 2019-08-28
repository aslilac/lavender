mod emulator;

use emulator::Emulator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn enable_drawing(io: *const u8, vram: *const u8);
}

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn start_gba(rom: &[u8]) {
    let mut gba = Emulator::start(&rom);

    gba.test();

    enable_drawing(
        &mut gba.memory.io[0] as *mut u8,
        &gba.memory.vram[0] as *const u8
    );
}
