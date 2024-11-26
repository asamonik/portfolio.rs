use leptos::*;
use crate::components::github_projects::GithubProjects;

#[component]
pub fn Projects() -> impl IntoView {
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

          <GithubProjects/>
        </div>

      </ErrorBoundary>
    }
}
