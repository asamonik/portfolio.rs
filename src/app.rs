use leptos::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::hooks::*;
use leptos_router::*;
use leptos::prelude::*;

use crate::components::navbar::Navbar;
use crate::pages::blog::Blog;
use crate::pages::home::Home;
use crate::pages::post::Post;
use crate::pages::projects::Projects;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
      <head>
      <Html attr:data-theme="light" />

      <Title text="asamonik.at" />

      <Meta charset="UTF-8" />
      <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
      </head>

    <Stylesheet id="leptos" href="/styles.css"/>

    <Navbar/>

      <Router>
      <Routes fallback=|| ()>
                <Route path=path!("/") view=move || view!{
                  <Home/>
                } />
                <Route path=path!("/blog") view=move || view!{
                  <Blog/>
                } />
                <Route path=path!("/projects") view=move || view!{
                  <Projects/>
                } />

                // post route
                <ParentRoute path=path!("/post/:id") view=move || {
                    let params = use_params_map();
                    let id = params.get().get("id");
                    view!{<Post id=id.expect("ERROR").to_string()></Post>}
                }>
                    <Route path=path!("/") view=move || {
                          view!{<div><Home/></div>}
                  } />
                </ParentRoute>
            </Routes>
            /*
            
        <Routes>
          <Route path="/projects" view=Projects />
          <Route path="/blog" view=Blog />
        </Routes>
             */
      </Router>
    }
}
