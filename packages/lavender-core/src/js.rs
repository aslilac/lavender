use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// When a macro is exported, it is always exported from the crate, and not the mod,
// i.e. you'll need `use crate::printjs` and not `use crate::js::printjs` like you would expect.
#[macro_export]
macro_rules! printjs {
    ($($t:tt)*) => (crate::js::log(&format_args!($($t)*).to_string()))
}
