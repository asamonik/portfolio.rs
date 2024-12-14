use cfg_if::cfg_if;
use leptos::*;
use leptos::mount::mount_to_body;

pub mod api;
pub mod app;
pub mod components;
pub mod pages;

#[cfg(feature = "ssr")]
pub mod fileserve;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use app::*;

        #[wasm_bindgen]
        pub fn hydrate() {
            //_ = console_log::init_with_level(log::Level::Debug);
            //console_error_panic_hook::set_once();
            log::log!(log::Level::Debug, "hydrate mode - hydrating");

            mount_to_body(|| {
                view! { <App/> }
            });
        }
    }
    else if #[cfg(feature = "csr")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen(start)]
        pub fn main() {
            use app::*;
            use leptos::*;
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            log!("csr mode - mounting to body");

            mount_to_body(|cx| {
                view! { cx, <App /> }
            });
        }
    }
}
