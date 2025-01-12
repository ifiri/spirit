use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
  rsx! {
    // document::Link { rel: "stylesheet", href: NAVBAR_CSS }

    div {
      class: "footer btn-group",

      Link {
        to: Route::Tasks {},
        class: "btn btn-primary", "Tasks",
      }
      Link {
        to: Route::Friends {},
        class: "btn btn-primary", "Friends",
      }
      Link {
        to: Route::Home {},
        class: "btn btn-primary", "Spirit",
      }
      Link {
        to: Route::Care {},
        class: "btn btn-primary", "Care",
      }
      Link {
        to: Route::Wallet {},
        class: "btn btn-primary", "Wallet",
      }
    }
  }
}
