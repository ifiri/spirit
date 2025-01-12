use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Loader() -> Element {
  let nav = navigator();

  nav.push(Route::Home {});

  rsx! {
    div { "Loader" }
  }
}
