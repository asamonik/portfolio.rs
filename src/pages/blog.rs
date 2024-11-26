use leptos::*;

#[component]
pub fn Blog() -> impl IntoView {
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
    <span>recent posts</span>
    <hr/>

    <table>


      <tr>
        <td>
         date
        </td>
        <td>
        title
        </td>
      </tr>

    </table>
    </div>

      </ErrorBoundary>
    }
}
