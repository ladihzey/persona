use wasm_bindgen::prelude::*;

pub mod window {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window)]
        pub fn print();
    }
}
