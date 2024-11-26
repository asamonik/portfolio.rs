use leptos::*;
use crate::components::navbar::Navbar;

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

        <div class="container">
        <Navbar/>

          hi, 
          <p/>
          i am andreas, a software engineer from austria. 
          <p/>
          i am studying cs in graz. i like low level programming, infrastructure and security
          <p/>

          you can check out my <a href="/blog"># blog</a>, <a href="/projects"># projects</a> and <a href="/work"># work experience</a>

          <p/>
          and i use nixos, btw
          <p/>
          maybe also mail me: <span>mail at asamonik dot at</span>

          <p/>
          gpg key: 0x1234


          <footer>
             <a href="https://leptos.dev/"># source</a>
          </footer>
        </div>

      </ErrorBoundary>
    }
}
