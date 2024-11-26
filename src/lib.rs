use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;

use crate::pages::home::Home;
use crate::pages::projects::Projects;
use crate::pages::blog::Blog;
use crate::pages::work::Work;
use crate::pages::not_found::NotFound;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
      <Html lang="en" dir="ltr" attr:data-theme="light"/>

      <Title text="asamonik.at"/>

      <Meta charset="UTF-8"/>
      <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

      <Router>
        <Routes>
          <Route path="/" view=Home/>
          <Route path="/work" view=Work/>
          <Route path="/projects" view=Projects/>
          <Route path="/blog" view=Blog/>
          <Route path="/*" view=NotFound/>
        </Routes>
      </Router>
    }
}
