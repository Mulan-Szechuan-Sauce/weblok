use wasm_bindgen::prelude::*;

// Taken from https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

pub fn bevy_log(s: &str) {
    #[cfg(target_arch = "wasm32")]
    log(s);
    #[cfg(not(target_arch = "wasm32"))]
    println!("{}", s);
}
