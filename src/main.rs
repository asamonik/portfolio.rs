pub mod api;
mod app;
pub mod components;
pub mod fileserve;
pub mod pages;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes};

        #[tokio::main(flavor = "current_thread")]
        async fn main() {
            use axum::Router;
            use app::*;
            use crate::fileserve;
            use leptos::prelude::*;

            let conf = get_configuration(None).unwrap();
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let routes = generate_route_list(App);

            let app = Router::new()
                .leptos_routes(&leptos_options, routes, App)
                .fallback(fileserve::file_and_error_handler)
                .with_state(leptos_options);

            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
                logging::log!("listening on http://{}", &addr);
                axum::serve(listener, app.into_make_service())
                .await
                .unwrap();
        }
    }
    else {
        pub fn main() {}
    }
}
