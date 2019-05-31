mod utils;

use {
    wasm_bindgen::prelude::*,
    stdweb::{
        Reference,
        web::{
            html_element::CanvasElement,
            CanvasRenderingContext2d,
        },
        unstable::{
            TryInto,
        }
    },
    crate::{
        utils::set_panic_hook,
    },
};

#[wasm_bindgen]
pub fn play_game(canvas: Reference) {
    // enable console.error panic messages for debugging
    set_panic_hook();

    let canvas: CanvasElement = canvas.try_into().unwrap_or_else(|err| {
        panic!("passed a non-canvas element. {}", err);
    });

    let rendering_context: CanvasRenderingContext2d = canvas.get_context().unwrap_or_else(|err| {
        panic!("could not get 2d rendering context from canvas element: {}", err);
    });

    rendering_context.stroke_text("Hello, World!", 0f64, 0f64, None);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
