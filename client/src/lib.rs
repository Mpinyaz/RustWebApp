use app::*;
use leptos::*;
use tracing::info;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    info!("tracing on frontend...");

    leptos::mount_to_body(move || {
        view! { <App/> }
    });
}
