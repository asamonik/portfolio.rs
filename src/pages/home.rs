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
          /*
          maybe also mail me:
          <span class="gray">mail at asamonik dot at</span>

          <p />
          gpg key: 0x1234
           */

          <footer>
            <a href="https://github.com/asamonik/portfolio.rs/"># source</a>
          </footer>
        </div>

      </ErrorBoundary>
    }
}
