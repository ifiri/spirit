use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
  rsx! {
    div {
      class: "header",

      div {
        class: "name", "Hyeon",
      }

      div {
        "CryptoBoar",
      }

      div {
        class: "wallet", "8342hd...109234",
      }
    }
  
    div {
      class: "center",

      div {
        class: "character",
      }
    }
  }
}
