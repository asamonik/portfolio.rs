use crate::api::posts::get_all_posts;
use leptos::error::ErrorBoundary;
use leptos::html::search;
use leptos::prelude::*;

#[component]
pub fn Blog() -> impl IntoView {
    let posts = Resource::new(
        move || (),
        move |_| async move { get_all_posts("".to_string()).await },
    );

    let (search_input, set_search_input) = signal("".to_string());

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
          <span>"recent posts " {search_input.get()}</span>
          /*
          <input
            type="text"
            value={search_input().clone()}
            on:change=move |ev| {
              println!("search_input: {:?}", ev.target());
                set_search_input.set("sdafasdf".to_string());
            }
            prop:value=search_input
          />
           */
          <hr/>

          <table>
            <thead>
              <tr>
                <th>"yyyy-mm-dd"</th>
                <th>"title"</th>
              </tr>
            </thead>
            <tbody>
              <Suspense fallback=move || view! {  }>
                {move || {
                    let search = search_input().to_lowercase();
                    posts.get().map(|posts_got| {
                        match posts_got {
                            Ok(postsvec) => {
                                {postsvec.into_iter()
                                    .filter(|post| post.title.to_lowercase().contains(&search))
                                    .map(|post| view! {
                                        <tr>
                                          <td>{post.date.format("%Y-%m-%d").to_string()}</td>
                                          <td>
                                            <a href={format!("/post/{}", post.filename)}>
                                              {post.title.clone()}
                                            </a>
                                          </td>
                                        </tr>
                                    }).collect_view()}.into_any()
                            },
                            // TODO: proper error handling
                            Err(error) => {
                              view! { <p>{error.to_string()}</p>}.into_any()
                            },
                        }
                    })
                }}
              </Suspense>
            </tbody>
          </table>
          <hr />
        </div>
      </ErrorBoundary>
    }
}
