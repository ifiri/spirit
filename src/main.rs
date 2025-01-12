use dioxus::prelude::*;

use views::{Loader, Welcome, Home, Tasks, Friends, Wallet, Care};
use layouts::NavbarLayout;

mod components;
mod layouts;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Loader {},

    #[layout(NavbarLayout)]
      #[route("/i")]
      Home {},

    #[route("/welcome")]
    Welcome {},

    #[route("/tasks")]
    Tasks {},

    #[route("/Friends")]
    Friends {},

    #[route("/wallet")]
    Wallet {},

    #[route("/care")]
    Care {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
  // Build cool things ✌️

  rsx! {
    // Global app resources
    document::Link { rel: "icon", href: FAVICON }
    document::Link { rel: "stylesheet", href: MAIN_CSS }

    Router::<Route> {}
  }
}
