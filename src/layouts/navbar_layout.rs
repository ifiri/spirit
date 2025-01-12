use crate::Route;
use crate::components::Navbar;

use dioxus::prelude::*;

#[component]
pub fn NavbarLayout() -> Element {
  rsx! {
    div {
      class: "container",

      div {
        class: "wrapper",

        Outlet::<Route> {}

        Navbar {}
      }
    }
  }
}
