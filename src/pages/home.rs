use crate::components::navbar::Navbar;
use leptos::prelude::*;
use leptos::error::ErrorBoundary;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <ErrorBoundary fallback=|errors| {
          view! {
            <h1>"Uh oh! Something went wrong!"</h1>

            <p>"Errors: "</p>
            <ul>
              {move || {
                  errors
                      .get()
                      .into_iter()
                      .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                      .collect_view()
              }}

            </ul>
          }
      }>

        <div class="main">
          hi,
          <p />
          i am andreas
          <p />
          i am studying cs in graz. i like low level programming, infrastructure and security
          <p />

          /*
          you can check out my
          <a href="/blog"># blog</a>
          */

          <p />
          and i use nixos, btw
          <p />
          maybe also mail me: mail 𝒂𝒕 asamonik ꓒσ𝗍 at
          <p />
          <p />
          gpg fingerprint: 5368 486D 270C 6698 B868  C881 5208 A1A6 76BE 91A0
          <p />

          <footer>
            <a href="https://github.com/asamonik/portfolio.rs/"># source</a>
          </footer>
        </div>

      </ErrorBoundary>
    }
}
