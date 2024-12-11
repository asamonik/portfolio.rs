use crate::api::posts::get_post;
use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn Post(id: String) -> impl IntoView {

    let post = Resource::new(
      move || (),
      move |_| {
          let id_clone = id.clone();
          async move {
              get_post(id_clone).await
          }
      },
  );

    view!{ 
        <div class="blog-post">

            <script src="/highlight/highlight.min.js"></script>
            <Stylesheet id="stylesheet-highlight" href="/highlight/gruvbox-dark-hard.min.css"/>
            <script>hljs.highlightAll();</script>

            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || {
                    post.get().map(|post_got| {
                        match post_got {
                            Ok(html_string) => {
                                view! { <div inner_html={html_string}></div> }.into_any()
                            },
                            // TODO: proper error handling
                            Err(error) => {
                                view! { <p>{error.to_string()}</p> }.into_any()
                            },
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
