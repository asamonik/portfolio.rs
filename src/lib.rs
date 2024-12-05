use cfg_if::cfg_if;

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
        use leptos::*;

        #[wasm_bindgen]
        pub fn hydrate() {
            //_ = console_log::init_with_level(log::Level::Debug);
            //console_error_panic_hook::set_once();

            log::log!(log::Level::Debug, "hydrate mode - hydrating");

            leptos::mount_to_body(|| {
                view! { <App/> }
            });
        }
    }
}
