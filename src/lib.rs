use wasm_bindgen::prelude::*;

mod conservation;

pub use conservation::*;

/// Initialize panic hook for better error messages in browser console
#[wasm_bindgen]
pub fn init_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Greet function for testing WASM loading
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Conservation law: γ + η = total_budget", name)
}
